<template>
  <div class="school-form-page bg-white text-slate-900 font-sans p-6 md:p-8" dir="rtl">
    <!-- Header Section -->
    <div class="flex items-start justify-between gap-4 mb-4">
      <!-- Right: Religious Affairs & Mosque info -->
      <div class="text-right text-xs leading-relaxed font-bold text-slate-800 shrink-0">
        <div>مديرية الشؤون الدينية والأوقاف</div>
        <div>{{ orgSettings?.org_name || 'مؤسسة المسجد' }}</div>
        <div>مجلس سبل الخيرات</div>
        <div>{{ orgSettings?.branch_name || 'فرع مسجد الإخلاص' }}</div>
      </div>

      <!-- Center: Republic, Wilaya, Title Badge -->
      <div class="flex-1 flex flex-col items-center text-center">
        <div class="text-xs font-extrabold text-slate-800">الجمهورية الجزائرية الديمقراطية الشعبية</div>
        <div class="text-xs font-bold text-slate-700 mt-0.5">ولاية {{ orgSettings?.wilaya || 'قسنطينة' }}</div>

        <!-- Distinct Double Oval Border Badge -->
        <div class="mt-2 px-8 py-2 border-2 border-slate-900 rounded-full shadow-xs relative bg-white">
          <div class="absolute inset-0.5 border border-slate-800 rounded-full pointer-events-none"></div>
          <h2 class="text-base font-black tracking-wide text-slate-900 relative z-10">
            استمارة الدخول المدرسي
          </h2>
          <div class="text-sm font-black font-mono text-slate-800 mt-0.5 relative z-10">
            {{ campaignLabel }}
          </div>
        </div>
      </div>

      <!-- Left: Photo Box -->
      <div class="w-24 h-32 border-2 border-slate-800 rounded-xl flex items-center justify-center text-slate-400 font-bold text-xs bg-slate-50/50 shrink-0 shadow-2xs">
        <span>صورة</span>
      </div>
    </div>

    <!-- Beneficiary Details Section -->
    <div class="space-y-2 text-xs font-medium border-t border-slate-300 pt-3 text-slate-800">
      
      <!-- Name & Surname -->
      <div class="flex items-center justify-between gap-6">
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">اللقب:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ splitName.lastName }}
          </span>
        </div>
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">الإسم:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ splitName.firstName }}
          </span>
        </div>
      </div>

      <!-- Birth Date and Place -->
      <div class="flex items-center justify-between gap-6">
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">تاريخ و مكان الإزدياد:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (beneficiary?.birth_date || '') }}
          </span>
        </div>
        <div class="w-1/3 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">بـــــــ:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (beneficiary?.birth_place || '') }}
          </span>
        </div>
      </div>

      <!-- Address -->
      <div class="flex items-center gap-2">
        <span class="font-bold text-slate-900 whitespace-nowrap">العنوان:</span>
        <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
          {{ isBlank ? '' : (beneficiary?.address || '') }}
        </span>
      </div>

      <!-- Marital Status (الحالة العائلية) -->
      <div class="flex items-center flex-wrap gap-x-4 gap-y-1 py-1">
        <span class="font-bold text-slate-900">الحالة العائلية:</span>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isMarital('أعزب') ? '✓' : '' }}</span>
          <span>أعزب - عزباء</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isMarital('متزوج') ? '✓' : '' }}</span>
          <span>متزوج(ة)</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isMarital('مطلق') ? '✓' : '' }}</span>
          <span>مطلق(ة)</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isMarital('أرمل') ? '✓' : '' }}</span>
          <span>أرمل(ة)</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isMarital('إهمال') ? '✓' : '' }}</span>
          <span>إهمال عائلي</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isMarital('أخرى') ? '✓' : '' }}</span>
          <span>حالات أخرى</span>
        </label>
      </div>

      <!-- Social Status (الحالة الإجتماعية) Checkboxes -->
      <div class="flex items-center flex-wrap gap-x-4 gap-y-1 py-1">
        <span class="font-bold text-slate-900">الحالة الإجتماعية:</span>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isSocial('معاق') ? '✓' : '' }}</span>
          <span>معاق</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isSocial('ضعيف') ? '✓' : '' }}</span>
          <span>دخل ضعيف</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isSocial('بطال') || isSocial('بدون دخل') ? '✓' : '' }}</span>
          <span>بطال</span>
        </label>
        <label class="flex items-center gap-1.5 cursor-default">
          <span class="checkbox-box">{{ isSocial('أخرى') ? '✓' : '' }}</span>
          <span>حالات أخرى:</span>
          <span class="border-b border-dotted border-slate-500 w-28 inline-block text-center font-bold">
            {{ isSocialOther ? beneficiary.social_status : '' }}
          </span>
        </label>
      </div>

      <!-- Parents info -->
      <div class="flex items-center justify-between gap-6">
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">إسم الأب:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (beneficiary?.father_name || '') }}
          </span>
        </div>
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">لقب و إسم الأم:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (beneficiary?.mother_name || '') }}
          </span>
        </div>
      </div>

      <!-- Spouse info -->
      <div class="flex items-center gap-2">
        <span class="font-bold text-slate-900 whitespace-nowrap">لقب و إسم الزوجة:</span>
        <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
          {{ isBlank ? '' : (beneficiary?.spouse_name || '') }}
        </span>
      </div>

      <!-- Status & Income -->
      <div class="flex items-center justify-between gap-6">
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">الحالة الاجتماعية:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (beneficiary?.social_status || '-') }}
          </span>
        </div>
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">الدخل الشهري:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (beneficiary?.monthly_income || '') }}
          </span>
        </div>
      </div>

      <!-- Kids Count -->
      <div class="flex items-center justify-between gap-6">
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">عدد الأولاد:</span>
          <span class="font-bold text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (beneficiary?.children_count ?? '') }}
          </span>
        </div>
        <div class="flex-1 flex items-center gap-2">
          <span class="font-bold text-slate-900 whitespace-nowrap">عدد الأولاد المتمدرسين:</span>
          <span class="font-black text-slate-900 border-b border-dotted border-slate-600 flex-1 px-2 py-0.5 min-h-[22px]">
            {{ isBlank ? '' : (totalChildrenBags || '') }}
          </span>
        </div>
      </div>

      <!-- Phone -->
      <div class="flex items-center gap-2">
        <span class="font-bold text-slate-900 whitespace-nowrap">رقم الهاتف:</span>
        <span class="font-bold text-slate-900 font-mono border-b border-dotted border-slate-600 px-3 py-0.5 min-h-[22px] flex-1" dir="ltr">
          {{ isBlank ? '' : (beneficiary?.phone || '') }}
        </span>
      </div>
    </div>

    <!-- Children Table Section -->
    <div class="mt-3">
      <div class="font-bold text-xs text-slate-900 mb-1">اسم ولقب الأبناء المتمدرسين:</div>
      <table class="w-full text-center border-collapse border border-slate-900 text-xs">
        <thead>
          <tr class="bg-slate-100 font-bold border-b border-slate-900 text-slate-900">
            <th class="border border-slate-900 py-1 px-1 w-10">الرقم</th>
            <th class="border border-slate-900 py-1 px-2 w-1/3">اللقب و الإسم</th>
            <th class="border border-slate-900 py-1 px-1 w-24">تاريخ الميلاد</th>
            <th class="border border-slate-900 py-1 px-1 w-20">هل/متمدرس</th>
            <th class="border border-slate-900 py-1 px-1 w-24">المستوى</th>
            <th class="border border-slate-900 py-1 px-2">المؤسسة</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in tableRows" :key="row.num" class="h-6 border-b border-slate-900">
            <td class="border border-slate-900 py-0.5 font-mono font-bold text-[11px]">{{ row.num }}</td>
            <td class="border border-slate-900 py-0.5 px-2 text-right font-bold text-[11px]">{{ row.child_name }}</td>
            <td class="border border-slate-900 py-0.5 font-bold text-[11px]">{{ row.birth_date }}</td>
            <td class="border border-slate-900 py-0.5 font-bold text-[11px]">{{ row.isStudying }}</td>
            <td class="border border-slate-900 py-0.5 font-bold text-[11px]">{{ row.level }}</td>
            <td class="border border-slate-900 py-0.5 px-2 font-bold text-[11px]">{{ row.school_name }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Footer & Signature Section -->
    <div class="mt-4 flex items-start justify-between gap-6 pt-2 text-xs font-bold text-slate-900">
      <div class="text-right">
        أشهد بأن المعلومات المدونة أعلاه صحيحة والله شاهد على ذلك
      </div>
      <div class="text-center pl-8">
        <div>إمضاء المعني (ة)</div>
        <div class="mt-8 border-b border-dotted border-slate-400 w-32 inline-block"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  beneficiary: {
    type: Object,
    default: () => ({})
  },
  orgSettings: {
    type: Object,
    default: () => ({})
  },
  campaignLabel: {
    type: String,
    default: '2026/2027'
  },
  isBlank: {
    type: Boolean,
    default: false
  }
});

// Total children who receive school bags
const totalChildrenBags = computed(() => {
  if (props.isBlank) return '';
  const b = props.beneficiary;
  const count = (Number(b?.primary_count) || 0) + (Number(b?.middle_count) || 0) + (Number(b?.secondary_count) || 0);
  return count > 0 ? count : '';
});

// Separate First Name and Last Name nicely if possible
const splitName = computed(() => {
  if (props.isBlank) return { firstName: '', lastName: '' };
  const raw = (props.beneficiary?.guardian_name || '').trim();
  if (!raw) return { firstName: '', lastName: '' };
  const parts = raw.split(/\s+/);
  if (parts.length === 1) {
    return { firstName: parts[0], lastName: '' };
  }
  // Common in Algeria: First word is surname (اللقب) and remainder is given name (الاسم)
  return {
    lastName: parts[0],
    firstName: parts.slice(1).join(' ')
  };
});

// Helper for marital status checkboxes
const isMarital = (keyword) => {
  if (props.isBlank) return false;
  const m = (props.beneficiary?.marital_status || '').trim();
  if (m) {
    if (keyword === 'أعزب' && (m.includes('أعزب') || m.includes('عزباء'))) return true;
    if (keyword === 'متزوج' && m.includes('متزوج')) return true;
    if (keyword === 'مطلق' && m.includes('مطلق')) return true;
    if (keyword === 'أرمل' && m.includes('أرمل')) return true;
    if (keyword === 'إهمال' && (m.includes('إهمال') || m.includes('مهجور'))) return true;
    if (keyword === 'أخرى' && (m.includes('أخرى') || m.includes('اخرى'))) return true;
  }
  const status = props.beneficiary?.social_status || '';
  if (keyword === 'أرمل' && (status.includes('أرمل') || status.includes('يتيم'))) return true;
  if (keyword === 'مطلق' && status.includes('مطلق')) return true;
  if (keyword === 'متزوج' && (status.includes('متزوج') || status.includes('ضعيف') || status.includes('بدون دخل'))) return true;
  return false;
};

// Helper for social status checkboxes
const isSocial = (keyword) => {
  if (props.isBlank) return false;
  const status = props.beneficiary?.social_status || '';
  if (keyword === 'معاق' && status.includes('معاق')) return true;
  if (keyword === 'ضعيف' && status.includes('ضعيف')) return true;
  if ((keyword === 'بطال' || keyword === 'بدون دخل') && (status.includes('بطال') || status.includes('بدون دخل'))) return true;
  return false;
};

const isSocialOther = computed(() => {
  if (props.isBlank) return false;
  const s = props.beneficiary?.social_status || '';
  return s && !s.includes('معاق') && !s.includes('ضعيف') && !s.includes('بطال') && !s.includes('بدون دخل');
});

// Generate 10 standard rows with prefilled level tags if available
const tableRows = computed(() => {
  const rows = [];
  const b = props.beneficiary || {};
  const children = props.isBlank ? [] : (b.children || []);

  const pCount = props.isBlank ? 0 : (Number(b?.primary_count) || 0);
  const mCount = props.isBlank ? 0 : (Number(b?.middle_count) || 0);
  const sCount = props.isBlank ? 0 : (Number(b?.secondary_count) || 0);

  const levelsList = [];
  for (let i = 0; i < pCount; i++) levelsList.push('ابتدائي');
  for (let i = 0; i < mCount; i++) levelsList.push('متوسط');
  for (let i = 0; i < sCount; i++) levelsList.push('ثانوي');

  const maxRows = Math.max(10, children.length);

  for (let i = 0; i < maxRows; i++) {
    const formattedNum = i + 1 < 10 ? `0${i + 1}` : `${i + 1}`;
    if (i < children.length) {
      const c = children[i];
      rows.push({
        num: formattedNum,
        child_name: c.child_name || '',
        birth_date: c.birth_date || '',
        isStudying: c.is_schooling ? 'نعم' : 'لا',
        level: c.education_level || '',
        school_name: c.school_name || ''
      });
    } else {
      const level = levelsList[i] || '';
      const isStudying = level ? 'نعم' : '';
      rows.push({
        num: formattedNum,
        child_name: '',
        birth_date: '',
        isStudying,
        level,
        school_name: ''
      });
    }
  }
  return rows;
});
</script>

<style scoped>
.checkbox-box {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 14px;
  height: 14px;
  border: 1.5px solid #0f172a;
  border-radius: 2px;
  font-size: 10px;
  font-weight: 900;
  line-height: 1;
  background-color: #ffffff;
}

@media print {
  .school-form-page {
    width: 100% !important;
    max-width: 100% !important;
    padding: 4mm 6mm !important;
    page-break-after: always !important;
    break-after: page !important;
    font-size: 11px !important;
  }
  table {
    border-collapse: collapse !important;
  }
  th, td {
    border: 1.5px solid #000000 !important;
  }
}
</style>
