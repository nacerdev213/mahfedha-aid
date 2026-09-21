import { defineStore } from 'pinia';
import { safeInvoke } from './tauri';

export const useCampaignStore = defineStore('campaigns', {
  state: () => ({
    campaigns: [],
    selectedCampaignId: null,
    showCampaignModal: false,
    newCampaignYear: '',
    rolloverPrevious: true,
    isLoading: false,
  }),

  getters: {
    activeCampaign: (state) => {
      return state.campaigns.find(c => c.id === state.selectedCampaignId) || null;
    },
    activeCampaignLabel: (state) => {
      const c = state.campaigns.find(item => item.id === state.selectedCampaignId);
      return c ? c.year_label : '';
    }
  },

  actions: {
    async loadCampaigns() {
      this.isLoading = true;
      try {
        const res = await safeInvoke('get_campaigns');
        if (Array.isArray(res)) {
          this.campaigns = res;
          if (this.campaigns.length > 0) {
            if (!this.selectedCampaignId || !this.campaigns.some(c => c.id === this.selectedCampaignId)) {
              const active = this.campaigns.find(c => c.is_active) || this.campaigns[0];
              this.selectedCampaignId = active.id;
            }
          }
        }
      } catch (e) {
        console.error('فشل جلب المواسم الدراسية:', e);
      } finally {
        this.isLoading = false;
      }
    },

    setSelectedCampaign(id) {
      this.selectedCampaignId = id;
    },

    openCampaignModal() {
      const currentYear = new Date().getFullYear();
      this.newCampaignYear = `${currentYear}/${currentYear + 1}`;
      this.rolloverPrevious = true;
      this.showCampaignModal = true;
    },

    closeCampaignModal() {
      this.showCampaignModal = false;
    },

    async createCampaign() {
      if (!this.newCampaignYear.trim()) {
        throw new Error('يرجى كتابة تسمية الموسم الدراسي');
      }

      // Auto-format: if user typed a bare 4-digit year like "2027", expand to "2027/2028"
      const raw = this.newCampaignYear.trim();
      const yearLabel = /^\d{4}$/.test(raw)
        ? `${raw}/${parseInt(raw) + 1}`
        : raw;

      this.isLoading = true;
      try {
        const newCamp = await safeInvoke('create_campaign', {
          yearLabel: yearLabel,
          rolloverFromId: this.rolloverPrevious ? this.selectedCampaignId : null
        });

        await this.loadCampaigns();
        if (newCamp && newCamp.id) {
          this.selectedCampaignId = newCamp.id;
        }
        this.showCampaignModal = false;
        return newCamp;
      } finally {
        this.isLoading = false;
      }
    }
  }
});
