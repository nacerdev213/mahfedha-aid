<template>
  <div dir="rtl" class="h-screen overflow-hidden flex flex-col p-3 sm:p-4 max-w-7xl mx-auto md:pr-20 print:h-auto print:overflow-visible print:p-0 print:m-0">
    <!-- Action Header Toolbar (Single-line, slim & space-efficient) -->
    <header class="flex items-center justify-between gap-3 mb-2.5 no-print shrink-0">
      <!-- Right (RTL Start): Season Selector & Management -->
      <div class="inline-flex items-center gap-2 bg-indigo-50/80 border border-indigo-200/80 px-2.5 py-1 rounded-xl shadow-2xs">
        <span class="text-xs font-bold text-indigo-700">الموسم:</span>
        <select v-model="selectedCampaignId" @change="onCampaignChange"
          class="text-xs sm:text-sm font-bold bg-white border border-indigo-300 text-indigo-900 rounded-lg px-2 py-0.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none cursor-pointer">
          <option v-for="c in campaigns" :key="c.id" :value="c.id">
            {{ c.year_label }} {{ c.is_active ? '(الحالي)' : '' }}
          </option>
        </select>
        <button @click="openCampaignModal"
          class="bg-indigo-600 hover:bg-indigo-700 text-white text-xs font-bold px-2.5 py-1 rounded-md transition flex items-center gap-1 shadow-2xs cursor-pointer whitespace-nowrap"
          title="إنشاء موسم دراسي جديد مع إمكانية ترحيل المستفيدين">
          + موسم جديد
        </button>
        <button v-if="campaigns.length > 1" @click="openRolloverModal"
          class="bg-amber-500 hover:bg-amber-600 text-white text-xs font-bold px-2.5 py-1 rounded-md transition flex items-center gap-1 shadow-2xs cursor-pointer whitespace-nowrap"
          title="ترحيل المستفيدين من موسم آخر إلى الموسم الحالي">
          ↩ ترحيل
        </button>
      </div>

      <!-- Left (RTL End): Primary Action Buttons -->
      <div class="flex items-center gap-2.5 no-print shrink-0">
        <!-- Unified Print Action Button -->
        <button @click="openCustomPrintModal('list')"
          class="bg-white border border-slate-300 hover:bg-indigo-50 hover:text-indigo-700 hover:border-indigo-300 text-slate-700 text-xs sm:text-sm font-bold px-3.5 py-1.5 rounded-xl shadow-2xs flex items-center gap-2 transition cursor-pointer whitespace-nowrap shrink-0"
          title="خيارات وطباعة المحاضر والاستمارات المدرسية">
          <span class="text-base">🖨️</span>
          <span>طباعة</span>
        </button>

        <!-- Primary Action: always single line, never wraps -->
        <button @click="openModal()"
          class="bg-indigo-600 hover:bg-indigo-700 text-white text-xs sm:text-sm font-bold px-4 py-1.5 rounded-xl shadow-sm flex items-center gap-2 transition cursor-pointer active:scale-95 whitespace-nowrap shrink-0">
          <span>➕</span>
          <span>إضافة مستفيد</span>
        </button>

        <!-- Mobile Sidebar Toggle (visible only on small screens) -->
        <button @click="toggleSidebar"
          class="md:hidden bg-white border border-slate-300 hover:bg-slate-100 text-slate-700 text-sm font-bold p-1.5 rounded-xl shadow-2xs flex items-center justify-center transition cursor-pointer shrink-0"
          title="القائمة الجانبية للأدوات">
          <span class="text-lg leading-none">{{ sidebarOpen ? '✕' : '☰' }}</span>
        </button>
      </div>
    </header>

    <!-- ======================================================== -->
    <!-- Slim Vertical Sidebar (Activity Bar) on right screen edge -->
    <!-- ======================================================== -->
    <aside 
      class="hidden md:flex fixed top-0 right-0 h-screen w-14 z-40 bg-white/95 backdrop-blur-md border-l border-slate-200/90 shadow-xs flex-col items-center justify-between py-3.5 no-print select-none transition-all duration-200"
      dir="rtl"
    >
      <!-- Top: Toggle Button on the sidebar edge -->
      <div class="flex flex-col items-center gap-3 w-full">
        <button 
          @click="toggleSidebar" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-slate-700 hover:text-indigo-700 hover:bg-indigo-50 border border-slate-200 hover:border-indigo-300 shadow-2xs transition-all duration-150 cursor-pointer group relative"
          :title="sidebarOpen ? 'إغلاق القائمة الجانبية (✕)' : 'فتح القائمة الجانبية الكاملة (الأدوات)'"
        >
          <span class="text-base font-bold transition-transform duration-200" :class="{ 'rotate-90 text-indigo-600': sidebarOpen }">
            {{ sidebarOpen ? '✕' : '☰' }}
          </span>
          <!-- Floating Tooltip Label -->
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            {{ sidebarOpen ? 'إغلاق القائمة' : 'لوحة الأدوات والإعدادات' }}
          </span>
        </button>

        <div class="w-7 h-px bg-slate-200 my-0.5"></div>
      </div>

      <!-- Middle: Quick-Access Action Icons with Tooltips -->
      <div class="flex flex-col items-center gap-2 w-full">
        <!-- 0. Statistics Dashboard Modal -->
        <button 
          @click="openStatsDetailsModal()" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-indigo-700 hover:bg-indigo-50 transition-all duration-150 cursor-pointer group relative"
          title="لوحة الإحصائيات الشاملة"
        >
          <span>📊</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            لوحة الإحصائيات الشاملة
          </span>
        </button>

        <!-- 1. Print -->
        <button 
          @click="openCustomPrintModal('list')" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-indigo-700 hover:bg-indigo-50 transition-all duration-150 cursor-pointer group relative"
          title="طباعة المحاضر والاستمارات"
        >
          <span>🖨️</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            طباعة المحاضر والاستمارات
          </span>
        </button>

        <!-- 2. Import Excel -->
        <button 
          @click="openImportModal()" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-indigo-700 hover:bg-indigo-50 transition-all duration-150 cursor-pointer group relative"
          title="استيراد من إكسيل"
        >
          <span>📥</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            استيراد من إكسيل
          </span>
        </button>

        <!-- 3. Export Excel -->
        <button 
          @click="exportToExcel()" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-emerald-700 hover:bg-emerald-50 transition-all duration-150 cursor-pointer group relative"
          title="تصدير إلى إكسيل"
        >
          <span>📤</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            تصدير إلى إكسيل
          </span>
        </button>

        <!-- 4. Rollover -->
        <button 
          @click="openRolloverModal()" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-amber-700 hover:bg-amber-50 transition-all duration-150 cursor-pointer group relative"
          title="ترحيل المستفيدين بين المواسم"
        >
          <span>↩</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            ترحيل المستفيدين
          </span>
        </button>

        <!-- 5. Sequence Gap Inspector -->
        <button 
          @click="openSequenceAuditModal(true)" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-violet-700 hover:bg-violet-50 transition-all duration-150 cursor-pointer group relative"
          title="فحص تسلسل الأرقام (كشف الفجوات والغائب)"
        >
          <span>🔢</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            فحص تسلسل الأرقام والغائب
          </span>
        </button>

        <div class="w-7 h-px bg-slate-200 my-1"></div>

        <!-- 6. Organization Settings -->
        <button 
          @click="openSettingsModal()" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-amber-700 hover:bg-amber-50 transition-all duration-150 cursor-pointer group relative"
          title="إعدادات الجمعية والطباعة"
        >
          <span>⚙️</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            إعدادات الجمعية
          </span>
        </button>

        <!-- 7. Social Statuses -->
        <button 
          @click="openStatusModal()" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-purple-700 hover:bg-purple-50 transition-all duration-150 cursor-pointer group relative"
          title="تخصيص الحالات الاجتماعية"
        >
          <span>🏷️</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            الحالات الاجتماعية
          </span>
        </button>

        <!-- 8. Campaign Management -->
        <button 
          @click="openCampaignMgmtModal()" 
          class="w-10 h-10 rounded-xl flex items-center justify-center text-lg text-slate-600 hover:text-teal-700 hover:bg-teal-50 transition-all duration-150 cursor-pointer group relative"
          title="إدارة وتنظيم المواسم"
        >
          <span>📅</span>
          <span class="absolute right-14 bg-slate-900 text-white text-[11px] font-medium px-2.5 py-1 rounded-md shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
            إدارة المواسم
          </span>
        </button>
      </div>

      <!-- Bottom: App Icon / Version indicator -->
      <div class="flex flex-col items-center gap-1">
        <button 
          @click="toggleSidebar" 
          class="w-8 h-8 rounded-lg flex items-center justify-center hover:bg-slate-100 transition cursor-pointer" 
          title="لوحة الأدوات والإعدادات"
        >
          <img src="/app-icon.png" alt="شعار" class="w-5 h-5 object-contain rounded opacity-80 hover:opacity-100 transition" />
        </button>
      </div>
    </aside>

    <!-- =========================================== -->
    <!-- Sidebar Drawer with all action tools        -->
    <!-- =========================================== -->
    <!-- Backdrop -->
    <transition name="fade-backdrop">
      <div v-if="sidebarOpen" class="fixed inset-0 bg-slate-900/30 backdrop-blur-[2px] z-40 no-print"
        @click="closeSidebar" />
    </transition>

    <!-- Drawer Panel (slides from right in RTL layout) -->
    <transition name="slide-drawer">
      <div v-if="sidebarOpen"
        class="fixed top-0 right-0 h-full w-72 sm:w-80 bg-white shadow-2xl border-l border-slate-200 z-50 flex flex-col no-print"
        dir="rtl">
        <!-- Drawer Header -->
        <div class="flex items-center justify-between px-5 py-4 border-b border-slate-100 bg-slate-50/80">
          <div class="flex items-center gap-2.5">
            <img src="/app-icon.png" alt="شعار" class="w-7 h-7 object-contain rounded-lg shadow-2xs border border-slate-200" />
            <div>
              <span class="font-black text-slate-900 text-base block">الأدوات والإعدادات</span>
              <span class="text-[10px] text-slate-500 font-medium">الوصول السريع للعمليات والتهيئة</span>
            </div>
          </div>
          <!-- Close button on the edge -->
          <button @click="closeSidebar"
            class="text-slate-400 hover:text-slate-700 hover:bg-slate-200/80 w-8 h-8 rounded-lg flex items-center justify-center transition text-lg font-bold cursor-pointer"
            title="طي القائمة الجانبية">✕</button>
        </div>

        <!-- Drawer Body -->
        <div class="flex-1 overflow-y-auto px-4 py-4 space-y-2">
          <!-- Section: Data -->
          <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-1 px-1">العمليات والبيانات</p>

          <!-- Statistics Dashboard Modal -->
          <button @click="openStatsDetailsModal(); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-indigo-950 bg-indigo-50/70 hover:bg-indigo-100/80 border border-indigo-200/70 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">📊</span>
            <div class="text-right">
              <div class="font-bold">لوحة الإحصائيات الشاملة</div>
              <div class="text-[11px] text-indigo-600 font-normal">تحليل تفصيلي للأطوار والحالات ونسب التسليم</div>
            </div>
          </button>

          <!-- Print item (All print options) -->
          <button @click="openCustomPrintModal('list'); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-slate-800 bg-slate-50/80 hover:bg-indigo-50 hover:text-indigo-950 border border-slate-200/80 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">🖨️</span>
            <div class="text-right">
              <div class="font-bold">طباعة المحاضر والاستمارات</div>
              <div class="text-[11px] text-slate-500 font-normal">محضر التوزيع، استمارات فردية، أو نماذج بيضاء</div>
            </div>
          </button>

          <!-- Rollover shortcut -->
          <button 
            @click="openRolloverModal(); closeSidebar()" 
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-amber-800 bg-amber-50/70 hover:bg-amber-100/80 border border-amber-200/70 transition group cursor-pointer"
          >
            <span class="text-xl group-hover:scale-110 transition-transform">↩</span>
            <div class="text-right">
              <div class="font-bold">ترحيل المستفيدين</div>
              <div class="text-[11px] text-amber-600 font-normal">نقل المستفيدين من موسم لآخر</div>
            </div>
          </button>

          <!-- Sequence Gap Inspector -->
          <button @click="openSequenceAuditModal(true); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-violet-950 bg-violet-50/60 hover:bg-violet-100/80 border border-violet-200/70 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">🔢</span>
            <div class="text-right flex-1">
              <div class="font-bold flex items-center justify-between">
                <span>فحص تسلسل الأرقام</span>
                <span v-if="seasonNumbersAnalysis && seasonNumbersAnalysis.missingCount > 0" class="bg-amber-100 text-amber-800 text-[10px] px-2 py-0.5 rounded-full font-mono font-bold">
                  {{ seasonNumbersAnalysis.missingCount }} شاغر
                </span>
              </div>
              <div class="text-[11px] text-violet-600 font-normal">كشف الأرقام المتخلفة والشاغرة في الترتيب</div>
            </div>
          </button>

          <!-- Import from Excel -->
          <button @click="openImportModal(); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-slate-700 hover:bg-indigo-50 hover:text-indigo-700 border border-transparent hover:border-indigo-200 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">📥</span>
            <div class="text-right">
              <div class="font-bold">استيراد من إكسيل</div>
              <div class="text-[11px] text-slate-400 font-normal">تحميل قائمة من ملف xlsx</div>
            </div>
          </button>

          <!-- Export to Excel -->
          <button @click="exportToExcel(); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-slate-700 hover:bg-emerald-50 hover:text-emerald-700 border border-transparent hover:border-emerald-200 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">📤</span>
            <div class="text-right">
              <div class="font-bold">تصدير إلى إكسيل</div>
              <div class="text-[11px] text-slate-400 font-normal">حفظ القائمة الحالية كملف xlsx</div>
            </div>
          </button>

          <div class="border-t border-slate-100 my-2.5"></div>

          <!-- Section: Settings -->
          <p class="text-[10px] font-black uppercase tracking-widest text-slate-400 mb-1 px-1">الإعدادات والتخصيص</p>

          <button @click="openSettingsModal(); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-slate-700 hover:bg-amber-50 hover:text-amber-700 border border-transparent hover:border-amber-200 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">⚙️</span>
            <div class="text-right">
              <div class="font-bold">إعدادات الجمعية</div>
              <div class="text-[11px] text-slate-400 font-normal">معلومات وترويسة الطباعة الرسمية</div>
            </div>
          </button>

          <button @click="openStatusModal(); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-slate-700 hover:bg-purple-50 hover:text-purple-700 border border-transparent hover:border-purple-200 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">🏷️</span>
            <div class="text-right">
              <div class="font-bold">الحالات الاجتماعية</div>
              <div class="text-[11px] text-slate-400 font-normal">إدارة وتخصيص الفئات</div>
            </div>
          </button>

          <button @click="openEducationLevelModal(); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-slate-700 hover:bg-emerald-50 hover:text-emerald-700 border border-transparent hover:border-emerald-200 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">🏫</span>
            <div class="text-right">
              <div class="font-bold">الأطوار الدراسية</div>
              <div class="text-[11px] text-slate-400 font-normal">إدارة السنوات والمراحل التعليمية</div>
            </div>
          </button>

          <button @click="openCampaignMgmtModal(); closeSidebar()"
            class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm font-semibold text-slate-700 hover:bg-teal-50 hover:text-teal-700 border border-transparent hover:border-teal-200 transition group cursor-pointer">
            <span class="text-xl group-hover:scale-110 transition-transform">📅</span>
            <div class="text-right">
              <div class="font-bold">إدارة المواسم</div>
              <div class="text-[11px] text-slate-400 font-normal">تعديل، حذف، وتنظيم المواسم</div>
            </div>
          </button>
        </div>

        <!-- Drawer Footer: info -->
        <div class="px-5 py-3 border-t border-slate-100 bg-slate-50/70 flex items-center justify-between text-[11px] text-slate-500">
          <span>الموسم الحالي: <strong class="text-indigo-600">{{ activeCampaignLabel }}</strong></span>
          <span class="font-mono font-bold text-slate-600 bg-slate-200/70 px-2 py-0.5 rounded text-[10px]">v1.4.0</span>
        </div>
      </div>
    </transition>

    <!-- Header for Print (Official Organization Profile) -->
    <div v-if="printDocType === 'list'" class="hidden print:block mb-6 border-b-2 border-slate-900 pb-4">
      <div class="flex items-start justify-between">
        <div class="text-right">
          <h2 class="text-lg font-black text-slate-900">{{ orgSettings.org_name }}</h2>
          <p class="text-sm font-bold text-slate-700">{{ orgSettings.branch_name }}</p>
          <p class="text-xs text-slate-600 mt-0.5">
            <span v-if="orgSettings.wilaya">ولاية: {{ orgSettings.wilaya }}</span>
            <span v-if="orgSettings.commune" class="mr-2">| بلدية: {{ orgSettings.commune }}</span>
            <span v-if="orgSettings.phone" class="mr-2">| هاتف: {{ orgSettings.phone }}</span>
          </p>
        </div>
        <div class="text-center">
          <h3 class="text-base font-black text-slate-900 border border-slate-800 px-4 py-1.5 rounded-lg inline-block">
            {{ printReportTitle }}
          </h3>
          <p class="text-xs font-bold text-indigo-900 mt-1">الموسم الدراسي: {{ activeCampaignLabel }}</p>
        </div>
        <div class="text-left text-xs text-slate-600 font-mono">
          <p>التاريخ: {{ currentDateFormatted }}</p>
          <p class="mt-1 font-semibold">إجمالي الأسر في المحضر: {{ printTargetBeneficiaries.length }}</p>
          <p class="font-semibold">المحافظ في المحضر: {{ printTotalBags }}</p>
        </div>
      </div>
    </div>

    <!-- Sleek Compact KPI Summary Ribbon (Saves vertical space & eliminates clutter) -->
    <div class="bg-white/95 backdrop-blur-xs px-3.5 py-2 rounded-xl border border-slate-200/90 shadow-2xs mb-2.5 flex flex-wrap items-center justify-between gap-2.5 no-print shrink-0">
      <!-- Summary Metrics Micro-Badges -->
      <div class="flex items-center flex-wrap gap-2 text-xs">
        <!-- Families Count -->
        <div class="flex items-center gap-1.5 bg-slate-50 border border-slate-200/90 px-2.5 py-1 rounded-lg text-slate-700">
          <span class="text-sm">👨‍👩‍👧‍👦</span>
          <span class="text-slate-500 font-medium">الأسر:</span>
          <strong class="text-slate-900 font-black text-sm font-mono">{{ stats.total_families }}</strong>
        </div>

        <!-- Bags Count & Breakdown Badges -->
        <div class="flex items-center gap-1.5 bg-indigo-50/70 border border-indigo-200/70 px-2.5 py-1 rounded-lg text-indigo-900">
          <span class="text-sm">🎒</span>
          <span class="text-indigo-700 font-medium">المحافظ:</span>
          <strong class="text-indigo-950 font-black text-sm font-mono">{{ stats.total_bags }}</strong>
          <div class="hidden sm:flex items-center gap-1.5 mr-1 text-[11px]">
            <span class="text-emerald-700 bg-emerald-100/70 px-1.5 py-0.5 rounded font-bold" title="ابتدائي">ابتدائي: {{ stats.primary_total }}</span>
            <span class="text-amber-700 bg-amber-100/70 px-1.5 py-0.5 rounded font-bold" title="متوسط">متوسط: {{ stats.middle_total }}</span>
            <span class="text-sky-700 bg-sky-100/70 px-1.5 py-0.5 rounded font-bold" title="ثانوي">ثانوي: {{ stats.secondary_total }}</span>
          </div>
        </div>

        <!-- Handover / Delivery Mini Progress -->
        <div class="flex items-center gap-2 bg-emerald-50/60 border border-emerald-200/70 px-2.5 py-1 rounded-lg text-emerald-900">
          <span class="text-sm">📦</span>
          <span class="text-emerald-800 font-medium">التسليم:</span>
          <strong class="text-emerald-950 font-bold font-mono">{{ stats.delivered_bags || 0 }} / {{ stats.total_bags }}</strong>
          <!-- Mini Progress bar -->
          <div class="w-14 sm:w-20 h-2 bg-emerald-200/70 rounded-full overflow-hidden inline-block">
            <div class="h-full bg-emerald-600 rounded-full transition-all duration-300" :style="{ width: `${stats.progress_percentage || 0}%` }"></div>
          </div>
          <span class="text-[11px] font-black text-emerald-800 font-mono">{{ stats.progress_percentage || 0 }}%</span>
        </div>
      </div>

      <!-- Right/End: Open Detailed Stats Modal Button -->
      <button 
        @click="openStatsDetailsModal" 
        class="flex items-center gap-1.5 bg-linear-to-r from-indigo-50 to-slate-100 hover:from-indigo-100 hover:to-indigo-50 text-indigo-900 border border-indigo-200/80 px-3 py-1.5 rounded-lg text-xs font-bold transition cursor-pointer active:scale-95 whitespace-nowrap shadow-2xs shrink-0 group"
        title="عرض لوحة الإحصائيات الكاملة وتحليل الحالات والأطوار والمؤشرات"
      >
        <span class="text-sm group-hover:scale-110 transition-transform">📊</span>
        <span>تفاصيل الإحصائيات</span>
        <span class="text-[10px] bg-white border border-indigo-200 text-indigo-700 px-1.5 py-0.2 rounded font-mono font-bold">شامل ↗</span>
      </button>
    </div>

    <!-- Search & Filter Controls -->
    <div
      class="bg-white p-2 sm:px-3 sm:py-1.5 rounded-xl border border-slate-200/90 shadow-2xs mb-2.5 flex flex-col lg:flex-row gap-2.5 items-center justify-between no-print shrink-0">
      <!-- Search Input -->
      <div class="w-full lg:w-72">
        <input v-model="searchQuery" @input="fetchData" type="text" placeholder="بحث باسم ولي الأمر أو الهاتف..."
          class="w-full text-xs sm:text-sm px-3 py-1.5 border border-slate-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
      </div>

      <!-- Center: Delivery Status Tabs -->
      <div class="flex items-center bg-slate-100 p-1 rounded-xl border border-slate-200 text-xs font-bold w-full sm:w-auto justify-center">
        <button 
          @click="deliveryFilter = 'all'" 
          :class="deliveryFilter === 'all' ? 'bg-white text-slate-900 shadow-xs font-black' : 'text-slate-500 hover:text-slate-800'"
          class="px-3.5 py-1.5 rounded-lg transition cursor-pointer flex items-center gap-1.5"
        >
          <span>🌐 الكل</span>
          <span class="bg-slate-200 text-slate-700 px-1.5 py-0.2 rounded-full text-[10px]">{{ beneficiaries.length }}</span>
        </button>
        <button 
          @click="deliveryFilter = 'delivered'" 
          :class="deliveryFilter === 'delivered' ? 'bg-emerald-600 text-white shadow-xs font-black' : 'text-slate-600 hover:text-emerald-700'"
          class="px-3.5 py-1.5 rounded-lg transition cursor-pointer flex items-center gap-1.5"
        >
          <span>✓ تم الاستلام</span>
          <span :class="deliveryFilter === 'delivered' ? 'bg-white/20 text-white' : 'bg-emerald-100 text-emerald-800'" class="px-1.5 py-0.2 rounded-full text-[10px] font-mono">{{ stats.delivered_families || 0 }}</span>
        </button>
        <button 
          @click="deliveryFilter = 'pending'" 
          :class="deliveryFilter === 'pending' ? 'bg-amber-600 text-white shadow-xs font-black' : 'text-slate-600 hover:text-amber-700'"
          class="px-3.5 py-1.5 rounded-lg transition cursor-pointer flex items-center gap-1.5"
        >
          <span>⏳ في الانتظار</span>
          <span :class="deliveryFilter === 'pending' ? 'bg-white/20 text-white' : 'bg-amber-100 text-amber-800'" class="px-1.5 py-0.2 rounded-full text-[10px] font-mono">{{ stats.pending_families || 0 }}</span>
        </button>
      </div>

      <!-- Actions / Alerts & Social Status Filter -->
      <div class="flex items-center gap-2.5 w-full sm:w-auto justify-end flex-wrap">
        <!-- Sequence Gap Alert Button -->
        <button 
          @click="openSequenceAuditModal()"
          class="flex items-center gap-1.5 bg-white hover:bg-slate-50 text-slate-700 border border-slate-300 px-3 py-1.5 rounded-xl text-xs font-bold transition cursor-pointer shadow-2xs"
          title="فحص الأرقام الشاغرة في ترتيب أرقام المستفيدين"
        >
          <span>🔢</span>
          <span>فحص الأرقام الشاغرة</span>
        </button>

        <div class="flex items-center gap-2">
          <label class="text-xs text-slate-500 font-medium">الحالة الاجتماعية:</label>
          <select v-model="selectedStatus" @change="fetchData"
            class="text-sm border border-slate-300 rounded-lg px-3 py-1.5 bg-white focus:ring-2 focus:ring-indigo-500 focus:outline-none">
            <option value="الكل">جميع الحالات</option>
            <option v-for="s in socialStatuses" :key="s.id" :value="s.name">{{ s.name }}</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Bulk Actions Alert Bar (shown above table when items are selected) -->
    <div 
      v-if="selectedBeneficiaryIds.length > 0" 
      class="bg-indigo-50 border-2 border-indigo-200 rounded-xl p-2.5 mb-2.5 flex flex-col sm:flex-row items-center justify-between gap-3 no-print shadow-sm transition-all shrink-0"
    >
      <div class="flex items-center flex-wrap gap-2 text-xs text-indigo-950 font-bold">
        <span class="w-2.5 h-2.5 rounded-full bg-indigo-600 animate-ping inline-block"></span>
        <span>تم تحديد <span class="bg-indigo-600 text-white px-2 py-0.5 rounded-md font-black">{{ selectedBeneficiaryIds.length }}</span> مستفيد</span>
        <span class="text-indigo-300">|</span>
        <button 
          v-if="selectedBeneficiaryIds.length < sortedBeneficiaries.length"
          @click="selectAllInCampaign" 
          class="text-indigo-700 hover:text-indigo-900 underline hover:no-underline font-semibold cursor-pointer"
        >
          تحديد جميع المسجلين في هذا الموسم ({{ sortedBeneficiaries.length }} مستفيد)
        </button>
      </div>
      <div class="flex items-center flex-wrap gap-2">
        <button 
          @click="openCustomPrintWithSelection('forms')" 
          class="flex items-center gap-1.5 bg-teal-600 hover:bg-teal-700 active:scale-95 text-white text-xs font-bold px-3 py-2 rounded-lg shadow-sm transition cursor-pointer"
          title="طباعة استمارات الدخول المدرسي للمستفيدين المحددين"
        >
          <span>📄</span>
          <span>طباعة استمارات المحدد ({{ selectedBeneficiaryIds.length }})</span>
        </button>
        <button 
          @click="openCustomPrintWithSelection('list')" 
          class="flex items-center gap-1.5 bg-indigo-700 hover:bg-indigo-800 active:scale-95 text-white text-xs font-bold px-3 py-2 rounded-lg shadow-sm transition cursor-pointer"
          title="طباعة محضر التوزيع للمستفيدين المحددين فقط"
        >
          <span>📋</span>
          <span>طباعة قائمة المحدد</span>
        </button>
        <button 
          @click="bulkDeliver(true)" 
          class="flex items-center gap-1.5 bg-emerald-600 hover:bg-emerald-700 active:scale-95 text-white text-xs font-bold px-3 py-2 rounded-lg shadow-sm transition cursor-pointer"
          title="تأكيد تسليم المحافظ للمستفيدين المحددين"
        >
          <span>✓</span>
          <span>تسليم المحدد ({{ selectedBeneficiaryIds.length }})</span>
        </button>
        <button 
          @click="bulkDeliver(false)" 
          class="flex items-center gap-1.5 bg-amber-600 hover:bg-amber-700 active:scale-95 text-white text-xs font-bold px-3 py-2 rounded-lg shadow-sm transition cursor-pointer"
          title="إلغاء حالة التسليم وإعادتهم للانتظار"
        >
          <span>⏳</span>
          <span>إلغاء التسليم</span>
        </button>
        <button 
          @click="bulkDelete" 
          class="flex items-center gap-1.5 bg-red-600 hover:bg-red-700 active:scale-95 text-white text-xs font-bold px-3 py-2 rounded-lg shadow-sm transition cursor-pointer"
        >
          <span>🗑️</span>
          <span>حذف المحدد</span>
        </button>
        <button 
          @click="clearSelection" 
          class="text-xs text-slate-600 hover:text-slate-900 px-3 py-2 rounded-lg hover:bg-indigo-100/70 transition font-medium cursor-pointer"
        >
          إلغاء التحديد
        </button>
      </div>
    </div>

    <!-- Data Table Container (Flex-1 column filling viewport with dedicated internal scroll) -->
    <div :class="{ 'print:hidden': printDocType === 'forms' }" class="flex-1 min-h-0 flex flex-col bg-white rounded-xl border border-slate-200/90 shadow-2xs overflow-hidden print:overflow-visible print:border-none print:shadow-none">
      <!-- Scrollable Table Body Container (Only rows scroll; header stays sticky) -->
      <div class="flex-1 overflow-y-auto overflow-x-auto relative">
        <table class="w-full text-right text-sm border-collapse">
          <thead class="bg-slate-100/95 backdrop-blur-xs text-slate-700 text-xs font-bold border-b border-slate-200 sticky top-0 z-10 shadow-2xs">
            <tr>
              <!-- Checkbox Column -->
              <th class="py-2.5 px-3 text-center w-10 no-print">
                <input 
                  type="checkbox" 
                  :checked="isAllPageSelected" 
                  :indeterminate.prop="isSomePageSelected && !isAllPageSelected"
                  @change="toggleSelectAllPage" 
                  class="w-4 h-4 text-indigo-600 rounded border-slate-300 focus:ring-indigo-500 cursor-pointer accent-indigo-600" 
                  title="تحديد / إلغاء تحديد الكل في هذه الصفحة"
                />
              </th>

              <!-- Sortable Column: Record Number -->
              <th @click="toggleSort('record_no')"
                class="py-2.5 px-3 text-center cursor-pointer select-none hover:bg-slate-200/70 transition group no-print"
                title="انقر للترتيب حسب الرقم">
                <div class="flex items-center justify-center gap-1.5">
                  <span>الرقم</span>
                  <span
                    class="inline-flex flex-col text-[8px] leading-tight text-slate-400 group-hover:text-indigo-600 transition">
                    <span
                      :class="{ 'text-indigo-600 font-black scale-110': sortKey === 'record_no' && sortOrder === 'asc' }">▲</span>
                    <span
                      :class="{ 'text-indigo-600 font-black scale-110': sortKey === 'record_no' && sortOrder === 'desc' }">▼</span>
                  </span>
                </div>
              </th>
              <th class="py-2.5 px-3 text-center hidden print:table-cell">الرقم</th>

              <!-- Sortable Column: Guardian Name -->
              <th @click="toggleSort('guardian_name')"
                class="py-2.5 px-4 cursor-pointer select-none hover:bg-slate-200/70 transition group no-print"
                title="انقر للترتيب الأبجدي حسب الاسم واللقب">
                <div class="flex items-center gap-1.5">
                  <span>اسم ولقب ولي الأمر</span>
                  <span
                    class="inline-flex flex-col text-[8px] leading-tight text-slate-400 group-hover:text-indigo-600 transition">
                    <span
                      :class="{ 'text-indigo-600 font-black scale-110': sortKey === 'guardian_name' && sortOrder === 'asc' }">▲</span>
                    <span
                      :class="{ 'text-indigo-600 font-black scale-110': sortKey === 'guardian_name' && sortOrder === 'desc' }">▼</span>
                  </span>
                </div>
              </th>
              <th class="py-2.5 px-4 hidden print:table-cell">اسم ولقب ولي الأمر</th>

              <th class="py-2.5 px-3">الهاتف</th>
              <th class="py-2.5 px-3">الحالة الإجتماعية</th>
              <th class="py-2.5 px-3 text-center col-primary">ابتدائي</th>
              <th class="py-2.5 px-3 text-center col-middle">متوسط</th>
              <th class="py-2.5 px-3 text-center col-secondary">ثانوي</th>
              <th class="py-2.5 px-3 text-center col-total">المجموع</th>
              <!-- Delivery Status Column (Screen only) -->
              <th class="py-2.5 px-3 text-center no-print">حالة الاستلام</th>
              <!-- Print-Only Beneficiary Signature Column replacing Delivery Status Column -->
              <th class="py-2.5 px-4 text-center hidden print:table-cell signature-header">
                الإمضاء
              </th>
              <th class="py-2.5 px-4 text-center no-print">إجراءات</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            <tr 
              v-for="(item, index) in displayedBeneficiaries" 
              :key="item.id" 
              :class="{ 
                'bg-indigo-50/60 hover:bg-indigo-50/80': selectedBeneficiaryIds.includes(item.id),
                'hover:bg-slate-50/80': !selectedBeneficiaryIds.includes(item.id) 
              }"
              class="transition"
            >
              <!-- Checkbox cell -->
              <td class="py-2 px-3 text-center no-print" @click.stop>
                <input 
                  type="checkbox" 
                  :value="item.id" 
                  v-model="selectedBeneficiaryIds" 
                  class="w-4 h-4 text-indigo-600 rounded border-slate-300 focus:ring-indigo-500 cursor-pointer accent-indigo-600" 
                />
              </td>
              <td class="py-2 px-3 text-center font-mono font-bold text-indigo-700 text-xs">{{ item.record_no || (index +
                1) }}</td>
              <td class="py-2 px-4 font-bold text-slate-800">{{ item.guardian_name }}</td>
              <td class="py-2 px-3 text-slate-500 font-mono text-xs" dir="ltr">{{ item.phone || '-' }}</td>
              <td class="py-2 px-3">
                <span class="px-2.5 py-0.5 rounded-full text-xs font-medium inline-block" :class="{
                  'bg-red-50 text-red-700 border border-red-200': item.social_status === 'بدون دخل',
                  'bg-orange-50 text-orange-700 border border-orange-200': item.social_status === 'ضعيف الدخل',
                  'bg-blue-50 text-blue-700 border border-blue-200': item.social_status === 'متقاعد',
                  'bg-purple-50 text-purple-700 border border-purple-200': item.social_status === 'مرض مزمن',
                  'bg-teal-50 text-teal-700 border border-teal-200': item.social_status === 'إعاقة',
                }">
                  {{ item.social_status }}
                </span>
              </td>
              <td class="py-2 px-3 text-center font-bold text-emerald-700 col-primary">{{ item.primary_count }}</td>
              <td class="py-2 px-3 text-center font-bold text-amber-700 col-middle">{{ item.middle_count }}</td>
              <td class="py-2 px-3 text-center font-bold text-sky-700 col-secondary">{{ item.secondary_count }}</td>
              <td class="py-2 px-3 text-center font-extrabold text-indigo-700 col-total bg-slate-50/50">
                {{ item.primary_count + item.middle_count + item.secondary_count }}
              </td>
              <!-- Delivery Status Cell (Compact single-line pill with hover tooltip for date/time) -->
              <td class="py-2 px-2.5 text-center no-print">
                <button
                  @click.stop="toggleDelivery(item)"
                  class="inline-flex items-center justify-center gap-1 px-2.5 py-1 rounded-lg text-xs font-bold transition no-print cursor-pointer active:scale-95 whitespace-nowrap min-w-[78px]"
                  :class="item.is_delivered 
                    ? 'bg-emerald-50 hover:bg-emerald-100 text-emerald-700 border border-emerald-300 shadow-2xs' 
                    : 'bg-slate-100 hover:bg-amber-50 text-slate-600 hover:text-amber-800 border border-slate-200 hover:border-amber-300'"
                  :title="item.is_delivered 
                    ? (item.delivered_at ? `✓ تم الاستلام بتاريخ: ${item.delivered_at}\n(انقر للتراجع إلى وضع الانتظار)` : '✓ تم الاستلام\n(انقر للتراجع إلى وضع الانتظار)') 
                    : '⏳ قيد الانتظار\n(انقر لتأكيد تسليم المحفظة)'"
                >
                  <span v-if="item.is_delivered" class="flex items-center gap-1">
                    <span>✓</span>
                    <span>مستلم</span>
                  </span>
                  <span v-else class="flex items-center gap-1">
                    <span>تسليم</span>
                    <span class="text-[10px]">📦</span>
                  </span>
                </button>
              </td>

              <!-- In Print: Beneficiary Signature Cell (Empty with ample width & height for signing) -->
              <td class="hidden print:table-cell text-center signature-cell">
                <div class="signature-box"></div>
              </td>

              <td class="py-2 px-4 text-center no-print">
                <div class="flex items-center justify-center gap-1.5">
                  <button @click="printSingleBeneficiaryForm(item)"
                    class="text-teal-700 hover:text-teal-900 bg-teal-50 hover:bg-teal-100 border border-teal-200 text-xs font-bold px-2 py-0.5 rounded-md transition flex items-center gap-1 shadow-2xs cursor-pointer"
                    title="طباعة استمارة الدخول المدرسي لهذا المستفيد">
                    <span>📄</span>
                    <span>إستمارة</span>
                  </button>
                  <button @click="openModal(item)"
                    class="text-indigo-600 hover:text-indigo-900 text-xs font-semibold px-2 py-0.5 rounded hover:bg-indigo-50 transition cursor-pointer">
                    تعديل
                  </button>
                  <button @click="remove(item.id)"
                    class="text-red-500 hover:text-red-700 text-xs font-semibold px-2 py-0.5 rounded hover:bg-red-50 transition cursor-pointer">
                    حذف
                  </button>
                </div>
              </td>
            </tr>
            <tr v-if="sortedBeneficiaries.length === 0">
              <td colspan="12" class="py-8 text-center text-slate-400 text-sm">
                لا توجد بيانات مسجلة في هذا الموسم. اضغط على "+ إضافة مستفيد" أو استورد ملف الإكسيل.
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Table Pagination Bar (Sticky / Fixed at bottom of table container) -->
      <div v-if="sortedBeneficiaries.length > 0"
        class="shrink-0 flex flex-col sm:flex-row items-center justify-between gap-2 px-4 py-2 bg-slate-50/90 border-t border-slate-200 no-print text-xs text-slate-600">
        <!-- Page size selector & info -->
        <div class="flex items-center flex-wrap gap-2.5">
          <span class="font-medium text-slate-600">عرض:</span>
          <select v-model="pageSize" @change="currentPage = 1"
            class="bg-white border border-slate-300 rounded-lg px-2.5 py-1 font-bold text-slate-700 focus:ring-2 focus:ring-indigo-500 focus:outline-none cursor-pointer text-xs">
            <option :value="10">10 أسطر</option>
            <option :value="25">25 سطر</option>
            <option :value="50">50 سطر</option>
            <option :value="100">100 سطر</option>
            <option value="all">عرض الكل ({{ totalCount }})</option>
          </select>

          <span class="text-slate-400">|</span>
          <span class="text-slate-500 font-medium">
            عرض <strong class="text-slate-700">{{ paginationRangeText }}</strong> من إجمالي <strong
              class="text-indigo-700">{{ totalCount }}</strong> مستفيد
          </span>
          <span v-if="sortKey" class="text-indigo-600 bg-indigo-50 px-2 py-0.5 rounded-full text-[11px]">
            مرتّب حسب {{ sortKey === 'record_no' ? 'الرقم' : 'الاسم واللقب' }} ({{ sortOrder === 'asc' ? 'تصاعدي' :
              'تنازلي' }})
          </span>
        </div>

        <!-- Navigation Buttons -->
        <div v-if="pageSize !== 'all' && totalPages > 1" class="flex items-center gap-1">
          <button @click="goToPage(currentPage - 1)" :disabled="currentPage === 1"
            class="px-2.5 py-1 rounded-lg border border-slate-300 bg-white hover:bg-slate-100 disabled:opacity-40 disabled:cursor-not-allowed font-medium transition text-xs">
            السابق
          </button>

          <template v-for="page in visiblePages" :key="page">
            <span v-if="page === '...'" class="px-1.5 text-slate-400 font-bold">...</span>
            <button v-else @click="goToPage(page)" :class="[
              'px-2.5 py-1 rounded-lg font-bold transition text-xs',
              currentPage === page
                ? 'bg-indigo-600 text-white shadow-xs'
                : 'border border-slate-300 bg-white hover:bg-slate-100 text-slate-700'
            ]">
              {{ page }}
            </button>
          </template>

          <button @click="goToPage(currentPage + 1)" :disabled="currentPage === totalPages"
            class="px-2.5 py-1 rounded-lg border border-slate-300 bg-white hover:bg-slate-100 disabled:opacity-40 disabled:cursor-not-allowed font-medium transition text-xs">
            التالي
          </button>
        </div>
      </div>
    </div>

    <!-- Floating Bulk Actions Dock (Sticky at bottom when scrolled) -->
    <transition
      enter-active-class="transition ease-out duration-200 transform"
      enter-from-class="translate-y-8 opacity-0 scale-95"
      enter-to-class="translate-y-0 opacity-100 scale-100"
      leave-active-class="transition ease-in duration-150 transform"
      leave-from-class="translate-y-0 opacity-100 scale-100"
      leave-to-class="translate-y-8 opacity-0 scale-95"
    >
      <div 
        v-if="selectedBeneficiaryIds.length > 0" 
        class="fixed bottom-6 left-1/2 -translate-x-1/2 z-40 bg-slate-900/95 backdrop-blur-md text-white px-5 py-3 rounded-2xl shadow-2xl flex items-center flex-wrap gap-4 border border-slate-700 max-w-xl no-print"
      >
        <div class="flex items-center gap-2">
          <span class="bg-indigo-600 text-white font-black text-xs px-2.5 py-1 rounded-lg">
            {{ selectedBeneficiaryIds.length }}
          </span>
          <span class="text-xs font-semibold text-slate-200">
            مستفيد محدد
          </span>
        </div>

        <div v-if="selectedBeneficiaryIds.length < sortedBeneficiaries.length" class="text-xs border-r border-slate-700 pr-3 mr-1">
          <button 
            @click="selectAllInCampaign" 
            class="text-indigo-300 hover:text-indigo-200 font-medium underline transition cursor-pointer"
          >
            تحديد الكل ({{ sortedBeneficiaries.length }})
          </button>
        </div>

        <button 
          @click="bulkDelete" 
          class="flex items-center gap-1.5 bg-red-600 hover:bg-red-500 text-white text-xs font-bold px-4 py-2 rounded-xl shadow-lg transition active:scale-95 cursor-pointer mr-auto"
        >
          <span>🗑️</span>
          <span>حذف المحدد ({{ selectedBeneficiaryIds.length }})</span>
        </button>

        <button 
          @click="clearSelection" 
          class="text-xs text-slate-400 hover:text-white px-2 py-1 rounded-lg transition cursor-pointer"
          title="إلغاء التحديد"
        >
          إلغاء
        </button>
      </div>
    </transition>

    <!-- Official Print Footer & Signatures -->
    <div class="hidden print:block mt-8 pt-4 border-t border-slate-300">
      <div class="grid grid-cols-2 gap-8 text-center text-sm mb-6">
        <div>
          <p class="font-bold text-slate-800 mb-14">مسؤول لجنة الإحصاء والتوزيع</p>
          <p class="text-xs text-slate-500">الختم والتوقيع</p>
        </div>
        <div>
          <p class="font-bold text-slate-800 mb-14">رئيس الجمعية / مسؤول المكتب</p>
          <p class="text-xs text-slate-500">الختم والتوقيع</p>
        </div>
      </div>
      <p class="text-center text-xs text-slate-500 italic mt-4">{{ orgSettings.footer_text }}</p>
    </div>

    <!-- Beneficiary Modal -->
    <div v-if="showModal"
      class="fixed inset-0 bg-slate-900/50 backdrop-blur-xs flex items-center justify-center p-3 sm:p-4 z-50">
      <div class="bg-white rounded-2xl max-w-2xl w-full max-h-[92vh] flex flex-col p-5 sm:p-6 shadow-2xl border border-slate-200 animate-in fade-in zoom-in-95 duration-200">
        <!-- Header -->
        <div class="flex items-center justify-between pb-3 mb-3 border-b border-slate-200 shrink-0">
          <h3 class="text-base sm:text-lg font-bold text-slate-800 flex items-center gap-2">
            <span>{{ form.id ? '✏️ تعديل بيانات مستفيد' : '➕ إضافة مستفيد جديد' }}</span>
            <span class="text-xs font-bold text-indigo-700 bg-indigo-50 border border-indigo-200 px-2.5 py-0.5 rounded-full">
              موسم {{ activeCampaignLabel }}
            </span>
          </h3>
          <button @click="showModal = false" class="text-slate-400 hover:text-slate-600 p-1 rounded-lg hover:bg-slate-100 transition cursor-pointer text-base leading-none">
            ✕
          </button>
        </div>

        <!-- Scrollable Modal Body -->
        <div class="overflow-y-auto pr-1 pl-1 space-y-4 flex-1">
          <!-- Live Duplicate Alert (Current Campaign) -->
          <div v-if="!form.id && matchResult.match_type === 'current_campaign'" 
            class="bg-rose-50 border-2 border-rose-300 rounded-xl p-3 text-rose-800 flex items-start gap-3 shadow-xs">
            <span class="text-2xl shrink-0">⛔</span>
            <div class="flex-1 text-xs">
              <div class="font-black text-sm text-rose-900 mb-0.5">تنبيه: هذا المستفيد مسجل بالفعل في هذا الموسم!</div>
              <div class="font-medium text-rose-800 leading-relaxed">{{ matchResult.message }}</div>
              <div class="mt-1 text-[11px] font-bold text-rose-700">تكرار تسجيل نفس المستفيد في نفس الموسم غير مسموح به تجنباً للازدواجية.</div>
            </div>
          </div>

          <!-- Live Previous Season Rollover Suggestion -->
          <div v-if="!form.id && matchResult.match_type === 'previous_campaign' && !rolloverApplied" 
            class="bg-amber-50 border-2 border-amber-300 rounded-xl p-3.5 text-amber-900 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 shadow-xs">
            <div class="flex items-start gap-2.5">
              <span class="text-2xl shrink-0">✨</span>
              <div class="text-xs">
                <div class="font-black text-sm text-amber-950 mb-0.5">
                  تم العثور على نفس المستفيد في موسم سابق ({{ matchResult.previous_campaign_year }})
                </div>
                <div class="text-amber-800 leading-relaxed">
                  نفس الشخص بنفس الاسم مسجل في الأرشيف. يمكنك ترحيله واستيراد كافة بياناته السابقة بنقرة واحدة!
                </div>
              </div>
            </div>
            <button type="button" @click="applyRolloverMatch" 
              class="bg-amber-600 hover:bg-amber-700 active:scale-95 text-white text-xs font-bold px-3.5 py-2 rounded-xl transition shadow-sm cursor-pointer whitespace-nowrap shrink-0 flex items-center gap-1.5">
              <span>↩</span>
              <span>ترحيل واستيراد البيانات</span>
            </button>
          </div>

          <!-- Rollover Success Notice -->
          <div v-if="rolloverApplied" class="bg-emerald-50 border border-emerald-300 rounded-xl p-2.5 text-emerald-800 text-xs flex items-center justify-between gap-2 shadow-2xs">
            <div class="flex items-center gap-2">
              <span class="text-lg">✅</span>
              <span class="font-bold">تم استيراد كافة بيانات المستفيد السابقة بنجاح! يمكنك مراجعتها وحفظها لهذا الموسم.</span>
            </div>
            <button type="button" @click="rolloverApplied = false" class="text-slate-400 hover:text-slate-600 text-xs">✕</button>
          </div>

          <!-- Primary Core Fields -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <div class="sm:col-span-1">
              <label class="block text-xs font-bold text-slate-600 mb-1">الرقم (فريد بالموسم) *</label>
              <input v-model.number="form.record_no" type="number" min="1"
                class="w-full text-center font-mono font-bold text-indigo-700 text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none bg-indigo-50/40"
                required placeholder="1" />
            </div>
            <div class="sm:col-span-2">
              <label class="block text-xs font-bold text-slate-600 mb-1">اسم ولقب ولي الأمر *</label>
              <input v-model="form.guardian_name" type="text"
                placeholder="اللقب والاسم الكامل"
                class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none"
                required />
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div>
              <label class="block text-xs font-bold text-slate-600 mb-1">رقم الهاتف</label>
              <input v-model="form.phone" type="text" placeholder="06 / 05 / 07 ..."
                class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none font-mono" dir="ltr" />
            </div>
            <div>
              <label class="block text-xs font-bold text-slate-600 mb-1">الحالة الاجتماعية</label>
              <select v-model="form.social_status"
                class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none bg-white">
                <option v-for="s in socialStatuses" :key="s.id" :value="s.name">{{ s.name }}</option>
              </select>
            </div>
          </div>

          <!-- Bag Breakdown -->
          <div class="bg-slate-50 p-3.5 rounded-xl border border-slate-200">
            <span class="block text-xs font-bold text-slate-700 mb-2">إحصاء المحافظ حسب الأطوار التعليمية:</span>
            <div class="grid grid-cols-3 gap-3">
              <div>
                <label class="block text-xs font-semibold text-slate-600 mb-1 text-center">ابتدائي</label>
                <input v-model.number="form.primary_count" type="number" min="0"
                  class="w-full text-center text-sm font-bold border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none bg-white" />
              </div>
              <div>
                <label class="block text-xs font-semibold text-slate-600 mb-1 text-center">متوسط</label>
                <input v-model.number="form.middle_count" type="number" min="0"
                  class="w-full text-center text-sm font-bold border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none bg-white" />
              </div>
              <div>
                <label class="block text-xs font-semibold text-slate-600 mb-1 text-center">ثانوي</label>
                <input v-model.number="form.secondary_count" type="number" min="0"
                  class="w-full text-center text-sm font-bold border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none bg-white" />
              </div>
            </div>
          </div>

          <!-- Collapsible Optional School Form Details -->
          <div class="border border-indigo-200/80 rounded-xl overflow-hidden bg-white shadow-2xs">
            <button 
              type="button" 
              @click="showOptionalFields = !showOptionalFields"
              class="w-full flex items-center justify-between px-3.5 py-2.5 bg-indigo-50/60 hover:bg-indigo-50 transition text-right cursor-pointer select-none"
            >
              <div class="flex items-center gap-2">
                <span class="text-base">📋</span>
                <span class="text-xs font-bold text-slate-800">بيانات الاستمارة المدرسية</span>
                <span class="text-[10px] font-semibold text-indigo-700 bg-white border border-indigo-200 px-2 py-0.5 rounded-md">
                  غير إلزامية (اختيارية)
                </span>
              </div>
              <div class="flex items-center gap-1.5 text-indigo-700 text-xs font-bold">
                <span>{{ showOptionalFields ? 'إخفاء التفاصيل' : 'إدخال تفاصيل الاستمارة' }}</span>
                <span class="text-sm transition-transform duration-200" :class="{ 'rotate-180': showOptionalFields }">▼</span>
              </div>
            </button>

            <div v-show="showOptionalFields" class="p-3.5 space-y-3 bg-white border-t border-indigo-100 text-xs">
              <!-- Row 1: Birth Date & Birth Place -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div>
                  <label class="block font-bold text-slate-600 mb-1">تاريخ الميلاد</label>
                  <input v-model="form.birth_date" type="text" placeholder="مثال: 1985/06/15 أو 15-06-1985"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
                </div>
                <div>
                  <label class="block font-bold text-slate-600 mb-1">مكان الازدياد</label>
                  <input v-model="form.birth_place" type="text" placeholder="مثال: قسنطينة"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
                </div>
              </div>

              <!-- Row 2: Parents -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div>
                  <label class="block font-bold text-slate-600 mb-1">إسم الأب</label>
                  <input v-model="form.father_name" type="text" placeholder="مثال: أحمد"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
                </div>
                <div>
                  <label class="block font-bold text-slate-600 mb-1">لقب وإسم الأم</label>
                  <input v-model="form.mother_name" type="text" placeholder="مثال: بن علي فاطمة"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
                </div>
              </div>

              <!-- Row 3: Marital status & Spouse name -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div>
                  <label class="block font-bold text-slate-600 mb-1">الحالة العائلية</label>
                  <select v-model="form.marital_status"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none bg-white">
                    <option value="متزوج">متزوج(ة)</option>
                    <option value="أعزب">أعزب - عزباء</option>
                    <option value="أرمل">أرمل(ة)</option>
                    <option value="مطلق">مطلق(ة)</option>
                    <option value="إهمال">إهمال عائلي</option>
                    <option value="أخرى">حالات أخرى</option>
                  </select>
                </div>
                <div>
                  <label class="block font-bold text-slate-600 mb-1">لقب وإسم الزوج(ة)</label>
                  <input v-model="form.spouse_name" type="text" placeholder="اسم الزوج(ة)"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
                </div>
              </div>

              <!-- Row 4: Address -->
              <div>
                <label class="block font-bold text-slate-600 mb-1">العنوان الشخصي / الإقامة</label>
                <input v-model="form.address" type="text" placeholder="مثال: حي النور عمارة 5 رقم 12"
                  class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
              </div>

              <!-- Row 5: Income & Children Count -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div>
                  <label class="block font-bold text-slate-600 mb-1">الدخل الشهري</label>
                  <input v-model="form.monthly_income" type="text" placeholder="مثال: بدون دخل أو 18000 دج"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
                </div>
                <div>
                  <label class="block font-bold text-slate-600 mb-1">عدد الأولاد الإجمالي</label>
                  <input v-model.number="form.children_count" type="number" min="0" placeholder="مثال: 4"
                    class="w-full text-xs border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
                </div>
              </div>

              <!-- Children Table -->
              <div class="mt-4 pt-4 border-t border-indigo-100">
                <div class="flex items-center justify-between mb-2">
                  <div>
                    <label class="font-bold text-slate-700">بيانات الأطفال (متمدرسين وغير متمدرسين)</label>
                    <p class="text-[11px] text-slate-500">
                      يمكن تسجيل أطفال غير متمدرسين، ولكن الأطفال المتمدرسين لا يمكن أن يتجاوزوا عدد المحافظ المحددة لكل طور.
                    </p>
                  </div>
                  <button type="button" @click="addChild" class="bg-indigo-100 hover:bg-indigo-200 text-indigo-700 text-[11px] px-2.5 py-1 rounded shadow-2xs font-bold transition cursor-pointer shrink-0">
                    + إضافة طفل
                  </button>
                </div>

                <!-- Live Consistency Status Bar -->
                <div class="flex flex-wrap items-center gap-2 mb-2.5 p-2 bg-slate-50 border rounded-xl text-[11px]"
                  :class="stageConsistencyError ? 'border-red-300 bg-red-50/50' : 'border-slate-200'">
                  <span class="font-bold text-slate-700 flex items-center gap-1 text-[11px]">
                    <span>🎒</span>
                    <span>تناسق المحافظ:</span>
                  </span>

                  <!-- Primary Stage Badge -->
                  <div class="px-2 py-0.5 rounded-lg font-mono text-[10px] font-bold flex items-center gap-1 border transition"
                    :class="{
                      'bg-red-100 text-red-800 border-red-300 ring-1 ring-red-400': schooledChildrenCountByStage['ابتدائي'] > stageBagsLimit['ابتدائي'],
                      'bg-emerald-100 text-emerald-800 border-emerald-300': schooledChildrenCountByStage['ابتدائي'] === stageBagsLimit['ابتدائي'] && stageBagsLimit['ابتدائي'] > 0,
                      'bg-white text-slate-700 border-slate-200': schooledChildrenCountByStage['ابتدائي'] < stageBagsLimit['ابتدائي'] && stageBagsLimit['ابتدائي'] > 0,
                      'bg-slate-100 text-slate-400 border-slate-200': stageBagsLimit['ابتدائي'] === 0
                    }">
                    <span>ابتدائي:</span>
                    <span>{{ schooledChildrenCountByStage['ابتدائي'] }}/{{ stageBagsLimit['ابتدائي'] }}</span>
                    <span v-if="schooledChildrenCountByStage['ابتدائي'] > stageBagsLimit['ابتدائي']" class="text-red-700">⚠️ تجاوز!</span>
                    <span v-else-if="schooledChildrenCountByStage['ابتدائي'] === stageBagsLimit['ابتدائي'] && stageBagsLimit['ابتدائي'] > 0" class="text-emerald-700">✓</span>
                  </div>

                  <!-- Middle Stage Badge -->
                  <div class="px-2 py-0.5 rounded-lg font-mono text-[10px] font-bold flex items-center gap-1 border transition"
                    :class="{
                      'bg-red-100 text-red-800 border-red-300 ring-1 ring-red-400': schooledChildrenCountByStage['متوسط'] > stageBagsLimit['متوسط'],
                      'bg-amber-100 text-amber-800 border-amber-300': schooledChildrenCountByStage['متوسط'] === stageBagsLimit['متوسط'] && stageBagsLimit['متوسط'] > 0,
                      'bg-white text-slate-700 border-slate-200': schooledChildrenCountByStage['متوسط'] < stageBagsLimit['متوسط'] && stageBagsLimit['متوسط'] > 0,
                      'bg-slate-100 text-slate-400 border-slate-200': stageBagsLimit['متوسط'] === 0
                    }">
                    <span>متوسط:</span>
                    <span>{{ schooledChildrenCountByStage['متوسط'] }}/{{ stageBagsLimit['متوسط'] }}</span>
                    <span v-if="schooledChildrenCountByStage['متوسط'] > stageBagsLimit['متوسط']" class="text-red-700">⚠️ تجاوز!</span>
                    <span v-else-if="schooledChildrenCountByStage['متوسط'] === stageBagsLimit['متوسط'] && stageBagsLimit['متوسط'] > 0" class="text-amber-700">✓</span>
                  </div>

                  <!-- Secondary Stage Badge -->
                  <div class="px-2 py-0.5 rounded-lg font-mono text-[10px] font-bold flex items-center gap-1 border transition"
                    :class="{
                      'bg-red-100 text-red-800 border-red-300 ring-1 ring-red-400': schooledChildrenCountByStage['ثانوي'] > stageBagsLimit['ثانوي'],
                      'bg-sky-100 text-sky-800 border-sky-300': schooledChildrenCountByStage['ثانوي'] === stageBagsLimit['ثانوي'] && stageBagsLimit['ثانوي'] > 0,
                      'bg-white text-slate-700 border-slate-200': schooledChildrenCountByStage['ثانوي'] < stageBagsLimit['ثانوي'] && stageBagsLimit['ثانوي'] > 0,
                      'bg-slate-100 text-slate-400 border-slate-200': stageBagsLimit['ثانوي'] === 0
                    }">
                    <span>ثانوي:</span>
                    <span>{{ schooledChildrenCountByStage['ثانوي'] }}/{{ stageBagsLimit['ثانوي'] }}</span>
                    <span v-if="schooledChildrenCountByStage['ثانوي'] > stageBagsLimit['ثانوي']" class="text-red-700">⚠️ تجاوز!</span>
                    <span v-else-if="schooledChildrenCountByStage['ثانوي'] === stageBagsLimit['ثانوي'] && stageBagsLimit['ثانوي'] > 0" class="text-sky-700">✓</span>
                  </div>

                  <!-- Non-schooling Badge -->
                  <div v-if="schooledChildrenCountByStage.nonSchooling > 0"
                    class="px-2 py-0.5 rounded-lg font-mono text-[10px] font-bold bg-slate-200/70 text-slate-600 border border-slate-300">
                    غير متمدرسين: {{ schooledChildrenCountByStage.nonSchooling }} (بدون محفظة)
                  </div>
                </div>

                <!-- Error Warning Alert if in conflict -->
                <div v-if="stageConsistencyError" class="mb-2 p-2.5 bg-red-100/90 border border-red-300 rounded-lg text-xs text-red-800 font-bold flex items-center gap-2">
                  <span class="text-sm">⚠️</span>
                  <span>{{ stageConsistencyError }}</span>
                </div>

                <div v-if="form.children && form.children.length > 0" class="overflow-x-auto">
                  <table class="w-full text-xs text-right border border-slate-200 rounded-lg overflow-hidden">
                    <thead class="bg-slate-50 border-b border-slate-200 text-slate-600">
                      <tr>
                        <th class="p-2 w-8 text-center">#</th>
                        <th class="p-2">الاسم</th>
                        <th class="p-2 w-28">تاريخ الميلاد</th>
                        <th class="p-2 w-16 text-center">متمدرس؟</th>
                        <th class="p-2 w-36">المستوى الدراسي</th>
                        <th class="p-2 w-32">المؤسسة</th>
                        <th class="p-2 w-10 text-center">حذف</th>
                      </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-100">
                      <tr v-for="(child, idx) in form.children" :key="idx" class="hover:bg-slate-50/50 transition">
                        <td class="p-1.5 text-center font-bold text-slate-400">{{ idx + 1 }}</td>
                        <td class="p-1.5">
                          <input type="text" v-model="child.child_name" placeholder="اسم الطفل" class="w-full bg-transparent border-b border-dashed border-slate-300 focus:border-indigo-500 focus:outline-none py-1" />
                        </td>
                        <td class="p-1.5">
                          <input type="text" v-model="child.birth_date" placeholder="تاريخ الميلاد" class="w-full bg-transparent border-b border-dashed border-slate-300 focus:border-indigo-500 focus:outline-none py-1" />
                        </td>
                        <td class="p-1.5 text-center">
                          <input type="checkbox" v-model="child.is_schooling" @change="onSchoolingToggle(child)" class="w-3.5 h-3.5 text-indigo-600 rounded border-slate-300 focus:ring-indigo-500 cursor-pointer" />
                        </td>
                        <td class="p-1.5">
                          <select 
                            v-model="child.education_level_id" 
                            @change="onChildLevelChange(child)"
                            class="w-full bg-transparent border-b border-dashed border-slate-300 focus:border-indigo-500 focus:outline-none py-1 text-xs cursor-pointer" 
                            :disabled="!child.is_schooling" 
                            :class="{'opacity-50 cursor-not-allowed': !child.is_schooling}">
                            <option :value="null">-- اختر المستوى --</option>
                            <optgroup 
                              v-for="stage in educationLevelStore.orderedStages" 
                              :key="stage" 
                              :label="'طور ال' + stage + ' (' + getStageSlotInfo(stage, child) + ')'"
                            >
                              <option 
                                v-for="lvl in educationLevelStore.groupedByStage[stage]" 
                                :key="lvl.id" 
                                :value="lvl.id"
                                :disabled="isStageDisabledForChild(stage, child) && child.education_level_id !== lvl.id"
                                :class="{'text-slate-400': isStageDisabledForChild(stage, child) && child.education_level_id !== lvl.id}"
                              >
                                {{ lvl.year_name }} {{ (isStageDisabledForChild(stage, child) && child.education_level_id !== lvl.id) ? '(المقاعد مكتملة)' : '' }}
                              </option>
                            </optgroup>
                          </select>
                        </td>
                        <td class="p-1.5">
                          <input type="text" v-model="child.school_name" placeholder="المؤسسة" class="w-full bg-transparent border-b border-dashed border-slate-300 focus:border-indigo-500 focus:outline-none py-1" :disabled="!child.is_schooling" :class="{'opacity-50 cursor-not-allowed': !child.is_schooling}" />
                        </td>
                        <td class="p-1.5 text-center">
                          <button type="button" @click="removeChild(idx)" class="text-red-400 hover:text-red-600 font-bold px-1 rounded hover:bg-red-50 transition cursor-pointer">✕</button>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
                <div v-else class="text-center text-slate-400 text-xs py-3 border border-dashed border-slate-200 rounded-lg">
                  لا يوجد أطفال مسجلين. اضغط على "+ إضافة طفل" للإدراج.
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Modal Footer -->
        <div class="flex items-center justify-between pt-3 mt-3 border-t border-slate-200 shrink-0">
          <button @click="showModal = false"
            class="px-4 py-2 text-xs sm:text-sm font-bold text-slate-600 hover:bg-slate-100 rounded-xl transition cursor-pointer">
            إلغاء
          </button>
          
          <button @click="save"
            :disabled="!form.id && matchResult.match_type === 'current_campaign'"
            :class="[
              'px-6 py-2 text-xs sm:text-sm rounded-xl font-bold transition shadow-sm flex items-center gap-1.5 cursor-pointer active:scale-95',
              (!form.id && matchResult.match_type === 'current_campaign')
                ? 'bg-slate-300 text-slate-500 cursor-not-allowed'
                : 'bg-indigo-600 hover:bg-indigo-700 text-white'
            ]"
            :title="(!form.id && matchResult.match_type === 'current_campaign') ? 'لا يمكن الحفظ بسبب التكرار' : 'حفظ البيانات'"
          >
            <span>💾</span>
            <span>حفظ</span>
          </button>
        </div>
      </div>
    </div>

    <!-- New Campaign Modal -->
    <div v-if="showCampaignModal"
      class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4 z-50">
      <div class="bg-white rounded-2xl max-w-md w-full p-6 shadow-xl border border-slate-200">
        <h3 class="text-lg font-bold text-slate-800 mb-4 flex items-center gap-2">
          📅 إنشاء موسم / حملة دراسية جديدة
        </h3>
        <div class="space-y-4">
          <div>
            <label class="block text-xs font-bold text-slate-600 mb-1">تسمية الموسم الدراسي:</label>
            <input v-model="newCampaignYear" type="text" placeholder="مثال: 2026/2027 أو اكتب 2027 فقط"
              class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
            <!-- Live preview of auto-formatted label -->
            <p v-if="newCampaignYear.trim()" class="mt-1.5 text-xs flex items-center gap-1">
              <span class="text-slate-400">سيُحفظ كـ:</span>
              <span class="font-black text-indigo-700">
                {{ /^\d{4}$/.test(newCampaignYear.trim()) ?
                  `${newCampaignYear.trim()}/${parseInt(newCampaignYear.trim()) +
                  1}` : newCampaignYear.trim() }}
              </span>
            </p>
          </div>
          <div class="bg-slate-50 p-3 rounded-xl border border-slate-200">
            <label class="flex items-center gap-2 cursor-pointer text-sm font-medium text-slate-700">
              <input type="checkbox" v-model="rolloverPrevious"
                class="w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500" />
              ترحيل المستفيدين المسجلين من الموسم السابق
            </label>
            <p class="text-xs text-slate-400 mt-1 mr-6">
              يتم نسخ أولياء الأمور إلى الموسم الجديد مع إمكانية تعديل أطوار أبنائهم دون المساس بسجلات الموسم الماضي.
            </p>
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-6">
          <button @click="showCampaignModal = false"
            class="px-4 py-2 text-sm text-slate-600 hover:bg-slate-100 rounded-lg">إلغاء</button>
          <button @click="submitNewCampaign"
            class="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium">إنشاء
            الموسم</button>
        </div>
      </div>
    </div>

    <!-- Rollover Modal: transfer beneficiaries between seasons -->
    <div v-if="showRolloverModal"
      class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4 z-50">
      <div class="bg-white rounded-2xl max-w-md w-full p-6 shadow-xl border border-slate-200">
        <h3 class="text-lg font-bold text-slate-800 mb-1 flex items-center gap-2">
          ↩ ترحيل المستفيدين بين المواسم
        </h3>
        <p class="text-xs text-slate-500 mb-5">
          يُنقل المستفيدون الغائبون من الموسم المصدر إلى الموسم الحالي، دون المساس بالسجلات الموجودة مسبقاً.
        </p>

        <!-- Target (current) -->
        <div class="bg-indigo-50 border border-indigo-200 rounded-xl p-3 mb-4 flex items-center gap-3">
          <span class="text-xl">🎯</span>
          <div>
            <p class="text-[11px] text-indigo-500 font-semibold">الموسم الهدف (الحالي)</p>
            <p class="text-sm font-black text-indigo-900">{{ activeCampaignLabel }}</p>
          </div>
        </div>

        <!-- Source selector -->
        <div class="mb-5">
          <label class="block text-xs font-bold text-slate-600 mb-1.5">اختر موسم المصدر (المنقول منه):</label>
          <select v-model="rolloverFromId"
            class="w-full text-sm border border-slate-300 rounded-xl p-2.5 bg-white focus:ring-2 focus:ring-amber-500 focus:outline-none font-medium">
            <option :value="null" disabled>— اختر موسماً —</option>
            <option v-for="c in campaigns.filter(c => c.id !== selectedCampaignId)" :key="c.id" :value="c.id">
              {{ c.year_label }}
            </option>
          </select>
        </div>

        <!-- Info notice -->
        <div class="bg-amber-50 border border-amber-200 rounded-xl p-3 mb-5 text-xs text-amber-800">
          <span class="font-bold">ملاحظة:</span>
          لن يُنقل إلا المستفيدون غير المسجلين في الموسم الحالي. السجلات الموجودة لن تُمس.
        </div>

        <div class="flex justify-end gap-2">
          <button @click="showRolloverModal = false"
            class="px-4 py-2 text-sm text-slate-600 hover:bg-slate-100 rounded-lg">إلغاء</button>
          <button @click="submitRollover" :disabled="!rolloverFromId"
            class="px-5 py-2 text-sm bg-amber-500 hover:bg-amber-600 disabled:opacity-40 disabled:cursor-not-allowed text-white rounded-lg font-bold transition">
            ↩ تنفيذ الترحيل
          </button>
        </div>
      </div>
    </div>

    <!-- Campaign Management Modal -->
    <div v-if="showCampaignMgmtModal"
      class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4 z-50">
      <div class="bg-white rounded-2xl max-w-xl w-full p-6 shadow-xl border border-slate-200 text-right">

        <!-- Header -->
        <div class="flex items-center justify-between border-b border-slate-100 pb-3 mb-4">
          <div>
            <h3 class="text-base font-bold text-slate-900 flex items-center gap-2">
              📅 إدارة المواسم الدراسية
            </h3>
            <p class="text-xs text-slate-500 mt-0.5">تعديل تسمية المواسم، حذف الفارغة، وتعيين الموسم النشط</p>
          </div>
          <button @click="showCampaignMgmtModal = false" class="text-slate-400 hover:text-slate-600 text-lg">✕</button>
        </div>

        <!-- Campaign List Table -->
        <div class="border border-slate-200 rounded-xl overflow-hidden max-h-80 overflow-y-auto mb-4">
          <table class="w-full text-right text-xs">
            <thead class="bg-slate-50 text-slate-700 font-bold border-b border-slate-200">
              <tr>
                <th class="py-2.5 px-3">الموسم الدراسي</th>
                <th class="py-2.5 px-3 text-center">المستفيدون</th>
                <th class="py-2.5 px-3 text-center">الحالة</th>
                <th class="py-2.5 px-3 text-center">إجراءات</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100">
              <tr v-for="c in campaignsWithCounts" :key="c.id" class="hover:bg-slate-50/60 transition">
                <!-- Campaign label (editable) -->
                <td class="py-2.5 px-3 font-semibold text-slate-800">
                  <div v-if="editingCampaignId === c.id" class="flex items-center gap-1.5">
                    <input v-model="editingCampaignLabel" type="text"
                      class="text-xs border border-indigo-300 rounded px-2 py-1 w-full focus:outline-none focus:ring-1 focus:ring-indigo-500"
                      @keyup.enter="saveCampaignLabel(c)" @keyup.esc="cancelEditCampaign" />
                    <button @click="saveCampaignLabel(c)" class="text-emerald-600 hover:text-emerald-700 px-1 font-bold"
                      title="حفظ">✓</button>
                    <button @click="cancelEditCampaign" class="text-slate-400 hover:text-slate-600 px-1 font-bold"
                      title="إلغاء">✕</button>
                  </div>
                  <span v-else class="font-bold">{{ c.year_label }}</span>
                </td>
                <!-- Count badge -->
                <td class="py-2.5 px-3 text-center">
                  <span
                    :class="c.record_count > 0 ? 'bg-indigo-50 text-indigo-700 border-indigo-200' : 'bg-slate-100 text-slate-500 border-slate-200'"
                    class="border text-[11px] font-bold px-2.5 py-0.5 rounded-full inline-block">
                    {{ c.record_count }} مستفيد
                  </span>
                </td>
                <!-- Active badge -->
                <td class="py-2.5 px-3 text-center">
                  <span v-if="c.is_active"
                    class="bg-emerald-50 text-emerald-700 border border-emerald-200 text-[11px] font-bold px-2.5 py-0.5 rounded-full">نشط</span>
                  <button v-else @click="setActiveCampaign(c)"
                    class="text-slate-400 hover:text-indigo-600 text-[11px] underline" title="تعيين كموسم نشط">
                    تعيين نشطاً
                  </button>
                </td>
                <!-- Actions -->
                <td class="py-2.5 px-3 text-center">
                  <div class="flex items-center justify-center gap-1.5" v-if="editingCampaignId !== c.id">
                    <button @click="startEditCampaign(c)"
                      class="text-slate-500 hover:text-indigo-600 p-1 hover:bg-slate-100 rounded transition"
                      title="تعديل التسمية">✏️</button>
                    <button @click="deleteCampaignItem(c)" :disabled="c.record_count > 0"
                      :title="c.record_count > 0 ? 'محمي: يحتوي على ' + c.record_count + ' مستفيد' : 'حذف الموسم'"
                      :class="c.record_count > 0 ? 'opacity-30 cursor-not-allowed text-slate-400' : 'text-red-500 hover:text-red-700 hover:bg-red-50 p-1 rounded transition'">🗑️</button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Footer -->
        <div class="flex items-center justify-between border-t border-slate-100 pt-3">
          <p class="text-[11px] text-slate-400">
            🔒 المواسم التي تحتوي على مستفيدين محمية من الحذف تلقائياً.
          </p>
          <button @click="showCampaignMgmtModal = false"
            class="px-4 py-1.5 text-xs text-slate-600 hover:bg-slate-100 rounded-lg">إغلاق</button>
        </div>
      </div>
    </div>

    <!-- Organization Settings Modal -->
    <div v-if="showSettingsModal"
      class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4 z-50">
      <div class="bg-white rounded-2xl max-w-lg w-full p-6 shadow-xl border border-slate-200">
        <h3 class="text-lg font-bold text-slate-800 mb-4 flex items-center gap-2">
          ⚙️ إعدادات وهوية الجمعية (للطباعة الرسمية)
        </h3>
        <div class="space-y-3">
          <div>
            <label class="block text-xs font-bold text-slate-600 mb-1">اسم الجمعية الخيرية *</label>
            <input v-model="settingsForm.org_name" type="text"
              class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none"
              required />
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs font-bold text-slate-600 mb-1">الفرع / المكتب</label>
              <input v-model="settingsForm.branch_name" type="text" placeholder="المكتب الولائي / البلدي"
                class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
            </div>
            <div>
              <label class="block text-xs font-bold text-slate-600 mb-1">الولاية</label>
              <input v-model="settingsForm.wilaya" type="text" placeholder="مثال: قسنطينة"
                class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
            </div>
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs font-bold text-slate-600 mb-1">البلدية</label>
              <input v-model="settingsForm.commune" type="text" placeholder="مثال: الخروب"
                class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
            </div>
            <div>
              <label class="block text-xs font-bold text-slate-600 mb-1">رقم الهاتف للتواصل</label>
              <input v-model="settingsForm.phone" type="text" placeholder="مثال: 0550123456"
                class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none" />
            </div>
          </div>
          <div>
            <label class="block text-xs font-bold text-slate-600 mb-1">نص تذييل الوثائق الرسمية المطبوعة</label>
            <textarea v-model="settingsForm.footer_text" rows="2"
              class="w-full text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none"></textarea>
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-6">
          <button @click="showSettingsModal = false"
            class="px-4 py-2 text-sm text-slate-600 hover:bg-slate-100 rounded-lg">إلغاء</button>
          <button @click="saveSettings"
            class="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg font-medium">حفظ
            الإعدادات</button>
        </div>
      </div>
    </div>

    <!-- Social Status Management Modal (Protected Lookup Table) -->
    <div v-if="showStatusModal"
      class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4 z-50">
      <div class="bg-white rounded-2xl max-w-lg w-full p-6 shadow-xl border border-slate-200 text-right">

        <!-- Header -->
        <div class="flex items-center justify-between border-b border-slate-100 pb-3 mb-4">
          <div>
            <h3 class="text-base font-bold text-slate-900 flex items-center gap-2">
              🏷️ إدارة الحالات الاجتماعية
            </h3>
            <p class="text-xs text-slate-500 mt-0.5">إضافة، تعديل، وحذف الحالات الاجتماعية مع حماية السجلات المرتبطة</p>
          </div>
          <button @click="showStatusModal = false" class="text-slate-400 hover:text-slate-600 text-lg">✕</button>
        </div>

        <!-- Add New Status Form -->
        <form @submit.prevent="addSocialStatus" class="flex gap-2 mb-4">
          <input v-model="newStatusName" type="text" placeholder="اسم الحالة الاجتماعية الجديدة (مثال: أرملة، أيتام...)"
            class="flex-1 text-sm border border-slate-300 rounded-lg p-2.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none"
            required />
          <button type="submit"
            class="bg-indigo-600 hover:bg-indigo-700 text-white text-xs font-bold px-4 py-2.5 rounded-lg transition shadow-xs flex items-center gap-1 shrink-0">
            + إضافة حالة
          </button>
        </form>

        <!-- Status Error message if any -->
        <div v-if="statusError"
          class="mb-4 text-xs text-red-600 bg-red-50 p-2.5 rounded-lg border border-red-200 flex items-center justify-between">
          <span>{{ statusError }}</span>
          <button @click="statusError = ''" class="text-red-400 hover:text-red-600 font-bold mr-2">✕</button>
        </div>

        <!-- Status List -->
        <div class="border border-slate-200 rounded-xl overflow-hidden max-h-72 overflow-y-auto mb-4">
          <table class="w-full text-right text-xs">
            <thead class="bg-slate-50 text-slate-700 font-bold border-b border-slate-200">
              <tr>
                <th class="py-2.5 px-3">الحالة الاجتماعية</th>
                <th class="py-2.5 px-3 text-center">المستفيدين المرتبطين</th>
                <th class="py-2.5 px-3 text-center">إجراءات</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100">
              <tr v-for="item in socialStatuses" :key="item.id" class="hover:bg-slate-50/60 transition">
                <td class="py-2.5 px-3 font-semibold text-slate-800">
                  <div v-if="editingStatusId === item.id" class="flex items-center gap-1.5">
                    <input v-model="editingStatusName" type="text"
                      class="text-xs border border-indigo-300 rounded px-2 py-1 w-full focus:outline-none focus:ring-1 focus:ring-indigo-500"
                      @keyup.enter="saveEditStatus(item)" @keyup.esc="cancelEditStatus" />
                    <button @click="saveEditStatus(item)" class="text-emerald-600 hover:text-emerald-700 px-1 font-bold"
                      title="حفظ">✓</button>
                    <button @click="cancelEditStatus" class="text-slate-400 hover:text-slate-600 px-1 font-bold"
                      title="إلغاء">✕</button>
                  </div>
                  <span v-else>{{ item.name }}</span>
                </td>
                <td class="py-2.5 px-3 text-center">
                  <span
                    :class="item.count > 0 ? 'bg-indigo-50 text-indigo-700 border-indigo-200' : 'bg-slate-100 text-slate-500 border-slate-200'"
                    class="border text-[11px] font-bold px-2.5 py-0.5 rounded-full inline-block">
                    {{ item.count }} مستفيد
                  </span>
                </td>
                <td class="py-2.5 px-3 text-center">
                  <div class="flex items-center justify-center gap-1.5" v-if="editingStatusId !== item.id">
                    <button @click="startEditStatus(item)"
                      class="text-slate-500 hover:text-indigo-600 p-1 hover:bg-slate-100 rounded transition"
                      title="تعديل التسمية">
                      ✏️
                    </button>
                    <button @click="deleteSocialStatus(item)" :disabled="item.count > 0"
                      :title="item.count > 0 ? 'محمية من الحذف: مرتبطة بـ ' + item.count + ' مستفيد' : 'حذف الحالة'"
                      :class="item.count > 0 ? 'opacity-30 cursor-not-allowed text-slate-400' : 'text-red-500 hover:text-red-700 hover:bg-red-50 p-1 rounded transition'">
                      🗑️
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Footer Notice -->
        <div class="flex items-center justify-between border-t border-slate-100 pt-3">
          <p class="text-[11px] text-slate-400">
            🔒 الحالات المرتبطة بمستفيدين تكون محمية تلقائياً للحفاظ على سلامة السجلات.
          </p>
          <button @click="showStatusModal = false"
            class="px-4 py-1.5 text-xs text-slate-600 hover:bg-slate-100 rounded-lg">إغلاق</button>
        </div>

      </div>
    </div>

    <!-- Education Levels Management Modal (Protected Lookup Table by Educational Stage) -->
    <div v-if="showEducationLevelModal"
      class="fixed inset-0 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4 z-50">
      <div class="bg-white rounded-2xl max-w-xl w-full p-6 shadow-xl border border-slate-200 text-right">

        <!-- Header -->
        <div class="flex items-center justify-between border-b border-slate-100 pb-3 mb-4">
          <div>
            <h3 class="text-base font-bold text-slate-900 flex items-center gap-2">
              🏫 إدارة الأطوار والسنوات الدراسية
            </h3>
            <p class="text-xs text-slate-500 mt-0.5">تخصيص السنوات والمراحل التعليمية مع حماية السجلات المرتبطة بأطفال</p>
          </div>
          <button @click="closeEducationLevelModal" class="text-slate-400 hover:text-slate-600 text-lg cursor-pointer">✕</button>
        </div>

        <!-- Add New Education Level Form -->
        <form @submit.prevent="addEducationLevel" class="flex flex-wrap sm:flex-nowrap gap-2 mb-4 bg-slate-50 p-3 rounded-xl border border-slate-100">
          <select v-model="newEduStage" class="text-xs border border-slate-300 rounded-lg px-3 py-2 bg-white focus:ring-2 focus:ring-emerald-500 focus:outline-none shrink-0 font-bold">
            <option value="ابتدائي">ابتدائي</option>
            <option value="متوسط">متوسط</option>
            <option value="ثانوي">ثانوي</option>
          </select>
          <input v-model="newEduYearName" type="text" placeholder="اسم السنة (مثال: السنة الأولى ابتدائي، سنة 1 متوسط...)"
            class="flex-1 text-xs border border-slate-300 rounded-lg px-3 py-2 bg-white focus:ring-2 focus:ring-emerald-500 focus:outline-none"
            required />
          <input v-model.number="newEduYearOrder" type="number" min="1" max="10" placeholder="الترتيب" title="ترتيب السنة داخل الطور"
            class="w-16 text-xs text-center border border-slate-300 rounded-lg px-2 py-2 bg-white focus:ring-2 focus:ring-emerald-500 focus:outline-none font-mono"
            required />
          <button type="submit"
            class="bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-bold px-4 py-2 rounded-lg transition shadow-xs flex items-center gap-1 shrink-0 cursor-pointer">
            + إضافة سنة
          </button>
        </form>

        <!-- Education Levels List Grouped by Stage -->
        <div class="border border-slate-200 rounded-xl overflow-hidden max-h-80 overflow-y-auto mb-4 divide-y divide-slate-200">
          <div v-for="stage in educationLevelStore.orderedStages" :key="stage" class="p-3 bg-white">
            <div class="flex items-center justify-between pb-2 mb-2 border-b border-slate-100">
              <span class="text-xs font-bold px-2.5 py-0.5 rounded-full" :class="{
                'bg-emerald-100 text-emerald-800': stage === 'ابتدائي',
                'bg-amber-100 text-amber-800': stage === 'متوسط',
                'bg-sky-100 text-sky-800': stage === 'ثانوي'
              }">
                الطور ال{{ stage }}
              </span>
              <span class="text-[11px] text-slate-400 font-mono">
                {{ educationLevelStore.groupedByStage[stage]?.length || 0 }} سنوات
              </span>
            </div>

            <div class="space-y-1.5">
              <div v-for="item in educationLevelStore.groupedByStage[stage]" :key="item.id"
                class="flex items-center justify-between p-2 rounded-lg hover:bg-slate-50 border border-slate-100 transition text-xs">
                
                <!-- Editing or View Name -->
                <div class="flex-1 ml-2">
                  <div v-if="editingEduId === item.id" class="flex items-center gap-1.5">
                    <input v-model="editingEduName" type="text"
                      class="text-xs border border-emerald-300 rounded px-2 py-1 w-full focus:outline-none focus:ring-1 focus:ring-emerald-500"
                      @keyup.enter="saveEditEdu(item)" @keyup.esc="cancelEditEdu" />
                    <button @click="saveEditEdu(item)" class="text-emerald-600 hover:text-emerald-700 px-1 font-bold cursor-pointer" title="حفظ">✓</button>
                    <button @click="cancelEditEdu" class="text-slate-400 hover:text-slate-600 px-1 font-bold cursor-pointer" title="إلغاء">✕</button>
                  </div>
                  <div v-else class="flex items-center gap-2">
                    <span class="font-bold text-slate-800">{{ item.year_name }}</span>
                    <span class="text-[10px] text-slate-400 font-mono">#{{ item.year_order }}</span>
                  </div>
                </div>

                <!-- Children Count Badge & Actions -->
                <div class="flex items-center gap-3 shrink-0">
                  <span
                    :class="item.count > 0 ? 'bg-indigo-50 text-indigo-700 border-indigo-200' : 'bg-slate-100 text-slate-500 border-slate-200'"
                    class="border text-[10px] font-bold px-2 py-0.5 rounded-full inline-block font-mono">
                    {{ item.count }} طفل
                  </span>

                  <div class="flex items-center gap-1" v-if="editingEduId !== item.id">
                    <button @click="startEditEdu(item)"
                      class="text-slate-500 hover:text-emerald-600 p-1 hover:bg-slate-100 rounded transition cursor-pointer"
                      title="تعديل التسمية">
                      ✏️
                    </button>
                    <button @click="deleteEducationLevel(item)" :disabled="item.count > 0"
                      :title="item.count > 0 ? 'محمية من الحذف: مرتبطة بـ ' + item.count + ' طفل' : 'حذف السنة'"
                      :class="item.count > 0 ? 'opacity-30 cursor-not-allowed text-slate-400' : 'text-red-500 hover:text-red-700 hover:bg-red-50 p-1 rounded transition cursor-pointer'">
                      🗑️
                    </button>
                  </div>
                </div>

              </div>
            </div>
          </div>
        </div>

        <!-- Footer Notice -->
        <div class="flex items-center justify-between border-t border-slate-100 pt-3">
          <p class="text-[11px] text-slate-400">
            🔒 السنوات المرتبطة بأطفال مستفيدين تكون محمية تلقائياً للحفاظ على سلامة الإحصائيات.
          </p>
          <button @click="closeEducationLevelModal"
            class="px-4 py-1.5 text-xs text-slate-600 hover:bg-slate-100 rounded-lg cursor-pointer">إغلاق</button>
        </div>

      </div>
    </div>

    <!-- Excel Import Modal Component with Dynamic Template Generator & Dropdown Validation -->
    <ImportModal v-model="showImportModal" @imported="onRecordsImported" />

    <!-- Print Section for School Entry Forms -->
    <div v-if="printDocType === 'forms'" class="print-only">
      <div 
        v-for="(item, idx) in printTargetBeneficiaries" 
        :key="item.id || idx" 
        class="page-break"
      >
        <SchoolFormPrint 
          :beneficiary="item" 
          :org-settings="orgSettings" 
          :campaign-label="activeCampaignLabel" 
          :is-blank="printScope === 'blank' || item._isBlank"
        />
      </div>
    </div>

    <!-- =========================================== -->
    <!-- Sequence Gap & Missing Numbers Audit Modal  -->
    <!-- =========================================== -->
    <div v-if="showSequenceAuditModal"
      class="fixed inset-0 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-2.5 sm:p-4 z-50 overflow-y-auto no-print">
      <div class="bg-white rounded-2xl w-full max-w-2xl sm:max-w-3xl shadow-2xl border border-slate-200 flex flex-col max-h-[85vh] m-auto overflow-hidden" dir="rtl">
        <!-- Header -->
        <div class="flex items-center justify-between border-b border-slate-100 px-5 py-3 sm:px-6 bg-gradient-to-r from-violet-50/70 to-indigo-50/40 shrink-0">
          <div class="flex items-center gap-2.5">
            <span class="text-2xl">🔢</span>
            <div>
              <h3 class="text-base font-black text-slate-900">فحص الأرقام المتخلفة والشاغرة في الترتيب</h3>
              <p class="text-[11px] text-slate-500 mt-0.5">كشف الأرقام الغائبة في الترقيم التسلسلي، حساب عددها، وتحديد الفجوات بدقة</p>
            </div>
          </div>
          <button @click="showSequenceAuditModal = false"
            class="text-slate-400 hover:text-slate-600 w-8 h-8 rounded-lg flex items-center justify-center hover:bg-slate-200/70 transition text-lg font-bold cursor-pointer">✕</button>
        </div>

        <!-- Body -->
        <div class="flex-1 overflow-y-auto p-4 sm:p-5 space-y-4">
          <div v-if="isSequenceAuditLoading" class="flex flex-col items-center justify-center p-8 text-slate-500">
            <span class="text-2xl animate-spin mb-2">⏳</span>
            <span class="text-xs font-bold">جاري جلب الأرقام المسجلة في الموسم...</span>
          </div>
          
          <template v-else>
          <!-- Live Analysis KPI Cards -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <!-- 1. Present Numbers Count -->
            <div class="bg-slate-50 border border-slate-200 rounded-xl p-3 text-right">
              <span class="text-[11px] text-slate-500 font-medium">الأرقام المسجلة (الموجودة)</span>
              <div class="text-xl font-black text-slate-800 font-mono mt-0.5">
                {{ sequenceAnalysis.present.length }} <span class="text-xs font-normal text-slate-400">رقم</span>
              </div>
            </div>

            <!-- 2. Missing Numbers Count (Highlight) -->
            <div 
              :class="sequenceAnalysis.missingCount > 0 ? 'bg-amber-50 border-amber-300 text-amber-900' : 'bg-emerald-50 border-emerald-300 text-emerald-900'"
              class="border rounded-xl p-3 text-right shadow-2xs"
            >
              <div class="flex items-center justify-between">
                <span class="text-[11px] font-bold">الأرقام المتخلفة (الغائبة)</span>
                <span 
                  :class="sequenceAnalysis.missingCount > 0 ? 'bg-amber-600 text-white' : 'bg-emerald-600 text-white'"
                  class="font-mono text-[10px] px-2 py-0.5 rounded-full font-black"
                >
                  {{ sequenceAnalysis.missingCount }} غائب
                </span>
              </div>
              <div class="text-2xl font-black font-mono mt-0.5">
                {{ sequenceAnalysis.missingCount }}
              </div>
            </div>

            <!-- 3. Sequence Range -->
            <div class="bg-slate-50 border border-slate-200 rounded-xl p-3 text-right">
              <span class="text-[11px] text-slate-500 font-medium">نطاق الترقيم الكامل</span>
              <div class="text-sm font-bold text-slate-800 font-mono mt-1">
                <span v-if="sequenceAnalysis.present.length > 0">
                  من <strong class="text-violet-700 text-base">{{ sequenceAnalysis.min }}</strong> إلى <strong class="text-violet-700 text-base">{{ sequenceAnalysis.max }}</strong>
                </span>
                <span v-else class="text-slate-400 text-xs">لا توجد أرقام</span>
              </div>
            </div>
          </div>

          <!-- Missing Numbers Details Area -->
          <div v-if="sequenceAnalysis.missingCount > 0" class="bg-rose-50/60 border border-rose-200 rounded-xl p-4 space-y-3">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-1.5 text-xs font-bold text-rose-900">
                <span>⚠️</span>
                <span>قائمة الأرقام غير الموجودة (الغائبة في الترتيب):</span>
                <span class="bg-rose-100 text-rose-800 px-2 py-0.5 rounded-full font-mono text-[11px]">
                  {{ sequenceAnalysis.missingCount }} أرقام
                </span>
              </div>

              <button 
                type="button" 
                @click="copyMissingNumbers" 
                class="bg-white hover:bg-rose-100/70 border border-rose-300 text-rose-800 text-[11px] font-semibold px-2.5 py-1 rounded-lg transition flex items-center gap-1 cursor-pointer"
              >
                <span>📋</span>
                <span>نسخ الأرقام الغائبة</span>
              </button>
            </div>

            <!-- Tags Flow -->
            <div class="flex flex-wrap gap-1.5 max-h-36 overflow-y-auto p-1">
              <span 
                v-for="num in sequenceAnalysis.missing" 
                :key="num"
                class="bg-white border border-rose-300 text-rose-800 font-mono font-bold text-xs px-2.5 py-1 rounded-lg shadow-2xs hover:bg-rose-600 hover:text-white transition cursor-default"
                title="رقم غائب / شاغر في التسلسل"
              >
                {{ num }}
              </span>
            </div>

            <!-- Quick Action: Fill vacancy -->
            <div class="pt-2 border-t border-rose-200 flex items-center justify-between text-xs flex-wrap gap-2">
              <span class="text-slate-600">يمكنك ملء الفجوات بإضافة مستفيد يحمل أول رقم شاغر:</span>
              <button 
                type="button" 
                @click="useMissingNumberToAdd(sequenceAnalysis.missing[0])" 
                class="bg-rose-600 hover:bg-rose-700 text-white font-bold px-3 py-1.5 rounded-lg shadow-2xs transition flex items-center gap-1 cursor-pointer active:scale-95"
              >
                <span>➕</span>
                <span>إضافة مستفيد بالرقم الشاغر ({{ sequenceAnalysis.missing[0] }})</span>
              </button>
            </div>
          </div>

          <!-- Perfect Sequence State (0 missing) -->
          <div v-else-if="sequenceAnalysis.present.length > 0" class="bg-emerald-50 border border-emerald-200 rounded-xl p-4 flex items-center gap-3">
            <span class="text-2xl">🎉</span>
            <div>
              <p class="text-xs font-bold text-emerald-900">ترقيم كامل ومتسلسل بنسبة 100%!</p>
              <p class="text-[11px] text-emerald-700 mt-0.5">
                لا توجد أي أرقام متخلفة أو فجوات بين الرقم {{ sequenceAnalysis.min }} والرقم {{ sequenceAnalysis.max }}.
              </p>
            </div>
          </div>

          </template>
        </div>

        <!-- Footer -->
        <div class="flex items-center justify-between px-5 py-3 border-t border-slate-100 bg-slate-50/80 shrink-0">
          <div class="text-[11px] text-slate-500 font-medium">
            تحديث فوري وتحليل تلقائي للفجوات التسلسلية
          </div>
          <button 
            type="button" 
            @click="showSequenceAuditModal = false" 
            class="px-5 py-2 text-xs font-bold text-slate-700 bg-white border border-slate-300 hover:bg-slate-100 rounded-xl transition cursor-pointer"
          >
            إغلاق
          </button>
        </div>
      </div>
    </div>

    <!-- =========================================== -->
    <!-- Custom Print Modal                          -->
    <!-- =========================================== -->
    <div v-if="showPrintCustomModal"
      class="fixed inset-0 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-2.5 sm:p-4 z-50 overflow-y-auto no-print">
      <div class="bg-white rounded-2xl w-full max-w-2xl sm:max-w-3xl shadow-2xl border border-slate-200 flex flex-col max-h-[85vh] m-auto overflow-hidden" dir="rtl">
        <!-- Header (Fixed at top) -->
        <div class="flex items-center justify-between border-b border-slate-100 px-5 py-3 sm:px-6 bg-slate-50/80 shrink-0">
          <div class="flex items-center gap-2.5">
            <span class="text-xl">🖨️</span>
            <div>
              <h3 class="text-base font-black text-slate-900">خيارات وتخصيص الطباعة</h3>
              <p class="text-[11px] text-slate-500 mt-0.5">تحديد نوع المطبوع، نطاق المستفيدين، وتوجيه الصفحة</p>
            </div>
          </div>
          <button @click="showPrintCustomModal = false"
            class="text-slate-400 hover:text-slate-600 w-8 h-8 rounded-lg flex items-center justify-center hover:bg-slate-200/70 transition text-lg font-bold cursor-pointer">✕</button>
        </div>

        <!-- Body (Scrollable if height exceeds screen) -->
        <div class="flex-1 overflow-y-auto p-3.5 sm:p-5">
          <div class="grid grid-cols-1 sm:grid-cols-12 gap-3.5 sm:gap-4">
            <!-- Right Column (7 cols): Document Type & Beneficiary Scope -->
            <div class="sm:col-span-7 space-y-3">
              <!-- 1. Document Type -->
              <div>
                <label class="block text-xs font-bold text-slate-700 mb-1.5">1. نوع المطبوع:</label>
                <div class="grid grid-cols-2 gap-2">
                  <button 
                    type="button"
                    @click="printDocType = 'list'; printOrientation = 'landscape'"
                    :class="printDocType === 'list' ? 'border-indigo-600 bg-indigo-50/80 ring-2 ring-indigo-500/20 text-indigo-950 font-bold' : 'border-slate-200 hover:border-slate-300 text-slate-700 bg-white'"
                    class="flex flex-col items-start p-2.5 rounded-xl border text-right transition cursor-pointer"
                  >
                    <div class="flex items-center justify-between w-full">
                      <span class="text-lg">📋</span>
                      <span v-if="printDocType === 'list'" class="w-2.5 h-2.5 rounded-full bg-indigo-600"></span>
                    </div>
                    <span class="font-bold text-xs mt-1.5">محضر التوزيع (قائمة)</span>
                    <span class="text-[10px] text-slate-500 font-normal mt-0.5">جدول إحصاء وتوزيع الحقائب</span>
                  </button>

                  <button 
                    type="button"
                    @click="printDocType = 'forms'; printOrientation = 'portrait'"
                    :class="printDocType === 'forms' ? 'border-teal-600 bg-teal-50/80 ring-2 ring-teal-500/20 text-teal-950 font-bold' : 'border-slate-200 hover:border-slate-300 text-slate-700 bg-white'"
                    class="flex flex-col items-start p-2.5 rounded-xl border text-right transition cursor-pointer"
                  >
                    <div class="flex items-center justify-between w-full">
                      <span class="text-lg">📄</span>
                      <span v-if="printDocType === 'forms'" class="w-2.5 h-2.5 rounded-full bg-teal-600"></span>
                    </div>
                    <span class="font-bold text-xs mt-1.5">إستمارة الدخول المدرسي</span>
                    <span class="text-[10px] text-slate-500 font-normal mt-0.5">استمارة رسمية لكل مستفيد</span>
                  </button>
                </div>
              </div>

              <!-- 2. Target Beneficiaries Scope -->
              <div>
                <label class="block text-xs font-bold text-slate-700 mb-1.5">2. المستفيدين المشمولين بالطباعة:</label>
                <div class="space-y-1.5">
                  <!-- All -->
                  <label 
                    @click="printScope = 'all'"
                    :class="printScope === 'all' ? 'border-indigo-400 bg-indigo-50/40 font-bold text-slate-900' : 'border-slate-200 hover:bg-slate-50 text-slate-700'"
                    class="flex items-center justify-between p-2 rounded-xl border cursor-pointer transition text-xs"
                  >
                    <div class="flex items-center gap-2">
                      <input type="radio" value="all" v-model="printScope" class="text-indigo-600 focus:ring-indigo-500" />
                      <span>جميع المسجلين بالموسم</span>
                    </div>
                    <span class="bg-slate-100 text-slate-700 px-2 py-0.5 rounded-md font-mono text-[10px]">
                      {{ sortedBeneficiaries.length }} مستفيد
                    </span>
                  </label>

                  <!-- Selected Checkboxes -->
                  <label 
                    @click="selectedBeneficiaryIds.length > 0 && (printScope = 'selected')"
                    :class="[
                      printScope === 'selected' ? 'border-indigo-400 bg-indigo-50/40 font-bold text-slate-900' : 'border-slate-200 text-slate-700',
                      selectedBeneficiaryIds.length === 0 ? 'opacity-50 cursor-not-allowed' : 'hover:bg-slate-50 cursor-pointer'
                    ]"
                    class="flex items-center justify-between p-2 rounded-xl border transition text-xs"
                  >
                    <div class="flex items-center gap-2">
                      <input type="radio" value="selected" v-model="printScope" :disabled="selectedBeneficiaryIds.length === 0" class="text-indigo-600 focus:ring-indigo-500" />
                      <span>المستفيدين المحددين (Checkbox)</span>
                    </div>
                    <span class="bg-indigo-100 text-indigo-800 px-2 py-0.5 rounded-md font-mono text-[10px]">
                      {{ selectedBeneficiaryIds.length }} محدد
                    </span>
                  </label>

                  <!-- Range -->
                  <label 
                    @click="printScope = 'range'"
                    :class="printScope === 'range' ? 'border-indigo-400 bg-indigo-50/40 font-bold text-slate-900' : 'border-slate-200 hover:bg-slate-50 text-slate-700'"
                    class="flex flex-col p-2 rounded-xl border cursor-pointer transition text-xs gap-1.5"
                  >
                    <div class="flex items-center justify-between">
                      <div class="flex items-center gap-2">
                        <input type="radio" value="range" v-model="printScope" class="text-indigo-600 focus:ring-indigo-500" />
                        <span>نطاق أرقام (من ... إلى ...)</span>
                      </div>
                      <span class="text-indigo-600 text-[10px] font-mono">حسب عمود الرقم</span>
                    </div>

                    <!-- Range Inputs -->
                    <div v-if="printScope === 'range'" class="flex items-center gap-2 pt-1 px-3" @click.stop>
                      <div class="flex items-center gap-1.5 flex-1">
                        <span class="text-slate-600 font-bold text-xs">من:</span>
                        <input 
                          type="number" 
                          v-model.number="printRangeFrom" 
                          min="1" 
                          class="w-full text-center font-mono font-bold bg-white border border-slate-300 rounded-lg py-1 px-1.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none text-xs"
                        />
                      </div>
                      <div class="flex items-center gap-1.5 flex-1">
                        <span class="text-slate-600 font-bold text-xs">إلى:</span>
                        <input 
                          type="number" 
                          v-model.number="printRangeTo" 
                          min="1" 
                          class="w-full text-center font-mono font-bold bg-white border border-slate-300 rounded-lg py-1 px-1.5 focus:ring-2 focus:ring-indigo-500 focus:outline-none text-xs"
                        />
                      </div>
                    </div>
                  </label>

                  <!-- Blank Form Option (for School Entry Forms) -->
                  <label 
                    v-if="printDocType === 'forms'"
                    @click="printScope = 'blank'"
                    :class="printScope === 'blank' ? 'border-teal-500 bg-teal-50/60 font-bold text-slate-900 ring-1 ring-teal-400' : 'border-slate-200 hover:bg-slate-50 text-slate-700'"
                    class="flex flex-col p-2 rounded-xl border cursor-pointer transition text-xs gap-1.5"
                  >
                    <div class="flex items-center justify-between">
                      <div class="flex items-center gap-2">
                        <input type="radio" value="blank" v-model="printScope" class="text-teal-600 focus:ring-teal-500" />
                        <span class="flex items-center gap-1.5 font-bold text-teal-950">
                          <span>📝</span>
                          <span>استمارة فارغة (للملء يدوياً)</span>
                        </span>
                      </div>
                      <span class="bg-teal-100 text-teal-800 px-2 py-0.5 rounded-md font-mono text-[10px] font-bold">
                        نموذج يدوي
                      </span>
                    </div>

                    <!-- Copies Counter -->
                    <div v-if="printScope === 'blank'" class="flex items-center justify-between gap-2 pt-1 px-3 border-t border-teal-200/60 mt-0.5" @click.stop>
                      <span class="text-slate-700 font-bold text-xs">عدد النسخ:</span>
                      <div class="flex items-center gap-1.5">
                        <div class="flex items-center gap-1">
                          <button 
                            type="button" 
                            @click="printBlankCopiesCount = Math.max(1, (Number(printBlankCopiesCount) || 1) - 1)" 
                            class="w-6 h-6 rounded-md bg-white border border-slate-300 hover:bg-slate-100 flex items-center justify-center font-bold text-slate-700 cursor-pointer text-xs"
                          >-</button>
                          <input 
                            type="number" 
                            v-model.number="printBlankCopiesCount" 
                            min="1" 
                            max="200" 
                            class="w-14 text-center font-mono font-bold bg-white border border-teal-400 rounded-md py-0.5 px-1 focus:ring-2 focus:ring-teal-500 focus:outline-none text-xs"
                          />
                          <button 
                            type="button" 
                            @click="printBlankCopiesCount = (Number(printBlankCopiesCount) || 1) + 1" 
                            class="w-6 h-6 rounded-md bg-white border border-slate-300 hover:bg-slate-100 flex items-center justify-center font-bold text-slate-700 cursor-pointer text-xs"
                          >+</button>
                        </div>
                        <!-- Quick Presets -->
                        <div class="flex items-center gap-1 mr-1">
                          <button type="button" @click="printBlankCopiesCount = 1" class="px-1.5 py-0.5 rounded bg-white hover:bg-teal-100 text-teal-800 border border-teal-200 text-[10px] font-mono cursor-pointer">1</button>
                          <button type="button" @click="printBlankCopiesCount = 5" class="px-1.5 py-0.5 rounded bg-white hover:bg-teal-100 text-teal-800 border border-teal-200 text-[10px] font-mono cursor-pointer">5</button>
                          <button type="button" @click="printBlankCopiesCount = 10" class="px-1.5 py-0.5 rounded bg-white hover:bg-teal-100 text-teal-800 border border-teal-200 text-[10px] font-mono cursor-pointer">10</button>
                          <button type="button" @click="printBlankCopiesCount = 20" class="px-1.5 py-0.5 rounded bg-white hover:bg-teal-100 text-teal-800 border border-teal-200 text-[10px] font-mono cursor-pointer">20</button>
                        </div>
                      </div>
                    </div>
                  </label>
                </div>
              </div>
            </div>

            <!-- Left Column (5 cols): Page Orientation & Print Summary Card -->
            <div class="sm:col-span-5 space-y-3 flex flex-col">
              <!-- 3. Page Orientation -->
              <div>
                <label class="block text-xs font-bold text-slate-700 mb-1.5">3. اتجاه الصفحة:</label>
                <div class="grid grid-cols-2 sm:grid-cols-1 gap-2">
                  <button 
                    type="button"
                    @click="printOrientation = 'landscape'"
                    :class="printOrientation === 'landscape' ? 'border-indigo-600 bg-indigo-50/80 ring-2 ring-indigo-500/20 text-indigo-950 font-bold' : 'border-slate-200 hover:border-slate-300 text-slate-700 bg-white'"
                    class="flex items-center justify-between p-2.5 rounded-xl border text-right transition cursor-pointer"
                  >
                    <div class="flex items-center gap-2">
                      <span class="text-lg">📑</span>
                      <div>
                        <span class="font-bold text-xs block">أفقي (Paysage)</span>
                        <span class="text-[10px] text-slate-500 font-normal">عرض واسع لعمود الإمضاء</span>
                      </div>
                    </div>
                    <span v-if="printOrientation === 'landscape'" class="w-2.5 h-2.5 rounded-full bg-indigo-600"></span>
                  </button>

                  <button 
                    type="button"
                    @click="printOrientation = 'portrait'"
                    :class="printOrientation === 'portrait' ? 'border-teal-600 bg-teal-50/80 ring-2 ring-teal-500/20 text-teal-950 font-bold' : 'border-slate-200 hover:border-slate-300 text-slate-700 bg-white'"
                    class="flex items-center justify-between p-2.5 rounded-xl border text-right transition cursor-pointer"
                  >
                    <div class="flex items-center gap-2">
                      <span class="text-lg">📄</span>
                      <div>
                        <span class="font-bold text-xs block">عمودي (Portrait)</span>
                        <span class="text-[10px] text-slate-500 font-normal">الوضع الرأسي للاستمارة</span>
                      </div>
                    </div>
                    <span v-if="printOrientation === 'portrait'" class="w-2.5 h-2.5 rounded-full bg-teal-600"></span>
                  </button>
                </div>
              </div>

              <!-- Summary Card (Centered in column) -->
              <div class="bg-indigo-50/60 border border-indigo-100 rounded-xl p-3 text-xs space-y-2 flex-1 flex flex-col justify-center">
                <div class="text-[11px] font-bold text-indigo-900 flex items-center gap-1.5 border-b border-indigo-100 pb-1.5">
                  <span>📊</span>
                  <span>ملخص أمر الطباعة</span>
                </div>
                <div class="space-y-1.5 text-slate-700">
                  <div class="flex items-center justify-between text-[11px]">
                    <span class="text-slate-500">نوع المطبوع:</span>
                    <strong class="text-slate-900 truncate max-w-[130px] text-left">
                      {{ printDocType === 'list' ? 'محضر التوزيع' : (printScope === 'blank' ? 'استمارة فارغة' : 'استمارة الدخول') }}
                    </strong>
                  </div>
                  <div class="flex items-center justify-between text-[11px]">
                    <span class="text-slate-500">الاتجاه:</span>
                    <strong class="text-indigo-700 font-bold">{{ printOrientation === 'landscape' ? 'أفقي (Paysage)' : 'عمودي (Portrait)' }}</strong>
                  </div>
                  <div class="flex items-center justify-between pt-1.5 border-t border-indigo-100">
                    <span class="text-slate-700 font-bold text-xs">إجمالي العدد:</span>
                    <span class="bg-indigo-600 text-white font-black px-2.5 py-0.5 rounded-md font-mono text-xs shadow-2xs">
                      {{ printTargetBeneficiaries.length }} {{ printDocType === 'forms' ? 'استمارة' : 'مستفيد' }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer Actions (Fixed at bottom) -->
        <div class="flex items-center justify-between gap-3 px-5 py-3 sm:px-6 border-t border-slate-100 bg-slate-50/80 shrink-0">
          <div class="text-[11px] text-slate-500 font-medium hidden sm:flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
            <span>جاهز للطباعة بدقة وتنسيق رسمي</span>
          </div>
          <div class="flex items-center gap-2 mr-auto sm:mr-0">
            <button 
              type="button" 
              @click="showPrintCustomModal = false" 
              class="px-4 py-2 text-xs font-semibold text-slate-600 hover:bg-slate-200/70 rounded-xl transition cursor-pointer"
            >
              إلغاء
            </button>
            <button 
              type="button" 
              @click="executeCustomPrint" 
              :disabled="printTargetBeneficiaries.length === 0"
              class="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed text-white text-xs font-bold px-5 py-2.5 rounded-xl shadow-md hover:shadow-lg transition cursor-pointer active:scale-95"
            >
              <span>🖨️</span>
              <span>بدء الطباعة الآن ({{ printTargetBeneficiaries.length }})</span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- =========================================== -->
    <!-- Detailed Statistics & Analytics Modal (عريض كبير) -->
    <!-- =========================================== -->
    <div v-if="showStatsDetailsModal"
      class="fixed inset-0 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-3 sm:p-4 z-50 overflow-y-auto no-print">
      <div class="bg-white rounded-2xl w-full max-w-4xl lg:max-w-5xl shadow-2xl border border-slate-200 flex flex-col max-h-[90vh] m-auto overflow-hidden" dir="rtl">
        
        <!-- Modal Header -->
        <div class="flex items-center justify-between border-b border-slate-100 px-6 py-3.5 bg-linear-to-r from-indigo-50/70 via-slate-50 to-white shrink-0">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-indigo-600 text-white flex items-center justify-center text-xl shadow-xs">
              📊
            </div>
            <div>
              <h2 class="text-lg font-black text-slate-900">لوحة الإحصائيات والتحليل الشامل</h2>
              <p class="text-xs text-slate-500 mt-0.5">
                الموسم الدراسي: <strong class="text-indigo-700 font-bold">{{ activeCampaignLabel }}</strong> • مؤشرات دقيقة وتوزيع المستفيدين والحقائب المدرسية
              </p>
            </div>
          </div>
          <button 
            @click="closeStatsDetailsModal"
            class="text-slate-400 hover:text-slate-700 hover:bg-slate-200/80 w-8 h-8 rounded-lg flex items-center justify-center transition text-lg font-bold cursor-pointer"
            title="إغلاق النافذة"
          >✕</button>
        </div>

        <!-- Modal Body (Scrollable, clean dashboard layout) -->
        <div class="flex-1 overflow-y-auto p-5 sm:p-6 space-y-5">
          
          <!-- Row 1: 4 Key Metric Highlight Cards -->
          <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
            <!-- Card 1: Total Families -->
            <div class="bg-slate-50/80 border border-slate-200 rounded-xl p-3.5 text-right relative overflow-hidden shadow-2xs">
              <span class="text-xs font-semibold text-slate-500 block">إجمالي الأسر المسجلة</span>
              <div class="text-2xl font-black text-slate-900 font-mono mt-1">{{ stats.total_families }}</div>
              <span class="text-[11px] text-slate-400">عائلة مسجلة بالموسم</span>
              <span class="absolute -bottom-2 -left-2 text-4xl opacity-10 pointer-events-none select-none">👨‍👩‍👧‍👦</span>
            </div>

            <!-- Card 2: Total Bags -->
            <div class="bg-indigo-50/60 border border-indigo-200 rounded-xl p-3.5 text-right relative overflow-hidden shadow-2xs">
              <span class="text-xs font-bold text-indigo-700 block">مجموع المحافظ المطلوبة</span>
              <div class="text-2xl font-black text-indigo-950 font-mono mt-1">{{ stats.total_bags }}</div>
              <span class="text-[11px] text-indigo-600">محفظة مدرسية مطلوبة</span>
              <span class="absolute -bottom-2 -left-2 text-4xl opacity-10 pointer-events-none select-none">🎒</span>
            </div>

            <!-- Card 3: Delivered Bags -->
            <div class="bg-emerald-50/60 border border-emerald-200 rounded-xl p-3.5 text-right relative overflow-hidden shadow-2xs">
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold text-emerald-800">المحافظ المسلّمة</span>
                <span class="text-[10px] font-black bg-emerald-200/80 text-emerald-900 px-1.5 py-0.2 rounded-full font-mono">
                  {{ stats.progress_percentage || 0 }}%
                </span>
              </div>
              <div class="text-2xl font-black text-emerald-950 font-mono mt-1">{{ stats.delivered_bags || 0 }}</div>
              <span class="text-[11px] text-emerald-700">{{ stats.delivered_families || 0 }} عائلة استلمت</span>
              <span class="absolute -bottom-2 -left-2 text-4xl opacity-10 pointer-events-none select-none">✓</span>
            </div>

            <!-- Card 4: Pending Bags -->
            <div class="bg-amber-50/60 border border-amber-200 rounded-xl p-3.5 text-right relative overflow-hidden shadow-2xs">
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold text-amber-800">في انتظار التسليم</span>
                <span class="text-[10px] font-black bg-amber-200/80 text-amber-900 px-1.5 py-0.2 rounded-full font-mono">
                  {{ 100 - (stats.progress_percentage || 0) }}%
                </span>
              </div>
              <div class="text-2xl font-black text-amber-950 font-mono mt-1">{{ stats.pending_bags || 0 }}</div>
              <span class="text-[11px] text-amber-700">{{ stats.pending_families || 0 }} عائلة بالانتظار</span>
              <span class="absolute -bottom-2 -left-2 text-4xl opacity-10 pointer-events-none select-none">⏳</span>
            </div>
          </div>

          <!-- Row 2: Educational Stages Breakdown (ابتدائي، متوسط، ثانوي) -->
          <div class="bg-white border border-slate-200 rounded-xl p-4 shadow-2xs">
            <div class="flex items-center justify-between mb-3 border-b border-slate-100 pb-2.5">
              <div class="flex items-center gap-2">
                <span class="text-base">🏫</span>
                <h3 class="text-sm font-black text-slate-800">توزيع المحافظ المدرسية حسب الأطوار التعليمية</h3>
              </div>
              <span class="text-xs text-slate-400 font-medium">من إجمالي {{ stats.total_bags }} محفظة</span>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
              <!-- Primary -->
              <div class="bg-emerald-50/40 border border-emerald-200/80 rounded-xl p-3.5 flex flex-col justify-between gap-2">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-bold text-emerald-800 flex items-center gap-1.5">
                    <span class="w-2.5 h-2.5 rounded-full bg-emerald-500"></span>
                    <span>الطور الابتدائي</span>
                  </span>
                  <span class="text-xs font-extrabold text-emerald-900 font-mono bg-emerald-100/80 px-2 py-0.5 rounded-md">
                    {{ primaryPercentage }}%
                  </span>
                </div>
                <div class="flex items-baseline justify-between mt-1">
                  <span class="text-2xl font-black text-emerald-950 font-mono">{{ stats.primary_total }}</span>
                  <div class="text-left flex flex-col items-end">
                    <span class="text-xs text-emerald-700">محفظة</span>
                    <span class="text-[10px] font-bold text-emerald-800 bg-emerald-200/50 px-1.5 py-0.5 rounded mt-1 border border-emerald-200/50">مسلمة: {{ stats.primary_delivered }}</span>
                  </div>
                </div>
                <!-- Progress Bar -->
                <div class="w-full h-2 bg-emerald-100 rounded-full overflow-hidden mt-1">
                  <div class="h-full bg-emerald-500 rounded-full transition-all duration-500" :style="{ width: `${primaryPercentage}%` }"></div>
                </div>
              </div>

              <!-- Middle -->
              <div class="bg-amber-50/40 border border-amber-200/80 rounded-xl p-3.5 flex flex-col justify-between gap-2">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-bold text-amber-800 flex items-center gap-1.5">
                    <span class="w-2.5 h-2.5 rounded-full bg-amber-500"></span>
                    <span>الطور المتوسط</span>
                  </span>
                  <span class="text-xs font-extrabold text-amber-900 font-mono bg-amber-100/80 px-2 py-0.5 rounded-md">
                    {{ middlePercentage }}%
                  </span>
                </div>
                <div class="flex items-baseline justify-between mt-1">
                  <span class="text-2xl font-black text-amber-950 font-mono">{{ stats.middle_total }}</span>
                  <div class="text-left flex flex-col items-end">
                    <span class="text-xs text-amber-700">محفظة</span>
                    <span class="text-[10px] font-bold text-amber-800 bg-amber-200/50 px-1.5 py-0.5 rounded mt-1 border border-amber-200/50">مسلمة: {{ stats.middle_delivered }}</span>
                  </div>
                </div>
                <!-- Progress Bar -->
                <div class="w-full h-2 bg-amber-100 rounded-full overflow-hidden mt-1">
                  <div class="h-full bg-amber-500 rounded-full transition-all duration-500" :style="{ width: `${middlePercentage}%` }"></div>
                </div>
              </div>

              <!-- Secondary -->
              <div class="bg-sky-50/40 border border-sky-200/80 rounded-xl p-3.5 flex flex-col justify-between gap-2">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-bold text-sky-800 flex items-center gap-1.5">
                    <span class="w-2.5 h-2.5 rounded-full bg-sky-500"></span>
                    <span>الطور الثانوي</span>
                  </span>
                  <span class="text-xs font-extrabold text-sky-900 font-mono bg-sky-100/80 px-2 py-0.5 rounded-md">
                    {{ secondaryPercentage }}%
                  </span>
                </div>
                <div class="flex items-baseline justify-between mt-1">
                  <span class="text-2xl font-black text-sky-950 font-mono">{{ stats.secondary_total }}</span>
                  <div class="text-left flex flex-col items-end">
                    <span class="text-xs text-sky-700">محفظة</span>
                    <span class="text-[10px] font-bold text-sky-800 bg-sky-200/50 px-1.5 py-0.5 rounded mt-1 border border-sky-200/50">مسلمة: {{ stats.secondary_delivered }}</span>
                  </div>
                </div>
                <!-- Progress Bar -->
                <div class="w-full h-2 bg-sky-100 rounded-full overflow-hidden mt-1">
                  <div class="h-full bg-sky-500 rounded-full transition-all duration-500" :style="{ width: `${secondaryPercentage}%` }"></div>
                </div>
              </div>
            </div>
          </div>

          <!-- Row 3: Field Handover Progress Track & Detailed Comparison -->
          <div class="bg-white border border-slate-200 rounded-xl p-4 shadow-2xs">
            <div class="flex items-center justify-between mb-3 border-b border-slate-100 pb-2.5">
              <div class="flex items-center gap-2">
                <span class="text-base">📦</span>
                <h3 class="text-sm font-black text-slate-800">متابعة عملية التسليم والتوزيع الميداني</h3>
              </div>
              <span 
                class="text-xs font-bold px-2.5 py-0.5 rounded-full"
                :class="stats.progress_percentage === 100 ? 'bg-emerald-100 text-emerald-800 border border-emerald-200' : 'bg-indigo-50 text-indigo-700 border border-indigo-200'"
              >
                {{ stats.progress_percentage || 0 }}% مكتمل
              </span>
            </div>

            <!-- Big Progress Bar -->
            <div class="w-full h-3.5 bg-slate-100 rounded-full overflow-hidden p-0.5 border border-slate-200/80 mb-3">
              <div 
                class="h-full rounded-full transition-all duration-500 bg-linear-to-r from-emerald-500 via-teal-500 to-emerald-600 shadow-xs"
                :style="{ width: `${stats.progress_percentage || 0}%` }"
              ></div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
              <div class="bg-emerald-50/50 border border-emerald-200/70 rounded-xl p-3 flex items-center justify-between">
                <div>
                  <span class="font-bold text-emerald-900 block">المحافظ والأسر المستلمة</span>
                  <span class="text-[11px] text-emerald-700">{{ stats.delivered_families || 0 }} عائلة استلمت محافظها</span>
                </div>
                <div class="text-left">
                  <span class="text-xl font-black text-emerald-900 font-mono">{{ stats.delivered_bags || 0 }}</span>
                  <span class="text-[10px] text-emerald-600 block">محفظة مسلّمة</span>
                </div>
              </div>

              <div class="bg-amber-50/50 border border-amber-200/70 rounded-xl p-3 flex items-center justify-between">
                <div>
                  <span class="font-bold text-amber-900 block">المحافظ والأسر في الانتظار</span>
                  <span class="text-[11px] text-amber-700">{{ stats.pending_families || 0 }} عائلة بانتظار الاستلام</span>
                </div>
                <div class="text-left">
                  <span class="text-xl font-black text-amber-900 font-mono">{{ stats.pending_bags || 0 }}</span>
                  <span class="text-[10px] text-amber-600 block">محفظة متبقية</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Row 4: Social Status Breakdown (توزيع الحالات الاجتماعية) -->
          <div class="bg-white border border-slate-200 rounded-xl p-4 shadow-2xs">
            <div class="flex items-center justify-between mb-3 border-b border-slate-100 pb-2.5">
              <div class="flex items-center gap-2">
                <span class="text-base">🏷️</span>
                <h3 class="text-sm font-black text-slate-800">توزيع المستفيدين والمحافظ حسب الحالة الاجتماعية</h3>
              </div>
              <span class="text-xs text-slate-400 font-medium">{{ socialStatusStats.length }} فئات مسجلة</span>
            </div>

            <div v-if="socialStatusStats.length > 0" class="overflow-x-auto">
              <table class="w-full text-right text-xs">
                <thead class="bg-slate-50 text-slate-600 font-bold border-b border-slate-200">
                  <tr>
                    <th class="py-2.5 px-3">الحالة الاجتماعية</th>
                    <th class="py-2.5 px-3 text-center">عدد الأسر</th>
                    <th class="py-2.5 px-3 text-center">النسبة من الأسر</th>
                    <th class="py-2.5 px-3 text-center">مجموع الحقائب</th>
                    <th class="py-2.5 px-3">التوزيع البياني</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-100">
                  <tr v-for="st in socialStatusStats" :key="st.status" class="hover:bg-slate-50/80 transition">
                    <td class="py-2.5 px-3 font-bold text-slate-800">
                      <span class="px-2.5 py-1 rounded-full text-[11px] font-medium" :class="{
                        'bg-red-50 text-red-700 border border-red-200': st.status === 'بدون دخل',
                        'bg-orange-50 text-orange-700 border border-orange-200': st.status === 'ضعيف الدخل',
                        'bg-blue-50 text-blue-700 border border-blue-200': st.status === 'متقاعد',
                        'bg-purple-50 text-purple-700 border border-purple-200': st.status === 'مرض مزمن',
                        'bg-teal-50 text-teal-700 border border-teal-200': st.status === 'إعاقة',
                        'bg-slate-100 text-slate-700 border border-slate-200': !['بدون دخل', 'ضعيف الدخل', 'متقاعد', 'مرض مزمن', 'إعاقة'].includes(st.status)
                      }">
                        {{ st.status }}
                      </span>
                    </td>
                    <td class="py-2.5 px-3 text-center font-mono font-bold text-slate-700">{{ st.familiesCount }}</td>
                    <td class="py-2.5 px-3 text-center font-mono font-bold text-indigo-700">{{ st.percentage }}%</td>
                    <td class="py-2.5 px-3 text-center font-mono font-black text-indigo-900">{{ st.bagsCount }}</td>
                    <td class="py-2.5 px-3">
                      <div class="w-full bg-slate-100 rounded-full h-2 overflow-hidden">
                        <div class="bg-indigo-500 h-full rounded-full transition-all duration-300" :style="{ width: `${st.percentage}%` }"></div>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
            <div v-else class="text-center py-6 text-slate-400 text-xs">
              لا توجد بيانات مسجلة في هذا الموسم حالياً.
            </div>
          </div>

          <!-- Row 5: Education Level Stats (توزيع الأطفال حسب السنة الدراسية) -->
          <div class="bg-white border border-slate-200 rounded-xl p-4 shadow-2xs">
            <div class="flex flex-wrap items-center justify-between gap-2 mb-3 border-b border-slate-100 pb-2.5">
              <div class="flex items-center gap-2">
                <span class="text-base">🏫</span>
                <h3 class="text-sm font-black text-slate-800">توزيع الأطفال المتمدرسين حسب السنة الدراسية</h3>
              </div>
              <div class="flex items-center gap-1.5 flex-wrap text-[11px] font-mono font-bold">
                <span class="bg-emerald-50 text-emerald-800 border border-emerald-200/80 px-2 py-0.5 rounded-md flex items-center gap-1">
                  <span>✓ مسلّم:</span>
                  <span>{{ educationLevelStore.totalDeliveredInStats }}</span>
                </span>
                <span class="bg-amber-50 text-amber-800 border border-amber-200/80 px-2 py-0.5 rounded-md flex items-center gap-1">
                  <span>⏳ انتظار:</span>
                  <span>{{ educationLevelStore.totalPendingInStats }}</span>
                </span>
                <span class="bg-slate-100 text-slate-800 border border-slate-200 px-2.5 py-0.5 rounded-md flex items-center gap-1">
                  <span>المجموع:</span>
                  <span>{{ educationLevelStore.totalChildrenInStats }} طفل</span>
                </span>
              </div>
            </div>

            <div v-if="educationLevelStats.length > 0">
              <div v-for="stageKey in educationLevelStore.orderedStages" :key="stageKey" class="mb-3.5 last:mb-0">
                <div v-if="educationLevelStore.statsGroupedByStage[stageKey]" class="bg-slate-50/60 rounded-xl p-3 border border-slate-200/70">
                  <!-- Stage Header with Delivered, Pending, and Total -->
                  <div class="flex flex-wrap items-center justify-between gap-2 mb-2.5">
                    <span class="text-xs font-black flex items-center gap-1.5" :class="{
                      'text-emerald-900': stageKey === 'ابتدائي',
                      'text-amber-900': stageKey === 'متوسط',
                      'text-sky-900': stageKey === 'ثانوي'
                    }">
                      <span class="w-2.5 h-2.5 rounded-full" :class="{
                        'bg-emerald-500': stageKey === 'ابتدائي',
                        'bg-amber-500': stageKey === 'متوسط',
                        'bg-sky-500': stageKey === 'ثانوي'
                      }"></span>
                      <span>الطور {{ stageKey }}</span>
                    </span>

                    <div class="flex items-center gap-1.5 font-mono text-[10px] font-bold">
                      <span class="bg-emerald-100/80 text-emerald-900 border border-emerald-200 px-2 py-0.5 rounded-full">
                        مسلّم: {{ educationLevelStore.statsGroupedByStage[stageKey].delivered }}
                      </span>
                      <span class="bg-amber-100/80 text-amber-900 border border-amber-200 px-2 py-0.5 rounded-full">
                        انتظار: {{ educationLevelStore.statsGroupedByStage[stageKey].pending }}
                      </span>
                      <span class="px-2.5 py-0.5 rounded-full border shadow-2xs" :class="{
                        'bg-emerald-600 text-white border-emerald-700': stageKey === 'ابتدائي',
                        'bg-amber-600 text-white border-amber-700': stageKey === 'متوسط',
                        'bg-sky-600 text-white border-sky-700': stageKey === 'ثانوي'
                      }">
                        المجموع: {{ educationLevelStore.statsGroupedByStage[stageKey].total }}
                      </span>
                    </div>
                  </div>

                  <!-- Levels Grid -->
                  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-2">
                    <div v-for="level in educationLevelStore.statsGroupedByStage[stageKey].levels" :key="level.id"
                      class="bg-white border rounded-xl p-2.5 flex flex-col justify-between transition-all hover:shadow-xs" :class="{
                        'border-emerald-200/90 hover:border-emerald-300': stageKey === 'ابتدائي',
                        'border-amber-200/90 hover:border-amber-300': stageKey === 'متوسط',
                        'border-sky-200/90 hover:border-sky-300': stageKey === 'ثانوي'
                      }">
                      <!-- Year Name & Total Badge -->
                      <div class="flex items-center justify-between gap-1 mb-2 pb-1.5 border-b border-slate-100">
                        <span class="text-[11px] font-bold text-slate-700 truncate" :title="level.year_name">
                          {{ level.year_name }}
                        </span>
                        <span class="text-xs font-black font-mono px-1.5 py-0.2 rounded" :class="{
                          'bg-emerald-50 text-emerald-800 border border-emerald-200/60': stageKey === 'ابتدائي',
                          'bg-amber-50 text-amber-800 border border-amber-200/60': stageKey === 'متوسط',
                          'bg-sky-50 text-sky-800 border border-sky-200/60': stageKey === 'ثانوي'
                        }">
                          {{ level.children_count }}
                        </span>
                      </div>

                      <!-- Sub-boxes: Delivered vs Pending -->
                      <div class="grid grid-cols-2 gap-1.5 text-center">
                        <!-- Delivered (مسلم) -->
                        <div class="bg-emerald-50/70 border border-emerald-200/60 rounded-lg py-1 px-1 flex flex-col items-center">
                          <span class="text-[9px] text-emerald-700 font-bold leading-tight">مسلّم</span>
                          <span class="text-xs font-black text-emerald-800 font-mono leading-tight mt-0.5">
                            {{ level.delivered_count || 0 }}
                          </span>
                        </div>

                        <!-- Pending (في الانتظار) -->
                        <div class="bg-amber-50/70 border border-amber-200/60 rounded-lg py-1 px-1 flex flex-col items-center">
                          <span class="text-[9px] text-amber-700 font-bold leading-tight">انتظار</span>
                          <span class="text-xs font-black text-amber-800 font-mono leading-tight mt-0.5">
                            {{ level.pending_count || 0 }}
                          </span>
                        </div>
                      </div>

                      <!-- Delivery Progress Mini-bar -->
                      <div class="w-full bg-slate-100 rounded-full h-1 mt-2 overflow-hidden" :title="`${level.children_count > 0 ? Math.round(((level.delivered_count || 0) / level.children_count) * 100) : 0}% تم التسليم`">
                        <div 
                          class="h-full rounded-full transition-all duration-300"
                          :class="{
                            'bg-emerald-500': stageKey === 'ابتدائي',
                            'bg-amber-500': stageKey === 'متوسط',
                            'bg-sky-500': stageKey === 'ثانوي'
                          }"
                          :style="{ width: `${level.children_count > 0 ? (((level.delivered_count || 0) / level.children_count) * 100) : 0}%` }"
                        ></div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div v-else class="text-center py-6 text-slate-400 text-xs">
              لا توجد بيانات أطفال متمدرسين مسجلة في هذا الموسم حالياً.
            </div>
          </div>

        </div>

        <!-- Modal Footer -->
        <div class="flex items-center justify-between px-6 py-3.5 border-t border-slate-100 bg-slate-50/80 shrink-0">
          <div class="text-[11px] text-slate-500 font-medium">
            بيانات رقمية وتحليلية دقيقة ومحدثة لحظياً
          </div>
          <div class="flex items-center gap-2">
            <button 
              type="button" 
              @click="openCustomPrintModal('list'); closeStatsDetailsModal()" 
              class="flex items-center gap-1.5 bg-white border border-slate-300 hover:bg-indigo-50 text-slate-700 hover:text-indigo-700 text-xs font-bold px-4 py-2 rounded-xl shadow-2xs transition cursor-pointer"
            >
              <span>🖨️</span>
              <span>طباعة تقرير ومحضر</span>
            </button>
            <button 
              type="button" 
              @click="closeStatsDetailsModal" 
              class="px-5 py-2 text-xs font-bold text-white bg-slate-800 hover:bg-slate-900 rounded-xl shadow-2xs transition cursor-pointer"
            >
              إغلاق
            </button>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, nextTick, onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import * as XLSX from 'xlsx';
import { safeInvoke } from './stores/tauri';
import { useCampaignStore } from './stores/campaigns';
import { useSocialStatusStore } from './stores/socialStatuses';
import { useOrganizationStore } from './stores/organization';
import { useBeneficiaryStore } from './stores/beneficiaries';
import { useEducationLevelStore } from './stores/educationLevels';
import ImportModal from './components/ImportModal.vue';
import SchoolFormPrint from './components/SchoolFormPrint.vue';
import {
  confirmDelete,
  notifySuccess,
  notifyError,
  notifyWarning,
  notifyInfo,
  toastSuccess
} from './utils/alerts';

// Sidebar state
const sidebarOpen = ref(false);
const toggleSidebar = () => { sidebarOpen.value = !sidebarOpen.value; };
const closeSidebar = () => { sidebarOpen.value = false; };

// Initialize Pinia stores
const campaignStore = useCampaignStore();
const statusStore = useSocialStatusStore();
const orgStore = useOrganizationStore();
const beneficiaryStore = useBeneficiaryStore();
const educationLevelStore = useEducationLevelStore();

// Education levels reactive store properties
const { educationLevels, educationLevelStats } = storeToRefs(educationLevelStore);
const showEducationLevelModal = ref(false);
const newEduStage = ref('ابتدائي');
const newEduYearName = ref('');
const newEduYearOrder = ref(1);
const editingEduId = ref(null);
const editingEduName = ref('');

// Reactive store properties via storeToRefs
const { campaigns, selectedCampaignId, showCampaignModal, newCampaignYear, rolloverPrevious, activeCampaignLabel } = storeToRefs(campaignStore);
const { socialStatuses, showStatusModal, newStatusName, editingStatusId, editingStatusName, statusError } = storeToRefs(statusStore);
const { orgSettings, showSettingsModal, settingsForm } = storeToRefs(orgStore);
const { beneficiaries, stats, searchQuery, selectedStatus, deliveryFilter, showModal, form, isLoading } = storeToRefs(beneficiaryStore);

// Formatted current date for report printing
const currentDateFormatted = computed(() => {
  const d = new Date();
  return `${d.getDate().toString().padStart(2, '0')}/${(d.getMonth() + 1).toString().padStart(2, '0')}/${d.getFullYear()}`;
});

// Dynamic print statistics & title based on delivery filter
const printTotalBags = computed(() => {
  const target = isPrinting.value ? printTargetBeneficiaries.value : sortedBeneficiaries.value;
  return target.reduce((acc, b) => {
    return acc + (Number(b.primary_count) || 0) + (Number(b.middle_count) || 0) + (Number(b.secondary_count) || 0);
  }, 0);
});

const printReportTitle = computed(() => {
  if (deliveryFilter.value === 'delivered') {
    return 'محضر تسليم الحقائب المدرسية (المستلمين)';
  } else if (deliveryFilter.value === 'pending') {
    return 'محضر تسليم وتوقيع الحقائب المدرسية (في الانتظار)';
  }
  return 'محضر إحصاء وتوزيع الحقائب المدرسية';
});

// ==========================================
// Sorting Logic
// ==========================================
const sortKey = ref('record_no'); // 'record_no' | 'guardian_name'
const sortOrder = ref('asc'); // 'asc' | 'desc'

const toggleSort = (key) => {
  if (sortKey.value === key) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortKey.value = key;
    sortOrder.value = 'asc';
  }
  currentPage.value = 1;
};

const sortedBeneficiaries = computed(() => {
  let list = [...beneficiaries.value];

  // Filter by delivery status
  if (deliveryFilter.value === 'delivered') {
    list = list.filter(b => !!b.is_delivered);
  } else if (deliveryFilter.value === 'pending') {
    list = list.filter(b => !b.is_delivered);
  }

  if (!sortKey.value) return list;

  return list.sort((a, b) => {
    if (sortKey.value === 'record_no') {
      const numA = a.record_no !== null && a.record_no !== undefined ? Number(a.record_no) : (a.id || 0);
      const numB = b.record_no !== null && b.record_no !== undefined ? Number(b.record_no) : (b.id || 0);
      return sortOrder.value === 'asc' ? numA - numB : numB - numA;
    } else if (sortKey.value === 'guardian_name') {
      const nameA = (a.guardian_name || '').trim();
      const nameB = (b.guardian_name || '').trim();
      const cmp = nameA.localeCompare(nameB, 'ar', { sensitivity: 'base' });
      return sortOrder.value === 'asc' ? cmp : -cmp;
    }
    return 0;
  });
});

// ==========================================
// Detailed Statistics Modal & Computed Analytics
// ==========================================
const showStatsDetailsModal = ref(false);
const socialStatusStats = ref([]);

const closeStatsDetailsModal = () => {
  showStatsDetailsModal.value = false;
};

const primaryPercentage = computed(() => {
  const total = Number(stats.value?.total_bags) || 0;
  if (total === 0) return 0;
  return Math.round(((Number(stats.value?.primary_total) || 0) / total) * 100);
});

const middlePercentage = computed(() => {
  const total = Number(stats.value?.total_bags) || 0;
  if (total === 0) return 0;
  return Math.round(((Number(stats.value?.middle_total) || 0) / total) * 100);
});

const secondaryPercentage = computed(() => {
  const total = Number(stats.value?.total_bags) || 0;
  if (total === 0) return 0;
  return Math.round(((Number(stats.value?.secondary_total) || 0) / total) * 100);
});

const openStatsDetailsModal = async () => {
  showStatsDetailsModal.value = true;
  try {
    const [statsData] = await Promise.all([
      safeInvoke('get_social_status_stats', { campaignId: selectedCampaignId.value }),
      educationLevelStore.fetchStats(selectedCampaignId.value),
    ]);
    
    // Calculate total families to compute percentages
    const totalFamilies = statsData.reduce((sum, item) => sum + Number(item.families_count), 0);
    
    socialStatusStats.value = statsData.map(item => ({
      status: item.status,
      familiesCount: item.families_count,
      bagsCount: item.bags_count,
      percentage: totalFamilies > 0 ? Math.round((Number(item.families_count) / totalFamilies) * 100) : 0
    }));
  } catch (err) {
    console.error("فشل جلب إحصائيات الحالة الاجتماعية", err);
    socialStatusStats.value = [];
  }
};
// ==========================================
// Pagination Logic
// ==========================================
const pageSize = ref(10); // Default to 10 rows to keep workspace compact and prevent scrollbar
const currentPage = ref(1);
const isPrinting = ref(false);

const totalCount = computed(() => sortedBeneficiaries.value.length);

const totalPages = computed(() => {
  if (pageSize.value === 'all' || totalCount.value === 0) return 1;
  return Math.max(1, Math.ceil(totalCount.value / Number(pageSize.value)));
});

// Auto-adjust page when total count or page size changes
watch([totalCount, pageSize], () => {
  if (currentPage.value > totalPages.value) {
    currentPage.value = Math.max(1, totalPages.value);
  }
});

// Reset page on search or filter change
watch([searchQuery, selectedStatus, selectedCampaignId, deliveryFilter], () => {
  currentPage.value = 1;
});

const displayedBeneficiaries = computed(() => {
  if (isPrinting.value) {
    if (printDocType.value === 'list') {
      return printTargetBeneficiaries.value;
    }
    return [];
  }
  if (pageSize.value === 'all') {
    return sortedBeneficiaries.value;
  }
  const size = Number(pageSize.value);
  const start = (currentPage.value - 1) * size;
  return sortedBeneficiaries.value.slice(start, start + size);
});

const paginationRangeText = computed(() => {
  if (totalCount.value === 0) return '0';
  if (pageSize.value === 'all') return `1 - ${totalCount.value}`;
  const size = Number(pageSize.value);
  const start = (currentPage.value - 1) * size + 1;
  const end = Math.min(currentPage.value * size, totalCount.value);
  return `${start} - ${end}`;
});

const visiblePages = computed(() => {
  const total = totalPages.value;
  const current = currentPage.value;
  if (total <= 7) {
    return Array.from({ length: total }, (_, i) => i + 1);
  }
  if (current <= 4) {
    return [1, 2, 3, 4, 5, '...', total];
  }
  if (current >= total - 3) {
    return [1, '...', total - 4, total - 3, total - 2, total - 1, total];
  }
  return [1, '...', current - 1, current, current + 1, '...', total];
});

const goToPage = (page) => {
  if (typeof page === 'number' && page >= 1 && page <= totalPages.value) {
    currentPage.value = page;
  }
};

// ==========================================
// Bulk Selection Logic (حذف جماعي حسب الاختيار)
// ==========================================
const selectedBeneficiaryIds = ref([]);

// ==========================================
// Custom Print State & Filtering Logic
// ==========================================
const showPrintCustomModal = ref(false);
const printDocType = ref('list'); // 'list' | 'forms'
const printScope = ref('all'); // 'all' | 'selected' | 'range' | 'single' | 'blank'
const printRangeFrom = ref(1);
const printRangeTo = ref(100);
const printBlankCopiesCount = ref(1);
const singlePrintBeneficiary = ref(null);
const printOrientation = ref('landscape'); // 'landscape' | 'portrait'

// Dynamically inject @page orientation CSS rule into document head
const updatePageOrientationStyle = (orientation = printOrientation.value) => {
  let styleEl = document.getElementById('dynamic-page-orientation-style');
  if (!styleEl) {
    styleEl = document.createElement('style');
    styleEl.id = 'dynamic-page-orientation-style';
    document.head.appendChild(styleEl);
  }
  styleEl.textContent = `@media print { @page { size: A4 ${orientation}; margin: 8mm 10mm; } }`;
};

watch(printOrientation, (newVal) => {
  updatePageOrientationStyle(newVal);
}, { immediate: true });

watch(printDocType, (newVal) => {
  if (newVal === 'list' && printScope.value === 'blank') {
    printScope.value = 'all';
  }
});

const fullBeneficiariesForPrint = ref([]);

const printTargetBeneficiaries = computed(() => {
  if (printDocType.value === 'forms' && printScope.value === 'blank') {
    const count = Math.max(Number(printBlankCopiesCount.value) || 1, 1);
    return Array.from({ length: count }, (_, i) => ({ id: `blank-${i}`, _isBlank: true }));
  }
  if (printScope.value === 'single' && singlePrintBeneficiary.value) {
    return [singlePrintBeneficiary.value];
  }
  if (printScope.value === 'selected') {
    const set = new Set(selectedBeneficiaryIds.value);
    return sortedBeneficiaries.value.filter(b => set.has(b.id));
  }
  
  // Use full, unfiltered beneficiaries for 'range' and 'all'
  let targetList = fullBeneficiariesForPrint.value.length > 0 
    ? fullBeneficiariesForPrint.value 
    : sortedBeneficiaries.value;
    
  // Ensure it's sorted by record_no
  targetList = [...targetList].sort((a, b) => {
    const numA = a.record_no !== null && a.record_no !== undefined ? Number(a.record_no) : (a.id || 0);
    const numB = b.record_no !== null && b.record_no !== undefined ? Number(b.record_no) : (b.id || 0);
    return numA - numB;
  });

  if (printScope.value === 'range') {
    const from = Number(printRangeFrom.value) || 1;
    const to = Number(printRangeTo.value) || 1;
    const min = Math.min(from, to);
    const max = Math.max(from, to);
    return targetList.filter((b, idx) => {
      const recNo = b.record_no !== null && b.record_no !== undefined ? Number(b.record_no) : (idx + 1);
      return recNo >= min && recNo <= max;
    });
  }
  // 'all'
  return targetList;
});

const isAllPageSelected = computed(() => {
  if (displayedBeneficiaries.value.length === 0) return false;
  return displayedBeneficiaries.value.every(item => selectedBeneficiaryIds.value.includes(item.id));
});

const isSomePageSelected = computed(() => {
  return displayedBeneficiaries.value.some(item => selectedBeneficiaryIds.value.includes(item.id));
});

const toggleSelectAllPage = () => {
  const pageIds = displayedBeneficiaries.value.map(item => item.id);
  if (isAllPageSelected.value) {
    const pageIdSet = new Set(pageIds);
    selectedBeneficiaryIds.value = selectedBeneficiaryIds.value.filter(id => !pageIdSet.has(id));
  } else {
    const combined = new Set([...selectedBeneficiaryIds.value, ...pageIds]);
    selectedBeneficiaryIds.value = Array.from(combined);
  }
};

const selectAllInCampaign = () => {
  selectedBeneficiaryIds.value = sortedBeneficiaries.value.map(item => item.id);
};

const clearSelection = () => {
  selectedBeneficiaryIds.value = [];
};

// Clean up selection when campaign or beneficiaries change
watch([selectedCampaignId, beneficiaries], () => {
  const validIds = new Set(beneficiaries.value.map(b => b.id));
  selectedBeneficiaryIds.value = selectedBeneficiaryIds.value.filter(id => validIds.has(id));
});

const bulkDelete = async () => {
  const count = selectedBeneficiaryIds.value.length;
  if (count === 0) return;

  const confirmed = await confirmDelete(
    'تأكيد الحذف الجماعي',
    `هل أنت متأكد من رغبتك في حذف ${count} مستفيد محدد من هذا الموسم؟ لن يمكنك التراجع بعد إتمام العملية.`
  );

  if (confirmed) {
    try {
      const deletedCount = await beneficiaryStore.bulkDeleteBeneficiaries(
        selectedBeneficiaryIds.value,
        selectedCampaignId.value
      );
      selectedBeneficiaryIds.value = [];
      await statusStore.loadSocialStatuses(); // update status usage count
      toastSuccess(`تم حذف ${deletedCount || count} مستفيد بنجاح`);
    } catch (e) {
      notifyError('خطأ أثناء الحذف الجماعي', e.message || e);
    }
  }
};

const bulkDeliver = async (isDelivered) => {
  const count = selectedBeneficiaryIds.value.length;
  if (count === 0) return;

  const actionText = isDelivered ? 'تأكيد استلام' : 'إلغاء استلام (في الانتظار)';
  const confirmed = await confirmDelete(
    'تأكيد العملية الجماعية',
    `هل أنت متأكد من تغيير حالة ${count} مستفيد محدد إلى (${actionText})؟`
  );

  if (confirmed) {
    try {
      await beneficiaryStore.bulkSetDelivery(
        selectedBeneficiaryIds.value,
        isDelivered,
        selectedCampaignId.value
      );
      toastSuccess(`تم تحديث حالة ${count} مستفيد بنجاح`);
    } catch (e) {
      notifyError('خطأ أثناء التحديث الجماعي', e.message || e);
    }
  }
};

// Organization Settings Methods
const openSettingsModal = () => orgStore.openSettingsModal();
const saveSettings = async () => {
  try {
    await orgStore.saveOrgSettings();
    notifySuccess('تم الحفظ بنجاح', 'تم حفظ إعدادات الجمعية بنجاح!');
  } catch (e) {
    notifyError('خطأ أثناء حفظ الإعدادات', e.message || e);
  }
};

// Social Status Methods
const openStatusModal = () => statusStore.openStatusModal();
const addSocialStatus = async () => {
  try {
    await statusStore.addSocialStatus();
    toastSuccess('تمت إضافة الحالة الاجتماعية بنجاح');
  } catch (e) { }
};
const startEditStatus = (item) => statusStore.startEditStatus(item);
const cancelEditStatus = () => statusStore.cancelEditStatus();
const saveEditStatus = async (item) => {
  try {
    await statusStore.saveEditStatus(item);
    await beneficiaryStore.fetchData(selectedCampaignId.value);
    toastSuccess('تم تعديل الحالة الاجتماعية بنجاح');
  } catch (e) { }
};
const deleteSocialStatus = async (item) => {
  if (item.count > 0) {
    notifyWarning('لا يمكن الحذف', `لا يمكن حذف الحالة الاجتماعية '${item.name}' لأنها مرتبطة حالياً بـ ${item.count} مستفيد.`);
    return;
  }
  const confirmed = await confirmDelete(
    'حذف الحالة الاجتماعية',
    `هل أنت متأكد من رغبتك في حذف الحالة الاجتماعية '${item.name}'؟`
  );
  if (!confirmed) {
    return;
  }
  try {
    await statusStore.deleteSocialStatus(item);
    toastSuccess(`تم حذف الحالة الاجتماعية '${item.name}' بنجاح`);
  } catch (e) {
    notifyError('خطأ أثناء الحذف', e.message || e);
  }
};

// Education Level Methods
const openEducationLevelModal = () => {
  showEducationLevelModal.value = true;
  educationLevelStore.fetchAll();
};
const closeEducationLevelModal = () => {
  showEducationLevelModal.value = false;
  editingEduId.value = null;
  editingEduName.value = '';
};
const addEducationLevel = async () => {
  if (!newEduYearName.value.trim()) {
    notifyWarning('تنبيه', 'يرجى إدخال اسم السنة الدراسية');
    return;
  }
  try {
    await educationLevelStore.create(newEduStage.value, newEduYearName.value, newEduYearOrder.value);
    newEduYearName.value = '';
    newEduYearOrder.value = 1;
    toastSuccess('تمت إضافة السنة الدراسية بنجاح');
  } catch (e) {
    notifyError('خطأ', e.message || e);
  }
};
const startEditEdu = (item) => {
  editingEduId.value = item.id;
  editingEduName.value = item.year_name;
};
const cancelEditEdu = () => {
  editingEduId.value = null;
  editingEduName.value = '';
};
const saveEditEdu = async (item) => {
  try {
    await educationLevelStore.update(item.id, editingEduName.value);
    editingEduId.value = null;
    editingEduName.value = '';
    toastSuccess('تم تعديل السنة الدراسية بنجاح');
  } catch (e) {
    notifyError('خطأ أثناء التعديل', e.message || e);
  }
};
const deleteEducationLevel = async (item) => {
  if (item.count > 0) {
    notifyWarning('لا يمكن الحذف', `لا يمكن حذف السنة الدراسية '${item.year_name}' لأنها مرتبطة حالياً بـ ${item.count} طفل.`);
    return;
  }
  const confirmed = await confirmDelete('حذف السنة الدراسية', `هل أنت متأكد من حذف 'السنة ${item.year_name}'؟`);
  if (!confirmed) return;
  try {
    await educationLevelStore.remove(item.id);
    toastSuccess(`تم حذف 'السنة ${item.year_name}' بنجاح`);
  } catch (e) {
    notifyError('خطأ أثناء الحذف', e.message || e);
  }
};
// Education Stage Bag Consistency Logic
const stageBagsLimit = computed(() => ({
  'ابتدائي': Math.max(0, Number(form.value?.primary_count) || 0),
  'متوسط': Math.max(0, Number(form.value?.middle_count) || 0),
  'ثانوي': Math.max(0, Number(form.value?.secondary_count) || 0),
}));

const schooledChildrenCountByStage = computed(() => {
  const counts = { 'ابتدائي': 0, 'متوسط': 0, 'ثانوي': 0, nonSchooling: 0 };
  if (!form.value?.children || !Array.isArray(form.value.children)) return counts;

  for (const child of form.value.children) {
    if (!child.is_schooling) {
      counts.nonSchooling++;
      continue;
    }
    let stage = null;
    if (child.education_level_id) {
      const lvl = educationLevels.value.find(l => l.id === child.education_level_id);
      if (lvl) stage = lvl.stage;
    } else if (child.education_level) {
      if (child.education_level.includes('ابتدائي')) stage = 'ابتدائي';
      else if (child.education_level.includes('متوسط')) stage = 'متوسط';
      else if (child.education_level.includes('ثانوي')) stage = 'ثانوي';
    }

    if (stage && counts[stage] !== undefined) {
      counts[stage]++;
    }
  }
  return counts;
});

const stageConsistencyError = computed(() => {
  const stages = ['ابتدائي', 'متوسط', 'ثانوي'];
  for (const s of stages) {
    const cur = schooledChildrenCountByStage.value[s] || 0;
    const max = stageBagsLimit.value[s] || 0;
    if (cur > max) {
      return `عدد الأطفال المتمدرسين في طور (${s}) هو (${cur}) ويتجاوز عدد المحافظ المحددة لهذا الطور (${max}). يرجى مطابقة الأعداد للمتابعة.`;
    }
  }
  return '';
});

const isStageDisabledForChild = (stage, currentChild) => {
  const max = stageBagsLimit.value[stage] ?? 0;
  if (max <= 0) return true; // Stage not requested at all

  let countOther = 0;
  for (const c of (form.value?.children || [])) {
    if (c === currentChild) continue;
    if (c.is_schooling && c.education_level_id) {
      const lvl = educationLevels.value.find(l => l.id === c.education_level_id);
      if (lvl && lvl.stage === stage) {
        countOther++;
      }
    }
  }
  return countOther >= max;
};

const getStageSlotInfo = (stage, currentChild) => {
  const max = stageBagsLimit.value[stage] ?? 0;
  if (max === 0) return '0 محفظة - غير محدد';

  let countOther = 0;
  for (const c of (form.value?.children || [])) {
    if (c === currentChild) continue;
    if (c.is_schooling && c.education_level_id) {
      const lvl = educationLevels.value.find(l => l.id === c.education_level_id);
      if (lvl && lvl.stage === stage) {
        countOther++;
      }
    }
  }
  const remaining = Math.max(0, max - countOther);
  if (remaining === 0) return `مكتمل (${max}/${max})`;
  return `متاح: ${remaining} من ${max}`;
};

const onSchoolingToggle = (child) => {
  if (!child.is_schooling) {
    child.education_level_id = null;
    child.education_level = '';
  }
};

const onChildLevelChange = (child) => {
  if (!child.education_level_id) {
    child.education_level = '';
    return;
  }
  const level = educationLevels.value.find(l => l.id === child.education_level_id);
  if (!level) return;

  const stage = level.stage;
  const maxAllowed = stageBagsLimit.value[stage] ?? 0;

  let countOther = 0;
  for (const c of (form.value?.children || [])) {
    if (c === child) continue;
    if (c.is_schooling && c.education_level_id) {
      const cLvl = educationLevels.value.find(l => l.id === c.education_level_id);
      if (cLvl && cLvl.stage === stage) {
        countOther++;
      }
    }
  }

  if (countOther + 1 > maxAllowed) {
    child.education_level_id = null;
    child.education_level = '';
    if (maxAllowed === 0) {
      notifyWarning(
        'طور غير محدد في المحافظ',
        `لم يتم طلب أي محفظة لطور (${stage}) في هذه الاستمارة (العدد المحدد: 0). يرجى زيادة عدد محافظ (${stage}) أولاً في قسم إحصاء المحافظ.`
      );
    } else {
      notifyWarning(
        'تجاوز عدد المحافظ المحدد',
        `عدد محافظ طور (${stage}) المحددة في الاستمارة هو (${maxAllowed}) فقط، وقد تم حجزها بالكامل لأطفال متمدرسين آخرين.`
      );
    }
    return;
  }

  child.education_level = level.year_name;
};

// Automatically match legacy text education_level to education_level_id
watch(() => form.value?.children, (kids) => {
  if (!kids || !Array.isArray(kids)) return;
  for (const child of kids) {
    if (child.education_level && !child.education_level_id && educationLevels.value.length > 0) {
      const match = educationLevels.value.find(l => l.year_name === child.education_level || child.education_level.includes(l.year_name));
      if (match) {
        child.education_level_id = match.id;
      }
    }
  }
}, { deep: true, immediate: true });


// Campaign Methods
const openCampaignModal = () => campaignStore.openCampaignModal();
const onCampaignChange = async () => {
  await beneficiaryStore.fetchData(selectedCampaignId.value);
  if (showStatsDetailsModal.value) {
    await educationLevelStore.fetchStats(selectedCampaignId.value);
  }
};
const submitNewCampaign = async () => {
  try {
    const newCamp = await campaignStore.createCampaign();
    await beneficiaryStore.fetchData(selectedCampaignId.value);
    notifySuccess('تم إنشاء الموسم', `تم إنشاء الموسم الدراسي "${newCamp.year_label}" بنجاح!`);
  } catch (e) {
    notifyError('خطأ أثناء إنشاء الموسم', e.message || e);
  }
};

// ==========================================
// Campaign Management (edit / delete / active)
// ==========================================
const showCampaignMgmtModal = ref(false);
const campaignsWithCounts = ref([]);
const editingCampaignId = ref(null);
const editingCampaignLabel = ref('');

const loadCampaignsWithCounts = async () => {
  try {
    const data = await safeInvoke('get_campaigns_with_counts');
    if (Array.isArray(data)) campaignsWithCounts.value = data;
  } catch (e) {
    console.error('فشل جلب المواسم مع العدد:', e);
  }
};

const openCampaignMgmtModal = async () => {
  await loadCampaignsWithCounts();
  editingCampaignId.value = null;
  showCampaignMgmtModal.value = true;
};

const startEditCampaign = (c) => {
  editingCampaignId.value = c.id;
  editingCampaignLabel.value = c.year_label;
};

const cancelEditCampaign = () => {
  editingCampaignId.value = null;
  editingCampaignLabel.value = '';
};

const saveCampaignLabel = async (c) => {
  const raw = editingCampaignLabel.value.trim();
  if (!raw) return;
  // Auto-format bare year
  const newLabel = /^\d{4}$/.test(raw) ? `${raw}/${parseInt(raw) + 1}` : raw;
  try {
    await safeInvoke('update_campaign_label', { id: c.id, newLabel });
    cancelEditCampaign();
    await loadCampaignsWithCounts();
    await campaignStore.loadCampaigns();
    toastSuccess(`تم تعديل اسم الموسم إلى "${newLabel}" بنجاح`);
  } catch (e) {
    notifyError('خطأ أثناء التعديل', e.message || e);
  }
};

const setActiveCampaign = async (c) => {
  try {
    await safeInvoke('set_active_campaign', { id: c.id });
    await loadCampaignsWithCounts();
    await campaignStore.loadCampaigns();
    toastSuccess(`تم تعيين موسم "${c.year_label}" كموسم نشط`);
  } catch (e) {
    notifyError('خطأ', e.message || e);
  }
};

const deleteCampaignItem = async (c) => {
  if (c.record_count > 0) {
    notifyWarning(
      'لا يمكن الحذف',
      `موسم "${c.year_label}" محمي لأنه يحتوي على ${c.record_count} مستفيد.`
    );
    return;
  }
  const confirmed = await confirmDelete(
    'حذف الموسم الدراسي',
    `هل أنت متأكد من رغبتك في حذف موسم "${c.year_label}"؟ لا يمكن التراجع.`
  );
  if (!confirmed) return;
  try {
    await safeInvoke('delete_campaign', { id: c.id });
    await loadCampaignsWithCounts();
    await campaignStore.loadCampaigns();
    if (selectedCampaignId.value === c.id && campaigns.value.length > 0) {
      selectedCampaignId.value = campaigns.value[0].id;
      await beneficiaryStore.fetchData(selectedCampaignId.value);
    }
    toastSuccess(`تم حذف موسم "${c.year_label}" بنجاح`);
  } catch (e) {
    notifyError('خطأ أثناء الحذف', e.message || e);
  }
};


// Rollover: transfer beneficiaries between seasons (post-creation)
const showRolloverModal = ref(false);
const rolloverFromId = ref(null);

const openRolloverModal = () => {
  // Pre-select the most recent other campaign if available
  const others = campaigns.value.filter(c => c.id !== selectedCampaignId.value);
  rolloverFromId.value = others.length > 0 ? others[0].id : null;
  showRolloverModal.value = true;
};

const submitRollover = async () => {
  if (!rolloverFromId.value || !selectedCampaignId.value) return;

  const fromCampaign = campaigns.value.find(c => c.id === rolloverFromId.value);
  const fromLabel = fromCampaign ? fromCampaign.year_label : '...';

  try {
    const count = await safeInvoke('rollover_campaign_records', {
      fromCampaignId: rolloverFromId.value,
      toCampaignId: selectedCampaignId.value,
    });

    showRolloverModal.value = false;
    await beneficiaryStore.fetchData(selectedCampaignId.value);

    if (count === 0) {
      notifyInfo(
        'لا يوجد جديد',
        `جميع مستفيدي موسم "${fromLabel}" مسجلون بالفعل في الموسم الحالي.`
      );
    } else {
      notifySuccess(
        'تم الترحيل بنجاح',
        `تم نقل ${count} مستفيد من موسم "${fromLabel}" إلى الموسم الحالي.`
      );
    }
  } catch (e) {
    notifyError('خطأ أثناء الترحيل', e.message || e);
  }
};


// Beneficiary Modal Extra State & Live Match Detection
const showOptionalFields = ref(false);
const matchResult = ref({ match_type: 'none', message: '', matched_record: null, previous_campaign_year: null });
const rolloverApplied = ref(false);
let matchCheckTimer = null;

const performMatchCheck = async () => {
  if (form.value.id || rolloverApplied.value) {
    matchResult.value = { match_type: 'none', message: '', matched_record: null, previous_campaign_year: null };
    return;
  }
  const name = (form.value.guardian_name || '').trim();
  if (name.length < 3 || !selectedCampaignId.value) {
    matchResult.value = { match_type: 'none', message: '', matched_record: null, previous_campaign_year: null };
    return;
  }
  try {
    const res = await beneficiaryStore.checkBeneficiaryMatch(
      selectedCampaignId.value,
      name,
      form.value.birth_date,
      form.value.father_name,
      form.value.mother_name
    );
    matchResult.value = res;
  } catch (e) {
    console.error('فشل فحص تشابه المستفيد:', e);
  }
};

watch(
  [
    () => form.value.guardian_name,
    () => form.value.birth_date,
    () => form.value.father_name,
    () => form.value.mother_name,
  ],
  () => {
    if (form.value.id || rolloverApplied.value) return;
    clearTimeout(matchCheckTimer);
    matchCheckTimer = setTimeout(performMatchCheck, 300);
  }
);

const applyRolloverMatch = () => {
  if (!matchResult.value.matched_record) return;
  const m = matchResult.value.matched_record;
  form.value.guardian_id = m.guardian_id;
  form.value.guardian_name = m.guardian_name || form.value.guardian_name;
  form.value.phone = m.phone || form.value.phone;
  form.value.social_status = m.social_status || form.value.social_status;
  form.value.birth_date = m.birth_date || '';
  form.value.birth_place = m.birth_place || '';
  form.value.address = m.address || '';
  form.value.marital_status = m.marital_status || 'متزوج';
  form.value.father_name = m.father_name || '';
  form.value.mother_name = m.mother_name || '';
  form.value.spouse_name = m.spouse_name || '';
  form.value.monthly_income = m.monthly_income || '';
  form.value.children_count = m.children_count ?? null;
  form.value.primary_count = m.primary_count || 0;
  form.value.middle_count = m.middle_count || 0;
  form.value.secondary_count = m.secondary_count || 0;

  showOptionalFields.value = true;
  rolloverApplied.value = true;
  matchResult.value = { match_type: 'none', message: '', matched_record: null, previous_campaign_year: null };
  toastSuccess('تم ترحيل واستيراد بيانات المستفيد من الموسم السابق بنجاح');
};

// Beneficiary CRUD Methods
const fetchData = async () => {
  await beneficiaryStore.fetchData(selectedCampaignId.value);
};
const openModal = async (item = null) => {
  let nextNo = 1;
  if (!item && selectedCampaignId.value) {
    nextNo = await beneficiaryStore.getNextRecordNo(selectedCampaignId.value);
  }
  matchResult.value = { match_type: 'none', message: '', matched_record: null, previous_campaign_year: null };
  rolloverApplied.value = false;
  showOptionalFields.value = !!(item && (item.birth_date || item.father_name || item.address || item.monthly_income));
  beneficiaryStore.openModal(item, statusStore.defaultStatusName, nextNo);
};

const addChild = () => {
  if (!form.value.children) form.value.children = [];
  form.value.children.push({
    child_name: '',
    birth_date: '',
    is_schooling: true,
    education_level: '',
    education_level_id: null,
    school_name: ''
  });
};

const removeChild = (idx) => {
  if (form.value.children) {
    form.value.children.splice(idx, 1);
  }
};

const save = async () => {
  if (!form.value.record_no || Number(form.value.record_no) <= 0) {
    notifyWarning('تنبيه', 'يرجى إدخال رقم المستفيد (يجب أن يكون رقماً أكبر من 0)');
    return;
  }
  if (!form.value.guardian_name.trim()) {
    notifyWarning('تنبيه', 'اسم ولي الأمر مطلوب');
    return;
  }
  if (!selectedCampaignId.value) {
    notifyWarning('تنبيه', 'يرجى تحديد أو إنشاء موسم دراسي أولاً');
    return;
  }
  if (!form.value.id && matchResult.value.match_type === 'current_campaign') {
    notifyWarning('تنبيه: تكرار التسجيل', matchResult.value.message || 'هذا المستفيد مسجل بالفعل في هذا الموسم الحالي ولا يمكن تكرار إدخاله مرتين.');
    return;
  }
  if (stageConsistencyError.value) {
    notifyWarning('تنبيه عدم تناسق المحافظ', stageConsistencyError.value);
    return;
  }
  try {
    await beneficiaryStore.saveBeneficiary(selectedCampaignId.value);
    await statusStore.loadSocialStatuses(); // update status usage count
    toastSuccess('تم حفظ بيانات المستفيد بنجاح');
  } catch (e) {
    notifyError('خطأ أثناء الحفظ', e.message || e);
  }
};
const remove = async (id) => {
  const confirmed = await confirmDelete(
    'تأكيد حذف المستفيد',
    'هل أنت متأكد من رغبتك في حذف هذا المستفيد من هذا الموسم؟ لا يمكن التراجع بعد الحذف.'
  );
  if (confirmed) {
    try {
      await beneficiaryStore.deleteBeneficiary(id, selectedCampaignId.value);
      await statusStore.loadSocialStatuses(); // update status usage count
      toastSuccess('تم حذف المستفيد بنجاح');
    } catch (e) {
      notifyError('خطأ أثناء الحذف', e.message || e);
    }
  }
};

const toggleDelivery = async (item) => {
  try {
    await beneficiaryStore.toggleDelivery(item.id, selectedCampaignId.value);
    if (showStatsDetailsModal.value) {
      educationLevelStore.fetchStats(selectedCampaignId.value);
    }
  } catch (err) {
    notifyError('خطأ أثناء تحديث حالة الاستلام', err.message || err);
  }
};

const openCustomPrintModal = async (type = 'list') => {
  if (type === 'blank_form') {
    printDocType.value = 'forms';
    printScope.value = 'blank';
    printBlankCopiesCount.value = 1;
    printOrientation.value = 'portrait';
  } else {
    printDocType.value = type;
    printOrientation.value = type === 'list' ? 'landscape' : 'portrait';
    if (selectedBeneficiaryIds.value.length > 0) {
      printScope.value = 'selected';
    } else {
      printScope.value = 'all';
    }
    
    // Fetch unfiltered beneficiaries for the current campaign to ignore search filters
    try {
      const list = await safeInvoke('get_beneficiaries', {
        campaignId: selectedCampaignId.value,
        search: '',
        statusFilter: ''
      });
      fullBeneficiariesForPrint.value = list || [];
    } catch (e) {
      console.error(e);
      fullBeneficiariesForPrint.value = sortedBeneficiaries.value;
    }
    
    printRangeFrom.value = 1;
    const maxNo = fullBeneficiariesForPrint.value.length > 0
      ? Math.max(...fullBeneficiariesForPrint.value.map((b, idx) => Number(b.record_no) || (idx + 1)))
      : 100;
    printRangeTo.value = Math.max(maxNo, 1);
  }
  showPrintCustomModal.value = true;
};

const openCustomPrintWithSelection = (type = 'list') => {
  printDocType.value = type;
  printOrientation.value = type === 'list' ? 'landscape' : 'portrait';
  printScope.value = 'selected';
  showPrintCustomModal.value = true;
};

const printSingleBeneficiaryForm = async (beneficiary) => {
  if (beneficiary.guardian_id) {
    try {
      const children = await safeInvoke('get_guardian_children', { guardianId: beneficiary.guardian_id });
      beneficiary.children = children || [];
    } catch (e) {
      console.error('فشل جلب بيانات الأطفال:', e);
      beneficiary.children = [];
    }
  } else {
    beneficiary.children = [];
  }

  printDocType.value = 'forms';
  printOrientation.value = 'portrait';
  printScope.value = 'single';
  singlePrintBeneficiary.value = beneficiary;
  updatePageOrientationStyle('portrait');
  isPrinting.value = true;
  nextTick(() => {
    window.print();
    setTimeout(() => {
      isPrinting.value = false;
      singlePrintBeneficiary.value = null;
    }, 800);
  });
};

const printBlankForm = (copies = 1) => {
  printDocType.value = 'forms';
  printScope.value = 'blank';
  printBlankCopiesCount.value = Math.max(Number(copies) || 1, 1);
  printOrientation.value = 'portrait';
  updatePageOrientationStyle('portrait');
  isPrinting.value = true;
  nextTick(() => {
    window.print();
    setTimeout(() => {
      isPrinting.value = false;
    }, 800);
  });
};

const executeCustomPrint = async () => {
  if (printTargetBeneficiaries.value.length === 0) {
    notifyWarning('تنبيه', 'لا يوجد مستفيدين مطابقين لخيارات التحديد الحالية للطباعة');
    return;
  }
  showPrintCustomModal.value = false;

  const prevSortKey = sortKey.value;
  const prevSortOrder = sortOrder.value;
  if (printScope.value !== 'blank') {
    sortKey.value = 'record_no';
    sortOrder.value = 'asc';
  }

  updatePageOrientationStyle(printOrientation.value);

  // If printing forms and not blank, fetch children for all targeted beneficiaries first
  if (printDocType.value === 'forms' && printScope.value !== 'blank') {
    for (const b of printTargetBeneficiaries.value) {
      if (b.guardian_id && !b.children) {
        try {
          const children = await safeInvoke('get_guardian_children', { guardianId: b.guardian_id });
          b.children = children || [];
        } catch (e) {
          b.children = [];
        }
      }
    }
  }

  isPrinting.value = true;
  nextTick(() => {
    window.print();
    setTimeout(() => {
      isPrinting.value = false;
      if (printScope.value !== 'blank') {
        sortKey.value = prevSortKey;
        sortOrder.value = prevSortOrder;
      }
    }, 800);
  });
};

const printList = () => {
  openCustomPrintModal('list');
};

// Excel Import Modal
const showImportModal = ref(false);
const openImportModal = () => {
  showImportModal.value = true;
};
const closeImportModal = () => {
  showImportModal.value = false;
};
const onRecordsImported = async () => {
  if (selectedCampaignId.value) {
    await beneficiaryStore.fetchData(selectedCampaignId.value);
  }
};

// ==========================================
// Sequence Gap & Missing Numbers Inspector
// ==========================================
const showSequenceAuditModal = ref(false);
const isSequenceAuditLoading = ref(false);
const dbRecordNumbers = ref([]);

const openSequenceAuditModal = async () => {
  showSequenceAuditModal.value = true;
  isSequenceAuditLoading.value = true;
  try {
    const nums = await safeInvoke('get_all_record_numbers', { campaignId: selectedCampaignId.value });
    if (nums && nums.length > 0) {
      dbRecordNumbers.value = nums;
    } else {
      dbRecordNumbers.value = (beneficiaries.value || [])
        .map(b => Number(b.record_no))
        .filter(n => Number.isInteger(n) && n > 0);
    }
  } catch (err) {
    console.error("فشل جلب الأرقام الشاغرة", err);
    dbRecordNumbers.value = (beneficiaries.value || [])
      .map(b => Number(b.record_no))
      .filter(n => Number.isInteger(n) && n > 0);
  } finally {
    isSequenceAuditLoading.value = false;
  }
};

const seasonNumbersAnalysis = computed(() => {
  const list = beneficiaries.value || [];
  if (!list.length) {
    return { missingCount: 0, missing: [] };
  }
  const numbers = list
    .map(b => Number(b.record_no))
    .filter(n => Number.isInteger(n) && n > 0);
  if (!numbers.length) {
    return { missingCount: 0, missing: [] };
  }
  const uniqueNumbers = [...new Set(numbers)].sort((a, b) => a - b);
  const maxNum = uniqueNumbers[uniqueNumbers.length - 1];
  const presentSet = new Set(uniqueNumbers);
  const missing = [];
  for (let i = 1; i <= maxNum; i++) {
    if (!presentSet.has(i)) {
      missing.push(i);
    }
  }
  return {
    missingCount: missing.length,
    missing
  };
});

const sequenceAnalysis = computed(() => {
  const numbers = dbRecordNumbers.value;
  if (numbers.length === 0) {
    return {
      present: [],
      missing: [],
      missingCount: 0,
      min: 0,
      max: 0,
      totalExpected: 0,
    };
  }

  // Unique sorted numbers
  const uniqueNumbers = [...new Set(numbers)].sort((a, b) => a - b);
  const minNum = 1;
  const maxNum = uniqueNumbers[uniqueNumbers.length - 1];

  const presentSet = new Set(uniqueNumbers);
  const missing = [];

  for (let i = minNum; i <= maxNum; i++) {
    if (!presentSet.has(i)) {
      missing.push(i);
    }
  }

  return {
    present: uniqueNumbers,
    missing,
    missingCount: missing.length,
    min: minNum,
    max: maxNum,
    totalExpected: Math.max(0, maxNum - minNum + 1),
  };
});

const copyMissingNumbers = async () => {
  if (sequenceAnalysis.value.missing.length === 0) return;
  const text = sequenceAnalysis.value.missing.join(', ');
  try {
    if (navigator.clipboard) {
      await navigator.clipboard.writeText(text);
      toastSuccess(`تم نسخ ${sequenceAnalysis.value.missing.length} رقم غائب إلى الحافظة`);
    } else {
      toastSuccess('الأرقام: ' + text);
    }
  } catch (err) {
    console.warn('فشل النسخ:', err);
  }
};

const useMissingNumberToAdd = (num) => {
  showSequenceAuditModal.value = false;
  beneficiaryStore.openModal(null, statusStore.defaultStatusName, num);
};

const saveWorkbookWithDialog = async (wb, defaultFileName, dialogTitle) => {
  try {
    let saveDialog = null;
    if (window.__TAURI__) {
      const dialogMod = await import('@tauri-apps/api/dialog');
      saveDialog = dialogMod.save;
    }

    if (saveDialog) {
      const selectedPath = await saveDialog({
        title: dialogTitle || "اختر مكان حفظ الملف في النظام",
        defaultPath: defaultFileName,
        filters: [{
          name: "Excel Workbook",
          extensions: ["xlsx"]
        }]
      });

      if (!selectedPath) {
        return false;
      }

      const fileData = XLSX.write(wb, { bookType: 'xlsx', type: 'array' });
      const bytes = Array.from(new Uint8Array(fileData));

      await safeInvoke('write_binary_file', {
        path: selectedPath,
        contents: bytes
      });

      notifySuccess('تم حفظ الملف بنجاح', `تم حفظ الملف بنجاح في:\n${selectedPath}`);
      return true;
    } else {
      XLSX.writeFile(wb, defaultFileName);
      toastSuccess('تم تنزيل ملف الإكسيل بنجاح');
      return true;
    }
  } catch (err) {
    console.warn("تعذر فتح نافذة الحفظ بالنظام، جاري التحويل للتنزيل التلقائي:", err);
    XLSX.writeFile(wb, defaultFileName);
    toastSuccess('تم تنزيل ملف الإكسيل بنجاح');
    return true;
  }
};

const exportToExcel = async () => {
  if (sortedBeneficiaries.value.length === 0) {
    notifyInfo('تنبيه', 'لا توجد بيانات لتصديرها حالياً في هذا الموسم.');
    return;
  }

  const exportData = [
    ['الرقم', 'الإسم و اللقب', 'الحالة الاجتماعية', 'الهاتف', 'إبتدائي', 'متوسط', 'ثانوي', 'حالة الاستلام', 'تاريخ وتوقيت الاستلام']
  ];

  sortedBeneficiaries.value.forEach((b, index) => {
    exportData.push([
      b.record_no || (index + 1),
      b.guardian_name,
      b.social_status,
      b.phone || '',
      b.primary_count,
      b.middle_count,
      b.secondary_count,
      b.is_delivered ? 'تم الاستلام' : 'في الانتظار',
      b.delivered_at || ''
    ]);
  });

  const ws = XLSX.utils.aoa_to_sheet(exportData);
  ws['!views'] = [{ rightToLeft: true }];
  const wb = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(wb, ws, 'جدول البيانات');
  const safeYear = (activeCampaignLabel.value || 'الدخول_المدرسي').replace(/[\/\\]/g, '_');
  await saveWorkbookWithDialog(wb, `قائمة_المستفيدين_موسم_${safeYear}.xlsx`, "اختر مكان حفظ ملف إكسيل للمستفيدين");
};

// Lifecycle: Load initial data across all stores
onMounted(async () => {

  window.addEventListener('beforeprint', () => { 
    isPrinting.value = true;
    updatePageOrientationStyle(printOrientation.value);
  });
  window.addEventListener('afterprint', () => { isPrinting.value = false; });

  await Promise.all([
    statusStore.loadSocialStatuses(),
    orgStore.loadOrgSettings(),
    campaignStore.loadCampaigns(),
    educationLevelStore.fetchAll()
  ]);
  if (selectedCampaignId.value) {
    await beneficiaryStore.fetchData(selectedCampaignId.value);
  }
});
</script>
