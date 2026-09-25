import { defineStore } from 'pinia';
import { safeInvoke } from './tauri';

export const useSocialStatusStore = defineStore('socialStatuses', {
  state: () => ({
    socialStatuses: [],
    showStatusModal: false,
    newStatusName: '',
    newStatusPoints: 20,
    editingStatusId: null,
    editingStatusName: '',
    editingStatusPoints: 20,
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
            { id: 1, name: 'بدون دخل', count: 0, base_points: 40 },
            { id: 2, name: 'ضعيف الدخل', count: 0, base_points: 20 },
            { id: 3, name: 'متقاعد', count: 0, base_points: 10 },
            { id: 4, name: 'مرض مزمن', count: 0, base_points: 30 },
            { id: 5, name: 'إعاقة', count: 0, base_points: 35 }
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
      this.newStatusPoints = 20;
      this.editingStatusId = null;
      this.editingStatusPoints = 20;
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
        const points = Number(this.newStatusPoints) || 20;
        await safeInvoke('create_social_status', { name, basePoints: points });
        this.newStatusName = '';
        this.newStatusPoints = 20;
        await this.loadSocialStatuses();
      } catch (e) {
        this.statusError = String(e);
        throw e;
      }
    },

    startEditStatus(item) {
      this.editingStatusId = item.id;
      this.editingStatusName = item.name;
      this.editingStatusPoints = item.base_points !== undefined ? item.base_points : 20;
      this.statusError = '';
    },

    cancelEditStatus() {
      this.editingStatusId = null;
      this.editingStatusName = '';
      this.editingStatusPoints = 20;
    },

    async saveEditStatus(item) {
      const name = this.editingStatusName.trim();
      if (!name) return;
      const points = Number(this.editingStatusPoints) || 20;
      if (name === item.name && points === item.base_points) {
        this.cancelEditStatus();
        return;
      }
      this.statusError = '';
      try {
        await safeInvoke('update_social_status', { id: item.id, newName: name, basePoints: points });
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
