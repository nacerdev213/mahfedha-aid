import { defineStore } from 'pinia';
import { safeInvoke } from './tauri';

export const useOrganizationStore = defineStore('organization', {
  state: () => ({
    orgSettings: {
      id: 1,
      org_name: 'الجمعية الخيرية لرعاية الأيتام والمحتاجين',
      branch_name: 'المكتب الولائي',
      wilaya: 'قسنطينة',
      commune: '',
      phone: '',
      footer_text: 'وثيقة إدارية داخلية مخصصة لضبط عملية التوزيع.'
    },
    showSettingsModal: false,
    settingsForm: {
      id: 1,
      org_name: 'الجمعية الخيرية لرعاية الأيتام والمحتاجين',
      branch_name: 'المكتب الولائي',
      wilaya: 'قسنطينة',
      commune: '',
      phone: '',
      footer_text: 'وثيقة إدارية داخلية مخصصة لضبط عملية التوزيع.'
    },
    isLoading: false,
  }),

  actions: {
    async loadOrgSettings() {
      this.isLoading = true;
      try {
        const res = await safeInvoke('get_org_settings');
        if (res && res.org_name) {
          this.orgSettings = { ...res };
          this.settingsForm = { ...res };
        }
      } catch (e) {
        console.error('فشل تحميل إعدادات الجمعية:', e);
      } finally {
        this.isLoading = false;
      }
    },

    openSettingsModal() {
      this.settingsForm = { ...this.orgSettings };
      this.showSettingsModal = true;
    },

    closeSettingsModal() {
      this.showSettingsModal = false;
    },

    async saveOrgSettings() {
      if (!this.settingsForm.org_name.trim()) {
        throw new Error('اسم الجمعية مطلوب');
      }

      this.isLoading = true;
      try {
        await safeInvoke('save_org_settings', { settings: this.settingsForm });
        this.orgSettings = { ...this.settingsForm };
        this.showSettingsModal = false;
      } finally {
        this.isLoading = false;
      }
    }
  }
});
