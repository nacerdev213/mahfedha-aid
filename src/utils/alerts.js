import Swal from 'sweetalert2';
import 'sweetalert2/dist/sweetalert2.min.css';

/**
 * Modern RTL SweetAlert2 helper module
 */

// Toast notification preset
const Toast = Swal.mixin({
  toast: true,
  position: 'top-end',
  showConfirmButton: false,
  timer: 3000,
  timerProgressBar: true,
  didOpen: (toast) => {
    toast.onmouseenter = Swal.stopTimer;
    toast.onmouseleave = Swal.resumeTimer;
  },
  customClass: {
    popup: 'rounded-xl shadow-lg border border-slate-100 text-right'
  }
});

/**
 * Toast success alert
 */
export const toastSuccess = (title) => {
  return Toast.fire({
    icon: 'success',
    title
  });
};

/**
 * Toast error alert
 */
export const toastError = (title) => {
  return Toast.fire({
    icon: 'error',
    title
  });
};

/**
 * Success modal dialog or notification
 */
export const notifySuccess = (title, text = '') => {
  return Swal.fire({
    icon: 'success',
    title,
    text,
    timer: 2500,
    showConfirmButton: false,
    timerProgressBar: true,
    customClass: {
      popup: 'rounded-2xl shadow-xl border border-slate-100'
    }
  });
};

/**
 * Error modal dialog
 */
export const notifyError = (title, text = '') => {
  return Swal.fire({
    icon: 'error',
    title,
    text,
    confirmButtonText: 'حسناً',
    confirmButtonColor: '#4f46e5',
    customClass: {
      popup: 'rounded-2xl shadow-xl border border-slate-100',
      confirmButton: 'rounded-xl px-5 py-2.5 font-bold'
    }
  });
};

/**
 * Warning modal dialog
 */
export const notifyWarning = (title, text = '') => {
  return Swal.fire({
    icon: 'warning',
    title,
    text,
    confirmButtonText: 'حسناً',
    confirmButtonColor: '#4f46e5',
    customClass: {
      popup: 'rounded-2xl shadow-xl border border-slate-100',
      confirmButton: 'rounded-xl px-5 py-2.5 font-bold'
    }
  });
};

/**
 * Info modal dialog
 */
export const notifyInfo = (title, text = '') => {
  return Swal.fire({
    icon: 'info',
    title,
    text,
    confirmButtonText: 'حسناً',
    confirmButtonColor: '#4f46e5',
    customClass: {
      popup: 'rounded-2xl shadow-xl border border-slate-100',
      confirmButton: 'rounded-xl px-5 py-2.5 font-bold'
    }
  });
};

/**
 * Standard confirmation dialog for deletion
 * @param {string} title
 * @param {string} text
 * @returns {Promise<boolean>}
 */
export const confirmDelete = async (
  title = 'تأكيد الحذف',
  text = 'هل أنت متأكد من رغبتك في حذف هذا السجل؟ لن يمكنك التراجع بعد إتمام العملية.'
) => {
  const result = await Swal.fire({
    title,
    text,
    icon: 'warning',
    showCancelButton: true,
    confirmButtonColor: '#e11d48', // rose-600
    cancelButtonColor: '#64748b', // slate-500
    confirmButtonText: 'نعم، قم بالحذف',
    cancelButtonText: 'إلغاء',
    reverseButtons: true,
    focusCancel: true,
    customClass: {
      popup: 'rounded-2xl shadow-2xl border border-slate-100',
      confirmButton: 'rounded-xl px-5 py-2.5 font-bold text-white shadow-md',
      cancelButton: 'rounded-xl px-4 py-2.5 font-semibold text-white'
    }
  });

  return result.isConfirmed;
};

export default Swal;
