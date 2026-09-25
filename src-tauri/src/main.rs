#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    db: Mutex<Connection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Campaign {
    pub id: i64,
    pub year_label: String,
    pub is_active: bool,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CampaignWithCount {
    pub id: i64,
    pub year_label: String,
    pub is_active: bool,
    pub created_at: Option<String>,
    pub record_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BeneficiaryRecord {
    pub id: Option<i64>, // campaign_record id
    pub record_no: Option<i64>, // Unique per campaign
    pub guardian_id: Option<i64>,
    pub guardian_name: String,
    pub phone: String,
    pub social_status: String,
    pub primary_count: i64,
    pub middle_count: i64,
    pub secondary_count: i64,
    pub is_delivered: Option<bool>,
    pub delivered_at: Option<String>,
    // Optional school form fields:
    #[serde(default)]
    pub birth_date: Option<String>,
    #[serde(default)]
    pub birth_place: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub marital_status: Option<String>,
    #[serde(default)]
    pub father_name: Option<String>,
    #[serde(default)]
    pub mother_name: Option<String>,
    #[serde(default)]
    pub spouse_name: Option<String>,
    #[serde(default)]
    pub monthly_income: Option<String>,
    #[serde(default)]
    pub children_count: Option<i64>,
    #[serde(default)]
    pub extra_priority_points: Option<i64>,
    #[serde(default)]
    pub photo_path: Option<String>,
    #[serde(default)]
    pub children: Vec<ChildRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChildRecord {
    pub id: Option<i64>,
    pub guardian_id: Option<i64>,
    pub child_name: String,
    pub birth_date: Option<String>,
    pub is_schooling: bool,
    pub education_level: Option<String>,
    #[serde(default)]
    pub education_level_id: Option<i64>,
    pub school_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BeneficiaryMatchResult {
    pub match_type: String, // "none" | "current_campaign" | "previous_campaign"
    pub message: String,
    pub matched_record: Option<BeneficiaryRecord>,
    pub previous_campaign_year: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Stats {
    pub total_families: i64,
    pub total_bags: i64,
    pub primary_total: i64,
    pub middle_total: i64,
    pub secondary_total: i64,
    pub primary_delivered: i64,
    pub middle_delivered: i64,
    pub secondary_delivered: i64,
    pub delivered_families: i64,
    pub pending_families: i64,
    pub delivered_bags: i64,
    pub pending_bags: i64,
    pub progress_percentage: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrgSettings {
    pub id: Option<i64>,
    pub org_name: String,
    pub branch_name: String,
    pub wilaya: String,
    pub commune: String,
    pub phone: String,
    pub footer_text: String,
    pub student_priority_points: Option<i64>,
    pub marital_points_widow: Option<i64>,
    pub marital_points_divorced: Option<i64>,
    pub marital_points_deserted: Option<i64>,
    pub marital_points_married: Option<i64>,
    pub marital_points_single: Option<i64>,
    pub marital_points_other: Option<i64>,
    pub priority_threshold_critical: Option<i64>,
    pub priority_threshold_high: Option<i64>,
    pub priority_threshold_medium: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SocialStatusItem {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub base_points: i64,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EducationLevel {
    pub id: i64,
    pub stage: String,
    pub year_name: String,
    pub year_order: i64,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EducationLevelWithCount {
    pub id: i64,
    pub stage: String,
    pub year_name: String,
    pub year_order: i64,
    pub count: i64,
    pub created_at: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct EducationLevelStat {
    pub id: i64,
    pub stage: String,
    pub year_name: String,
    pub year_order: i64,
    pub children_count: i64,
    pub delivered_count: i64,
    pub pending_count: i64,
}

#[tauri::command]
fn get_campaigns(state: State<AppState>) -> Result<Vec<Campaign>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, year_label, is_active, created_at FROM campaigns ORDER BY id DESC").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(Campaign {
            id: row.get(0)?,
            year_label: row.get(1)?,
            is_active: row.get::<_, i64>(2)? != 0,
            created_at: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for item in rows {
        list.push(item.map_err(|e| e.to_string())?);
    }
    Ok(list)
}

#[tauri::command]
fn get_campaigns_with_counts(state: State<AppState>) -> Result<Vec<CampaignWithCount>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT c.id, c.year_label, c.is_active, c.created_at,
                COUNT(r.id) as record_count
         FROM campaigns c
         LEFT JOIN campaign_records r ON r.campaign_id = c.id
         GROUP BY c.id
         ORDER BY c.id DESC"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(CampaignWithCount {
            id: row.get(0)?,
            year_label: row.get(1)?,
            is_active: row.get::<_, i64>(2)? != 0,
            created_at: row.get(3)?,
            record_count: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    for item in rows { list.push(item.map_err(|e| e.to_string())?); }
    Ok(list)
}

#[tauri::command]
fn get_all_record_numbers(state: State<AppState>, campaign_id: i64) -> Result<Vec<i64>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT record_no FROM campaign_records WHERE campaign_id = ?1 AND record_no IS NOT NULL ORDER BY record_no ASC").map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map(params![campaign_id], |row| {
        row.get::<_, i64>(0)
    }).map_err(|e| e.to_string())?;

    let mut numbers = Vec::new();
    for num in rows {
        if let Ok(n) = num {
            numbers.push(n);
        }
    }
    Ok(numbers)
}

#[derive(Serialize)]
pub struct SocialStatusStat {
    pub status: String,
    pub families_count: i64,
    pub bags_count: i64,
}

#[tauri::command]
fn get_social_status_stats(state: State<AppState>, campaign_id: i64) -> Result<Vec<SocialStatusStat>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT COALESCE(NULLIF(TRIM(g.social_status), ''), 'غير محدد') as status,
                COUNT(r.id) as families_count,
                COALESCE(SUM(COALESCE(r.primary_count, 0) + COALESCE(r.middle_count, 0) + COALESCE(r.secondary_count, 0)), 0) as bags_count
         FROM campaign_records r
         JOIN guardians g ON r.guardian_id = g.id
         WHERE r.campaign_id = ?1
         GROUP BY status
         ORDER BY families_count DESC"
    ).map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map(params![campaign_id], |row| {
        Ok(SocialStatusStat {
            status: row.get(0)?,
            families_count: row.get(1)?,
            bags_count: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut stats = Vec::new();
    for item in rows {
        stats.push(item.map_err(|e| e.to_string())?);
    }
    Ok(stats)
}

#[tauri::command]
fn update_campaign_label(state: State<AppState>, id: i64, new_label: String) -> Result<(), String> {
    let label = new_label.trim();
    if label.is_empty() {
        return Err("تسمية الموسم لا يمكن أن تكون فارغة".into());
    }
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE campaigns SET year_label = ?1 WHERE id = ?2",
        params![label, id],
    ).map_err(|e| format!("فشل تعديل اسم الموسم (قد يكون الاسم مكرراً): {}", e))?;
    Ok(())
}

#[tauri::command]
fn delete_campaign(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM campaign_records WHERE campaign_id = ?1",
        params![id],
        |r| r.get(0),
    ).unwrap_or(0);
    if count > 0 {
        return Err(format!(
            "لا يمكن حذف هذا الموسم لأنه يحتوي على {} مستفيد. احذف جميع سجلاتهم أولاً.",
            count
        ));
    }
    conn.execute("DELETE FROM campaigns WHERE id = ?1", params![id])
        .map_err(|e| format!("فشل حذف الموسم: {}", e))?;
    Ok(())
}

#[tauri::command]
fn set_active_campaign(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE campaigns SET is_active = 0", []).map_err(|e| e.to_string())?;
    conn.execute("UPDATE campaigns SET is_active = 1 WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn create_campaign(state: State<AppState>, year_label: String, rollover_from_id: Option<i64>) -> Result<Campaign, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let label = year_label.trim();
    if label.is_empty() {
        return Err("يرجى إدخال تسمية الموسم (مثال: 2026/2027)".into());
    }

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute("UPDATE campaigns SET is_active = 0", []).map_err(|e| e.to_string())?;

    tx.execute(
        "INSERT INTO campaigns (year_label, is_active) VALUES (?1, 1)",
        params![label],
    ).map_err(|e| format!("فشل في إنشاء الموسم (قد يكون الاسم مكرراً): {}", e))?;

    let new_campaign_id = tx.last_insert_rowid();

    if let Some(prev_id) = rollover_from_id {
        tx.execute(
            "INSERT OR IGNORE INTO campaign_records (campaign_id, guardian_id, record_no, primary_count, middle_count, secondary_count)
             SELECT ?1, guardian_id, record_no, primary_count, middle_count, secondary_count FROM campaign_records WHERE campaign_id = ?2",
            params![new_campaign_id, prev_id],
        ).map_err(|e| format!("فشل في ترحيل المستفيدين من الموسم السابق: {}", e))?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(Campaign {
        id: new_campaign_id,
        year_label: label.to_string(),
        is_active: true,
        created_at: None,
    })
}

#[tauri::command]
fn rollover_campaign_records(
    state: State<AppState>,
    from_campaign_id: i64,
    to_campaign_id: i64,
) -> Result<i64, String> {
    if from_campaign_id == to_campaign_id {
        return Err("لا يمكن الترحيل من الموسم إلى نفسه.".into());
    }

    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Max record_no already in target campaign
    let max_no: i64 = tx.query_row(
        "SELECT COALESCE(MAX(record_no), 0) FROM campaign_records WHERE campaign_id = ?1",
        params![to_campaign_id],
        |r| r.get(0),
    ).unwrap_or(0);

    // Fetch source records that are NOT yet in target (by guardian_id)
    let mut stmt = tx.prepare(
        "SELECT guardian_id, primary_count, middle_count, secondary_count
         FROM campaign_records
         WHERE campaign_id = ?1
           AND guardian_id NOT IN (
               SELECT guardian_id FROM campaign_records WHERE campaign_id = ?2
           )
         ORDER BY record_no ASC"
    ).map_err(|e| e.to_string())?;

    let records: Vec<(i64, i64, i64, i64)> = stmt
        .query_map(params![from_campaign_id, to_campaign_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    drop(stmt);

    let mut counter = max_no;
    for (guardian_id, primary_count, middle_count, secondary_count) in &records {
        counter += 1;
        tx.execute(
            "INSERT INTO campaign_records
                 (campaign_id, guardian_id, record_no, primary_count, middle_count, secondary_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![to_campaign_id, guardian_id, counter, primary_count, middle_count, secondary_count],
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(records.len() as i64)
}

#[tauri::command]
fn get_next_record_no(state: State<AppState>, campaign_id: i64) -> Result<i64, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let max_no: i64 = conn.query_row(
        "SELECT COALESCE(MAX(record_no), 0) FROM campaign_records WHERE campaign_id = ?1",
        params![campaign_id],
        |r| r.get(0),
    ).unwrap_or(0);
    Ok(max_no + 1)
}

#[tauri::command]
fn get_beneficiaries(state: State<AppState>, campaign_id: i64, search: String, status_filter: String) -> Result<Vec<BeneficiaryRecord>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut query = String::from(
        "SELECT r.id, g.id, g.guardian_name, g.phone, g.social_status, 
                r.primary_count, r.middle_count, r.secondary_count, r.record_no,
                COALESCE(r.is_delivered, 0), r.delivered_at,
                g.birth_date, g.birth_place, g.address, g.marital_status,
                g.father_name, g.mother_name, g.spouse_name, g.monthly_income, g.children_count,
                COALESCE(r.extra_priority_points, 0), g.photo_path
         FROM campaign_records r
         JOIN guardians g ON r.guardian_id = g.id
         WHERE r.campaign_id = ?1 AND (g.guardian_name LIKE ?2 OR g.phone LIKE ?2 OR CAST(r.record_no AS TEXT) LIKE ?2)"
    );

    let pattern = format!("%{}%", search);
    let filter_active = !status_filter.is_empty() && status_filter != "الكل";

    if filter_active {
        query.push_str(" AND g.social_status = ?3");
    }
    query.push_str(" ORDER BY r.record_no ASC, r.id ASC");

    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let mut list = Vec::new();

    let mapper = |row: &rusqlite::Row| -> rusqlite::Result<BeneficiaryRecord> {
        let is_del_int: i64 = row.get(9)?;
        Ok(BeneficiaryRecord {
            children: Vec::new(),
            id: Some(row.get(0)?),
            guardian_id: Some(row.get(1)?),
            guardian_name: row.get(2)?,
            phone: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            social_status: row.get(4)?,
            primary_count: row.get(5)?,
            middle_count: row.get(6)?,
            secondary_count: row.get(7)?,
            record_no: row.get(8)?,
            is_delivered: Some(is_del_int != 0),
            delivered_at: row.get(10)?,
            birth_date: row.get(11)?,
            birth_place: row.get(12)?,
            address: row.get(13)?,
            marital_status: row.get(14)?,
            father_name: row.get(15)?,
            mother_name: row.get(16)?,
            spouse_name: row.get(17)?,
            monthly_income: row.get(18)?,
            children_count: row.get(19)?,
            extra_priority_points: row.get::<_, Option<i64>>(20)?,
            photo_path: row.get(21)?,
        })
    };

    if filter_active {
        let rows = stmt.query_map(params![campaign_id, pattern, status_filter], mapper).map_err(|e| e.to_string())?;
        for item in rows {
            list.push(item.map_err(|e| e.to_string())?);
        }
    } else {
        let rows = stmt.query_map(params![campaign_id, pattern], mapper).map_err(|e| e.to_string())?;
        for item in rows {
            list.push(item.map_err(|e| e.to_string())?);
        }
    }

    Ok(list)
}

#[tauri::command]
fn get_guardian_children(state: State<AppState>, guardian_id: i64) -> Result<Vec<ChildRecord>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT gc.id, gc.guardian_id, gc.child_name, gc.birth_date, gc.is_schooling, 
                COALESCE(el.year_name, gc.education_level) as education_level, 
                gc.school_name, gc.education_level_id
         FROM guardian_children gc
         LEFT JOIN education_levels el ON gc.education_level_id = el.id
         WHERE gc.guardian_id = ?1 
         ORDER BY gc.id ASC"
    ).map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map(params![guardian_id], |row| {
        Ok(ChildRecord {
            id: Some(row.get(0)?),
            guardian_id: Some(row.get(1)?),
            child_name: row.get(2)?,
            birth_date: row.get(3)?,
            is_schooling: row.get::<_, i64>(4)? != 0,
            education_level: row.get(5)?,
            education_level_id: row.get(7)?,
            school_name: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for item in rows {
        list.push(item.map_err(|e| e.to_string())?);
    }
    Ok(list)
}

#[tauri::command]
fn check_beneficiary_match(
    state: State<AppState>,
    campaign_id: i64,
    guardian_name: String,
    birth_date: Option<String>,
    father_name: Option<String>,
    mother_name: Option<String>,
) -> Result<BeneficiaryMatchResult, String> {
    let name = guardian_name.trim();
    if name.chars().count() < 3 {
        return Ok(BeneficiaryMatchResult {
            match_type: "none".into(),
            message: String::new(),
            matched_record: None,
            previous_campaign_year: None,
        });
    }

    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let b_date = birth_date.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let f_name = father_name.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let m_name = mother_name.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

    // Search guardians with exact name or containing name
    let mut stmt = conn.prepare(
        "SELECT id, guardian_name, phone, social_status, birth_date, birth_place, address,
                marital_status, father_name, mother_name, spouse_name, monthly_income, children_count, photo_path
         FROM guardians
         WHERE TRIM(guardian_name) = ?1 OR TRIM(guardian_name) LIKE ?2"
    ).map_err(|e| e.to_string())?;

    let like_pattern = format!("%{}%", name);
    let rows = stmt.query_map(params![name, like_pattern], |row| {
        Ok(BeneficiaryRecord {
            children: Vec::new(),
            id: None,
            record_no: None,
            guardian_id: Some(row.get(0)?),
            guardian_name: row.get(1)?,
            phone: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            social_status: row.get(3)?,
            primary_count: 0,
            middle_count: 0,
            secondary_count: 0,
            is_delivered: None,
            delivered_at: None,
            birth_date: row.get(4)?,
            birth_place: row.get(5)?,
            address: row.get(6)?,
            marital_status: row.get(7)?,
            father_name: row.get(8)?,
            mother_name: row.get(9)?,
            spouse_name: row.get(10)?,
            monthly_income: row.get(11)?,
            children_count: row.get(12)?,
            extra_priority_points: Some(0),
            photo_path: row.get(13)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut candidates = Vec::new();
    for r in rows {
        if let Ok(rec) = r {
            // Check compatibility if details are provided
            let mut mismatch = false;
            if let (Some(b1), Some(b2)) = (&b_date, rec.birth_date.as_deref()) {
                if !b1.is_empty() && !b2.is_empty() && b1 != b2 { mismatch = true; }
            }
            if let (Some(f1), Some(f2)) = (&f_name, rec.father_name.as_deref()) {
                if !f1.is_empty() && !f2.is_empty() && f1 != f2 { mismatch = true; }
            }
            if let (Some(m1), Some(m2)) = (&m_name, rec.mother_name.as_deref()) {
                if !m1.is_empty() && !m2.is_empty() && m1 != m2 { mismatch = true; }
            }
            if !mismatch {
                candidates.push(rec);
            }
        }
    }

    if candidates.is_empty() {
        return Ok(BeneficiaryMatchResult {
            match_type: "none".into(),
            message: String::new(),
            matched_record: None,
            previous_campaign_year: None,
        });
    }

    // 1. Check candidates in CURRENT campaign first (Duplicate check)
    for cand in &candidates {
        let gid = cand.guardian_id.unwrap();
        let curr: Option<(i64, i64, i64, i64, i64)> = conn.query_row(
            "SELECT id, record_no, primary_count, middle_count, secondary_count
             FROM campaign_records
             WHERE campaign_id = ?1 AND guardian_id = ?2
             LIMIT 1",
            params![campaign_id, gid],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        ).optional().map_err(|e| e.to_string())?;

        if let Some((rid, rec_no, p, m, s)) = curr {
            let mut matched = cand.clone();
            matched.id = Some(rid);
            matched.record_no = Some(rec_no);
            matched.primary_count = p;
            matched.middle_count = m;
            matched.secondary_count = s;
            matched.extra_priority_points = conn.query_row(
                "SELECT COALESCE(extra_priority_points, 0) FROM campaign_records WHERE id = ?1",
                params![rid],
                |r| r.get(0)
            ).ok();

            return Ok(BeneficiaryMatchResult {
                match_type: "current_campaign".into(),
                message: format!(
                    "المستفيد '{}' مسجل بالفعل في هذا الموسم الحالي برقم ({}). لا يمكن تكرار إدخاله مرتين.",
                    cand.guardian_name, rec_no
                ),
                matched_record: Some(matched),
                previous_campaign_year: None,
            });
        }
    }

    // 2. Check candidates in PREVIOUS / other campaigns (Rollover proposal)
    for cand in &candidates {
        let gid = cand.guardian_id.unwrap();
        let prev: Option<(i64, String, i64, i64, i64, i64)> = conn.query_row(
            "SELECT c.id, c.year_label, r.primary_count, r.middle_count, r.secondary_count, r.record_no
             FROM campaign_records r
             JOIN campaigns c ON r.campaign_id = c.id
             WHERE r.guardian_id = ?1 AND r.campaign_id != ?2
             ORDER BY c.id DESC
             LIMIT 1",
            params![gid, campaign_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
        ).optional().map_err(|e| e.to_string())?;

        if let Some((_prev_cid, year_label, p, m, s, _prev_rec_no)) = prev {
            let mut matched = cand.clone();
            matched.id = None;
            matched.record_no = None;
            matched.primary_count = p;
            matched.middle_count = m;
            matched.secondary_count = s;

            return Ok(BeneficiaryMatchResult {
                match_type: "previous_campaign".into(),
                message: format!(
                    "تم العثور على نفس المستفيد '{}' مسجلاً في موسم سابق ({}). يمكنك ترحيله واستيراد كافة بياناته لهذا الموسم.",
                    cand.guardian_name, year_label
                ),
                matched_record: Some(matched),
                previous_campaign_year: Some(year_label),
            });
        }
    }

    Ok(BeneficiaryMatchResult {
        match_type: "none".into(),
        message: String::new(),
        matched_record: None,
        previous_campaign_year: None,
    })
}

#[tauri::command]
fn save_beneficiary(state: State<AppState>, campaign_id: i64, payload: BeneficiaryRecord) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let name = payload.guardian_name.trim();
    if name.is_empty() {
        return Err("اسم ولي الأمر مطلوب".into());
    }

    let rec_no = match payload.record_no {
        Some(n) if n > 0 => n,
        _ => {
            let max_no: i64 = tx.query_row(
                "SELECT COALESCE(MAX(record_no), 0) FROM campaign_records WHERE campaign_id = ?1",
                params![campaign_id],
                |r| r.get(0),
            ).unwrap_or(0);
            max_no + 1
        }
    };

    // Check if record_no is already taken by another record in this campaign
    let conflict: Option<i64> = if let Some(rid) = payload.id {
        tx.query_row(
            "SELECT id FROM campaign_records WHERE campaign_id = ?1 AND record_no = ?2 AND id != ?3 LIMIT 1",
            params![campaign_id, rec_no, rid],
            |r| r.get(0),
        ).optional().map_err(|e| e.to_string())?
    } else {
        tx.query_row(
            "SELECT id FROM campaign_records WHERE campaign_id = ?1 AND record_no = ?2 LIMIT 1",
            params![campaign_id, rec_no],
            |r| r.get(0),
        ).optional().map_err(|e| e.to_string())?
    };

    if conflict.is_some() {
        return Err(format!("الرقم ({}) مسجل بالفعل لمستفيد آخر في هذا الموسم. يرجى اختيار رقم فريد.", rec_no));
    }

    let status = if !payload.social_status.trim().is_empty() {
        payload.social_status.trim().to_string()
    } else {
        "بدون دخل".to_string()
    };

    let birth_date = payload.birth_date.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let birth_place = payload.birth_place.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let address = payload.address.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let marital_status = payload.marital_status.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let father_name = payload.father_name.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let mother_name = payload.mother_name.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let spouse_name = payload.spouse_name.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let monthly_income = payload.monthly_income.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let children_count = payload.children_count;
    let photo_path = payload.photo_path.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

    // Check duplicate in current campaign if creating a new record (payload.id is None)
    if payload.id.is_none() {
        if let Some(gid) = payload.guardian_id {
            let already_in_campaign: Option<i64> = tx.query_row(
                "SELECT record_no FROM campaign_records WHERE campaign_id = ?1 AND guardian_id = ?2 LIMIT 1",
                params![campaign_id, gid],
                |r| r.get(0),
            ).optional().map_err(|e| e.to_string())?;

            if let Some(existing_no) = already_in_campaign {
                return Err(format!("المستفيد '{}' مسجل بالفعل في هذا الموسم برقم ({}). لا يمكن تكرار إدخاله مرتين.", name, existing_no));
            }
        } else {
            // Check if a guardian with same name (and matching details) is already in this campaign
            let mut stmt = tx.prepare(
                "SELECT g.id, g.birth_date, g.father_name, g.mother_name, r.record_no
                 FROM guardians g
                 JOIN campaign_records r ON r.guardian_id = g.id
                 WHERE r.campaign_id = ?1 AND TRIM(g.guardian_name) = ?2"
            ).map_err(|e| e.to_string())?;

            let rows = stmt.query_map(params![campaign_id, name], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, i64>(4)?,
                ))
            }).map_err(|e| e.to_string())?;

            for item in rows {
                if let Ok((_gid, cand_bdate, cand_fname, cand_mname, cand_rec_no)) = item {
                    let mut mismatch = false;
                    if let (Some(b1), Some(b2)) = (&birth_date, &cand_bdate) {
                        if !b1.is_empty() && !b2.is_empty() && b1 != b2 { mismatch = true; }
                    }
                    if let (Some(f1), Some(f2)) = (&father_name, &cand_fname) {
                        if !f1.is_empty() && !f2.is_empty() && f1 != f2 { mismatch = true; }
                    }
                    if let (Some(m1), Some(m2)) = (&mother_name, &cand_mname) {
                        if !m1.is_empty() && !m2.is_empty() && m1 != m2 { mismatch = true; }
                    }
                    if !mismatch {
                        return Err(format!("المستفيد '{}' مسجل بالفعل في هذا الموسم برقم ({}). لا يمكن تكرار إدخاله مرتين.", name, cand_rec_no));
                    }
                }
            }
        }
    }

    let guardian_id = if let Some(gid) = payload.guardian_id {
        tx.execute(
            "UPDATE guardians SET guardian_name = ?1, phone = ?2, social_status = ?3,
             birth_date = ?4, birth_place = ?5, address = ?6, marital_status = ?7,
             father_name = ?8, mother_name = ?9, spouse_name = ?10, monthly_income = ?11, children_count = ?12,
             photo_path = ?13
             WHERE id = ?14",
            params![name, payload.phone.trim(), status, birth_date, birth_place, address, marital_status, father_name, mother_name, spouse_name, monthly_income, children_count, photo_path, gid],
        ).map_err(|e| e.to_string())?;
        gid
    } else {
        // Try finding existing guardian by name (and matching details)
        let mut stmt = tx.prepare(
            "SELECT id, birth_date, father_name, mother_name FROM guardians WHERE TRIM(guardian_name) = ?1"
        ).map_err(|e| e.to_string())?;
        let candidates: Vec<(i64, Option<String>, Option<String>, Option<String>)> = stmt.query_map(params![name], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

        let matched_gid = candidates.into_iter().find(|(_gid, cand_bdate, cand_fname, cand_mname)| {
            if let (Some(b1), Some(b2)) = (&birth_date, cand_bdate) {
                if !b1.is_empty() && !b2.is_empty() && b1 != b2 { return false; }
            }
            if let (Some(f1), Some(f2)) = (&father_name, cand_fname) {
                if !f1.is_empty() && !f2.is_empty() && f1 != f2 { return false; }
            }
            if let (Some(m1), Some(m2)) = (&mother_name, cand_mname) {
                if !m1.is_empty() && !m2.is_empty() && m1 != m2 { return false; }
            }
            true
        }).map(|(gid, _, _, _)| gid);

        if let Some(gid) = matched_gid {
            tx.execute(
                "UPDATE guardians SET phone = ?1, social_status = ?2,
                 birth_date = COALESCE(?3, birth_date),
                 birth_place = COALESCE(?4, birth_place),
                 address = COALESCE(?5, address),
                 marital_status = COALESCE(?6, marital_status),
                 father_name = COALESCE(?7, father_name),
                 mother_name = COALESCE(?8, mother_name),
                 spouse_name = COALESCE(?9, spouse_name),
                 monthly_income = COALESCE(?10, monthly_income),
                 children_count = COALESCE(?11, children_count),
                 photo_path = COALESCE(?12, photo_path)
                 WHERE id = ?13",
                params![payload.phone.trim(), status, birth_date, birth_place, address, marital_status, father_name, mother_name, spouse_name, monthly_income, children_count, photo_path, gid],
            ).map_err(|e| e.to_string())?;
            gid
        } else {
            tx.execute(
                "INSERT INTO guardians (guardian_name, phone, social_status, birth_date, birth_place, address, marital_status, father_name, mother_name, spouse_name, monthly_income, children_count, photo_path)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![name, payload.phone.trim(), status, birth_date, birth_place, address, marital_status, father_name, mother_name, spouse_name, monthly_income, children_count, photo_path],
            ).map_err(|e| e.to_string())?;
            tx.last_insert_rowid()
        }
    };

    let extra_pts = payload.extra_priority_points.unwrap_or(0);

    if let Some(rid) = payload.id {
        tx.execute(
            "UPDATE campaign_records SET record_no = ?1, primary_count = ?2, middle_count = ?3, secondary_count = ?4, extra_priority_points = ?5 WHERE id = ?6 AND campaign_id = ?7",
            params![rec_no, payload.primary_count.max(0), payload.middle_count.max(0), payload.secondary_count.max(0), extra_pts, rid, campaign_id],
        ).map_err(|e| e.to_string())?;
    } else {
        tx.execute(
            "INSERT INTO campaign_records (campaign_id, guardian_id, record_no, primary_count, middle_count, secondary_count, extra_priority_points)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![campaign_id, guardian_id, rec_no, payload.primary_count.max(0), payload.middle_count.max(0), payload.secondary_count.max(0), extra_pts],
        ).map_err(|e| e.to_string())?;
    }

    // Save children records
    tx.execute("DELETE FROM guardian_children WHERE guardian_id = ?1", params![guardian_id]).map_err(|e| e.to_string())?;
    for child in &payload.children {
        if !child.child_name.trim().is_empty() {
            // Resolve education_level text from education_level_id if available
            let edu_level_text = if let Some(el_id) = child.education_level_id {
                tx.query_row(
                    "SELECT year_name FROM education_levels WHERE id = ?1",
                    params![el_id],
                    |r| r.get::<_, String>(0),
                ).ok().or_else(|| child.education_level.as_ref().map(|s| s.trim().to_string()))
            } else {
                child.education_level.as_ref().map(|s| s.trim().to_string())
            };
            tx.execute(
                "INSERT INTO guardian_children (guardian_id, child_name, birth_date, is_schooling, education_level, education_level_id, school_name) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![guardian_id, child.child_name.trim(), child.birth_date.as_deref().map(|s| s.trim()), child.is_schooling, edu_level_text, child.education_level_id, child.school_name.as_deref().map(|s| s.trim())],
            ).map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn delete_beneficiary(state: State<AppState>, record_id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM campaign_records WHERE id = ?1", params![record_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn bulk_delete_beneficiaries(state: State<AppState>, record_ids: Vec<i64>) -> Result<usize, String> {
    if record_ids.is_empty() {
        return Ok(0);
    }
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let mut count = 0;
    {
        let mut stmt = tx.prepare("DELETE FROM campaign_records WHERE id = ?1").map_err(|e| e.to_string())?;
        for id in &record_ids {
            count += stmt.execute(params![id]).map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(count)
}

#[tauri::command]
fn get_stats(state: State<AppState>, campaign_id: i64) -> Result<Stats, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stats = Stats::default();

    stats.total_families = conn.query_row(
        "SELECT COUNT(*) FROM campaign_records WHERE campaign_id = ?1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.primary_total = conn.query_row(
        "SELECT COALESCE(SUM(primary_count), 0) FROM campaign_records WHERE campaign_id = ?1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.middle_total = conn.query_row(
        "SELECT COALESCE(SUM(middle_count), 0) FROM campaign_records WHERE campaign_id = ?1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.secondary_total = conn.query_row(
        "SELECT COALESCE(SUM(secondary_count), 0) FROM campaign_records WHERE campaign_id = ?1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.total_bags = stats.primary_total + stats.middle_total + stats.secondary_total;

    stats.delivered_families = conn.query_row(
        "SELECT COUNT(*) FROM campaign_records WHERE campaign_id = ?1 AND is_delivered = 1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.pending_families = stats.total_families.saturating_sub(stats.delivered_families);

    stats.delivered_bags = conn.query_row(
        "SELECT COALESCE(SUM(primary_count + middle_count + secondary_count), 0) 
         FROM campaign_records WHERE campaign_id = ?1 AND is_delivered = 1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.pending_bags = stats.total_bags.saturating_sub(stats.delivered_bags);

    stats.progress_percentage = if stats.total_bags > 0 {
        ((stats.delivered_bags as f64 / stats.total_bags as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };

    stats.primary_delivered = conn.query_row(
        "SELECT COALESCE(SUM(primary_count), 0) FROM campaign_records WHERE campaign_id = ?1 AND is_delivered = 1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.middle_delivered = conn.query_row(
        "SELECT COALESCE(SUM(middle_count), 0) FROM campaign_records WHERE campaign_id = ?1 AND is_delivered = 1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    stats.secondary_delivered = conn.query_row(
        "SELECT COALESCE(SUM(secondary_count), 0) FROM campaign_records WHERE campaign_id = ?1 AND is_delivered = 1",
        params![campaign_id],
        |r| r.get(0)
    ).unwrap_or(0);

    Ok(stats)
}

#[tauri::command]
fn toggle_delivery_status(state: State<AppState>, record_id: i64) -> Result<(bool, Option<String>), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let current_delivered: i64 = conn.query_row(
        "SELECT COALESCE(is_delivered, 0) FROM campaign_records WHERE id = ?1",
        params![record_id],
        |r| r.get(0),
    ).map_err(|e| format!("السجل غير موجود: {}", e))?;

    let new_delivered = current_delivered == 0;
    if new_delivered {
        conn.execute(
            "UPDATE campaign_records 
             SET is_delivered = 1, 
                 delivered_at = strftime('%Y-%m-%d %H:%M', 'now', 'localtime') 
             WHERE id = ?1",
            params![record_id],
        ).map_err(|e| e.to_string())?;
        let timestamp: Option<String> = conn.query_row(
            "SELECT delivered_at FROM campaign_records WHERE id = ?1",
            params![record_id],
            |r| r.get(0),
        ).optional().unwrap_or(None);
        Ok((true, timestamp))
    } else {
        conn.execute(
            "UPDATE campaign_records SET is_delivered = 0, delivered_at = NULL WHERE id = ?1",
            params![record_id],
        ).map_err(|e| e.to_string())?;
        Ok((false, None))
    }
}

#[tauri::command]
fn bulk_set_delivery_status(state: State<AppState>, record_ids: Vec<i64>, is_delivered: bool) -> Result<usize, String> {
    if record_ids.is_empty() {
        return Ok(0);
    }
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let mut count = 0;
    {
        if is_delivered {
            let mut stmt = tx.prepare(
                "UPDATE campaign_records 
                 SET is_delivered = 1, 
                     delivered_at = COALESCE(delivered_at, strftime('%Y-%m-%d %H:%M', 'now', 'localtime')) 
                 WHERE id = ?1"
            ).map_err(|e| e.to_string())?;
            for id in &record_ids {
                count += stmt.execute(params![id]).map_err(|e| e.to_string())?;
            }
        } else {
            let mut stmt = tx.prepare(
                "UPDATE campaign_records SET is_delivered = 0, delivered_at = NULL WHERE id = ?1"
            ).map_err(|e| e.to_string())?;
            for id in &record_ids {
                count += stmt.execute(params![id]).map_err(|e| e.to_string())?;
            }
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(count)
}

#[tauri::command]
fn import_beneficiaries(state: State<AppState>, campaign_id: i64, items: Vec<BeneficiaryRecord>) -> Result<usize, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let mut count = 0;
    {
        for item in items {
            let name = item.guardian_name.trim();
            if name.is_empty() {
                continue;
            }

            let status = if !item.social_status.trim().is_empty() {
                item.social_status.trim().to_string()
            } else {
                "بدون دخل".to_string()
            };

            let existing: Option<i64> = tx.query_row(
                "SELECT id FROM guardians WHERE guardian_name = ?1 LIMIT 1",
                params![name],
                |r| r.get(0),
            ).optional().map_err(|e| e.to_string())?;

            let guardian_id = if let Some(gid) = existing {
                if !item.phone.trim().is_empty() {
                    let _ = tx.execute("UPDATE guardians SET phone = ?1 WHERE id = ?2", params![item.phone.trim(), gid]);
                }
                gid
            } else {
                tx.execute(
                    "INSERT INTO guardians (guardian_name, phone, social_status) VALUES (?1, ?2, ?3)",
                    params![name, item.phone.trim(), status],
                ).map_err(|e| e.to_string())?;
                tx.last_insert_rowid()
            };

            let rec_no = if let Some(nr) = item.record_no {
                if nr > 0 {
                    let exists: bool = tx.query_row(
                        "SELECT COUNT(*) > 0 FROM campaign_records WHERE campaign_id = ?1 AND record_no = ?2",
                        params![campaign_id, nr],
                        |r| r.get(0),
                    ).unwrap_or(false);
                    if !exists {
                        nr
                    } else {
                        let next_val: i64 = tx.query_row(
                            "SELECT COALESCE(MAX(record_no), 0) FROM campaign_records WHERE campaign_id = ?1",
                            params![campaign_id],
                            |r| r.get(0),
                        ).unwrap_or(0);
                        next_val + 1
                    }
                } else {
                    let next_val: i64 = tx.query_row(
                        "SELECT COALESCE(MAX(record_no), 0) FROM campaign_records WHERE campaign_id = ?1",
                        params![campaign_id],
                        |r| r.get(0),
                    ).unwrap_or(0);
                    next_val + 1
                }
            } else {
                let next_val: i64 = tx.query_row(
                    "SELECT COALESCE(MAX(record_no), 0) FROM campaign_records WHERE campaign_id = ?1",
                    params![campaign_id],
                    |r| r.get(0),
                ).unwrap_or(0);
                next_val + 1
            };

            tx.execute(
                "INSERT INTO campaign_records (campaign_id, guardian_id, record_no, primary_count, middle_count, secondary_count)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(campaign_id, guardian_id) DO UPDATE SET
                 primary_count = excluded.primary_count,
                 middle_count = excluded.middle_count,
                 secondary_count = excluded.secondary_count",
                params![campaign_id, guardian_id, rec_no, item.primary_count.max(0), item.middle_count.max(0), item.secondary_count.max(0)],
            ).map_err(|e| e.to_string())?;

            count += 1;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(count)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ValidatedImportItem {
    pub guardian_name: String,
    pub phone: String,
    pub social_status_id: i64,
    pub primary_count: i64,
    pub middle_count: i64,
    pub secondary_count: i64,
}

#[tauri::command]
fn safe_import_records(
    state: State<AppState>,
    campaign_id: i64,
    records: Vec<ValidatedImportItem>,
) -> Result<usize, String> {
    if records.is_empty() {
        return Ok(0);
    }

    // 1. إنشاء نسخة احتياطية فورية قبل بدء المعاملة
    let db_path = get_db_path();
    let backup_path = db_path.with_extension("db.pre_import");
    if db_path.exists() {
        let _ = std::fs::copy(&db_path, &backup_path);
    }

    let mut conn = state.db.lock().map_err(|e| e.to_string())?;

    // 2. التحقق من وجود الموسم الدراسي المستهدف
    let campaign_exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM campaigns WHERE id = ?1",
            rusqlite::params![campaign_id],
            |r| r.get(0),
        )
        .map_err(|e| format!("الموسم الدراسي غير موجود: {}", e))?;

    if !campaign_exists {
        return Err("معرّف الموسم الدراسي غير صالح أو تم حذفه".into());
    }

    // 3. بدء المعاملة الذرية (ACID Transaction)
    let tx = conn.transaction().map_err(|e| format!("فشل بدء المعاملة: {}", e))?;

    let mut success_count = 0;
    {
        for r in records {
            let clean_name = r.guardian_name.trim();
            if clean_name.is_empty() {
                continue;
            }

            let clean_phone = r.phone.trim();
            let p_count = r.primary_count.clamp(0, 20);
            let m_count = r.middle_count.clamp(0, 20);
            let s_count = r.secondary_count.clamp(0, 20);

            // استرجاع اسم الحالة الاجتماعية الموافقة للمعرف
            let status_name: String = tx.query_row(
                "SELECT name FROM social_statuses WHERE id = ?1",
                rusqlite::params![r.social_status_id],
                |row| row.get(0),
            ).unwrap_or_else(|_| "بدون دخل".to_string());

            // البحث عن الولي بالاسم والهاتف لتفادي تكراره في دليل الأولياء العام
            let guardian_id: i64 = match tx.query_row(
                "SELECT id FROM guardians WHERE guardian_name = ?1 AND (phone = ?2 OR ?2 = '') LIMIT 1",
                rusqlite::params![clean_name, clean_phone],
                |row| row.get(0),
            ) {
                Ok(existing_id) => {
                    tx.execute(
                        "UPDATE guardians SET phone = ?1, social_status = ?2, social_status_id = ?3 WHERE id = ?4",
                        rusqlite::params![clean_phone, status_name, r.social_status_id, existing_id],
                    ).map_err(|e| e.to_string())?;
                    existing_id
                },
                Err(_) => {
                    tx.execute(
                        "INSERT INTO guardians (guardian_name, phone, social_status, social_status_id) VALUES (?1, ?2, ?3, ?4)",
                        rusqlite::params![clean_name, clean_phone, status_name, r.social_status_id],
                    ).map_err(|e| format!("خطأ في إضافة ولي الأمر: {}", e))?;
                    tx.last_insert_rowid()
                }
            };

            let next_no: i64 = tx.query_row(
                "SELECT COALESCE(MAX(record_no), 0) FROM campaign_records WHERE campaign_id = ?1",
                rusqlite::params![campaign_id],
                |r| r.get(0),
            ).unwrap_or(0) + 1;

            // إدراج أو تحديث سجل التوزيع السنوي للموسم المختار
            tx.execute(
                "INSERT INTO campaign_records (campaign_id, guardian_id, record_no, primary_count, middle_count, secondary_count)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(campaign_id, guardian_id) DO UPDATE SET
                    primary_count = excluded.primary_count,
                    middle_count = excluded.middle_count,
                    secondary_count = excluded.secondary_count",
                rusqlite::params![campaign_id, guardian_id, next_no, p_count, m_count, s_count],
            ).map_err(|e| format!("خطأ في تسجيل بيانات التوزيع السنوي: {}", e))?;

            success_count += 1;
        }
    }

    // 4. تأكيد المعاملة النهائي
    tx.commit().map_err(|e| {
        if backup_path.exists() {
            let _ = std::fs::copy(&backup_path, &db_path);
        }
        format!("فشل تثبيت البيانات وتم التراجع عن كافة التغييرات: {}", e)
    })?;

    // حذف ملف النسخة المؤقتة بعد النجاح
    if backup_path.exists() {
        let _ = std::fs::remove_file(&backup_path);
    }

    Ok(success_count)
}

#[tauri::command]
fn batch_import_records(state: State<AppState>, campaign_id: i64, records: Vec<BeneficiaryRecord>) -> Result<usize, String> {
    import_beneficiaries(state, campaign_id, records)
}

#[tauri::command]
fn get_org_settings(state: State<AppState>) -> Result<OrgSettings, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let settings = conn.query_row(
        "SELECT id, org_name, branch_name, wilaya, commune, phone, footer_text, student_priority_points,
                marital_points_widow, marital_points_divorced, marital_points_deserted, marital_points_married,
                marital_points_single, marital_points_other, priority_threshold_critical, priority_threshold_high,
                priority_threshold_medium
         FROM organization_settings WHERE id = 1",
        [],
        |row| {
            Ok(OrgSettings {
                id: Some(row.get(0)?),
                org_name: row.get(1)?,
                branch_name: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                wilaya: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                commune: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                phone: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                footer_text: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                student_priority_points: row.get::<_, Option<i64>>(7)?.or(Some(5)),
                marital_points_widow: row.get::<_, Option<i64>>(8)?.or(Some(30)),
                marital_points_divorced: row.get::<_, Option<i64>>(9)?.or(Some(20)),
                marital_points_deserted: row.get::<_, Option<i64>>(10)?.or(Some(25)),
                marital_points_married: row.get::<_, Option<i64>>(11)?.or(Some(10)),
                marital_points_single: row.get::<_, Option<i64>>(12)?.or(Some(5)),
                marital_points_other: row.get::<_, Option<i64>>(13)?.or(Some(5)),
                priority_threshold_critical: row.get::<_, Option<i64>>(14)?.or(Some(60)),
                priority_threshold_high: row.get::<_, Option<i64>>(15)?.or(Some(45)),
                priority_threshold_medium: row.get::<_, Option<i64>>(16)?.or(Some(30)),
            })
        },
    ).optional().map_err(|e| e.to_string())?;

    Ok(settings.unwrap_or_else(|| OrgSettings {
        id: Some(1),
        org_name: "الجمعية الخيرية لرعاية الأيتام والمحتاجين".into(),
        branch_name: "المكتب الولائي".into(),
        wilaya: "قسنطينة".into(),
        commune: "".into(),
        phone: "".into(),
        footer_text: "وثيقة إدارية داخلية مخصصة لضبط عملية التوزيع.".into(),
        student_priority_points: Some(5),
        marital_points_widow: Some(30),
        marital_points_divorced: Some(20),
        marital_points_deserted: Some(25),
        marital_points_married: Some(10),
        marital_points_single: Some(5),
        marital_points_other: Some(5),
        priority_threshold_critical: Some(60),
        priority_threshold_high: Some(45),
        priority_threshold_medium: Some(30),
    }))
}

#[tauri::command]
fn save_org_settings(state: State<AppState>, settings: OrgSettings) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let pts = settings.student_priority_points.unwrap_or(5);
    let pts_widow = settings.marital_points_widow.unwrap_or(30);
    let pts_divorced = settings.marital_points_divorced.unwrap_or(20);
    let pts_deserted = settings.marital_points_deserted.unwrap_or(25);
    let pts_married = settings.marital_points_married.unwrap_or(10);
    let pts_single = settings.marital_points_single.unwrap_or(5);
    let pts_other = settings.marital_points_other.unwrap_or(5);
    let th_crit = settings.priority_threshold_critical.unwrap_or(60);
    let th_high = settings.priority_threshold_high.unwrap_or(45);
    let th_med = settings.priority_threshold_medium.unwrap_or(30);

    conn.execute(
        "INSERT INTO organization_settings (
            id, org_name, branch_name, wilaya, commune, phone, footer_text,
            student_priority_points, marital_points_widow, marital_points_divorced,
            marital_points_deserted, marital_points_married, marital_points_single,
            marital_points_other, priority_threshold_critical, priority_threshold_high,
            priority_threshold_medium
         )
         VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
         ON CONFLICT(id) DO UPDATE SET
         org_name = excluded.org_name,
         branch_name = excluded.branch_name,
         wilaya = excluded.wilaya,
         commune = excluded.commune,
         phone = excluded.phone,
         footer_text = excluded.footer_text,
         student_priority_points = excluded.student_priority_points,
         marital_points_widow = excluded.marital_points_widow,
         marital_points_divorced = excluded.marital_points_divorced,
         marital_points_deserted = excluded.marital_points_deserted,
         marital_points_married = excluded.marital_points_married,
         marital_points_single = excluded.marital_points_single,
         marital_points_other = excluded.marital_points_other,
         priority_threshold_critical = excluded.priority_threshold_critical,
         priority_threshold_high = excluded.priority_threshold_high,
         priority_threshold_medium = excluded.priority_threshold_medium",
        params![
            settings.org_name.trim(),
            settings.branch_name.trim(),
            settings.wilaya.trim(),
            settings.commune.trim(),
            settings.phone.trim(),
            settings.footer_text.trim(),
            pts,
            pts_widow,
            pts_divorced,
            pts_deserted,
            pts_married,
            pts_single,
            pts_other,
            th_crit,
            th_high,
            th_med,
        ],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn write_binary_file(path: String, contents: Vec<u8>) -> Result<(), String> {
    std::fs::write(&path, contents).map_err(|e| format!("فشل حفظ الملف: {}", e))
}

#[tauri::command]
fn get_social_statuses(state: State<AppState>) -> Result<Vec<SocialStatusItem>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT s.id, s.name, COUNT(g.id) as count, s.base_points, s.created_at
         FROM social_statuses s
         LEFT JOIN guardians g ON g.social_status = s.name
         GROUP BY s.id, s.name
         ORDER BY s.id ASC"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(SocialStatusItem {
            id: row.get(0)?,
            name: row.get(1)?,
            count: row.get(2)?,
            base_points: row.get(3).unwrap_or(20),
            created_at: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for r in rows {
        if let Ok(item) = r {
            list.push(item);
        }
    }
    Ok(list)
}

#[tauri::command]
fn create_social_status(state: State<AppState>, name: String, base_points: Option<i64>) -> Result<SocialStatusItem, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("اسم الحالة الاجتماعية مطلوب".into());
    }
    let points = base_points.unwrap_or(20);

    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO social_statuses (name, base_points) VALUES (?1, ?2)",
        params![name, points]
    ).map_err(|e| format!("فشل إضافة الحالة الاجتماعية (قد تكون موجودة مسبقاً): {}", e))?;

    let id = conn.last_insert_rowid();
    Ok(SocialStatusItem {
        id,
        name: name.to_string(),
        count: 0,
        base_points: points,
        created_at: None,
    })
}

#[tauri::command]
fn update_social_status(state: State<AppState>, id: i64, new_name: String, base_points: Option<i64>) -> Result<(), String> {
    let new_name = new_name.trim();
    if new_name.is_empty() {
        return Err("اسم الحالة الاجتماعية لا يمكن أن يكون فارغاً".into());
    }
    let points = base_points.unwrap_or(20);

    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let old_name: String = tx.query_row(
        "SELECT name FROM social_statuses WHERE id = ?1",
        params![id],
        |row| row.get(0)
    ).map_err(|_| "الحالة الاجتماعية غير موجودة".to_string())?;

    tx.execute(
        "UPDATE social_statuses SET name = ?1, base_points = ?2 WHERE id = ?3",
        params![new_name, points, id]
    ).map_err(|e| format!("فشل تعديل الحالة الاجتماعية (قد يكون الاسم مكرراً): {}", e))?;

    // Cascade rename to guardians table so existing records remain linked!
    tx.execute(
        "UPDATE guardians SET social_status = ?1 WHERE social_status = ?2",
        params![new_name, old_name]
    ).map_err(|e| format!("فشل تحديث سجلات المستفيدين: {}", e))?;

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn delete_social_status(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    let name: String = conn.query_row(
        "SELECT name FROM social_statuses WHERE id = ?1",
        params![id],
        |row| row.get(0)
    ).map_err(|_| "الحالة الاجتماعية غير موجودة".to_string())?;

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM guardians WHERE social_status = ?1",
        params![name],
        |row| row.get(0)
    ).unwrap_or(0);

    if count > 0 {
        return Err(format!(
            "لا يمكن حذف الحالة الاجتماعية '{}' لأنها مرتبطة حالياً بـ {} من أولياء الأمور/المستفيدين.",
            name, count
        ));
    }

    conn.execute("DELETE FROM social_statuses WHERE id = ?1", params![id])
        .map_err(|e| format!("فشل حذف الحالة الاجتماعية: {}", e))?;

    Ok(())
}

// ==================== Education Levels CRUD ====================

#[tauri::command]
fn get_education_levels(state: State<AppState>) -> Result<Vec<EducationLevelWithCount>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT el.id, el.stage, el.year_name, el.year_order, 
                COUNT(gc.id) as count, el.created_at
         FROM education_levels el
         LEFT JOIN guardian_children gc ON gc.education_level_id = el.id
         GROUP BY el.id
         ORDER BY 
           CASE el.stage 
             WHEN 'ابتدائي' THEN 1 
             WHEN 'متوسط' THEN 2 
             WHEN 'ثانوي' THEN 3 
             ELSE 4 
           END,
           el.year_order ASC"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(EducationLevelWithCount {
            id: row.get(0)?,
            stage: row.get(1)?,
            year_name: row.get(2)?,
            year_order: row.get(3)?,
            count: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut list = Vec::new();
    for r in rows {
        if let Ok(item) = r {
            list.push(item);
        }
    }
    Ok(list)
}

#[tauri::command]
fn create_education_level(state: State<AppState>, stage: String, year_name: String, year_order: i64) -> Result<EducationLevel, String> {
    let stage = stage.trim();
    let year_name = year_name.trim();
    if stage.is_empty() || year_name.is_empty() {
        return Err("اسم الطور والسنة الدراسية مطلوبان".into());
    }
    if year_order <= 0 {
        return Err("ترتيب السنة يجب أن يكون رقماً أكبر من 0".into());
    }

    let conn = state.db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO education_levels (stage, year_name, year_order) VALUES (?1, ?2, ?3)",
        params![stage, year_name, year_order]
    ).map_err(|e| format!("فشل إضافة السنة الدراسية (قد تكون موجودة مسبقاً): {}", e))?;

    let id = conn.last_insert_rowid();
    Ok(EducationLevel {
        id,
        stage: stage.to_string(),
        year_name: year_name.to_string(),
        year_order,
        created_at: None,
    })
}

#[tauri::command]
fn update_education_level(state: State<AppState>, id: i64, year_name: String) -> Result<(), String> {
    let year_name = year_name.trim();
    if year_name.is_empty() {
        return Err("اسم السنة الدراسية لا يمكن أن يكون فارغاً".into());
    }

    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let _old_name: String = tx.query_row(
        "SELECT year_name FROM education_levels WHERE id = ?1",
        params![id],
        |row| row.get(0)
    ).map_err(|_| "السنة الدراسية غير موجودة".to_string())?;

    tx.execute(
        "UPDATE education_levels SET year_name = ?1 WHERE id = ?2",
        params![year_name, id]
    ).map_err(|e| format!("فشل تعديل السنة الدراسية: {}", e))?;

    // Cascade rename to guardian_children so existing records stay consistent
    tx.execute(
        "UPDATE guardian_children SET education_level = ?1 WHERE education_level_id = ?2",
        params![year_name, id]
    ).map_err(|e| format!("فشل تحديث سجلات الأطفال: {}", e))?;

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn delete_education_level(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let year_name: String = conn.query_row(
        "SELECT year_name FROM education_levels WHERE id = ?1",
        params![id],
        |row| row.get(0)
    ).map_err(|_| "السنة الدراسية غير موجودة".to_string())?;

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM guardian_children WHERE education_level_id = ?1",
        params![id],
        |row| row.get(0)
    ).unwrap_or(0);

    if count > 0 {
        return Err(format!(
            "لا يمكن حذف السنة الدراسية '{}' لأنها مرتبطة حالياً بـ {} من الأطفال المسجلين.",
            year_name, count
        ));
    }

    conn.execute("DELETE FROM education_levels WHERE id = ?1", params![id])
        .map_err(|e| format!("فشل حذف السنة الدراسية: {}", e))?;

    Ok(())
}

#[tauri::command]
fn get_education_level_stats(state: State<AppState>, campaign_id: i64) -> Result<Vec<EducationLevelStat>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT 
            el.id, 
            el.stage, 
            el.year_name, 
            el.year_order, 
            COUNT(c_data.child_id) as children_count,
            COUNT(CASE WHEN c_data.is_delivered = 1 THEN c_data.child_id END) as delivered_count,
            COUNT(CASE WHEN c_data.is_delivered = 0 THEN c_data.child_id END) as pending_count
         FROM education_levels el
         LEFT JOIN (
            SELECT 
                gc.id as child_id,
                gc.education_level_id,
                COALESCE(cr.is_delivered, 0) as is_delivered
            FROM guardian_children gc
            JOIN campaign_records cr ON cr.guardian_id = gc.guardian_id AND cr.campaign_id = ?1
            WHERE gc.is_schooling = 1
         ) c_data ON c_data.education_level_id = el.id
         GROUP BY el.id
         ORDER BY 
           CASE el.stage 
             WHEN 'ابتدائي' THEN 1 
             WHEN 'متوسط' THEN 2 
             WHEN 'ثانوي' THEN 3 
             ELSE 4 
           END,
           el.year_order ASC"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(params![campaign_id], |row| {
        Ok(EducationLevelStat {
            id: row.get(0)?,
            stage: row.get(1)?,
            year_name: row.get(2)?,
            year_order: row.get(3)?,
            children_count: row.get(4)?,
            delivered_count: row.get(5)?,
            pending_count: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut stats = Vec::new();
    for item in rows {
        stats.push(item.map_err(|e| e.to_string())?);
    }
    Ok(stats)
}

fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    let clean_input = if let Some(idx) = input.find(',') {
        &input[idx + 1..]
    } else {
        input
    }.trim();

    let mut buf = Vec::with_capacity(clean_input.len() * 3 / 4);
    let mut accumulator: u32 = 0;
    let mut bits_collected: u32 = 0;

    for b in clean_input.bytes() {
        let val = match b {
            b'A'..=b'Z' => (b - b'A') as u32,
            b'a'..=b'z' => (b - b'a' + 26) as u32,
            b'0'..=b'9' => (b - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            b'=' => continue,
            b' ' | b'\r' | b'\n' | b'\t' => continue,
            _ => return Err(format!("رمز base64 غير صالح: {}", b as char)),
        };

        accumulator = (accumulator << 6) | val;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            let byte = ((accumulator >> bits_collected) & 0xFF) as u8;
            buf.push(byte);
        }
    }

    Ok(buf)
}

fn encode_base64(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        result.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

fn get_avatars_dir(app_handle: &tauri::AppHandle) -> std::path::PathBuf {
    let base_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .or_else(|| app_handle.path_resolver().app_data_dir())
        .unwrap_or_else(|| {
            get_db_path()
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::path::PathBuf::from("."))
        });
    let dir = base_dir.join("avatars");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[tauri::command]
async fn save_avatar_file(app_handle: tauri::AppHandle, base64_image: String) -> Result<String, String> {
    let bytes = decode_base64(&base64_image)?;
    if bytes.is_empty() {
        return Err("محتوى الصورة فارغ".to_string());
    }

    let avatars_dir = get_avatars_dir(&app_handle);

    use std::time::{SystemTime, UNIX_EPOCH};
    use std::sync::atomic::{AtomicU64, Ordering};
    static AVATAR_COUNTER: AtomicU64 = AtomicU64::new(1);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let timestamp = now.as_secs();
    let nanos = now.subsec_nanos();
    let counter = AVATAR_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    let uuid_str = format!("{:08x}{:04x}{:04x}", nanos, (pid & 0xFFFF) as u16, (counter & 0xFFFF) as u16);
    let filename = format!("avatar_{}_{}.webp", timestamp, uuid_str);
    let rel_path = format!("avatars/{}", filename);
    let file_path = avatars_dir.join(&filename);

    std::fs::write(&file_path, &bytes).map_err(|e| format!("فشل حفظ ملف الصورة: {}", e))?;

    // Also mirror to DB directory if different
    if let Some(db_parent) = get_db_path().parent() {
        let db_avatars = db_parent.join("avatars");
        if db_avatars != avatars_dir {
            let _ = std::fs::create_dir_all(&db_avatars);
            let _ = std::fs::write(db_avatars.join(&filename), &bytes);
        }
    }

    Ok(rel_path)
}

#[tauri::command]
async fn load_avatar_file(app_handle: tauri::AppHandle, filename: String) -> Result<String, String> {
    if filename.trim().is_empty() {
        return Err("اسم الملف فارغ".to_string());
    }
    let raw_name = filename.replace('\\', "/");
    let simple_name = raw_name.split('/').last().unwrap_or(&raw_name);

    let avatar_dir = get_avatars_dir(&app_handle);
    let target = avatar_dir.join(simple_name);

    let path_to_read = if target.exists() {
        target
    } else {
        let fallback = get_db_path()
            .parent()
            .map(|p| p.join("avatars").join(simple_name))
            .unwrap_or_else(|| std::path::PathBuf::from("avatars").join(simple_name));
        if fallback.exists() {
            fallback
        } else {
            return Err("ملف الصورة غير موجود".to_string());
        }
    };

    let bytes = std::fs::read(&path_to_read).map_err(|e| e.to_string())?;
    let b64 = encode_base64(&bytes);
    Ok(format!("data:image/webp;base64,{}", b64))
}

#[tauri::command]
async fn delete_avatar_file(app_handle: tauri::AppHandle, filename: String) -> Result<(), String> {
    if filename.trim().is_empty() {
        return Ok(());
    }
    let raw_name = filename.replace('\\', "/");
    let simple_name = raw_name.split('/').last().unwrap_or(&raw_name);
    let avatar_dir = get_avatars_dir(&app_handle);
    let target = avatar_dir.join(simple_name);
    if target.exists() {
        let _ = std::fs::remove_file(target);
    }
    if let Some(parent) = get_db_path().parent() {
        let fallback = parent.join("avatars").join(simple_name);
        if fallback.exists() {
            let _ = std::fs::remove_file(fallback);
        }
    }
    Ok(())
}

#[tauri::command]
async fn open_scanner_app() -> Result<String, String> {
    // 1. Candidate third-party scanner applications
    let candidate_paths = [
        r"C:\Program Files\NAPS2\NAPS2.exe",
        r"C:\Program Files (x86)\NAPS2\NAPS2.exe",
        r"C:\Program Files (x86)\Canon\IJ Scan Utility\SCANUTILITY.exe",
        r"C:\Program Files\Canon\IJ Scan Utility\SCANUTILITY.exe",
        r"C:\Program Files (x86)\HP\HP Scan\HPScan.exe",
        r"C:\Program Files\HP\HP Scan\HPScan.exe",
        r"C:\Program Files (x86)\Brother\iPrint&Scan\Brother iPrint&Scan.exe",
        r"C:\Program Files (x86)\epson\Epson Scan 2\Core\es2.exe",
        r"C:\Program Files\epson\Epson Scan 2\Core\es2.exe",
    ];

    for path in candidate_paths {
        if std::path::Path::new(path).exists() {
            if std::process::Command::new(path).spawn().is_ok() {
                return Ok(format!("تم تشغيل تطبيق الماسح: {}", path));
            }
        }
    }

    // 2. Check Windows native wiaacmgr.exe or WFS.exe (Windows Fax and Scan)
    let wia_path = r"C:\Windows\System32\wiaacmgr.exe";
    let wfs_path = r"C:\Windows\System32\WFS.exe";

    if std::path::Path::new(wia_path).exists() {
        if std::process::Command::new(wia_path).spawn().is_ok() {
            return Ok("تم تشغيل معالج المسح الضوئي لويندوز (WIA)".into());
        }
    }

    if std::path::Path::new(wfs_path).exists() {
        if std::process::Command::new(wfs_path).spawn().is_ok() {
            return Ok("تم تشغيل فاكس ومسح ويندوز (WFS)".into());
        }
    }

    // 3. Fallback: Launch Windows Modern Scan App protocol via explorer.exe
    let res = std::process::Command::new("explorer.exe")
        .arg(r"shell:appsFolder\Microsoft.WindowsScan_8wekyb3d8bbwe!App")
        .spawn();

    if res.is_ok() {
        return Ok("تم تشغيل تطبيق الماسح الضوئي لويندوز".into());
    }

    // 4. Last resort: launch wiaacmgr via cmd start
    let _ = std::process::Command::new("cmd.exe")
        .args(["/c", "start", "wiaacmgr.exe"])
        .spawn()
        .map_err(|e| format!("تعذر تشغيل تطبيق الماسح الضوئي: {}", e))?;

    Ok("تم إرسال أمر تشغيل الماسح الضوئي".into())
}

fn get_db_path() -> std::path::PathBuf {
    // 1. If running in development (inside the project repo)
    if let Ok(cur) = std::env::current_dir() {
        if cur.join("src-tauri").exists() {
            // Running from project root
            let target = cur.join("school_aid.db");
            let legacy = cur.join("src-tauri").join("school_aid.db");
            if legacy.exists() && !target.exists() {
                let _ = std::fs::copy(&legacy, &target);
                let _ = std::fs::remove_file(&legacy);
            }
            return target;
        } else if cur.ends_with("src-tauri") {
            // Running with CWD inside src-tauri
            if let Some(parent) = cur.parent() {
                let target = parent.join("school_aid.db");
                let legacy = cur.join("school_aid.db");
                if legacy.exists() && !target.exists() {
                    let _ = std::fs::copy(&legacy, &target);
                    let _ = std::fs::remove_file(&legacy);
                }
                return target;
            }
        }
    }

    // 2. Production fallback: place next to executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let s = parent.to_string_lossy();
            if s.contains("target") {
                if let Ok(cur) = std::env::current_dir() {
                    if cur.ends_with("src-tauri") {
                        if let Some(p) = cur.parent() {
                            return p.join("school_aid.db");
                        }
                    }
                    return cur.join("school_aid.db");
                }
            }
            return parent.join("school_aid.db");
        }
    }

    std::path::PathBuf::from("school_aid.db")
}

fn main() {
    let db_path = get_db_path();

    // Automatic safety backup on startup if DB exists and contains data
    if db_path.exists() {
        if let Ok(meta) = std::fs::metadata(&db_path) {
            if meta.len() > 0 {
                let backup_path = db_path.with_extension("db.auto_backup");
                let _ = std::fs::copy(&db_path, &backup_path);
            }
        }
    }

    let conn = Connection::open(&db_path).expect("فشل في فتح قاعدة البيانات");
    
    conn.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA temp_store = MEMORY;
        PRAGMA cache_size = -64000;
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS guardians (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guardian_name TEXT NOT NULL,
            phone TEXT,
            social_status TEXT NOT NULL,
            photo_path TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS social_statuses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        INSERT OR IGNORE INTO social_statuses (name) VALUES 
        ('بدون دخل'), 
        ('ضعيف الدخل'), 
        ('متقاعد'), 
        ('مرض مزمن'), 
        ('إعاقة');

        CREATE TABLE IF NOT EXISTS campaigns (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            year_label TEXT NOT NULL UNIQUE,
            is_active BOOLEAN DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS campaign_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            campaign_id INTEGER NOT NULL,
            guardian_id INTEGER NOT NULL,
            record_no INTEGER,
            primary_count INTEGER DEFAULT 0,
            middle_count INTEGER DEFAULT 0,
            secondary_count INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
            FOREIGN KEY(guardian_id) REFERENCES guardians(id) ON DELETE CASCADE,
            UNIQUE(campaign_id, guardian_id)
        );

        CREATE TABLE IF NOT EXISTS guardian_children (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            guardian_id INTEGER NOT NULL,
            child_name TEXT NOT NULL,
            birth_date TEXT,
            is_schooling BOOLEAN DEFAULT 1,
            education_level TEXT,
            school_name TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(guardian_id) REFERENCES guardians(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS organization_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            org_name TEXT NOT NULL DEFAULT 'اسم الجمعية الخيرية',
            branch_name TEXT DEFAULT 'المكتب الولائي / البلدي',
            wilaya TEXT DEFAULT 'قسنطينة',
            commune TEXT DEFAULT '',
            phone TEXT DEFAULT '',
            footer_text TEXT DEFAULT 'وثيقة إدارية داخلية مخصصة لضبط عملية التوزيع.'
        );

        INSERT OR IGNORE INTO organization_settings (id, org_name, branch_name, wilaya, phone) 
        VALUES (1, 'الجمعية الخيرية لرعاية الأيتام والمحتاجين', 'المكتب الولائي', 'قسنطينة', '');

        CREATE TABLE IF NOT EXISTS education_levels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stage TEXT NOT NULL,
            year_name TEXT NOT NULL,
            year_order INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(stage, year_order)
        );

        INSERT OR IGNORE INTO education_levels (stage, year_name, year_order) VALUES
        ('ابتدائي', 'السنة 1 ابتدائي', 1),
        ('ابتدائي', 'السنة 2 ابتدائي', 2),
        ('ابتدائي', 'السنة 3 ابتدائي', 3),
        ('ابتدائي', 'السنة 4 ابتدائي', 4),
        ('ابتدائي', 'السنة 5 ابتدائي', 5),
        ('متوسط', 'السنة 1 متوسط', 1),
        ('متوسط', 'السنة 2 متوسط', 2),
        ('متوسط', 'السنة 3 متوسط', 3),
        ('متوسط', 'السنة 4 متوسط', 4),
        ('ثانوي', 'السنة 1 ثانوي', 1),
        ('ثانوي', 'السنة 2 ثانوي', 2),
        ('ثانوي', 'السنة 3 ثانوي', 3);
    ").unwrap();

    let _ = conn.execute("ALTER TABLE campaign_records ADD COLUMN record_no INTEGER", []);
    let _ = conn.execute(
        "UPDATE campaign_records SET record_no = (
            SELECT COUNT(*) FROM campaign_records cr2 
            WHERE cr2.campaign_id = campaign_records.campaign_id AND cr2.id <= campaign_records.id
         ) WHERE record_no IS NULL",
        [],
    );
    let _ = conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_campaign_records_no ON campaign_records(campaign_id, record_no)",
        [],
    );

    // Performance indexes for fast relational lookups, child records, stats, and search
    let _ = conn.execute("CREATE INDEX IF NOT EXISTS idx_children_guardian_id ON guardian_children(guardian_id)", []);
    let _ = conn.execute("CREATE INDEX IF NOT EXISTS idx_children_education_level_id ON guardian_children(education_level_id)", []);
    let _ = conn.execute("CREATE INDEX IF NOT EXISTS idx_records_guardian_id ON campaign_records(guardian_id)", []);
    let _ = conn.execute("CREATE INDEX IF NOT EXISTS idx_records_campaign_delivered ON campaign_records(campaign_id, is_delivered)", []);
    let _ = conn.execute("CREATE INDEX IF NOT EXISTS idx_guardians_name ON guardians(guardian_name)", []);
    let _ = conn.execute("CREATE INDEX IF NOT EXISTS idx_guardians_phone ON guardians(phone)", []);
    let _ = conn.execute("CREATE INDEX IF NOT EXISTS idx_guardians_social_status_id ON guardians(social_status_id)", []);

    let _ = conn.execute("ALTER TABLE campaign_records ADD COLUMN is_delivered INTEGER DEFAULT 0", []);
    let _ = conn.execute("ALTER TABLE campaign_records ADD COLUMN delivered_at TEXT", []);

    let table_sql: Option<String> = conn.query_row(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name='guardians'",
        [],
        |r| r.get(0),
    ).optional().unwrap_or(None);

    if let Some(sql) = table_sql {
        if sql.contains("CHECK(social_status IN") {
            let _ = conn.execute_batch("
                PRAGMA foreign_keys = OFF;
                CREATE TABLE IF NOT EXISTS guardians_new (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    guardian_name TEXT NOT NULL,
                    phone TEXT,
                    social_status TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
                INSERT OR IGNORE INTO guardians_new (id, guardian_name, phone, social_status, created_at)
                SELECT id, guardian_name, phone, social_status, created_at FROM guardians;
                DROP TABLE guardians;
                ALTER TABLE guardians_new RENAME TO guardians;
                PRAGMA foreign_keys = ON;
            ");
        }
    }

    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN social_status_id INTEGER REFERENCES social_statuses(id)", []);
    let _ = conn.execute("UPDATE guardians SET social_status_id = (SELECT id FROM social_statuses WHERE social_statuses.name = guardians.social_status) WHERE social_status_id IS NULL", []);

    // Migration: add base_points to social_statuses
    let _ = conn.execute("ALTER TABLE social_statuses ADD COLUMN base_points INTEGER DEFAULT 20", []);
    let _ = conn.execute("UPDATE social_statuses SET base_points = 40 WHERE name = 'بدون دخل' AND (base_points IS NULL OR base_points = 20)", []);
    let _ = conn.execute("UPDATE social_statuses SET base_points = 35 WHERE name = 'إعاقة' AND (base_points IS NULL OR base_points = 20)", []);
    let _ = conn.execute("UPDATE social_statuses SET base_points = 30 WHERE name = 'مرض مزمن' AND (base_points IS NULL OR base_points = 20)", []);
    let _ = conn.execute("UPDATE social_statuses SET base_points = 20 WHERE name = 'ضعيف الدخل' AND (base_points IS NULL OR base_points = 20)", []);
    let _ = conn.execute("UPDATE social_statuses SET base_points = 10 WHERE name = 'متقاعد' AND (base_points IS NULL OR base_points = 20)", []);

    // Migration: add student_priority_points to organization_settings
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN student_priority_points INTEGER DEFAULT 5", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN marital_points_widow INTEGER DEFAULT 30", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN marital_points_divorced INTEGER DEFAULT 20", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN marital_points_deserted INTEGER DEFAULT 25", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN marital_points_married INTEGER DEFAULT 10", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN marital_points_single INTEGER DEFAULT 5", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN marital_points_other INTEGER DEFAULT 5", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN priority_threshold_critical INTEGER DEFAULT 60", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN priority_threshold_high INTEGER DEFAULT 45", []);
    let _ = conn.execute("ALTER TABLE organization_settings ADD COLUMN priority_threshold_medium INTEGER DEFAULT 30", []);

    // Migration: add extra_priority_points to campaign_records
    let _ = conn.execute("ALTER TABLE campaign_records ADD COLUMN extra_priority_points INTEGER DEFAULT 0", []);

    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN birth_date TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN birth_place TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN address TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN marital_status TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN father_name TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN mother_name TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN spouse_name TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN monthly_income TEXT", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN children_count INTEGER", []);
    let _ = conn.execute("ALTER TABLE guardians ADD COLUMN photo_path TEXT", []);

    // Migration: add education_level_id to guardian_children
    let _ = conn.execute("ALTER TABLE guardian_children ADD COLUMN education_level_id INTEGER REFERENCES education_levels(id)", []);
    let _ = conn.execute(
        "UPDATE guardian_children 
         SET education_level_id = (SELECT id FROM education_levels WHERE year_name = guardian_children.education_level LIMIT 1)
         WHERE education_level_id IS NULL AND education_level IS NOT NULL",
        [],
    );

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM campaigns", [], |r| r.get(0)).unwrap_or(0);
    if count == 0 {
        conn.execute(
            "INSERT INTO campaigns (year_label, is_active) VALUES ('2025/2026', 1)",
            [],
        ).unwrap();
    }

    let legacy_exists: bool = conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name='beneficiaries'",
        [],
        |_| Ok(true)
    ).unwrap_or(false);

    if legacy_exists {
        let first_campaign_id: i64 = conn.query_row(
            "SELECT id FROM campaigns ORDER BY id ASC LIMIT 1",
            [],
            |r| r.get(0)
        ).unwrap_or(1);

        let _ = conn.execute(
            "INSERT OR IGNORE INTO guardians (id, guardian_name, phone, social_status, created_at)
             SELECT id, guardian_name, phone, social_status, created_at FROM beneficiaries",
            [],
        );

        let _ = conn.execute(
            &format!(
                "INSERT OR IGNORE INTO campaign_records (campaign_id, guardian_id, primary_count, middle_count, secondary_count, created_at)
                 SELECT {}, id, primary_count, middle_count, secondary_count, created_at FROM beneficiaries",
                first_campaign_id
            ),
            [],
        );

        let _ = conn.execute("ALTER TABLE beneficiaries RENAME TO beneficiaries_old_backup", []);
    }

    tauri::Builder::default()
        .manage(AppState { db: Mutex::new(conn) })
        .invoke_handler(tauri::generate_handler![
            get_campaigns,
            create_campaign,
            rollover_campaign_records,
            get_next_record_no,
            get_beneficiaries,
            get_guardian_children,
            check_beneficiary_match,
            save_beneficiary,
            delete_beneficiary,
            bulk_delete_beneficiaries,
            toggle_delivery_status,
            bulk_set_delivery_status,
            get_stats,
            import_beneficiaries,
            batch_import_records,
            safe_import_records,
            get_org_settings,
            save_org_settings,
            write_binary_file,
            get_social_statuses,
            create_social_status,
            update_social_status,
            delete_social_status,
            get_campaigns_with_counts,
            get_all_record_numbers,
            get_social_status_stats,
            update_campaign_label,
            delete_campaign,
            set_active_campaign,
            get_education_levels,
            create_education_level,
            update_education_level,
            delete_education_level,
            get_education_level_stats,
            save_avatar_file,
            load_avatar_file,
            delete_avatar_file,
            open_scanner_app
        ])
        .run(tauri::generate_context!())
        .expect("خطأ أثناء تشغيل Tauri");
}
