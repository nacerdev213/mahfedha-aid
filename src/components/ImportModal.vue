<template>
  <div v-if="modelValue" class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4 z-50">
    <div class="bg-white rounded-2xl max-w-lg w-full p-6 shadow-xl border border-slate-200 text-right">
      
      <!-- Title -->
      <div class="flex items-center justify-between border-b border-slate-100 pb-3 mb-4">
        <h3 class="text-base font-bold text-slate-900 flex items-center gap-2">
          📥 استيراد قائمة المستفيدين من ملف إكسل
          <span class="text-xs font-normal text-indigo-700 bg-indigo-50 px-2.5 py-0.5 rounded-full">موسم {{ activeCampaignLabel }}</span>
        </h3>
        <button @click="close" class="text-slate-400 hover:text-slate-600 text-lg">✕</button>
      </div>

      <!-- Step 1: Download Dynamic Template -->
      <div class="bg-indigo-50/70 border border-indigo-100 rounded-xl p-4 mb-4 flex items-center justify-between">
        <div>
          <p class="text-xs font-bold text-indigo-950">الخطوة الأولى: القالب المعتمد الديناميكي</p>
          <p class="text-xs text-indigo-700 mt-0.5">يتضمن قائمة منسدلة تلقائية بالحالات الاجتماعية المعتمدة</p>
        </div>
        <button 
          @click="downloadTemplate" 
          :disabled="isGeneratingTemplate"
          class="bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 text-white text-xs font-semibold px-3 py-2 rounded-lg transition shadow-xs flex items-center gap-1"
        >
          <span>📄</span>
          <span>{{ isGeneratingTemplate ? 'جارٍ الإنشاء...' : 'تحميل القالب (.xlsx)' }}</span>
        </button>
      </div>

      <!-- Step 2: Instructions Checklist -->
      <div class="bg-slate-50 border border-slate-200 rounded-xl p-3.5 mb-4 text-xs text-slate-600 space-y-1.5 leading-relaxed">
        <p class="font-bold text-slate-800 mb-1">تعليمات ملء الملف:</p>
        <p>• لا تقم بتغيير أسماء الأعمدة أو ترتيبها في السطر الأول.</p>
        <p>• اختر الحالة الاجتماعية من <strong class="text-indigo-700 font-bold">القائمة المنسدلة المدمجة</strong> في عمود "الحالة الاجتماعية".</p>
        <p>• الحالات المعتمدة حالياً: ({{ activeStatusesList.join('، ') }}).</p>
        <p>• اكتب الأرقام (0، 1، 2...) في خانات الأطوار، ولا تترك الخانات فارغة.</p>
      </div>

      <!-- Step 3: File Input & Upload -->
      <div 
        @dragover.prevent 
        @drop.prevent="handleFileDrop"
        class="border-2 border-dashed border-slate-300 hover:border-indigo-500 rounded-xl p-6 text-center transition cursor-pointer mb-4 bg-slate-50/50 hover:bg-indigo-50/20"
      >
        <input type="file" accept=".xlsx, .xls" @change="handleFileSelect" class="hidden" id="excelFileModal" ref="excelFileInputRef" />
        <label for="excelFileModal" class="cursor-pointer block">
          <span class="text-3xl block mb-1">📊</span>
          <span class="text-xs font-bold text-slate-700 block">
            {{ selectedFileName || 'اضغط هنا لاختيار ملف الإكسل المكتمل أو اسحبه إلى هنا' }}
          </span>
          <span class="text-[11px] text-slate-400 block mt-1">صيغة XLSX أو XLS</span>
        </label>
      </div>

      <!-- Error Message if any -->
      <div v-if="parseError" class="mb-4 text-xs text-red-600 bg-red-50 p-2.5 rounded-lg border border-red-200">
        {{ parseError }}
      </div>

      <!-- Preview & Action Buttons -->
      <div class="flex items-center justify-between border-t border-slate-100 pt-4">
        <div>
          <span v-if="parsedCount > 0" class="text-xs font-bold text-emerald-600 flex items-center gap-1">
            ✓ تم اكتشاف {{ parsedCount }} سجل جاهز للاستيراد
          </span>
        </div>
        <div class="flex gap-2 mr-auto">
          <button @click="close" class="px-4 py-2 text-xs text-slate-600 hover:bg-slate-100 rounded-lg">إلغاء</button>
          <button 
            @click="submitImport" 
            :disabled="parsedCount === 0 || isImporting" 
            class="px-4 py-2 text-xs bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-lg font-bold transition flex items-center gap-1"
          >
            <span v-if="isImporting">جارٍ الاستيراد...</span>
            <span v-else>تأكيد الاستيراد</span>
          </button>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { storeToRefs } from 'pinia';
import ExcelJS from 'exceljs';
import * as XLSX from 'xlsx';
import { safeInvoke } from '../stores/tauri';
import { useCampaignStore } from '../stores/campaigns';
import { useStatusStore } from '../stores/socialStatuses';
import { useBeneficiaryStore } from '../stores/beneficiaries';
import { notifySuccess, notifyError, notifyWarning, toastSuccess } from '../utils/alerts';

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['update:modelValue', 'close', 'imported']);

// Stores
const statusStore = useStatusStore();
const campaignStore = useCampaignStore();
const beneficiaryStore = useBeneficiaryStore();

const { activeCampaignLabel, selectedCampaignId } = storeToRefs(campaignStore);

// State
const isGeneratingTemplate = ref(false);
const selectedFileName = ref('');
const parsedItems = ref([]);
const parsedCount = ref(0);
const parseError = ref('');
const isImporting = ref(false);
const excelFileInputRef = ref(null);

// Active statuses list
const activeStatusesList = computed(() => {
  if (statusStore.statuses && statusStore.statuses.length > 0) {
    return statusStore.statuses.map(s => s.name);
  }
  if (statusStore.socialStatuses && statusStore.socialStatuses.length > 0) {
    return statusStore.socialStatuses.map(s => s.name);
  }
  return ['بدون دخل', 'ضعيف الدخل', 'متقاعد', 'مرض مزمن', 'إعاقة'];
});

const close = () => {
  emit('update:modelValue', false);
  emit('close');
};

/**
 * Save Workbook using Tauri Save Dialog or browser fallback download
 */
const saveWorkbookWithDialog = async (workbook, defaultFileName, dialogTitle) => {
  try {
    let saveDialog = null;
    if (window.__TAURI__) {
      const dialogMod = await import('@tauri-apps/api/dialog');
      saveDialog = dialogMod.save;
    }

    const buffer = await workbook.xlsx.writeBuffer();
    const bytes = Array.from(new Uint8Array(buffer));

    if (saveDialog) {
      const selectedPath = await saveDialog({
        title: dialogTitle || 'اختر مكان حفظ الملف في النظام',
        defaultPath: defaultFileName,
        filters: [{
          name: 'Excel Workbook',
          extensions: ['xlsx']
        }]
      });

      if (!selectedPath) {
        return false;
      }

      await safeInvoke('write_binary_file', {
        path: selectedPath,
        contents: bytes
      });

      notifySuccess('تم حفظ الملف بنجاح', `تم حفظ الملف بنجاح في:\n${selectedPath}`);
      return true;
    } else {
      // Browser fallback
      const blob = new Blob([buffer], { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = defaultFileName;
      a.click();
      URL.revokeObjectURL(url);
      toastSuccess('تم تنزيل الملف بنجاح');
      return true;
    }
  } catch (err) {
    console.warn('تعذر فتح نافذة الحفظ بالنظام، جاري التحويل للتنزيل التلقائي:', err);
    try {
      const buffer = await workbook.xlsx.writeBuffer();
      const blob = new Blob([buffer], { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = defaultFileName;
      a.click();
      URL.revokeObjectURL(url);
      toastSuccess('تم تنزيل الملف بنجاح');
      return true;
    } catch (e) {
      notifyError('خطأ أثناء حفظ الملف', e.message || e);
      return false;
    }
  }
};

/**
 * Dynamic Excel Template Generator with Native Dropdown Data Validation
 */
const downloadTemplate = async () => {
  isGeneratingTemplate.value = true;
  try {
    const activeStatuses = activeStatusesList.value;

    const workbook = new ExcelJS.Workbook();
    workbook.creator = 'منظومة إحصاء وتوزيع المحافظ المدرسية';
    workbook.created = new Date();

    // Add main worksheet with Right-to-Left (RTL) enabled
    const worksheet = workbook.addWorksheet('المستفيدين', {
      views: [{ rightToLeft: true }]
    });

    // Configure Columns
    worksheet.columns = [
      { header: 'الرقم', key: 'record_no', width: 10 },
      { header: 'اسم ولقب ولي الأمر', key: 'guardian_name', width: 28 },
      { header: 'الهاتف', key: 'phone', width: 18 },
      { header: 'الحالة الاجتماعية', key: 'social_status', width: 22 },
      { header: 'ابتدائي', key: 'primary_count', width: 12 },
      { header: 'متوسط', key: 'middle_count', width: 12 },
      { header: 'ثانوي', key: 'secondary_count', width: 12 }
    ];

    // Style Header Row
    const headerRow = worksheet.getRow(1);
    headerRow.height = 28;
    headerRow.font = { name: 'Segoe UI', size: 11, bold: true, color: { argb: 'FFFFFFFF' } };
    headerRow.fill = {
      type: 'pattern',
      pattern: 'solid',
      fgColor: { argb: 'FF4F46E5' } // Indigo 600
    };
    headerRow.alignment = { vertical: 'middle', horizontal: 'center' };

    // Dynamic sample rows using currently active statuses
    const sampleRows = [
      {
        record_no: 1,
        guardian_name: 'محمد بن عبد الله',
        phone: '0550123456',
        social_status: activeStatuses[0] || 'بدون دخل',
        primary_count: 1,
        middle_count: 1,
        secondary_count: 0
      },
      {
        record_no: 2,
        guardian_name: 'فاطمة الزهراء قاسمي',
        phone: '0661987654',
        social_status: activeStatuses[1] || activeStatuses[0] || 'ضعيف الدخل',
        primary_count: 2,
        middle_count: 0,
        secondary_count: 1
      },
      {
        record_no: 3,
        guardian_name: 'أحمد بلقاسم',
        phone: '0770345678',
        social_status: activeStatuses[2] || activeStatuses[0] || 'متقاعد',
        primary_count: 0,
        middle_count: 1,
        secondary_count: 1
      },
      {
        record_no: 4,
        guardian_name: 'علي بوزيد',
        phone: '0560234567',
        social_status: activeStatuses[3] || activeStatuses[0] || 'مرض مزمن',
        primary_count: 1,
        middle_count: 0,
        secondary_count: 0
      },
      {
        record_no: 5,
        guardian_name: 'خالد دريدي',
        phone: '0670345678',
        social_status: activeStatuses[4] || activeStatuses[0] || 'إعاقة',
        primary_count: 0,
        middle_count: 0,
        secondary_count: 2
      }
    ];

    sampleRows.forEach(item => worksheet.addRow(item));

    // Style data rows
    for (let r = 2; r <= worksheet.rowCount; r++) {
      const row = worksheet.getRow(r);
      row.height = 22;
      row.font = { name: 'Segoe UI', size: 10 };
      row.alignment = { vertical: 'middle' };
      row.getCell('record_no').alignment = { vertical: 'middle', horizontal: 'center' };
      row.getCell('guardian_name').alignment = { vertical: 'middle', horizontal: 'right' };
      row.getCell('phone').alignment = { vertical: 'middle', horizontal: 'center' };
      row.getCell('social_status').alignment = { vertical: 'middle', horizontal: 'center' };
      row.getCell('primary_count').alignment = { vertical: 'middle', horizontal: 'center' };
      row.getCell('middle_count').alignment = { vertical: 'middle', horizontal: 'center' };
      row.getCell('secondary_count').alignment = { vertical: 'middle', horizontal: 'center' };
    }

    // Apply native Excel Data Validation Dropdown to column C (rows 2 to 1000)
    const validationFormula = `"${activeStatuses.join(',')}"`;
    for (let r = 2; r <= 1000; r++) {
      worksheet.getCell(`C${r}`).dataValidation = {
        type: 'list',
        allowBlank: true,
        formulae: [validationFormula],
        showErrorMessage: true,
        errorTitle: 'حالة اجتماعية غير صالحة',
        error: 'يرجى اختيار إحدى الحالات الاجتماعية المعتمدة من القائمة المنسدلة.'
      };
    }

    await saveWorkbookWithDialog(workbook, 'قالب_استيراد_المستفيدين.xlsx', 'اختر مكان حفظ قالب استيراد المستفيدين');
  } catch (e) {
    console.error('فشل إنشاء قالب الإكسل:', e);
    notifyError('خطأ أثناء إنشاء القالب', e.message || e);
  } finally {
    isGeneratingTemplate.value = false;
  }
};

/**
 * Drag and Drop & File Upload handling
 */
const handleFileDrop = (e) => {
  const files = e.dataTransfer?.files;
  if (files && files.length > 0) {
    processExcelFile(files[0]);
  }
};

const handleFileSelect = (e) => {
  const files = e.target.files;
  if (files && files.length > 0) {
    processExcelFile(files[0]);
  }
};

const processExcelFile = async (file) => {
  selectedFileName.value = file.name;
  parseError.value = '';
  parsedItems.value = [];
  parsedCount.value = 0;

  try {
    const data = await file.arrayBuffer();
    const workbook = XLSX.read(data, { type: 'array' });

    let targetSheetName = workbook.SheetNames.find(n => n.includes('المستفيد') || n.includes('جدول البيانات') || n.includes('البيانات'));
    if (!targetSheetName) {
      targetSheetName = workbook.SheetNames[0];
    }

    const worksheet = workbook.Sheets[targetSheetName];
    if (!worksheet) {
      parseError.value = 'لم يتم العثور على ورقة بيانات صالحة في الملف.';
      return;
    }

    const rows = XLSX.utils.sheet_to_json(worksheet, { header: 1, defval: '' });
    if (!rows || rows.length < 2) {
      parseError.value = 'الملف لا يحتوي على صفوف بيانات كافية (يجب أن يحتوي على سطر العناوين وسطر بيانات واحد على الأقل).';
      return;
    }

    const headerRow = rows[0].map(h => String(h).trim().toLowerCase());
    
    const findColIdx = (keywords) => {
      return headerRow.findIndex(h => keywords.some(k => h.includes(k.toLowerCase())));
    };

    const noIdx = findColIdx(['الرقم', 'رقم', 'رقم الملف', 'رقم المستفيد', 'no', 'num']);
    const nameIdx = findColIdx(['اسم ولقب', 'الاسم واللقب', 'ولي الأمر', 'الاسم', 'إسم و لقب', 'الاسم']);
    const phoneIdx = findColIdx(['هاتف', 'الهاتف', 'المحمول', 'الجوال', 'phone']);
    const statusIdx = findColIdx(['حالة اجتماعية', 'الحالة الاجتماعية', 'الحالة', 'الوضعية', 'status']);
    const primaryIdx = findColIdx(['ابتدائي', 'إبتدائي', 'الابتدائي', 'الإبتدائي', 'primary']);
    const middleIdx = findColIdx(['متوسط', 'المتوسط', 'إكمالي', 'الإكمالي', 'middle']);
    const secondaryIdx = findColIdx(['ثانوي', 'الثانوي', 'secondary']);

    if (nameIdx === -1) {
      parseError.value = 'تعذر العثور على عمود "اسم ولقب ولي الأمر" في السطر الأول للملف.';
      return;
    }

    const validStatuses = activeStatusesList.value;
    const defaultStatus = validStatuses[0] || 'بدون دخل';
    const items = [];

    for (let i = 1; i < rows.length; i++) {
      const row = rows[i];
      if (!row || row.length === 0) continue;

      const guardianName = nameIdx !== -1 && row[nameIdx] ? String(row[nameIdx]).trim() : '';
      if (!guardianName) continue;

      let recordNo = noIdx !== -1 && row[noIdx] !== undefined && row[noIdx] !== '' ? parseInt(row[noIdx], 10) : null;
      if (isNaN(recordNo) || recordNo <= 0) recordNo = null;

      const phone = phoneIdx !== -1 && row[phoneIdx] !== undefined ? String(row[phoneIdx]).trim() : '';
      let socialStatus = statusIdx !== -1 && row[statusIdx] ? String(row[statusIdx]).trim() : defaultStatus;

      const matchedStatus = validStatuses.find(s => socialStatus.includes(s));
      if (matchedStatus) {
        socialStatus = matchedStatus;
      } else if (!validStatuses.includes(socialStatus)) {
        socialStatus = defaultStatus;
      }

      const parseNum = (val) => {
        const num = parseInt(val, 10);
        return isNaN(num) || num < 0 ? 0 : num;
      };

      items.push({
        id: null,
        record_no: recordNo,
        guardian_id: null,
        guardian_name: guardianName,
        phone: phone,
        social_status: socialStatus,
        primary_count: primaryIdx !== -1 ? parseNum(row[primaryIdx]) : 0,
        middle_count: middleIdx !== -1 ? parseNum(row[middleIdx]) : 0,
        secondary_count: secondaryIdx !== -1 ? parseNum(row[secondaryIdx]) : 0
      });
    }

    if (items.length === 0) {
      parseError.value = 'تمت قراءة الملف ولكن جميع صفوف البيانات فارغة من أسماء أولياء الأمور.';
      return;
    }

    parsedItems.value = items;
    parsedCount.value = items.length;
  } catch (err) {
    console.error('فشل معالجة ملف الإكسل:', err);
    parseError.value = 'حدث خطأ أثناء قراءة الملف: ' + (err.message || err);
  }
};

const submitImport = async () => {
  if (parsedItems.value.length === 0) return;
  if (!selectedCampaignId.value) {
    notifyWarning('تنبيه', 'يرجى تحديد موسم دراسي أولاً.');
    return;
  }

  isImporting.value = true;
  try {
    const count = await beneficiaryStore.batchImport(selectedCampaignId.value, parsedItems.value);
    notifySuccess('اكتمل الاستيراد بنجاح', `تم بنجاح استيراد ${count} مستفيد إلى موسم ${activeCampaignLabel.value}!`);
    close();
    await statusStore.loadSocialStatuses();
    emit('imported', count);
  } catch (e) {
    notifyError('خطأ أثناء الاستيراد', e.message || e);
  } finally {
    isImporting.value = false;
  }
};
</script>
