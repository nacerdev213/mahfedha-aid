<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center p-2 sm:p-4 bg-slate-950/75 backdrop-blur-sm transition-all" dir="rtl">
    <div class="bg-white rounded-3xl shadow-2xl border border-slate-200 w-[96vw] max-w-2xl overflow-hidden flex flex-col animate-fade-in max-h-[92vh]">
      
      <!-- Modal Header -->
      <div class="px-6 py-4 bg-gradient-to-r from-slate-950 via-emerald-950 to-slate-900 text-white flex items-center justify-between shrink-0 border-b border-emerald-900/50">
        <div class="flex items-center gap-3.5">
          <div class="w-12 h-12 rounded-2xl bg-white/10 flex items-center justify-center text-2xl shadow-inner border border-white/15">
            📷
          </div>
          <div>
            <h2 class="text-xl sm:text-2xl font-black text-white tracking-wide">قص وضبط الصورة الشخصية</h2>
            <p class="text-xs sm:text-sm text-emerald-200 font-medium mt-0.5">حدد إطار الصورة الشخصية بدقة (إطار صورة شمسية عمودي 3:4)</p>
          </div>
        </div>
        <button 
          type="button" 
          @click="$emit('close')" 
          class="text-white/70 hover:text-white hover:bg-white/15 rounded-xl p-2 transition cursor-pointer"
          title="إغلاق"
        >
          <span class="text-2xl leading-none font-bold">✕</span>
        </button>
      </div>

      <!-- Cropper Viewport -->
      <div class="p-4 bg-slate-950 flex items-center justify-center min-h-[340px] max-h-[55vh] overflow-hidden select-none flex-1">
        <div class="w-full h-full flex items-center justify-center">
          <img 
            ref="imageEl" 
            :src="imageUrl" 
            alt="صورة للقص" 
            class="max-w-full max-h-[50vh] block"
            @load="initCropper"
          />
        </div>
      </div>

      <!-- Controls Toolbar -->
      <div class="px-6 py-3.5 bg-slate-50 border-t border-slate-200 flex flex-wrap items-center justify-between gap-3 shrink-0">
        <div class="flex items-center gap-2 flex-wrap">
          <button 
            type="button" 
            @click="rotateLeft" 
            class="px-3.5 py-2 bg-white border border-slate-300 hover:bg-slate-100 text-slate-800 text-xs sm:text-sm font-bold rounded-xl shadow-2xs flex items-center gap-1.5 transition cursor-pointer active:scale-95"
            title="تدوير 90 درجة إلى اليسار"
          >
            <span>↩️</span><span>تدوير يسار (90°)</span>
          </button>
          <button 
            type="button" 
            @click="rotateRight" 
            class="px-3.5 py-2 bg-white border border-slate-300 hover:bg-slate-100 text-slate-800 text-xs sm:text-sm font-bold rounded-xl shadow-2xs flex items-center gap-1.5 transition cursor-pointer active:scale-95"
            title="تدوير 90 درجة إلى اليمين"
          >
            <span>↪️</span><span>تدوير يمين (90°)</span>
          </button>
          <button 
            type="button" 
            @click="resetCrop" 
            class="px-3.5 py-2 bg-white border border-slate-300 hover:bg-slate-100 text-slate-800 text-xs sm:text-sm font-bold rounded-xl shadow-2xs flex items-center gap-1.5 transition cursor-pointer active:scale-95"
            title="إعادة ضبط الموضع والأبعاد"
          >
            <span>🔄</span><span>إعادة ضبط</span>
          </button>
        </div>

        <div class="flex items-center gap-1.5">
          <button 
            type="button" 
            @click="zoomIn" 
            class="w-9 h-9 bg-white border border-slate-300 hover:bg-slate-100 text-slate-800 text-sm font-black rounded-xl shadow-2xs flex items-center justify-center transition cursor-pointer active:scale-95"
            title="تكبير"
          >
            🔍+
          </button>
          <button 
            type="button" 
            @click="zoomOut" 
            class="w-9 h-9 bg-white border border-slate-300 hover:bg-slate-100 text-slate-800 text-sm font-black rounded-xl shadow-2xs flex items-center justify-center transition cursor-pointer active:scale-95"
            title="تصغير"
          >
            🔍-
          </button>
        </div>
      </div>

      <!-- Action Buttons Footer -->
      <div class="px-6 py-4 bg-slate-50 border-t border-slate-200 flex items-center justify-end gap-3 shrink-0">
        <button 
          type="button" 
          @click="$emit('close')" 
          class="px-6 py-2.5 border border-slate-300 text-slate-700 hover:bg-slate-200 text-sm font-bold rounded-xl transition cursor-pointer"
        >
          إلغاء
        </button>
        <button 
          type="button" 
          @click="confirmCrop" 
          class="px-7 py-2.5 bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-700 hover:to-teal-700 text-white text-sm font-black rounded-xl shadow-md shadow-emerald-500/20 flex items-center gap-2 transition cursor-pointer active:scale-95"
        >
          <span>✂️</span><span>اعتماد الصورة وحفظها (3:4)</span>
        </button>
      </div>

    </div>
  </div>
</template>


<script setup>
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue';
import Cropper from 'cropperjs';
import 'cropperjs/dist/cropper.css';

const props = defineProps({
  imageUrl: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(['cropped', 'close']);

const imageEl = ref(null);
let cropperInstance = null;

const initCropper = () => {
  if (!imageEl.value) return;
  if (cropperInstance) {
    cropperInstance.destroy();
  }

  cropperInstance = new Cropper(imageEl.value, {
    aspectRatio: 3 / 4, // Portrait vertical 3:4 ratio for identity/personal photos
    viewMode: 1, // Restrict crop box within canvas
    dragMode: 'move',
    autoCropArea: 0.85,
    responsive: true,
    restore: false,
    guides: true,
    center: true,
    highlight: false,
    cropBoxMovable: true,
    cropBoxResizable: true,
    toggleDragModeOnDblclick: false,
  });
};

onMounted(() => {
  nextTick(() => {
    if (imageEl.value && imageEl.value.complete) {
      initCropper();
    }
  });
});

onBeforeUnmount(() => {
  if (cropperInstance) {
    cropperInstance.destroy();
    cropperInstance = null;
  }
});

const rotateLeft = () => {
  cropperInstance?.rotate(-90);
};

const rotateRight = () => {
  cropperInstance?.rotate(90);
};

const resetCrop = () => {
  cropperInstance?.reset();
};

const zoomIn = () => {
  cropperInstance?.zoom(0.1);
};

const zoomOut = () => {
  cropperInstance?.zoom(-0.1);
};

const confirmCrop = () => {
  if (!cropperInstance) return;

  // Export cropped canvas scaled to 3:4 portrait (360x480)
  const canvas = cropperInstance.getCroppedCanvas({
    width: 360,
    height: 480,
    imageSmoothingEnabled: true,
    imageSmoothingQuality: 'high',
  });

  if (!canvas) return;

  // Compress to WebP at 85% quality (~25KB to 35KB)
  const dataUrl = canvas.toDataURL('image/webp', 0.85);
  emit('cropped', dataUrl);
  emit('close');
};
</script>

<style scoped>
/* Custom overrides for smooth cropper frame */
:deep(.cropper-view-box) {
  border-radius: 8px;
  outline: 2px solid #10b981;
  outline-color: rgba(16, 185, 129, 0.9);
}
:deep(.cropper-line) {
  background-color: #10b981;
}
:deep(.cropper-point) {
  background-color: #10b981;
  width: 8px;
  height: 8px;
}
</style>
