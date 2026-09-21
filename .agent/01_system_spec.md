# مواصفات المنظومة (School Aid Desktop App)

## 1. التقنيات
- Tauri v1/v2 (Rust backend)
- SQLite (Local Embedded DB via rusqlite)
- Vue 3 (Composition API, `<script setup>`)
- Vite + Tailwind CSS (RTL support)

## 2. هيكل قاعدة البيانات العلائقية (Multi-Year Schema)

```sql
PRAGMA foreign_keys = ON;

-- 1. الدليل الدائم لأولياء الأمور
CREATE TABLE IF NOT EXISTS guardians (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guardian_name TEXT NOT NULL,
    phone TEXT,
    social_status TEXT CHECK(social_status IN ('بدون دخل', 'ضعيف الدخل', 'متقاعد', 'مرض مزمن', 'إعاقة')) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. مواسم وحملات التوزيع
CREATE TABLE IF NOT EXISTS campaigns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    year_label TEXT NOT NULL UNIQUE, -- مثال: "2026/2027"
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 3. سجلات التوزيع السنوية
CREATE TABLE IF NOT EXISTS campaign_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    campaign_id INTEGER NOT NULL,
    guardian_id INTEGER NOT NULL,
    primary_count INTEGER DEFAULT 0,
    middle_count INTEGER DEFAULT 0,
    secondary_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE,
    FOREIGN KEY(guardian_id) REFERENCES guardians(id) ON DELETE CASCADE,
    UNIQUE(campaign_id, guardian_id) -- الولي لا يسجل أكثر من مرة في نفس الموسم
);

-- 4. إعدادات وهوية الجمعية (لترويسة الطباعة الرسمية)
CREATE TABLE IF NOT EXISTS organization_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    org_name TEXT NOT NULL DEFAULT 'اسم الجمعية الخيرية',
    branch_name TEXT DEFAULT 'المكتب الولائي / البلدي',
    wilaya TEXT DEFAULT 'قسنطينة',
    commune TEXT DEFAULT '',
    phone TEXT DEFAULT '',
    footer_text TEXT DEFAULT 'وثيقة إدارية داخلية مخصصة لضبط عملية التوزيع.'
);
```

