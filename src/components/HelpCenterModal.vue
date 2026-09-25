<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-2 sm:p-4 bg-slate-900/65 backdrop-blur-xs overflow-y-auto print:static print:inset-auto print:z-auto print:p-0 print:m-0 print:bg-white print:overflow-visible print:w-full print:h-auto print:block"
    dir="rtl">
    <div
      class="bg-white rounded-3xl shadow-2xl border border-slate-200 w-[98vw] max-w-7xl overflow-hidden my-auto animate-fade-in flex flex-col h-[94vh] print:rounded-none print:shadow-none print:border-none print:w-full print:max-w-none print:h-auto print:overflow-visible print:m-0 print:block">

      <!-- ======================================================== -->
      <!-- Modal Header                                             -->
      <!-- ======================================================== -->
      <div
        class="help-header-banner px-6 py-4 bg-gradient-to-r from-slate-950 via-indigo-950 to-slate-900 text-white flex items-center justify-between shrink-0 border-b border-indigo-900/60 print:bg-none print:bg-white print:text-slate-900 print:border-b-2 print:border-slate-800 print:px-0 print:py-3 print:mb-4">
        <div class="flex items-center gap-3.5">
          <div
            class="w-12 h-12 rounded-2xl bg-indigo-500/20 flex items-center justify-center text-2xl shadow-inner border border-indigo-400/30 print:bg-slate-100 print:border-slate-300">
            📖
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h2 class="text-xl sm:text-2xl font-black text-white tracking-wide print:text-slate-900">
                مركز المساعدة ودليل الاستخدام الشامل
              </h2>
              <span class="bg-indigo-500/30 text-indigo-300 text-[11px] font-mono font-bold px-2 py-0.5 rounded-full border border-indigo-400/30 print:bg-slate-100 print:text-slate-800 print:border-slate-400">
                v1.6.0
              </span>
            </div>
            <p class="text-xs sm:text-sm text-indigo-200 font-medium mt-0.5 print:text-slate-600">
              مرجعك الكامل والمفصل لإدارة منظومة تسجيل وتوزيع المحافظ المدرسية بكفاءة واحترافية
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2.5 no-print">
          <!-- Print Manual Button -->
          <button
            type="button"
            @click="printHelpGuide"
            class="hidden sm:flex items-center gap-1.5 bg-white/10 hover:bg-white/20 text-white text-xs font-bold px-3 py-2 rounded-xl transition border border-white/15 cursor-pointer shadow-xs"
            title="طباعة دليل الاستخدام"
          >
            <span>🖨️</span>
            <span>طباعة الدليل</span>
          </button>

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
      <!-- Interactive Sub-Header: Search & Quick Navigation Filter -->
      <!-- ======================================================== -->
      <div class="px-6 py-3 bg-slate-50 border-b border-slate-200 flex flex-wrap items-center justify-between gap-3 shrink-0 no-print">
        <!-- Live Search Input -->
        <div class="relative flex-1 min-w-[260px] max-w-md">
          <input
            type="text"
            v-model="searchQuery"
            placeholder="ابحث عن ميزة، أمر، أو سؤال... (مثال: الماسح، استرجاع، إكسيل، نقاط)"
            class="w-full bg-white border border-slate-300 rounded-xl pr-9 pl-4 py-2 text-xs sm:text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 text-slate-800 shadow-2xs font-medium"
          />
          <span class="absolute right-3 top-2.5 text-slate-400 text-sm">🔍</span>
          <button
            v-if="searchQuery"
            @click="searchQuery = ''"
            class="absolute left-3 top-2 text-slate-400 hover:text-slate-600 font-bold text-xs"
          >
            ✕
          </button>
        </div>

        <!-- Quick Section Filter Tags -->
        <div class="flex items-center gap-1.5 overflow-x-auto py-1 text-xs font-bold no-scrollbar">
          <button
            v-for="cat in categories"
            :key="cat.id"
            @click="activeCategory = cat.id; scrollToSection(cat.id)"
            :class="activeCategory === cat.id ? 'bg-indigo-600 text-white shadow-2xs' : 'bg-white text-slate-600 hover:bg-slate-200/80 border border-slate-200'"
            class="px-3 py-1.5 rounded-lg whitespace-nowrap transition cursor-pointer flex items-center gap-1.5"
          >
            <span>{{ cat.icon }}</span>
            <span>{{ cat.title }}</span>
          </button>
        </div>
      </div>

      <!-- ======================================================== -->
      <!-- Main Content: Sidebar TOC + Content Area                 -->
      <!-- ======================================================== -->
      <div class="flex-1 overflow-hidden flex flex-col md:flex-row print:overflow-visible print:block">

        <!-- Right Side: Sticky Table of Contents (On Desktop) -->
        <div class="hidden md:flex flex-col w-72 lg:w-80 bg-slate-50/80 border-l border-slate-200 p-4 overflow-y-auto shrink-0 space-y-1.5 text-right select-none no-print">
          <div class="text-[11px] font-black uppercase tracking-wider text-slate-400 mb-2 px-2 flex items-center justify-between">
            <span>فهرس الأقسام والتفاصيل</span>
            <span>{{ filteredSections.length }} أقسام</span>
          </div>

          <button
            v-for="section in filteredSections"
            :key="section.id"
            @click="scrollToSection(section.id)"
            :class="activeCategory === section.id ? 'bg-indigo-100/90 text-indigo-950 font-black border-r-4 border-indigo-600 pl-2' : 'text-slate-700 hover:bg-slate-200/60 font-semibold'"
            class="w-full flex items-center justify-between p-2.5 rounded-xl text-xs transition text-right cursor-pointer group"
          >
            <div class="flex items-center gap-2.5 truncate">
              <span class="text-base group-hover:scale-110 transition-transform">{{ section.icon }}</span>
              <span class="truncate">{{ section.title }}</span>
            </div>
            <span class="text-[10px] text-slate-400 font-mono">{{ section.itemsCount }} نقطة</span>
          </button>

          <!-- Support & Info Box in TOC -->
          <div class="pt-4 mt-auto border-t border-slate-200 space-y-2">
            <div class="bg-indigo-50 border border-indigo-200/70 rounded-xl p-3 text-[11px] text-indigo-900 leading-relaxed">
              <div class="font-bold flex items-center gap-1.5 mb-1 text-indigo-950">
                <span>💡</span>
                <span>نصيحة ميدانية:</span>
              </div>
              قم دائماً بأخذ نسخة احتياطية مشفرة (.scaid) في نهاية كل يوم عمل وحفظها في قرص خارجي (Flash Drive) لحماية السجلات.
            </div>

            <div class="text-[10px] text-center text-slate-400 font-medium">
              منظومة تسجيل وتوزيع المحافظ v1.5.0 © 2026
            </div>
          </div>
        </div>

        <!-- Center/Left: Detailed Guides & Content (Scrollable) -->
        <div id="help-content-scroll" class="flex-1 p-5 sm:p-7 overflow-y-auto space-y-8 text-slate-800 leading-relaxed print:p-0 print:overflow-visible print:h-auto print:space-y-6 print:block">

          <!-- Welcome Banner -->
          <div class="help-welcome-banner bg-gradient-to-br from-indigo-900 via-indigo-950 to-slate-900 rounded-3xl p-6 text-white shadow-md relative overflow-hidden print:bg-none print:bg-slate-50 print:text-slate-900 print:border print:border-slate-300 print:shadow-none print:rounded-xl print:p-4 mb-4">
            <div class="relative z-10 space-y-2 max-w-3xl">
              <div class="inline-flex items-center gap-2 bg-indigo-500/20 px-3 py-1 rounded-full text-xs font-bold text-indigo-300 border border-indigo-400/30 print:bg-indigo-100 print:text-indigo-950 print:border-indigo-300">
                <span>🎒</span>
                <span>دليل الاستخدام الرسمي المعتمد</span>
              </div>
              <h1 class="text-2xl sm:text-3xl font-black text-white print:text-slate-900">
                أهلاً بك في منظومة تسيير وتوزيع المحافظ المدرسية
              </h1>
              <p class="text-xs sm:text-sm text-indigo-200 leading-relaxed print:text-slate-700">
                تم تصميم هذا الدليل ليمنحك شرحاً وافياً لكل زر، قائمة، ووظيفة داخل المنظومة. يمكنك تصفح الأقسام من الفهرس الجانبي، أو استخدام شريط البحث بالأعلى للوصول الفوري للإجابة المطلوبة.
              </p>
            </div>
            <span class="absolute -left-4 -bottom-6 text-9xl opacity-10 select-none pointer-events-none no-print">🎒</span>
          </div>

          <!-- ======================================================== -->
          <!-- SECTION 1: دورة العمل والبداية السريعة                  -->
          <!-- ======================================================== -->
          <section id="workflow" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-indigo-100 text-indigo-800 flex items-center justify-center text-xl font-black">
                🚀
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">1. دورة العمل الميداني والبداية السريعة (Workflow)</h2>
                <p class="text-xs sm:text-sm text-slate-500">الخطوات المتسلسلة لإدارة حملة الحقيبة المدرسية من البداية حتى التسليم</p>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2 hover:border-indigo-400 transition">
                <div class="w-7 h-7 rounded-lg bg-indigo-600 text-white font-mono font-black text-xs flex items-center justify-center">1</div>
                <h3 class="font-bold text-sm text-slate-900">تهيئة الموسم وهوية الجمعية</h3>
                <p class="text-xs text-slate-600">إنشاء الموسم الدراسي الجديد (مثال: 2026/2027) وضبط بيانات وشعار الجمعية من "إعدادات الجمعية".</p>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2 hover:border-indigo-400 transition">
                <div class="w-7 h-7 rounded-lg bg-indigo-600 text-white font-mono font-black text-xs flex items-center justify-center">2</div>
                <h3 class="font-bold text-sm text-slate-900">تسجيل أو استيراد المستفيدين</h3>
                <p class="text-xs text-slate-600">إدخال العائلات يدوياً مع إرفاق الصور والوثائق أو استيراد مجمّع عبر قالب Excel الذكي المعتمد.</p>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2 hover:border-indigo-400 transition">
                <div class="w-7 h-7 rounded-lg bg-indigo-600 text-white font-mono font-black text-xs flex items-center justify-center">3</div>
                <h3 class="font-bold text-sm text-slate-900">فرز الأولويات وطباعة المحاضر</h3>
                <p class="text-xs text-slate-600">ترتيب الأسر الأكثر استحقاقاً حسب نقاط الحالة، وطباعة استمارات الدخول المدرسي ومحاضر التسليم.</p>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2 hover:border-indigo-400 transition">
                <div class="w-7 h-7 rounded-lg bg-indigo-600 text-white font-mono font-black text-xs flex items-center justify-center">4</div>
                <h3 class="font-bold text-sm text-slate-900">التوزيع والتأمين الاحتياطي</h3>
                <p class="text-xs text-slate-600">تأكيد تسليم الحقائب بنقرة زر وتوثيق التوقيت، وأخذ نسخة احتياطية مشفرة (.scaid) في ختام الحملة.</p>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 2: إدارة المستفيدين والأطفال                     -->
          <!-- ======================================================== -->
          <section id="beneficiaries" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-blue-100 text-blue-800 flex items-center justify-center text-xl font-black">
                👨‍👩‍👧‍👦
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">2. إدارة المستفيدين والأطفال المتمدرسين (Beneficiaries)</h2>
                <p class="text-xs sm:text-sm text-slate-500">إضافة وتعديل بيانات الأسر، الأطفال، فحص التكرار، وتأكيد التسليم</p>
              </div>
            </div>

            <div class="space-y-3">
              <!-- Feature Card 1 -->
              <div class="bg-slate-50/80 border border-slate-200/90 rounded-2xl p-4 space-y-2">
                <h3 class="font-black text-sm text-slate-900 flex items-center gap-2">
                  <span class="text-blue-600">➕</span>
                  <span>إضافة مستفيد جديد (نافذة الإدخال):</span>
                </h3>
                <ul class="text-xs text-slate-700 space-y-1.5 pr-5 list-disc">
                  <li><strong>رقم المستفيد (السجل التسلسلي):</strong> يُقترح تلقائياً الرقم الموالي لأعلى رقم موجود. إذا كانت هناك فجوة سابقة، يُنبهك النظام لإمكانية شغل الرقم المفقود.</li>
                  <li><strong>بيانات ولي الأمر:</strong> الاسم، اللقب، رقم الهاتف للتواصل، والحالة الاجتماعية (يتحدد على إثرها نقاط الأولوية تلقائياً).</li>
                  <li><strong>قائمة الأطفال المتمدرسين:</strong> اضغط <code>+ إضافة طفل</code> لتسجيل اسم الطفل، تاريخ الميلاد، الطور التعليمي (ابتدائي، متوسط، ثانوي)، والسنة الدراسية. يقوم النظام تلقائياً بحساب عدد الحقائب لكل طور.</li>
                </ul>
              </div>

              <!-- Feature Card 2 -->
              <div class="bg-slate-50/80 border border-slate-200/90 rounded-2xl p-4 space-y-2">
                <h3 class="font-black text-sm text-slate-900 flex items-center gap-2">
                  <span class="text-amber-600">🛡️</span>
                  <span>نظام الحماية من التكرار (Duplicate Prevention):</span>
                </h3>
                <p class="text-xs text-slate-700">
                  بمجرد كتابة اسم ولي الأمر، يقوم النظام فورياً بفحص قاعدة البيانات. إذا كان المستفيد مسجلاً بالفعل في هذا الموسم، يمنع النظام الحفظ مع إشعار تحذيري فوري. وإذا كان مسجلاً في موسم سابق، يُظهر لك خيار استرجاع بيانات أطفاله لتوفير وقت الكتابة!
                </p>
              </div>

              <!-- Feature Card 3 -->
              <div class="bg-slate-50/80 border border-slate-200/90 rounded-2xl p-4 space-y-2">
                <h3 class="font-black text-sm text-slate-900 flex items-center gap-2">
                  <span class="text-emerald-600">📦</span>
                  <span>تسليم الحقائب وتوثيق الاستلام الميداني:</span>
                </h3>
                <p class="text-xs text-slate-700">
                  في جدول المستفيدين، انقر على زر <span class="bg-amber-100 text-amber-800 px-2 py-0.5 rounded font-bold">في الانتظار</span> لتحويله بنقرة واحدة إلى <span class="bg-emerald-100 text-emerald-800 px-2 py-0.5 rounded font-bold">تم الاستلام ✓</span> مع تسجيل تاريخ وتوقيت الاستلام الدقيق، مع تحديث فوري لعدادات لوحة التحكم.
                </p>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 3: الصور الشخصية والماسح الضوئي                 -->
          <!-- ======================================================== -->
          <section id="avatars" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-teal-100 text-teal-800 flex items-center justify-center text-xl font-black">
                📸
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">3. نظام الصور الشخصية وقص 1:1 والماسح الضوئي (Avatars & Scanner)</h2>
                <p class="text-xs sm:text-sm text-slate-500">إرفاق بطاقات الهوية، تشغيل الماسح، اللصق من الحافظة، والضغط بصيغة WebP</p>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="bg-teal-50/60 border border-teal-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-teal-950 flex items-center gap-2">
                  <span>🖨️</span>
                  <span>تشغيل الماسح الضوئي (Windows Scanner Launcher)</span>
                </h3>
                <p class="text-xs text-slate-700 leading-relaxed">
                  يحتوي نموذج المستفيد على زر مخصص للماسح الضوئي. بالضغط عليه، يُطلق التطبيق أداة المسح الضوئي الخاصة بنظام ويندوز (Windows Fax and Scan) لسحب بطاقة التعريف الوطنية أو الشهادة المدرسية مباشرة دون الحاجة لفتح برامج خارجية.
                </p>
              </div>

              <div class="bg-indigo-50/60 border border-indigo-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-indigo-950 flex items-center gap-2">
                  <span>📋</span>
                  <span>اللصق الفوري من الحافظة (Ctrl + V) والسحب والإفلات</span>
                </h3>
                <p class="text-xs text-slate-700 leading-relaxed">
                  يمكنك ببساطة نسخ أي صورة من جهازك أو التقاط لقطة شاشة لبطاقة المستفيد، ثم الضغط على <code>Ctrl + V</code> داخل نافذة المستفيد، أو سحب وإفلات الصورة مباشرة في مربع الصورة لتفتح أداة القص فورياً.
                </p>
              </div>

              <div class="bg-purple-50/60 border border-purple-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-purple-950 flex items-center gap-2">
                  <span>✂️</span>
                  <span>قص تفاعلي دقيق 1:1 (Cropper.js)</span>
                </h3>
                <p class="text-xs text-slate-700 leading-relaxed">
                  تتيح أداة القص تدوير الصورة، تكبيرها، تصغيرها، ومحاذاة الوجه بدقة متناهية داخل إطار بنسبة 1:1 المعتمدة لبطاقات الهوية والاستمارات الرسمية.
                </p>
              </div>

              <div class="bg-emerald-50/60 border border-emerald-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-emerald-950 flex items-center gap-2">
                  <span>⚡</span>
                  <span>ضغط فائق بصيغة WebP (400×400 بكسل)</span>
                </h3>
                <p class="text-xs text-slate-700 leading-relaxed">
                  تُحفظ الصور محلياً داخل مجلد <code>avatars/</code> بصيغة WebP المتطورة، مما يقلص حجم الصورة من 5 ميغابايت إلى أقل من 30 كيلوبايت فقط مع وضوح عالي، ما يضمن سرعة البرامج حتى مع آلاف الصور.
                </p>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 4: الحالات الاجتماعية ونقاط الأولوية            -->
          <!-- ======================================================== -->
          <section id="social-statuses" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-purple-100 text-purple-800 flex items-center justify-center text-xl font-black">
                ⚖️
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">4. الحالات الاجتماعية ومنظومة نقاط الأولوية والاستحقاق</h2>
                <p class="text-xs sm:text-sm text-slate-500">إدارة الفئات، معايير التنقيط الذاتي، وتوجيه المحافظ للأسر الأكثر عوزاً</p>
              </div>
            </div>

            <div class="bg-white border border-slate-200 rounded-2xl p-5 space-y-4 shadow-2xs">
              <p class="text-xs text-slate-700 leading-relaxed">
                تتيح لك شاشة <strong>"الحالات الاجتماعية"</strong> تخصيص فئات الأسر وفق لوائح جمعيتكم الخيرية مع إمكانية تعيين <strong>نقاط أولوية</strong> لكل فئة.
              </p>

              <div class="grid grid-cols-1 sm:grid-cols-4 gap-3 text-center">
                <div class="bg-rose-50 border border-rose-200 rounded-xl p-3">
                  <span class="text-xs font-bold text-rose-800 block">أولوية قصوى</span>
                  <span class="text-xs text-rose-600 mt-1 block">الأيتام ومعدومي الدخل (نقاط مرتفعة 80-100)</span>
                </div>
                <div class="bg-orange-50 border border-orange-200 rounded-xl p-3">
                  <span class="text-xs font-bold text-orange-800 block">أولوية ضرورية</span>
                  <span class="text-xs text-orange-600 mt-1 block">الأرامل والمطلقات المعوزات (نقاط 60-79)</span>
                </div>
                <div class="bg-amber-50 border border-amber-200 rounded-xl p-3">
                  <span class="text-xs font-bold text-amber-800 block">أولوية متوسطة</span>
                  <span class="text-xs text-amber-600 mt-1 block">أصحاب الدخل الضعيف والعمال اليوميون (40-59)</span>
                </div>
                <div class="bg-emerald-50 border border-emerald-200 rounded-xl p-3">
                  <span class="text-xs font-bold text-emerald-800 block">أولوية عادية</span>
                  <span class="text-xs text-emerald-600 mt-1 block">حالات خاصة ومتفرقة (أقل من 40)</span>
                </div>
              </div>

              <div class="bg-purple-50 border border-purple-200 rounded-xl p-3 text-xs text-purple-900 flex items-start gap-2">
                <span class="text-base">🛡️</span>
                <span><strong>حماية سلامة البيانات:</strong> يمنع النظام حذف أي حالة اجتماعية طالما أنها مرتبطة بسجلات أسر مسجلة، وذلك لمنع حدوث أي فراغ في السجلات أو تقارير الإحصاء.</span>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 5: لوحة التحكم والإحصائيات التفاعلية              -->
          <!-- ======================================================== -->
          <section id="dashboard" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-indigo-100 text-indigo-800 flex items-center justify-center text-xl font-black">
                📊
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">5. لوحة التحكم والتحليل البياني المتقدم (Dashboard)</h2>
                <p class="text-xs sm:text-sm text-slate-500">رصد نسب الإنجاز، تفصيل الأطوار والسنوات الدراسية، ومؤشرات التوزيع</p>
              </div>
            </div>

            <div class="space-y-3 text-xs text-slate-700">
              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2">
                <h3 class="font-bold text-sm text-slate-900">العدادات الأربعة الرئيسية:</h3>
                <ul class="list-disc pr-5 space-y-1">
                  <li><strong>إجمالي العائلات:</strong> عدد الأسر المسجلة في الموسم الحالي مع نسبة التغطية.</li>
                  <li><strong>إجمالي الحقائب المطلوبة:</strong> مجموع المحافظ المطلوبة عبر كافة الأطوار التعليمية.</li>
                  <li><strong>الحقائب المسلّمة:</strong> عدد المحافظ الموزعة فعلياً ونسبة الإنجاز المئوية.</li>
                  <li><strong>الحقائب في الانتظار:</strong> الحصص المتبقية التي لم يتم استلامها بعد.</li>
                </ul>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2">
                <h3 class="font-bold text-sm text-slate-900">تفصيل الأطوار والسنوات التعليمية (الميزة المحدثة في v1.5.0):</h3>
                <p>
                  من خلال الضغط على زر <strong>"📊 لوحة الإحصائيات الشاملة"</strong>، تظهر لك مصفوفة متكاملة تفصل كل طور (ابتدائي، متوسط، ثانوي) وكل سنة دراسية على حدة (مثال: أولى ابتدائي، ثانية متوسط... إلخ)، مع شريط تقدم يوضح كم تم تسليمه وكم بالانتظار لكل مستوى تعليمي.
                </p>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 6: استيراد وتصدير ملفات Excel                   -->
          <!-- ======================================================== -->
          <section id="excel" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-emerald-100 text-emerald-800 flex items-center justify-center text-xl font-black">
                📥
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">6. استيراد وتصدير ملفات Excel وتوليد القوالب الذكية</h2>
                <p class="text-xs sm:text-sm text-slate-500">القوائم المنسدلة التلقائية، الاستيراد المجمّع، وتصدير التقارير</p>
              </div>
            </div>

            <div class="space-y-3">
              <div class="bg-emerald-50/70 border border-emerald-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-emerald-950 flex items-center gap-2">
                  <span>✨</span>
                  <span>توليد قوالب Excel ذات قوائم منسدلة أصلية (Data Validation):</span>
                </h3>
                <p class="text-xs text-slate-700 leading-relaxed">
                  عند النقر على "استيراد من إكسيل" ثم "تحميل قالب إكسيل المعتمد"، يقوم التطبيق آلياً بتوليد ملف <code>.xlsx</code> يحتوي على قائمة منسدلة في عمود <strong>الحالة الاجتماعية</strong> مطابقة تماماً للحالات المعرفة في نظامكم، مما يمنع الأخطاء الإملائية التي قد تقع أثناء إدخال البيانات في مكاتب الفروع!
                </p>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2 text-xs text-slate-700">
                <h3 class="font-bold text-sm text-slate-900">خطوات الاستيراد المجمّع:</h3>
                <ol class="list-decimal pr-5 space-y-1">
                  <li>قم بتحميل القالب وتعبئة بيانات الأسر (الاسم، الهاتف، الحالة، عدد الأطفال في كل طور).</li>
                  <li>اختر الملف واضغط على "فحص ومعاينة البيانات".</li>
                  <li>يقوم التطبيق بالتحقق من صحة الأعمدة والأرقام، وتنبيهك في حال وجود أرقام هواتف غير صحيحة أو تكرارات.</li>
                  <li>اضغط "تأكيد الاستيراد" لإضافة مئات السجلات دفعة واحدة خلال ثانيتين فقط!</li>
                </ol>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 7: خيارات وطباعة المحاضر والاستمارات            -->
          <!-- ======================================================== -->
          <section id="printing" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-slate-100 text-slate-800 flex items-center justify-center text-xl font-black">
                🖨️
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">7. نظام الطباعة والمحاضر والاستمارات الرسمية</h2>
                <p class="text-xs sm:text-sm text-slate-500">طباعة المحاضر الجماعية، الاستمارات الفردية مع الصور، وتخصيص النطاقات</p>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs text-slate-700">
              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2">
                <h3 class="font-bold text-sm text-indigo-950 flex items-center gap-2">
                  <span>📋</span>
                  <span>محضر التوزيع الجماعي (Landscape / Portrait)</span>
                </h3>
                <p>
                  جدول رسمي معتمد يحتوي على أرقام السجلات، أسماء أولياء الأمور، الحالات الاجتماعية، توزيع الحقائب، خانة التوقيع والبصمة، وترويسة الجمعية الرسمية. يُفضل طباعته بالعرض (Landscape) للوضوح التام.
                </p>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2">
                <h3 class="font-bold text-sm text-teal-950 flex items-center gap-2">
                  <span>📄</span>
                  <span>إستمارة الدخول المدرسي الفردية والجماعية</span>
                </h3>
                <p>
                  استمارة فردية لكل عائلة تحتوي على صورة المستفيد المرفقة، بيانات ولي الأمر، تفصيل كل طفل بمدرسته وطوره، وإقرار الاستلام والتوقيع مع ختم الجمعية.
                </p>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2">
                <h3 class="font-bold text-sm text-amber-950 flex items-center gap-2">
                  <span>🎯</span>
                  <span>خيارات النطاق الذكية</span>
                </h3>
                <p>
                  يمكنك الطباعة: لجميع المستفيدين، أو للمحددين عبر مربعات الاختيار، أو بنطاق أرقام محدد (مثال: من رقم 1 إلى 50)، أو للمستلمين فقط أو المعلقين.
                </p>
              </div>

              <div class="bg-white border border-slate-200 rounded-2xl p-4 shadow-2xs space-y-2">
                <h3 class="font-bold text-sm text-purple-950 flex items-center gap-2">
                  <span>📝</span>
                  <span>طباعة استمارات فارغة (Blank Forms)</span>
                </h3>
                <p>
                  يمكنك طباعة عدد مخصص من الاستمارات الفارغة بترويسة الجمعية لأخذها في الزيارات الميدانية وتسجيل الأسر يدوياً عند عدم توفر الحاسوب.
                </p>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 8: إدارة المواسم والترحيل السنوي                -->
          <!-- ======================================================== -->
          <section id="campaigns" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-amber-100 text-amber-800 flex items-center justify-center text-xl font-black">
                🔄
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">8. إدارة المواسم المتعددة والترحيل السنوي للأسر</h2>
                <p class="text-xs sm:text-sm text-slate-500">إنشاء حملات جديدة، عزل البيانات التاريخية، وترقية الأطفال بين السنوات</p>
              </div>
            </div>

            <div class="bg-white border border-slate-200 rounded-2xl p-4 space-y-3 shadow-2xs text-xs text-slate-700">
              <p>
                تسمح المنظومة بالاحتفاظ بأرشيف تاريخي كامل لكافة المواسم السابقة. عند حلول موسم دراسي جديد:
              </p>
              <ul class="list-disc pr-5 space-y-1.5">
                <li>اضغط <strong>"+ موسم جديد"</strong> وأدخل تسمية الموسم (مثال: <code>2026/2027</code>).</li>
                <li><strong>الترحيل التلقائي:</strong> يمكنك ترحيل جميع العائلات السابقة بنقرة زر إلى الموسم الجديد، مع ترقية الأطفال المسجلين تلقائياً إلى السنة الدراسية الموالية، وتحويل حالة استلامهم إلى "في الانتظار" استعداداً للموسم الجديد دون فقدان السجلات الأصلية للموسم السابق!</li>
              </ul>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 9: فحص الفجوات وتسلسل الأرقام                  -->
          <!-- ======================================================== -->
          <section id="sequence" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-violet-100 text-violet-800 flex items-center justify-center text-xl font-black">
                🔢
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">9. فحص تسلسل الأرقام وكشف الفجوات (Gap Inspector)</h2>
                <p class="text-xs sm:text-sm text-slate-500">التدقيق في الأرقام المحذوفة أو المتخطاة وشغلها تلقائياً</p>
              </div>
            </div>

            <div class="bg-violet-50/70 border border-violet-200 rounded-2xl p-4 space-y-2 text-xs text-slate-700">
              <p>
                في العمل الميداني، قد يُلغى تسجيل عائلة أو يُحذف سجل، مما يترك فجوة في تسلسل الأرقام (مثال: وجود رقم 12 ثم رقم 14 واختفاء رقم 13).
              </p>
              <p>
                تقوم أداة <strong>"فحص تسلسل الأرقام"</strong> بفحص قاعدة البيانات فورياً، وإظهار قائمة الأرقام الشاغرة بدقة، مع إمكانية ملء تلك الأرقام تلقائياً عند تسجيل المستفيد التالي بنقرة زر.
              </p>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 10: النسخ الاحتياطي والأمان وضبط المصنع         -->
          <!-- ======================================================== -->
          <section id="security" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-emerald-100 text-emerald-800 flex items-center justify-center text-xl font-black">
                🔐
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">10. النسخ الاحتياطي المشفر والاسترجاع الآمن وضبط المصنع</h2>
                <p class="text-xs sm:text-sm text-slate-500">حماية البيانات المشفرة (.scaid)، الأمان السيادي، والنسخ الوقائي</p>
              </div>
            </div>

            <div class="space-y-3 text-xs text-slate-700">
              <div class="bg-indigo-50/70 border border-indigo-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-indigo-950 flex items-center gap-2">
                  <span>📦</span>
                  <span>تصدير نسخة احتياطية مشفرة (.scaid):</span>
                </h3>
                <p>
                  يجمع هذا الأمر قاعدة بيانات SQLite وجميع صور المستفيدين في ملف أرشيف واحد مشفر بتشفير <strong>AES-256-GCM</strong> مع بصمة تحقق <strong>SHA-256</strong> وبادئة حماية <code>SCA1</code>، بحيث لا يمكن فتح الملف أو التلاعب به من خارج التطبيق.
                </p>
              </div>

              <div class="bg-emerald-50/70 border border-emerald-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-emerald-950 flex items-center gap-2">
                  <span>🛡️</span>
                  <span>الاسترجاع الآمن مع النسخة الوقائية الإلزامية:</span>
                </h3>
                <p>
                  عند اختيار استرجاع ملف احتياطي، يقوم التطبيق <strong>تلقائياً وبشكل إجباري</strong> بأخذ نسخة احتياطية فورية من وضعك الحالي قبل إجراء الاسترجاع (تحت اسم <code>safety_before_restore_*.scaid</code>)، بحيث تستطيع العودة لوضعك في أي لحظة في حال اخترت ملفاً قديماً بالخطأ!
                </p>
              </div>

              <div class="bg-rose-50/70 border border-rose-200 rounded-2xl p-4 space-y-2">
                <h3 class="font-bold text-sm text-rose-950 flex items-center gap-2">
                  <span>⚠️</span>
                  <span>إعادة ضبط المصنع الآمنة (Factory Reset):</span>
                </h3>
                <p>
                  إذا أردت تصفير البرنامج لبدء عمل جمعية جديدة، تتطلب العملية كتابة عبارة تأكيد صريحة (<code>تصفير شامل</code>)، ويقوم النظام تلقائياً بتوليد نسخة احتياطية للأمان قبل مسح السجلات وإعادة تهيئة الحالات الاجتماعية الافتراضية.
                </p>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 11: تخصيص وهوية الجمعية                        -->
          <!-- ======================================================== -->
          <section id="settings" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-amber-100 text-amber-800 flex items-center justify-center text-xl font-black">
                🏢
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">11. تخصيص وهوية الجمعية وإعدادات الطباعة الرسمية</h2>
                <p class="text-xs sm:text-sm text-slate-500">شعار الجمعية، بيانات الاعتماد الرسمي، الترويسة والتذييل</p>
              </div>
            </div>

            <div class="bg-white border border-slate-200 rounded-2xl p-4 space-y-2 text-xs text-slate-700 shadow-2xs">
              <p>من القائمة الجانبية، اختر <strong>"إعدادات الجمعية"</strong> لتحديد:</p>
              <ul class="list-disc pr-5 space-y-1">
                <li><strong>اسم الجمعية والفرع البلدي/الولائي:</strong> ليظهر في الترويسة الرسمية لكل الاستمارات.</li>
                <li><strong>رقم الاعتماد وتاريخ التأسيس:</strong> لإعطاء الصفة القانونية والرسمية للمحاضر المطبوعة.</li>
                <li><strong>شعار الجمعية (Logo):</strong> يمكنك رفع شعار الجمعية ليظهر في الزاوية العلوية للاستمارات والمحاضر.</li>
                <li><strong>نص التذييل الرسمي وختم اللجنة:</strong> النص المعتمد أسفل الصفحات المطبوعة لتوقيع رئيس اللجنة.</li>
              </ul>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 12: الأسئلة الشائعة وحلول المشاكل               -->
          <!-- ======================================================== -->
          <section id="faq" class="space-y-4 scroll-mt-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-rose-100 text-rose-800 flex items-center justify-center text-xl font-black">
                ❓
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">12. الأسئلة الشائعة وحلول المشاكل (FAQ)</h2>
                <p class="text-xs sm:text-sm text-slate-500">إجابات فورية لأبرز الأسئلة والمواقف الميدانية</p>
              </div>
            </div>

            <div class="space-y-2.5">
              <div
                v-for="(faq, idx) in faqs"
                :key="idx"
                class="bg-white border border-slate-200 rounded-2xl overflow-hidden transition shadow-2xs"
              >
                <button
                  type="button"
                  @click="faq.open = !faq.open"
                  class="w-full flex items-center justify-between p-4 text-right font-bold text-xs sm:text-sm text-slate-800 hover:bg-slate-50 transition cursor-pointer"
                >
                  <span class="flex items-center gap-2">
                    <span class="text-indigo-600">❓</span>
                    <span>{{ faq.q }}</span>
                  </span>
                  <span class="text-slate-400 text-sm font-mono font-bold">{{ faq.open ? '▲' : '▼' }}</span>
                </button>
                <div v-show="faq.open" class="px-5 pb-4 pt-1 text-xs text-slate-600 leading-relaxed border-t border-slate-100 bg-slate-50/50">
                  {{ faq.a }}
                </div>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 13: اختصارات لوحة المفاتيح والإنتاجية          -->
          <!-- ======================================================== -->
          <section id="shortcuts" class="space-y-4 scroll-mt-6 pb-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-slate-200 text-slate-800 flex items-center justify-center text-xl font-black">
                ⌨️
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">13. اختصارات لوحة المفاتيح الميدانية (Shortcuts)</h2>
                <p class="text-xs sm:text-sm text-slate-500">أوامر سريعة لتسريع إدخال البيانات أثناء ضغط العمل</p>
              </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
              <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between shadow-2xs">
                <span class="text-xs text-slate-700 font-semibold">لصق صورة المستفيد من الحافظة</span>
                <kbd class="px-2 py-1 bg-slate-100 border border-slate-300 rounded font-mono text-[11px] font-bold text-slate-800">Ctrl + V</kbd>
              </div>

              <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between shadow-2xs">
                <span class="text-xs text-slate-700 font-semibold">إغلاق النوافذ المنبثقة والقوائم</span>
                <kbd class="px-2 py-1 bg-slate-100 border border-slate-300 rounded font-mono text-[11px] font-bold text-slate-800">Esc</kbd>
              </div>

              <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between shadow-2xs">
                <span class="text-xs text-slate-700 font-semibold">فتح دليل المساعدة الشامل</span>
                <kbd class="px-2 py-1 bg-slate-100 border border-slate-300 rounded font-mono text-[11px] font-bold text-slate-800">F1</kbd>
              </div>

              <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between shadow-2xs">
                <span class="text-xs text-slate-700 font-semibold">البحث السريع في القائمة</span>
                <kbd class="px-2 py-1 bg-slate-100 border border-slate-300 rounded font-mono text-[11px] font-bold text-slate-800">Ctrl + F</kbd>
              </div>

              <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between shadow-2xs">
                <span class="text-xs text-slate-700 font-semibold">طباعة المستند الحالي</span>
                <kbd class="px-2 py-1 bg-slate-100 border border-slate-300 rounded font-mono text-[11px] font-bold text-slate-800">Ctrl + P</kbd>
              </div>

              <div class="bg-white border border-slate-200 rounded-xl p-3 flex items-center justify-between shadow-2xs">
                <span class="text-xs text-slate-700 font-semibold">التنقل السريع بين الحقول</span>
                <kbd class="px-2 py-1 bg-slate-100 border border-slate-300 rounded font-mono text-[11px] font-bold text-slate-800">Tab</kbd>
              </div>
            </div>
          </section>

          <!-- ======================================================== -->
          <!-- SECTION 14: معلومات المطور والدعم الفني                 -->
          <!-- ======================================================== -->
          <section id="developer" class="space-y-4 scroll-mt-6 pb-6">
            <div class="flex items-center gap-3 border-b border-slate-200 pb-3">
              <span class="w-10 h-10 rounded-xl bg-indigo-100 text-indigo-800 flex items-center justify-center text-xl font-black">
                👨‍💻
              </span>
              <div>
                <h2 class="text-lg sm:text-xl font-black text-slate-900">14. معلومات المطور والدعم الفني المباشر (Developer Contact)</h2>
                <p class="text-xs sm:text-sm text-slate-500">قنوات التواصل مع مطور المنظومة للاستفسارات، التخصيص، والدعم التقني</p>
              </div>
            </div>

            <div class="help-developer-banner bg-gradient-to-br from-slate-900 via-indigo-950 to-slate-950 text-white rounded-3xl p-6 shadow-md space-y-4 print:bg-none print:bg-slate-50 print:text-slate-900 print:border print:border-slate-300 print:shadow-none">
              <div class="flex items-center gap-4 flex-wrap">
                <div class="w-14 h-14 rounded-2xl bg-indigo-600/40 border border-indigo-400/30 flex items-center justify-center text-3xl print:bg-indigo-100 print:border-indigo-300">
                  👨‍💻
                </div>
                <div>
                  <h3 class="text-lg sm:text-xl font-black text-white print:text-slate-900">نصرالدين حداد (Nasreddine Haddad)</h3>
                  <p class="text-xs sm:text-sm text-indigo-200 print:text-slate-600">مطور برمجيات شامل (Développeur Full-Stack)</p>
                </div>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2">
                <div class="bg-white/10 rounded-2xl p-3.5 border border-white/10 space-y-1 print:bg-white print:border-slate-300 print:text-slate-900">
                  <span class="text-xs text-indigo-300 font-bold block print:text-indigo-950">📞 الهاتف / واتساب:</span>
                  <a href="tel:+213774119955" class="font-mono font-bold text-sm text-white hover:text-indigo-300 block print:text-slate-900" dir="ltr">+213 774 11 99 55</a>
                </div>

                <div class="bg-white/10 rounded-2xl p-3.5 border border-white/10 space-y-1 print:bg-white print:border-slate-300 print:text-slate-900">
                  <span class="text-xs text-indigo-300 font-bold block print:text-indigo-950">✉️ البريد الإلكتروني:</span>
                  <a href="mailto:contact@nacerdev.com" class="font-mono font-bold text-xs sm:text-sm text-white hover:text-indigo-300 truncate block print:text-slate-900" dir="ltr">contact@nacerdev.com</a>
                </div>

                <div class="bg-white/10 rounded-2xl p-3.5 border border-white/10 space-y-1 print:bg-white print:border-slate-300 print:text-slate-900">
                  <span class="text-xs text-indigo-300 font-bold block print:text-indigo-950">🌐 المواقع الإلكترونية:</span>
                  <div class="space-y-0.5 text-xs font-mono">
                    <a href="https://nacerdev.com" target="_blank" class="text-indigo-200 hover:text-white block underline print:text-indigo-700">nacerdev.com</a>
                    <a href="https://haddad-developpeurfull-stack.dz/" target="_blank" class="text-teal-200 hover:text-white block underline print:text-teal-700">haddad-developpeurfull-stack.dz</a>
                  </div>
                </div>
              </div>
            </div>
          </section>

        </div>
      </div>

      <!-- ======================================================== -->
      <!-- Modal Footer                                             -->
      <!-- ======================================================== -->
      <div class="px-6 py-3.5 bg-slate-50 border-t border-slate-200 flex items-center justify-between shrink-0 no-print">
        <div class="text-xs text-slate-500 font-medium hidden sm:flex items-center gap-2">
          <span>💡</span>
          <span>يمكنك فتح هذا الدليل في أي وقت بالضغط على زر <kbd class="px-1.5 py-0.5 bg-slate-200 rounded font-mono font-bold text-[10px]">F1</kbd></span>
        </div>

        <div class="flex items-center gap-2.5 mr-auto">
          <button
            type="button"
            @click="printHelpGuide"
            class="px-4 py-2 text-xs font-bold text-slate-700 bg-white border border-slate-300 hover:bg-slate-100 rounded-xl transition cursor-pointer shadow-2xs"
          >
            🖨️ طباعة الدليل
          </button>

          <button
            type="button"
            @click="$emit('close')"
            class="px-6 py-2 text-xs font-black text-white bg-indigo-600 hover:bg-indigo-700 rounded-xl transition cursor-pointer shadow-xs active:scale-95"
          >
            إغلاق الدليل
          </button>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const emit = defineEmits(['close']);

// Search & Active Filter State
const searchQuery = ref('');
const activeCategory = ref('all');

// Categories definition
const categories = [
  { id: 'all', title: 'كافة الأقسام', icon: '📚' },
  { id: 'workflow', title: 'دورة العمل', icon: '🚀' },
  { id: 'beneficiaries', title: 'المستفيدين والأطفال', icon: '👨‍👩‍👧‍👦' },
  { id: 'avatars', title: 'الصور والماسح', icon: '📸' },
  { id: 'social-statuses', title: 'الحالات والنقاط', icon: '⚖️' },
  { id: 'dashboard', title: 'لوحة التحكم', icon: '📊' },
  { id: 'excel', title: 'Excel والقوالب', icon: '📥' },
  { id: 'printing', title: 'الطباعة والمحاضر', icon: '🖨️' },
  { id: 'campaigns', title: 'المواسم والترحيل', icon: '🔄' },
  { id: 'security', title: 'النسخ الاحتياطي والأمان', icon: '🔐' },
  { id: 'faq', title: 'الأسئلة الشائعة', icon: '❓' },
  { id: 'shortcuts', title: 'الاختصارات', icon: '⌨️' },
  { id: 'developer', title: 'معلومات المطور', icon: '👨‍💻' }
];

// Sections metadata for TOC
const sections = [
  { id: 'workflow', title: '1. دورة العمل الميداني والبداية السريعة', icon: '🚀', itemsCount: 4 },
  { id: 'beneficiaries', title: '2. إدارة المستفيدين والأطفال', icon: '👨‍👩‍👧‍👦', itemsCount: 3 },
  { id: 'avatars', title: '3. الصور الشخصية وقص 1:1 والماسح', icon: '📸', itemsCount: 4 },
  { id: 'social-statuses', title: '4. الحالات الاجتماعية ونقاط الأولوية', icon: '⚖️', itemsCount: 4 },
  { id: 'dashboard', title: '5. لوحة التحكم والتحليل البياني', icon: '📊', itemsCount: 2 },
  { id: 'excel', title: '6. استيراد وتصدير Excel والقوالب', icon: '📥', itemsCount: 4 },
  { id: 'printing', title: '7. نظام الطباعة والمحاضر الرسمية', icon: '🖨️', itemsCount: 4 },
  { id: 'campaigns', title: '8. إدارة المواسم والترحيل السنوي', icon: '🔄', itemsCount: 2 },
  { id: 'sequence', title: '9. فحص تسلسل الأرقام والفجوات', icon: '🔢', itemsCount: 2 },
  { id: 'security', title: '10. النسخ المشفر والاسترجاع وضبط المصنع', icon: '🔐', itemsCount: 3 },
  { id: 'settings', title: '11. تخصيص وهوية الجمعية', icon: '🏢', itemsCount: 4 },
  { id: 'faq', title: '12. الأسئلة الشائعة وحلول المشاكل', icon: '❓', itemsCount: 6 },
  { id: 'shortcuts', title: '13. اختصارات لوحة المفاتيح', icon: '⌨️', itemsCount: 6 },
  { id: 'developer', title: '14. معلومات المطور والدعم الفني', icon: '👨‍💻', itemsCount: 3 }
];

// Filter sections based on search query
const filteredSections = computed(() => {
  if (!searchQuery.value.trim()) return sections;
  const q = searchQuery.value.trim().toLowerCase();
  return sections.filter(s => s.title.toLowerCase().includes(q) || s.id.toLowerCase().includes(q));
});

// Scroll helper
const scrollToSection = (id) => {
  if (id === 'all') {
    const el = document.getElementById('help-content-scroll');
    if (el) el.scrollTo({ top: 0, behavior: 'smooth' });
    return;
  }
  const el = document.getElementById(id);
  if (el) {
    el.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
};

// FAQ Accordion items
const faqs = ref([
  {
    q: 'هل يحتاج التطبيق إلى اتصال بشبكة الإنترنت ليعمل؟',
    a: 'لا، التطبيق يعمل بنسبة 100% بدون إنترنت (Offline First). يتم حفظ قاعدة البيانات والصور الشخصية محلياً على جهازك مع حماية كاملة لسرية بيانات العائلات.',
    open: true
  },
  {
    q: 'كيف يمكنني نقل بيانات التطبيق إلى جهاز كمبيوتر آخر؟',
    a: 'الأمر في غاية السهولة: من شاشة "النسخ الاحتياطي والأمان"، اضغط على "تصدير نسخة احتياطية مشفرة (.scaid)" واحفظ الملف على فلاش ديسك. ثم افتح التطبيق في الجهاز الآخر واختر "استرجاع من نسخة احتياطية مشفرة". ستُنقل كافة السجلات والصور في ثوانٍ!',
    open: false
  },
  {
    q: 'ماذا لو قمت باسترجاع نسخة قديمة بالخطأ؟ هل تضيع بياناتي الحالية؟',
    a: 'أبداً! المنظومة مجهزة بنظام أمان استباقي يقوم تلقائياً بإنشاء نسخة احتياطية وقائية قبل أي استرجاع وحفظها في مجلد backups. يمكنك استعادتها في أي وقت.',
    open: false
  },
  {
    q: 'كيف أتعامل مع المستفيد الذي يمتلك أكثر من 5 أطفال في أطوار مختلفة؟',
    a: 'لا توجد أي قيود على عدد الأطفال. يمكنك إضافة أي عدد تريده من الأطفال عبر زر "+ إضافة طفل" وتحديد طور وسنة كل طفل على حدة، وسيقوم النظام بحساب الإجماليات بدقة متناهية.',
    open: false
  },
  {
    q: 'عند طباعة محضر التوزيع، لماذا تظهر بعض الأعمدة مقصوصة أحياناً؟',
    a: 'يُنصح دائماً باختيار التوجيه العرضي (Landscape) لمحضر التوزيع من نافذة الطباعة لأن جدول التوزيع يحتوي على أعمدة متعددة تتطلب مساحة عرضية ورقية مريحة.',
    open: false
  },
  {
    q: 'أين تُحفظ الصور الشخصية للمستفيدين؟',
    a: 'تُحفظ مضغوطة بصيغة WebP داخل مجلد avatars في مجلد التطبيق، وتُدمج تلقائياً داخل أرشيف النسخ الاحتياطي المشفر (.scaid).',
    open: false
  }
]);

// Print Help Guide Action
const printHelpGuide = () => {
  const previousState = (faqs.value || []).map(f => f.open);
  (faqs.value || []).forEach(f => { f.open = true; });

  setTimeout(() => {
    window.print();
    setTimeout(() => {
      (faqs.value || []).forEach((f, i) => { f.open = previousState[i]; });
    }, 500);
  }, 100);
};
</script>

<style scoped>
/* Scrollbar tweaks for seamless reading */
#help-content-scroll::-webkit-scrollbar {
  width: 6px;
}
#help-content-scroll::-webkit-scrollbar-track {
  background: #f1f5f9;
}
#help-content-scroll::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 9999px;
}
#help-content-scroll::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

@media print {
  @page {
    size: A4 portrait;
    margin: 12mm 15mm;
  }

  /* Force light backgrounds, remove dark gradients, and set dark text */
  .help-header-banner,
  .help-welcome-banner,
  .help-developer-banner,
  [class*="bg-gradient-"],
  [class*="from-slate-"],
  [class*="from-indigo-"] {
    background-image: none !important;
    background-color: #f8fafc !important;
    color: #0f172a !important;
    border: 1px solid #cbd5e1 !important;
    box-shadow: none !important;
  }

  .help-header-banner {
    border: none !important;
    border-bottom: 2px solid #0f172a !important;
    padding-bottom: 12px !important;
    margin-bottom: 18px !important;
    background: transparent !important;
    background-image: none !important;
  }

  .help-header-banner *,
  .help-welcome-banner *,
  .help-developer-banner * {
    color: #0f172a !important;
    text-shadow: none !important;
  }

  .help-header-banner .text-indigo-200,
  .help-welcome-banner .text-indigo-200,
  .help-developer-banner .text-indigo-200 {
    color: #475569 !important;
  }

  .help-header-banner .text-white,
  .help-welcome-banner .text-white,
  .help-developer-banner .text-white {
    color: #0f172a !important;
  }

  .help-developer-banner .bg-white\/10 {
    background-color: #ffffff !important;
    border: 1px solid #cbd5e1 !important;
  }

  .help-developer-banner a {
    color: #1d4ed8 !important;
    text-decoration: underline !important;
  }

  section {
    page-break-inside: auto;
    break-inside: auto;
    margin-bottom: 24px !important;
  }

  section > div:first-child {
    page-break-after: avoid !important;
    break-after: avoid !important;
  }

  .shadow-2xs, .shadow-xs, .shadow-md, .shadow-2xl, .shadow-lg {
    box-shadow: none !important;
  }

  #help-content-scroll {
    overflow: visible !important;
    height: auto !important;
    max-height: none !important;
  }
}
</style>
