<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-2 sm:p-4 bg-slate-900/65 backdrop-blur-xs overflow-y-auto no-print"
    dir="rtl">
    <div
      class="bg-white rounded-3xl shadow-2xl border border-slate-200 w-[96vw] max-w-3xl overflow-hidden my-auto animate-fade-in flex flex-col max-h-[92vh]">

      <!-- ======================================================== -->
      <!-- Modal Header: Developer Profile Banner                  -->
      <!-- ======================================================== -->
      <div
        class="relative px-6 py-6 bg-gradient-to-r from-slate-950 via-indigo-950 to-slate-900 text-white shrink-0 border-b border-indigo-900/50 overflow-hidden">
        
        <!-- Decorative Glow Background Shapes -->
        <div class="absolute -top-12 -left-12 w-48 h-48 bg-indigo-500/20 rounded-full blur-3xl pointer-events-none"></div>
        <div class="absolute -bottom-12 -right-12 w-48 h-48 bg-teal-500/15 rounded-full blur-3xl pointer-events-none"></div>

        <div class="relative z-10 flex items-start justify-between gap-4">
          <div class="flex items-center gap-4">
            <!-- Developer Avatar / Tech Icon -->
            <div
              class="w-16 h-16 sm:w-20 sm:h-20 rounded-2xl bg-gradient-to-br from-indigo-500 to-indigo-700 flex items-center justify-center text-3xl sm:text-4xl shadow-lg border-2 border-white/20 shrink-0 select-none">
              👨‍💻
            </div>

            <div class="space-y-1">
              <div class="flex items-center gap-2 flex-wrap">
                <h2 class="text-xl sm:text-2xl font-black text-white tracking-wide">
                  نصرالدين حداد
                </h2>
                <span class="text-indigo-200 text-xs font-mono font-semibold">
                  (Nasreddine Haddad)
                </span>
                <span class="bg-emerald-500/20 text-emerald-300 border border-emerald-400/30 text-[10px] font-bold px-2 py-0.5 rounded-full">
                  Développeur Full-Stack
                </span>
              </div>
              <p class="text-xs sm:text-sm text-indigo-200 font-medium">
                مطور برمجيات شامل (Développeur Full-Stack) ومؤسس منظومة تسجيل وتوزيع المحافظ المدرسية
              </p>
            </div>
          </div>

          <!-- Close Button -->
          <button
            type="button"
            @click="$emit('close')"
            class="text-white/70 hover:text-white hover:bg-white/15 rounded-xl p-2 transition cursor-pointer"
            title="إغلاق (Esc)"
          >
            <span class="text-2xl leading-none font-bold">✕</span>
          </button>
        </div>
      </div>

      <!-- ======================================================== -->
      <!-- Modal Body (Scrollable & Responsive)                     -->
      <!-- ======================================================== -->
      <div class="p-5 sm:p-6 overflow-y-auto flex-1 space-y-5 text-slate-800">

        <!-- Notification Toast when Copied -->
        <transition name="fade">
          <div v-if="copiedToast"
            class="bg-emerald-600 text-white text-xs sm:text-sm font-bold px-4 py-2 rounded-xl text-center shadow-md flex items-center justify-center gap-2 animate-bounce">
            <span>✓</span>
            <span>تم نسخ {{ copiedItemName }} إلى الحافظة بنجاح!</span>
          </div>
        </transition>

        <!-- Primary Contact Channels Grid -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">

          <!-- 1. Phone & WhatsApp Card -->
          <div class="bg-gradient-to-br from-emerald-50/70 to-emerald-100/30 border border-emerald-200/90 rounded-2xl p-4.5 flex flex-col justify-between space-y-3 shadow-2xs hover:shadow-sm transition">
            <div class="flex items-start justify-between">
              <div class="flex items-center gap-3">
                <span class="w-10 h-10 rounded-xl bg-emerald-600 text-white flex items-center justify-center text-xl shadow-xs">
                  📞
                </span>
                <div>
                  <h3 class="font-black text-sm text-emerald-950">الهاتف والواتساب (WhatsApp)</h3>
                  <span class="text-[11px] text-emerald-700 font-medium">اتصال مباشر ورسائل فورية</span>
                </div>
              </div>
              <span class="bg-emerald-100 text-emerald-800 text-[10px] font-bold px-2 py-0.5 rounded-full">
                متاح يومياً
              </span>
            </div>

            <!-- Phone Number Display -->
            <div class="bg-white rounded-xl p-3 border border-emerald-200/70 flex items-center justify-between gap-2 shadow-2xs">
              <span class="font-mono font-black text-sm sm:text-base text-slate-900 tracking-wider" dir="ltr">
                +213 774 11 99 55
              </span>
              <button
                type="button"
                @click="copyText('+213 774 11 99 55', 'رقم الهاتف')"
                class="bg-emerald-50 hover:bg-emerald-100 text-emerald-800 text-xs font-bold px-2.5 py-1 rounded-lg border border-emerald-300 transition cursor-pointer flex items-center gap-1 active:scale-95"
                title="نسخ الرقم"
              >
                <span>📋</span>
                <span>نسخ</span>
              </button>
            </div>

            <!-- Actions Row -->
            <div class="grid grid-cols-2 gap-2 pt-1">
              <a
                href="tel:+213774119955"
                class="bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-bold py-2 px-3 rounded-xl transition flex items-center justify-center gap-1.5 shadow-xs text-center"
              >
                <span>📞</span>
                <span>اتصال هاتفياً</span>
              </a>
              <button
                type="button"
                @click="openExternalUrl('https://wa.me/213774119955')"
                class="bg-emerald-700 hover:bg-emerald-800 text-white text-xs font-bold py-2 px-3 rounded-xl transition flex items-center justify-center gap-1.5 shadow-xs cursor-pointer"
              >
                <span>💬</span>
                <span>دردشة واتساب</span>
              </button>
            </div>
          </div>

          <!-- 2. Email Card -->
          <div class="bg-gradient-to-br from-indigo-50/70 to-indigo-100/30 border border-indigo-200/90 rounded-2xl p-4.5 flex flex-col justify-between space-y-3 shadow-2xs hover:shadow-sm transition">
            <div class="flex items-start justify-between">
              <div class="flex items-center gap-3">
                <span class="w-10 h-10 rounded-xl bg-indigo-600 text-white flex items-center justify-center text-xl shadow-xs">
                  ✉️
                </span>
                <div>
                  <h3 class="font-black text-sm text-indigo-950">البريد الإلكتروني الرسمي</h3>
                  <span class="text-[11px] text-indigo-700 font-medium">للاستفسارات والطلبات الرسمية</span>
                </div>
              </div>
              <span class="bg-indigo-100 text-indigo-800 text-[10px] font-bold px-2 py-0.5 rounded-full">
                Email
              </span>
            </div>

            <!-- Email Display -->
            <div class="bg-white rounded-xl p-3 border border-indigo-200/70 flex items-center justify-between gap-2 shadow-2xs">
              <span class="font-mono font-bold text-xs sm:text-sm text-slate-900 truncate" dir="ltr">
                contact@nacerdev.com
              </span>
              <button
                type="button"
                @click="copyText('contact@nacerdev.com', 'البريد الإلكتروني')"
                class="bg-indigo-50 hover:bg-indigo-100 text-indigo-800 text-xs font-bold px-2.5 py-1 rounded-lg border border-indigo-300 transition cursor-pointer flex items-center gap-1 active:scale-95 shrink-0"
                title="نسخ البريد الإلكتروني"
              >
                <span>📋</span>
                <span>نسخ</span>
              </button>
            </div>

            <!-- Send Email Action -->
            <div class="pt-1">
              <a
                href="mailto:contact@nacerdev.com?subject=استفسار بخصوص منظومة المحافظ المدرسية"
                class="w-full bg-indigo-600 hover:bg-indigo-700 text-white text-xs font-bold py-2 px-3 rounded-xl transition flex items-center justify-center gap-1.5 shadow-xs text-center"
              >
                <span>📨</span>
                <span>إرسال رسالة بريد إلكتروني</span>
              </a>
            </div>
          </div>

        </div>

        <!-- Official Websites Box -->
        <div class="bg-slate-50 border border-slate-200/90 rounded-2xl p-4.5 space-y-3 shadow-2xs">
          <div class="flex items-center gap-2.5 border-b border-slate-200/80 pb-2.5">
            <span class="text-xl">🌐</span>
            <div>
              <h3 class="font-black text-sm text-slate-900">المواقع الإلكترونية الرسمية والمعرض الشخصي</h3>
              <p class="text-[11px] text-slate-500">تصفح آخر المشاريع البرمجية والحلول الرقمية</p>
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <!-- Primary Domain -->
            <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between gap-2 shadow-2xs">
              <div class="truncate">
                <span class="text-[10px] text-slate-400 font-bold block">الموقع الرئيسي:</span>
                <span class="font-mono font-bold text-xs sm:text-sm text-indigo-700 truncate block" dir="ltr">
                  https://nacerdev.com
                </span>
              </div>
              <button
                type="button"
                @click="openExternalUrl('https://nacerdev.com')"
                class="bg-indigo-50 hover:bg-indigo-100 text-indigo-800 text-xs font-bold px-3 py-1.5 rounded-lg border border-indigo-200 transition cursor-pointer flex items-center gap-1 shrink-0"
              >
                <span>🔗</span>
                <span>زيارة</span>
              </button>
            </div>

            <!-- Secondary Portfolio Domain -->
            <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between gap-2 shadow-2xs">
              <div class="truncate">
                <span class="text-[10px] text-slate-400 font-bold block">معرض الأعمال الوطني (.dz):</span>
                <span class="font-mono font-bold text-xs sm:text-sm text-teal-700 truncate block" dir="ltr">
                  haddad-developpeurfull-stack.dz
                </span>
              </div>
              <button
                type="button"
                @click="openExternalUrl('https://haddad-developpeurfull-stack.dz/')"
                class="bg-teal-50 hover:bg-teal-100 text-teal-800 text-xs font-bold px-3 py-1.5 rounded-lg border border-teal-200 transition cursor-pointer flex items-center gap-1 shrink-0"
              >
                <span>🔗</span>
                <span>زيارة</span>
              </button>
            </div>
          </div>
        </div>

        <!-- Professional Full-Stack Software Services Box -->
        <div class="bg-gradient-to-br from-slate-50 to-indigo-50/40 border border-indigo-100 rounded-2xl p-4.5 space-y-3 text-xs text-slate-700 shadow-2xs">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2 text-indigo-950 font-black text-sm">
              <span class="text-indigo-600 text-base">💻</span>
              <span>خدمات البرمجة وتطوير الأنظمة المخصصة (Services & Solutions):</span>
            </div>
            <span class="text-[10px] font-bold px-2 py-0.5 rounded-full bg-indigo-100 text-indigo-800">حسب الطلب</span>
          </div>

          <p class="leading-relaxed text-slate-600">
            تقديم خدمات هندسة وتطوير البرمجيات المخصصة للأفراد، الشركات، المؤسسات والمهنيين لتسريع ورقمنة الأعمال بأحدث التقنيات:
          </p>

          <div class="grid grid-cols-1 sm:grid-cols-3 gap-2.5 pt-0.5 text-xs">
            <div class="bg-white border border-slate-200/80 rounded-xl p-3 shadow-2xs hover:border-indigo-300 transition space-y-1">
              <div class="flex items-center gap-1.5 font-bold text-slate-900">
                <span class="text-base text-indigo-600">🖥️</span>
                <span>تطبيقات سطح المكتب</span>
              </div>
              <p class="text-[11px] text-slate-500 leading-normal">
                برمجيات Windows سريعة وآمنة تعمل دون إنترنت (Offline-First) مع قواعد بيانات محلية سريعة وطباعة الفواتير والتقارير.
              </p>
            </div>

            <div class="bg-white border border-slate-200/80 rounded-xl p-3 shadow-2xs hover:border-indigo-300 transition space-y-1">
              <div class="flex items-center gap-1.5 font-bold text-slate-900">
                <span class="text-base text-cyan-600">🌐</span>
                <span>تطبيقات ومواقع الويب</span>
              </div>
              <p class="text-[11px] text-slate-500 leading-normal">
                منصات ويب سحابية حديثة، لوحات تحكم متطورة، وواجهات برمجية (APIs) متجاوبة مع الحواسيب والهواتف بأعلى معايير الأداء.
              </p>
            </div>

            <div class="bg-white border border-slate-200/80 rounded-xl p-3 shadow-2xs hover:border-indigo-300 transition space-y-1">
              <div class="flex items-center gap-1.5 font-bold text-slate-900">
                <span class="text-base text-emerald-600">📊</span>
                <span>أنظمة التسيير المخصصة</span>
              </div>
              <p class="text-[11px] text-slate-500 leading-normal">
                برامج متكاملة لإدارة الأنشطة، المبيعات، المخزون، والمحاسبة مصممة خصيصاً لتلبية أدق متطلبات عملك ونشاطك التجاري.
              </p>
            </div>
          </div>
        </div>

      </div>

      <!-- ======================================================== -->
      <!-- Modal Footer                                             -->
      <!-- ======================================================== -->
      <div class="px-6 py-3.5 bg-slate-50 border-t border-slate-200 flex items-center justify-between shrink-0">
        <div class="text-xs text-slate-600 font-semibold flex items-center gap-1.5">
          <span>⚡</span>
          <span>تطوير حلول برمجية احترافية ومبتكرة تلبي متطلباتكم</span>
        </div>

        <button
          type="button"
          @click="$emit('close')"
          class="px-6 py-2 text-xs font-black text-white bg-slate-800 hover:bg-slate-900 rounded-xl transition cursor-pointer shadow-xs active:scale-95"
        >
          إغلاق النافذة
        </button>
      </div>

    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

defineEmits(['close']);

const copiedToast = ref(false);
const copiedItemName = ref('');

// Copy to Clipboard Helper
const copyText = async (text, itemName) => {
  try {
    await navigator.clipboard.writeText(text);
    copiedItemName.value = itemName;
    copiedToast.value = true;
    setTimeout(() => {
      copiedToast.value = false;
    }, 2800);
  } catch (err) {
    console.error('فشل النسخ:', err);
  }
};

// Open external URL in browser safely
const openExternalUrl = async (url) => {
  try {
    if (window.__TAURI__ && window.__TAURI__.shell) {
      await window.__TAURI__.shell.open(url);
    } else {
      window.open(url, '_blank');
    }
  } catch (err) {
    console.warn('استخدام نافذة المتصفح الافتراضية:', err);
    window.open(url, '_blank');
  }
};
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
