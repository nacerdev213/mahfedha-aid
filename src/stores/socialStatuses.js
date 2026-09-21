import { defineStore } from 'pinia';
import { safeInvoke } from './tauri';

export const useSocialStatusStore = defineStore('socialStatuses', {
  state: () => ({
    socialStatuses: [],
    showStatusModal: false,
    newStatusName: '',
    editingStatusId: null,
    editingStatusName: '',
    statusError: '',
    isLoading: false,
  }),

  getters: {
    statusNames: (state) => {
      return state.socialStatuses.map(s => s.name);
    },
    statuses: (state) => {
      return state.socialStatuses;
    },
    defaultStatusName: (state) => {
      return state.socialStatuses[0]?.name || 'بدون دخل';
    },
    defaultStatusId: (state) => {
      return state.socialStatuses[0]?.id || 1;
    }
  },

  actions: {
    async loadSocialStatuses() {
      this.isLoading = true;
      try {
        const list = await safeInvoke('get_social_statuses');
        if (Array.isArray(list) && list.length > 0) {
          this.socialStatuses = list;
        } else {
          this.socialStatuses = [
            { id: 1, name: 'بدون دخل', count: 0 },
            { id: 2, name: 'ضعيف الدخل', count: 0 },
            { id: 3, name: 'متقاعد', count: 0 },
            { id: 4, name: 'مرض مزمن', count: 0 },
            { id: 5, name: 'إعاقة', count: 0 }
          ];
        }
      } catch (e) {
        console.error('فشل تحميل الحالات الاجتماعية:', e);
      } finally {
        this.isLoading = false;
      }
    },

    openStatusModal() {
      this.statusError = '';
      this.newStatusName = '';
      this.editingStatusId = null;
      this.loadSocialStatuses();
      this.showStatusModal = true;
    },

    closeStatusModal() {
      this.showStatusModal = false;
      this.editingStatusId = null;
      this.statusError = '';
    },

    async addSocialStatus() {
      const name = this.newStatusName.trim();
      if (!name) return;
      this.statusError = '';
      try {
        await safeInvoke('create_social_status', { name });
        this.newStatusName = '';
        await this.loadSocialStatuses();
      } catch (e) {
        this.statusError = String(e);
        throw e;
      }
    },

    startEditStatus(item) {
      this.editingStatusId = item.id;
      this.editingStatusName = item.name;
      this.statusError = '';
    },

    cancelEditStatus() {
      this.editingStatusId = null;
      this.editingStatusName = '';
    },

    async saveEditStatus(item) {
      const name = this.editingStatusName.trim();
      if (!name) return;
      if (name === item.name) {
        this.cancelEditStatus();
        return;
      }
      this.statusError = '';
      try {
        await safeInvoke('update_social_status', { id: item.id, newName: name });
        this.cancelEditStatus();
        await this.loadSocialStatuses();
      } catch (e) {
        this.statusError = String(e);
        throw e;
      }
    },

    async deleteSocialStatus(item) {
      if (item.count > 0) {
        throw new Error(`لا يمكن حذف الحالة الاجتماعية '${item.name}' لأنها مرتبطة حالياً بـ ${item.count} مستفيد.`);
      }
      this.statusError = '';
      try {
        await safeInvoke('delete_social_status', { id: item.id });
        await this.loadSocialStatuses();
      } catch (e) {
        this.statusError = String(e);
        throw e;
      }
    }
  }
});

export const useStatusStore = useSocialStatusStore;
