// Centralized Tauri invoke wrapper with development fallback
let tauriInvoke = null;

export const safeInvoke = async (cmd, args) => {
  if (window.__TAURI__) {
    if (!tauriInvoke) {
      try {
        const mod = await import('@tauri-apps/api/tauri');
        tauriInvoke = mod.invoke;
      } catch (e) {
        if (window.__TAURI__.invoke) {
          tauriInvoke = window.__TAURI__.invoke;
        }
      }
    }
    if (tauriInvoke) {
      return await tauriInvoke(cmd, args);
    }
  }

  console.warn(`[Mock Tauri] Command: ${cmd}`, args);
  if (cmd === 'get_stats') return { 
    total_families: 0, total_bags: 0, primary_total: 0, middle_total: 0, secondary_total: 0,
    delivered_families: 0, pending_families: 0, delivered_bags: 0, pending_bags: 0, progress_percentage: 0 
  };
  if (cmd === 'toggle_delivery_status') return [true, '2026-09-18 11:45'];
  if (cmd === 'bulk_set_delivery_status') return (args?.recordIds || []).length;
  if (cmd === 'get_org_settings') return {
    id: 1,
    org_name: 'الجمعية الخيرية لرعاية الأيتام والمحتاجين',
    branch_name: 'المكتب الولائي',
    wilaya: 'قسنطينة',
    commune: '',
    phone: '',
    footer_text: 'وثيقة إدارية داخلية مخصصة لضبط عملية التوزيع.'
  };
  if (cmd === 'get_social_statuses') return [
    { id: 1, name: 'بدون دخل', count: 0 },
    { id: 2, name: 'ضعيف الدخل', count: 0 },
    { id: 3, name: 'متقاعد', count: 0 },
    { id: 4, name: 'مرض مزمن', count: 0 },
    { id: 5, name: 'إعاقة', count: 0 }
  ];
  if (cmd === 'batch_import_records') return (args?.records || []).length;
  return null;
};
