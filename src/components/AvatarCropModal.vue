<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/70 backdrop-blur-sm transition-all" dir="rtl">
    <div class="bg-white rounded-2xl shadow-2xl border border-slate-200 w-full max-w-xl overflow-hidden flex flex-col animate-in fade-in zoom-in-95 duration-200">
      
      <!-- Modal Header -->
      <div class="px-6 py-4 bg-slate-50 border-b border-slate-200 flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <span class="flex items-center justify-center w-8 h-8 rounded-lg bg-emerald-100 text-emerald-700 text-lg">
            📷
          </span>
          <div>
            <h3 class="font-bold text-slate-800 text-base">قص وضبط الصورة الشخصية</h3>
            <p class="text-xs text-slate-500">حدد إطار الوجه بدقة (إطار مربع ثابت 1:1)</p>
          </div>
        </div>
        <button 
          type="button" 
          @click="$emit('close')" 
          class="text-slate-400 hover:text-slate-600 hover:bg-slate-200 rounded-lg w-8 h-8 flex items-center justify-center transition-colors text-lg"
          title="إغلاق"
        >
          ✕
        </button>
      </div>

      <!-- Cropper Viewport -->
      <div class="p-4 bg-slate-900 flex items-center justify-center min-h-[320px] max-h-[55vh] overflow-hidden select-none">
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
      <div class="px-6 py-3 bg-slate-50 border-t border-slate-200 flex flex-wrap items-center justify-between gap-2">
        <div class="flex items-center gap-1.5 flex-wrap">
          <button 
            type="button" 
            @click="rotateLeft" 
            class="px-3 py-1.5 bg-white border border-slate-300 hover:bg-slate-100 text-slate-700 text-xs font-semibold rounded-lg shadow-sm flex items-center gap-1 transition-all active:scale-95"
            title="تدوير 90 درجة إلى اليسار"
          >
            <span>↩️</span> تدوير يسار (90°)
          </button>
          <button 
            type="button" 
            @click="rotateRight" 
            class="px-3 py-1.5 bg-white border border-slate-300 hover:bg-slate-100 text-slate-700 text-xs font-semibold rounded-lg shadow-sm flex items-center gap-1 transition-all active:scale-95"
            title="تدوير 90 درجة إلى اليمين"
          >
            <span>↪️</span> تدوير يمين (90°)
          </button>
          <button 
            type="button" 
            @click="resetCrop" 
            class="px-3 py-1.5 bg-white border border-slate-300 hover:bg-slate-100 text-slate-700 text-xs font-semibold rounded-lg shadow-sm flex items-center gap-1 transition-all active:scale-95"
            title="إعادة ضبط الموضع والأبعاد"
          >
            <span>🔄</span> إعادة ضبط
          </button>
        </div>

        <div class="flex items-center gap-1">
          <button 
            type="button" 
            @click="zoomIn" 
            class="w-8 h-8 bg-white border border-slate-300 hover:bg-slate-100 text-slate-700 text-xs font-bold rounded-lg shadow-sm flex items-center justify-center transition-all active:scale-95"
            title="تكبير"
          >
            🔍+
          </button>
          <button 
            type="button" 
            @click="zoomOut" 
            class="w-8 h-8 bg-white border border-slate-300 hover:bg-slate-100 text-slate-700 text-xs font-bold rounded-lg shadow-sm flex items-center justify-center transition-all active:scale-95"
            title="تصغير"
          >
            🔍-
          </button>
        </div>
      </div>

      <!-- Action Buttons Footer -->
      <div class="px-6 py-4 bg-white border-t border-slate-200 flex items-center justify-end gap-3">
        <button 
          type="button" 
          @click="$emit('close')" 
          class="px-4 py-2 border border-slate-300 text-slate-600 hover:bg-slate-100 text-sm font-medium rounded-xl transition-all"
        >
          إلغاء
        </button>
        <button 
          type="button" 
          @click="confirmCrop" 
          class="px-5 py-2 bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-700 hover:to-teal-700 text-white text-sm font-bold rounded-xl shadow-md shadow-emerald-500/20 flex items-center gap-2 transition-all active:scale-95"
        >
          <span>✂️</span> اعتماد الصورة وحفظها (400x400)
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
    aspectRatio: 1, // Fixed 1:1 square
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

  // Export cropped canvas scaled to exactly 400x400
  const canvas = cropperInstance.getCroppedCanvas({
    width: 400,
    height: 400,
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
