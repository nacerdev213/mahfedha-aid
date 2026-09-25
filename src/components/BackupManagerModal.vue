<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-2 sm:p-4 bg-slate-900/60 backdrop-blur-xs overflow-y-auto no-print"
    dir="rtl">
    <div
      class="bg-white rounded-3xl shadow-2xl border border-slate-200 w-[96vw] max-w-6xl overflow-hidden my-auto animate-fade-in flex flex-col h-[92vh]">

      <!-- Modal Header -->
      <div
        class="px-6 py-4.5 bg-gradient-to-r from-slate-950 via-indigo-950 to-slate-900 text-white flex items-center justify-between shrink-0 border-b border-indigo-900/50">
        <div class="flex items-center gap-3.5">
          <div
            class="w-12 h-12 rounded-2xl bg-white/10 flex items-center justify-center text-2xl shadow-inner border border-white/15">
            💾
          </div>
          <div>
            <h2 class="text-xl sm:text-2xl font-black text-white pt-3 tracking-wide">النسخ الاحتياطي والاسترجاع والأمان
            </h2>
            <p class="text-xs sm:text-sm text-indigo-200 font-medium mt-0.5 p-2">
              نظام التشفير (AES-256-GCM) لحماية الأرشيف الشامل وسجلات المستفيدين والصور
            </p>
          </div>
        </div>
        <button type="button" @click="$emit('close')" :disabled="isProcessing"
          class="text-white/70 hover:text-white hover:bg-white/15 rounded-xl p-2 transition cursor-pointer disabled:opacity-50"
          title="إغلاق">
          <span class="text-2xl leading-none font-bold">✕</span>
        </button>
      </div>

      <!-- Modal Body (Scrollable, Responsive Grid) -->
      <div class="p-6 overflow-y-auto flex-1 space-y-6 text-slate-800">

        <!-- Top Grid: Section 1 (Export) & Section 2 (Restore) -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">

          <!-- ======================================================== -->
          <!-- SECTION 1: Export Encrypted Backup (.scaid)              -->
          <!-- ======================================================== -->
          <section
            class="bg-gradient-to-br from-indigo-50/70 to-indigo-100/30 border border-indigo-200/90 rounded-2xl p-5 flex flex-col justify-between space-y-4 shadow-xs">
            <div class="space-y-4">
              <div class="flex items-start justify-between gap-3 border-b border-indigo-100 pb-3">
                <div class="flex items-center gap-3">
                  <span class="text-3xl">📦</span>
                  <div>
                    <h3 class="font-black text-indigo-950 text-lg sm:text-xl">إنشاء نسخة احتياطية مشفرة</h3>
                    <p class="text-xs sm:text-sm text-indigo-700 font-medium mt-0.5">
                      تصدير أرشيف محصن (<code
                        class="font-mono font-black bg-indigo-200/60 px-1.5 py-0.5 rounded text-indigo-950">.scaid</code>)
                      لكافة السجلات والصور
                    </p>
                  </div>
                </div>
                <span
                  class="bg-indigo-600 text-white text-xs font-bold px-3 py-1 rounded-full whitespace-nowrap shadow-xs">
                  AES-256-GCM
                </span>
              </div>

              <div
                class="bg-white rounded-xl p-4 border border-indigo-100 shadow-2xs space-y-2.5 text-xs sm:text-sm text-slate-700 leading-relaxed">
                <div class="flex items-start gap-2.5">
                  <span class="text-emerald-600 font-black text-base shrink-0 mt-0.5">✓</span>
                  <span>يشمل كافة السجلات العائلية، المواسم، الحالات الاجتماعية، ومعلومات وإعدادات الجمعية.</span>
                </div>
                <div class="flex items-start gap-2.5">
                  <span class="text-emerald-600 font-black text-base shrink-0 mt-0.5">✓</span>
                  <span>يحفظ جميع الصور الشخصية المرفقة للمستفيدين (<code
                      class="font-mono font-bold text-slate-900 bg-slate-100 px-1 rounded">avatars/*.webp</code>) بدقة
                    كاملة.</span>
                </div>
                <div class="flex items-start gap-2.5">
                  <span class="text-indigo-600 font-black text-base shrink-0 mt-0.5">🛡️</span>
                  <span>الملف محمي بتشفير سيادي مغلق، غير قابل للاختراق أو الفتح ببرامج الأرشفة العادية (WinRAR /
                    7-Zip).</span>
                </div>
              </div>
            </div>

            <!-- Export Action Button -->
            <div class="pt-2 flex items-center justify-between gap-3 flex-wrap border-t border-indigo-100">
              <button type="button" @click="handleExportBackup" :disabled="isProcessing"
                class="w-full sm:w-auto bg-indigo-600 hover:bg-indigo-700 text-white font-black text-sm sm:text-base px-6 py-3 rounded-xl shadow-md transition flex items-center justify-center gap-2.5 cursor-pointer active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed">
                <span v-if="exportLoading" class="animate-spin text-base">⏳</span>
                <span v-else class="text-lg">💾</span>
                <span>تصدير نسخة احتياطية مشفرة (.scaid)</span>
              </button>

              <span v-if="lastExportResult"
                class="text-emerald-800 text-xs sm:text-sm font-black bg-emerald-50 border border-emerald-300 px-3 py-1.5 rounded-xl shadow-2xs">
                ✓ تم التصدير بنجاح
              </span>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 2: Safe Restore from Encrypted Backup            -->
          <!-- ======================================================== -->
          <section
            class="bg-gradient-to-br from-slate-50 to-slate-100/50 border border-slate-200 rounded-2xl p-5 flex flex-col justify-between space-y-4 shadow-xs">
            <div class="space-y-4">
              <div class="flex items-start justify-between gap-3 border-b border-slate-200 pb-3">
                <div class="flex items-center gap-3">
                  <span class="text-3xl">📥</span>
                  <div>
                    <h3 class="font-black text-slate-900 text-lg sm:text-xl">استرجاع نسخة احتياطية سابقة</h3>
                    <p class="text-xs sm:text-sm text-slate-600 font-medium mt-0.5">
                      استيراد واسترجاع كامل البيانات والصور من ملف أرشيف مشفر
                    </p>
                  </div>
                </div>
                <span
                  class="bg-emerald-600 text-white text-xs font-bold px-3 py-1 rounded-full whitespace-nowrap shadow-xs">
                  أمان استباقي تلقائي
                </span>
              </div>

              <!-- Explicit Security Alert -->
              <div class="bg-amber-50 border-2 border-amber-300 rounded-xl p-4 text-amber-950 shadow-2xs space-y-2">
                <div class="font-black flex items-center gap-2 text-sm sm:text-base text-amber-900">
                  <span class="text-xl">⚠️</span>
                  <span>تنبيه أمني هام جداً:</span>
                </div>
                <p class="text-xs sm:text-sm text-amber-900 leading-relaxed font-medium">
                  ستستبدل هذه العملية كافة البيانات الحالية بالنسخة المحددة. سيقوم البرنامج <strong>بإنشاء نسخة أمان
                    تلقائية للبيانات الحالية</strong> قبل الاسترجاع، لضمان حماية بياناتكم تحت كل الظروف.
                </p>
              </div>
            </div>

            <!-- Restore Action Button -->
            <div class="pt-2 border-t border-slate-200">
              <button type="button" @click="handleRestoreBackup" :disabled="isProcessing"
                class="w-full sm:w-auto bg-slate-900 hover:bg-slate-800 text-white font-black text-sm sm:text-base px-6 py-3 rounded-xl shadow-md transition flex items-center justify-center gap-2.5 cursor-pointer active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed">
                <span v-if="restoreLoading" class="animate-spin text-base">⏳</span>
                <span v-else class="text-lg">📥</span>
                <span>استرجاع نسخة احتياطية</span>
              </button>
            </div>
          </section>

        </div>

        <!-- ======================================================== -->
        <!-- SECTION 3: Danger Zone - Factory Reset                   -->
        <!-- ======================================================== -->
        <section
          class="bg-gradient-to-br from-rose-50/80 via-white to-rose-50/50 border-2 border-rose-300 rounded-2xl p-5 space-y-4 shadow-sm">
          <div class="flex items-start justify-between gap-3 border-b border-rose-200 pb-3">
            <div class="flex items-center gap-3">
              <span class="text-3xl">⚠️</span>
              <div>
                <h3 class="font-black text-rose-950 text-lg sm:text-xl">منطقة الخطر: إعادة ضبط المصنع (Factory Reset)
                </h3>
                <p class="text-xs sm:text-sm text-rose-700 font-medium mt-0.5">
                  تصفير شامل للبرنامج وحذف كافة السجلات والصور للبدء بقاعدة بيانات جديدة كلياً
                </p>
              </div>
            </div>
            <span
              class="bg-rose-600 text-white text-xs font-black px-3.5 py-1 rounded-full whitespace-nowrap shadow-xs">
              إجراء حساس
            </span>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-3 text-xs sm:text-sm text-rose-950">
            <div class="bg-white/90 p-3.5 rounded-xl border border-rose-200 shadow-2xs">
              <span class="font-bold block text-rose-800 mb-1">🗑️ حذف شامل للمستفيدين</span>
              <span>حذف كافة المستفيدين وسجلات الحملات والأطفال المتمدرسين والصور الشخصية نهائياً.</span>
            </div>
            <div class="bg-white/90 p-3.5 rounded-xl border border-rose-200 shadow-2xs">
              <span class="font-bold block text-rose-800 mb-1">⚙️ استرجاع الضبط الافتراضي</span>
              <span>استعادة الحالات الاجتماعية الـ 5 المعتمدة مع نقاط استحقاقها وإعدادات الجمعية النظيفة.</span>
            </div>
            <div class="bg-white/90 p-3.5 rounded-xl border border-rose-200 shadow-2xs">
              <span class="font-bold block text-rose-800 mb-1">🛡️ نسخة أمان وقائية</span>
              <span>يقوم البرنامج تلقائياً بإنشاء نسخة احتياطية كاملة في مجلد الأمان قبل الشروع في التصفير.</span>
            </div>
          </div>

          <!-- Danger Trigger Button -->
          <div class="pt-2 flex items-center justify-between border-t border-rose-200 flex-wrap gap-3">
            <button type="button" @click="openResetConfirmDialog" :disabled="isProcessing"
              class="bg-rose-600 hover:bg-rose-700 text-white font-black text-sm sm:text-base px-6 py-3 rounded-xl shadow-md transition flex items-center gap-2.5 cursor-pointer active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed">
              <span class="text-lg">⚠️</span>
              <span>إعادة ضبط المصنع</span>
            </button>
            <span class="text-xs sm:text-sm font-bold text-rose-800">
              * يتطلب تأكيداً إضافياً بكتابة عبارة التأكيد المحددة
            </span>
          </div>
        </section>

      </div>

      <!-- Modal Footer -->
      <div
        class="px-6 py-3.5 bg-slate-50 border-t border-slate-200 flex items-center justify-between text-xs sm:text-sm text-slate-600 shrink-0">
        <span class="flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
          <span>نظام التشفير والحماية: <strong class="text-slate-900 font-mono font-bold">AES-256-GCM
              مُفعّل</strong></span>
        </span>
        <button type="button" @click="$emit('close')" :disabled="isProcessing"
          class="px-6 py-2 text-xs sm:text-sm font-bold text-slate-700 hover:text-slate-900 hover:bg-slate-200 rounded-xl transition cursor-pointer">
          إغلاق النافذة
        </button>
      </div>

    </div>

    <!-- ======================================================== -->
    <!-- Sub-Dialog: Factory Reset Confirmation Confirmation      -->
    <!-- ======================================================== -->
    <div v-if="showResetModal"
      class="fixed inset-0 z-60 flex items-center justify-center p-3 bg-slate-900/70 backdrop-blur-xs animate-fade-in"
      dir="rtl">
      <div
        class="bg-white rounded-3xl shadow-2xl border-2 border-rose-500 w-full max-w-lg overflow-hidden p-6 space-y-5">
        <div class="flex items-center gap-3.5 text-rose-600 border-b border-rose-100 pb-3.5">
          <span class="text-4xl">⚠️</span>
          <div>
            <h3 class="font-black text-rose-950 text-lg sm:text-xl">تأكيد التصفير الشامل</h3>
            <p class="text-xs sm:text-sm text-rose-700 font-medium">هذا الإجراء سيقوم بتصفير قاعدة البيانات بالكامل</p>
          </div>
        </div>

        <div class="text-xs sm:text-sm text-slate-800 space-y-3 leading-relaxed">
          <p class="font-bold text-slate-900">
            لإتمام إعادة ضبط المصنع، يرجى كتابة العبارة التالية بالضبط في الحقل أدناه لتأكيد الإجراء:
          </p>
          <div
            class="text-center py-3 bg-rose-50 border-2 border-dashed border-rose-300 rounded-2xl font-black text-rose-800 text-lg sm:text-xl select-all tracking-wider shadow-inner">
            تصفير شامل
          </div>
        </div>

        <!-- Input Field -->
        <div>
          <input type="text" v-model="resetConfirmationPhrase" placeholder="اكتب: تصفير شامل"
            class="w-full text-center font-black text-base sm:text-lg bg-white border-2 border-slate-300 rounded-2xl py-3 px-4 focus:ring-2 focus:ring-rose-500 focus:border-rose-500 focus:outline-none text-slate-900 placeholder:text-slate-400 shadow-2xs"
            @keyup.enter="isPhraseMatch && handleExecuteReset()" />
        </div>

        <!-- Dialog Buttons -->
        <div class="flex items-center justify-end gap-3 pt-3 border-t border-slate-100">
          <button type="button" @click="closeResetConfirmDialog" :disabled="resetLoading"
            class="px-5 py-2.5 text-xs sm:text-sm font-bold text-slate-700 hover:bg-slate-100 rounded-xl transition cursor-pointer">
            إلغاء وتراجع
          </button>
          <button type="button" @click="handleExecuteReset" :disabled="!isPhraseMatch || resetLoading"
            class="px-6 py-2.5 text-xs sm:text-sm font-black text-white bg-rose-600 hover:bg-rose-700 rounded-xl shadow-md transition cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed flex items-center gap-2">
            <span v-if="resetLoading" class="animate-spin text-sm">⏳</span>
            <span>تأكيد التصفير النهائي</span>
          </button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { safeInvoke } from '../stores/tauri';
import { useCampaignStore } from '../stores/campaigns';
import { useSocialStatusStore } from '../stores/socialStatuses';
import { useOrganizationStore } from '../stores/organization';
import { useBeneficiaryStore } from '../stores/beneficiaries';
import { useEducationLevelStore } from '../stores/educationLevels';
import { notifySuccess, notifyError, confirmDelete } from '../utils/alerts';

const emit = defineEmits(['close', 'restored']);

// Stores
const campaignStore = useCampaignStore();
const statusStore = useSocialStatusStore();
const orgStore = useOrganizationStore();
const beneficiaryStore = useBeneficiaryStore();
const educationLevelStore = useEducationLevelStore();

// Loading states
const exportLoading = ref(false);
const restoreLoading = ref(false);
const resetLoading = ref(false);
const isProcessing = computed(() => exportLoading.value || restoreLoading.value || resetLoading.value);

const lastExportResult = ref(null);

// Reset confirmation modal state
const showResetModal = ref(false);
const resetConfirmationPhrase = ref('');
const isPhraseMatch = computed(() => resetConfirmationPhrase.value.trim() === 'تصفير شامل');

const openResetConfirmDialog = () => {
  resetConfirmationPhrase.value = '';
  showResetModal.value = true;
};

const closeResetConfirmDialog = () => {
  showResetModal.value = false;
  resetConfirmationPhrase.value = '';
};

// Helper: Reload all stores across the application
const reloadAllStores = async () => {
  await Promise.all([
    campaignStore.loadCampaigns(),
    statusStore.loadSocialStatuses(),
    orgStore.loadOrgSettings(),
    educationLevelStore.fetchAll(),
  ]);
  if (campaignStore.selectedCampaignId) {
    await Promise.all([
      beneficiaryStore.fetchData(campaignStore.selectedCampaignId),
      educationLevelStore.fetchStats(campaignStore.selectedCampaignId),
    ]);
  }
};

// ========================================================
// Action 1: Export Backup
// ========================================================
const handleExportBackup = async () => {
  if (isProcessing.value) return;

  try {
    let saveDialog = null;
    if (window.__TAURI__) {
      const dialogMod = await import('@tauri-apps/api/dialog');
      saveDialog = dialogMod.save;
    }

    const todayStr = new Date().toISOString().slice(0, 10);
    const defaultFileName = `نسخة_احتياطية_شاملة_${todayStr}.scaid`;

    let selectedPath = null;
    if (saveDialog) {
      selectedPath = await saveDialog({
        title: 'اختر مكان حفظ ملف النسخة الاحتياطية المشفرة',
        defaultPath: defaultFileName,
        filters: [{
          name: 'School Aid Encrypted Backup',
          extensions: ['scaid']
        }]
      });
    } else {
      selectedPath = defaultFileName;
    }

    if (!selectedPath) {
      return; // User cancelled save dialog
    }

    exportLoading.value = true;

    // Call Rust backend command create_backup
    const resPath = await safeInvoke('create_backup', { targetPath: selectedPath });

    lastExportResult.value = {
      path: resPath,
      sizeFormatted: 'أرشيف مشفر'
    };

    notifySuccess(
      'تم إنشاء النسخة الاحتياطية بنجاح',
      `تم تصدير وحفظ الأرشيف المشفر بنجاح في:\n${resPath}`
    );
  } catch (err) {
    console.error('فشل تصدير النسخة الاحتياطية:', err);
    notifyError('خطأ أثناء إنشاء النسخة الاحتياطية', err?.toString() || 'حدث خطأ غير متوقع');
  } finally {
    exportLoading.value = false;
  }
};

// ========================================================
// Action 2: Restore Backup
// ========================================================
const handleRestoreBackup = async () => {
  if (isProcessing.value) return;

  try {
    let openDialog = null;
    if (window.__TAURI__) {
      const dialogMod = await import('@tauri-apps/api/dialog');
      openDialog = dialogMod.open;
    }

    let selectedFilePath = null;
    if (openDialog) {
      selectedFilePath = await openDialog({
        title: 'اختر ملف النسخة الاحتياطية المشفرة (.scaid)',
        multiple: false,
        filters: [{
          name: 'School Aid Encrypted Backup',
          extensions: ['scaid']
        }]
      });
    }

    if (!selectedFilePath) {
      return; // User cancelled file picker
    }

    const pathStr = Array.isArray(selectedFilePath) ? selectedFilePath[0] : selectedFilePath;

    // Show explicit security warning dialog before proceeding
    const confirmed = await confirmDelete(
      '⚠️ تأكيد استرجاع النسخة الاحتياطية',
      'تنبيه أمني: ستستبدل هذه العملية كافة البيانات الحالية بالنسخة المحددة. سيقوم البرنامج تلقائياً بإنشاء نسخة أمان للبيانات الحالية قبل الاسترجاع. هل ترغب في المتابعة؟'
    );

    if (!confirmed) {
      return;
    }

    restoreLoading.value = true;

    // Call Rust backend restore_backup
    const summaryMsg = await safeInvoke('restore_backup', { backupFilePath: pathStr });

    // Refresh all Pinia stores
    await reloadAllStores();

    emit('restored');

    notifySuccess(
      'تم استرجاع البيانات بنجاح',
      summaryMsg || 'تم استرجاع كافة السجلات والصور بنجاح!'
    );
  } catch (err) {
    console.error('فشل استرجاع النسخة الاحتياطية:', err);
    notifyError('فشل الاسترجاع', err?.toString() || 'حدث خطأ أثناء فك تشفير أو استرجاع الأرشيف');
  } finally {
    restoreLoading.value = false;
  }
};

// ========================================================
// Action 3: Factory Reset
// ========================================================
const handleExecuteReset = async () => {
  if (!isPhraseMatch.value || resetLoading.value) return;

  try {
    resetLoading.value = true;

    await safeInvoke('factory_reset');

    // Reload all stores to reflect pristine initial state
    await reloadAllStores();

    closeResetConfirmDialog();
    emit('restored');

    notifySuccess(
      'تمت إعادة ضبط المصنع بنجاح',
      'تم تصفير قاعدة البيانات وإرجاع البرنامج إلى حالته الأولية النظيفة بنجاح. تم حفظ نسخة وقائية في مجلد backups.'
    );

    emit('close');
  } catch (err) {
    console.error('فشل إعادة ضبط المصنع:', err);
    notifyError('خطأ أثناء إعادة الضبط', err?.toString() || 'حدث خطأ غير متوقع');
  } finally {
    resetLoading.value = false;
  }
};
</script>

<style scoped>
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: scale(0.98);
  }

  to {
    opacity: 1;
    transform: scale(1);
  }
}

.animate-fade-in {
  animation: fadeIn 0.15s ease-out forwards;
}
</style>
