import { defineStore } from 'pinia';
import { safeInvoke } from './tauri';

export const useEducationLevelStore = defineStore('educationLevels', {
  state: () => ({
    educationLevels: [],
    educationLevelStats: [],
    isLoading: false,
  }),

  getters: {
    // Group levels by stage (ابتدائي، متوسط، ثانوي)
    groupedByStage: (state) => {
      const groups = {};
      for (const level of state.educationLevels) {
        if (!groups[level.stage]) {
          groups[level.stage] = [];
        }
        groups[level.stage].push(level);
      }
      return groups;
    },

    // Ordered stages for display
    orderedStages: (state) => {
      const stageOrder = { 'ابتدائي': 1, 'متوسط': 2, 'ثانوي': 3 };
      const stages = [...new Set(state.educationLevels.map(l => l.stage))];
      return stages.sort((a, b) => (stageOrder[a] || 99) - (stageOrder[b] || 99));
    },

    // Stats grouped by stage
    statsGroupedByStage: (state) => {
      const groups = {};
      for (const stat of state.educationLevelStats) {
        if (!groups[stat.stage]) {
          groups[stat.stage] = {
            stage: stat.stage,
            levels: [],
            total: 0,
            delivered: 0,
            pending: 0,
          };
        }
        groups[stat.stage].levels.push(stat);
        groups[stat.stage].total += (stat.children_count || 0);
        groups[stat.stage].delivered += (stat.delivered_count || 0);
        groups[stat.stage].pending += (stat.pending_count || 0);
      }
      return groups;
    },

    totalChildrenInStats: (state) => {
      return state.educationLevelStats.reduce((sum, s) => sum + (s.children_count || 0), 0);
    },

    totalDeliveredInStats: (state) => {
      return state.educationLevelStats.reduce((sum, s) => sum + (s.delivered_count || 0), 0);
    },

    totalPendingInStats: (state) => {
      return state.educationLevelStats.reduce((sum, s) => sum + (s.pending_count || 0), 0);
    },
  },

  actions: {
    async fetchAll() {
      this.isLoading = true;
      try {
        const data = await safeInvoke('get_education_levels', {});
        if (Array.isArray(data)) {
          this.educationLevels = data;
        }
      } catch (e) {
        console.error('فشل جلب الأطوار الدراسية:', e);
      } finally {
        this.isLoading = false;
      }
    },

    async fetchStats(campaignId) {
      if (!campaignId) return;
      try {
        const data = await safeInvoke('get_education_level_stats', { campaignId });
        if (Array.isArray(data)) {
          this.educationLevelStats = data;
        }
      } catch (e) {
        console.error('فشل جلب إحصائيات الأطوار الدراسية:', e);
      }
    },

    async create(stage, yearName, yearOrder) {
      const result = await safeInvoke('create_education_level', {
        stage: stage.trim(),
        yearName: yearName.trim(),
        yearOrder: Number(yearOrder),
      });
      await this.fetchAll();
      return result;
    },

    async update(id, yearName) {
      await safeInvoke('update_education_level', {
        id,
        yearName: yearName.trim(),
      });
      await this.fetchAll();
    },

    async remove(id) {
      await safeInvoke('delete_education_level', { id });
      await this.fetchAll();
    },
  },
});
