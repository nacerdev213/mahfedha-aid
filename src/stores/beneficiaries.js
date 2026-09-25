import { defineStore } from 'pinia';
import { safeInvoke } from './tauri';

export const useBeneficiaryStore = defineStore('beneficiaries', {
  state: () => ({
    beneficiaries: [],
    stats: {
      total_families: 0,
      total_bags: 0,
      primary_total: 0,
      middle_total: 0,
      secondary_total: 0,
      primary_delivered: 0,
      middle_delivered: 0,
      secondary_delivered: 0,
      delivered_families: 0,
      pending_families: 0,
      delivered_bags: 0,
      pending_bags: 0,
      progress_percentage: 0,
    },
    searchQuery: '',
    selectedStatus: 'الكل',
    deliveryFilter: 'all', // 'all' | 'delivered' | 'pending'
    showModal: false,
    form: {
      id: null,
      record_no: null,
      guardian_id: null,
      guardian_name: '',
      phone: '',
      social_status: 'بدون دخل',
      primary_count: 0,
      middle_count: 0,
      secondary_count: 0,
      birth_date: '',
      birth_place: '',
      address: '',
      marital_status: 'متزوج',
      father_name: '',
      mother_name: '',
      spouse_name: '',
      monthly_income: '',
      children_count: null,
      extra_priority_points: 0,
      photo_path: '',
      children: [],
    },
    isLoading: false,
  }),

  actions: {
    async checkBeneficiaryMatch(campaignId, guardianName, birthDate = '', fatherName = '', motherName = '') {
      if (!campaignId || !guardianName || guardianName.trim().length < 3) {
        return { match_type: 'none', message: '', matched_record: null, previous_campaign_year: null };
      }
      try {
        const res = await safeInvoke('check_beneficiary_match', {
          campaignId,
          guardianName: guardianName.trim(),
          birthDate: birthDate || null,
          fatherName: fatherName || null,
          motherName: motherName || null,
        });
        return res || { match_type: 'none', message: '', matched_record: null, previous_campaign_year: null };
      } catch (e) {
        console.error('فشل فحص تشابه المستفيد:', e);
        return { match_type: 'none', message: '', matched_record: null, previous_campaign_year: null };
      }
    },

    async fetchData(campaignId) {
      if (!campaignId) return;
      this.isLoading = true;
      try {
        const [data, s] = await Promise.all([
          safeInvoke('get_beneficiaries', {
            campaignId: campaignId,
            search: this.searchQuery.trim() || '',
            statusFilter: (!this.selectedStatus || this.selectedStatus === 'الكل') ? '' : this.selectedStatus,
          }),
          safeInvoke('get_stats', { campaignId: campaignId })
        ]);

        if (Array.isArray(data)) {
          this.beneficiaries = data;
        }
        if (s) {
          this.stats = s;
        }
      } catch (e) {
        console.error('فشل جلب بيانات المستفيدين والإحصائيات:', e);
      } finally {
        this.isLoading = false;
      }
    },

    async getNextRecordNo(campaignId) {
      if (!campaignId) return 1;
      try {
        const nextNo = await safeInvoke('get_next_record_no', { campaignId });
        return nextNo || 1;
      } catch (e) {
        console.error('فشل جلب الرقم التالي للمستفيد:', e);
        return 1;
      }
    },

    openModal(record = null, defaultStatus = 'بدون دخل', nextRecordNo = 1) {
      if (record) {
        this.form = {
          id: record.id ?? null,
          record_no: record.record_no ?? null,
          guardian_id: record.guardian_id ?? null,
          guardian_name: record.guardian_name || '',
          phone: record.phone || '',
          social_status: record.social_status || defaultStatus,
          primary_count: record.primary_count || 0,
          middle_count: record.middle_count || 0,
          secondary_count: record.secondary_count || 0,
          birth_date: record.birth_date || '',
          birth_place: record.birth_place || '',
          address: record.address || '',
          marital_status: record.marital_status || 'متزوج',
          father_name: record.father_name || '',
          mother_name: record.mother_name || '',
          spouse_name: record.spouse_name || '',
          monthly_income: record.monthly_income || '',
          children_count: record.children_count ?? null,
          extra_priority_points: record.extra_priority_points ?? 0,
          photo_path: record.photo_path || '',
          children: [],
        };
        
        // Fetch children asynchronously without blocking modal display
        if (record.guardian_id) {
          safeInvoke('get_guardian_children', { guardianId: record.guardian_id })
            .then(children => {
              this.form.children = children || [];
            })
            .catch(e => {
              console.error('فشل جلب بيانات الأطفال:', e);
              this.form.children = [];
            });
        }
      } else {
        this.form = {
          id: null,
          record_no: nextRecordNo,
          guardian_id: null,
          guardian_name: '',
          phone: '',
          social_status: defaultStatus,
          primary_count: 0,
          middle_count: 0,
          secondary_count: 0,
          birth_date: '',
          birth_place: '',
          address: '',
          marital_status: 'متزوج',
          father_name: '',
          mother_name: '',
          spouse_name: '',
          monthly_income: '',
          children_count: null,
          extra_priority_points: 0,
          photo_path: '',
          children: [],
        };
      }
      this.showModal = true;
    },

    closeModal() {
      this.showModal = false;
    },

    async saveBeneficiary(campaignId) {
      if (!this.form.record_no || Number(this.form.record_no) <= 0) {
        throw new Error('يرجى إدخال رقم المستفيد (يجب أن يكون رقماً أكبر من 0)');
      }
      if (!this.form.guardian_name.trim()) {
        throw new Error('اسم ولقب ولي الأمر مطلوب');
      }

      this.form.record_no = Number(this.form.record_no);
      this.isLoading = true;
      try {
        await safeInvoke('save_beneficiary', {
          campaignId: campaignId,
          payload: this.form
        });
        this.showModal = false;
        await this.fetchData(campaignId);
      } finally {
        this.isLoading = false;
      }
    },

    async deleteBeneficiary(id, campaignId) {
      this.isLoading = true;
      try {
        await safeInvoke('delete_beneficiary', { recordId: id });
        await this.fetchData(campaignId);
      } finally {
        this.isLoading = false;
      }
    },

    async bulkDeleteBeneficiaries(recordIds, campaignId) {
      if (!recordIds || recordIds.length === 0) return 0;
      this.isLoading = true;
      try {
        let count = 0;
        try {
          count = await safeInvoke('bulk_delete_beneficiaries', { recordIds });
        } catch (err) {
          console.warn('bulk_delete_beneficiaries invoke failed, using sequential fallback:', err);
          for (const id of recordIds) {
            await safeInvoke('delete_beneficiary', { recordId: id });
            count++;
          }
        }
        await this.fetchData(campaignId);
        return count;
      } finally {
        this.isLoading = false;
      }
    },

    async toggleDelivery(recordId, campaignId) {
      if (!recordId) return;
      try {
        // Optimistic UI update for instantaneous responsiveness
        const record = this.beneficiaries.find(b => b.id === recordId);
        if (record) {
          record.is_delivered = !record.is_delivered;
          if (record.is_delivered) {
            const now = new Date();
            const y = now.getFullYear();
            const m = String(now.getMonth() + 1).padStart(2, '0');
            const d = String(now.getDate()).padStart(2, '0');
            const h = String(now.getHours()).padStart(2, '0');
            const min = String(now.getMinutes()).padStart(2, '0');
            record.delivered_at = `${y}-${m}-${d} ${h}:${min}`;
          } else {
            record.delivered_at = null;
          }
        }
        await safeInvoke('toggle_delivery_status', { recordId });
        await this.fetchData(campaignId);
      } catch (e) {
        console.error('فشل تغيير حالة الاستلام:', e);
        await this.fetchData(campaignId);
        throw e;
      }
    },

    async bulkSetDelivery(recordIds, isDelivered, campaignId) {
      if (!recordIds || recordIds.length === 0) return 0;
      this.isLoading = true;
      try {
        const count = await safeInvoke('bulk_set_delivery_status', {
          recordIds,
          isDelivered
        });
        await this.fetchData(campaignId);
        return count;
      } finally {
        this.isLoading = false;
      }
    },

    async batchImport(campaignId, records) {
      if (!records || records.length === 0) return 0;
      this.isLoading = true;
      try {
        const count = await safeInvoke('batch_import_records', {
          campaignId: campaignId,
          records: records
        });
        await this.fetchData(campaignId);
        return count;
      } finally {
        this.isLoading = false;
      }
    },

    async safeImport(campaignId, records) {
      if (!records || records.length === 0) return 0;
      this.isLoading = true;
      try {
        const count = await safeInvoke('safe_import_records', {
          campaignId: campaignId,
          records: records
        });
        await this.fetchData(campaignId);
        return count;
      } finally {
        this.isLoading = false;
      }
    }
  }
});

