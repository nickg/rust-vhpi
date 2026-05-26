#![allow(clippy::missing_safety_doc)]

#[cfg(windows)]
mod windows_shim {
    use std::ffi::{c_char, c_int, c_long, c_void};
    use std::sync::OnceLock;

    type VhpiHandleT = *mut u32;

    #[repr(C)]
    pub struct VhpiPhysS {
        pub high: i32,
        pub low: u32,
    }

    #[repr(C)]
    pub struct VhpiTimeS {
        pub high: i32,
        pub low: u32,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleA(module_name: *const c_char) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, proc_name: *const c_char) -> *mut c_void;
    }

    static HOST_MODULE: OnceLock<usize> = OnceLock::new();

    fn host_module() -> *mut c_void {
        let module = *HOST_MODULE.get_or_init(|| unsafe {
            // Use the host executable where simulators export VHPI symbols.
            let module = GetModuleHandleA(std::ptr::null());
            if module.is_null() {
                eprintln!("vhpi-winlib: GetModuleHandleA(NULL) failed");
                std::process::abort();
            }
            module as usize
        });
        module as *mut c_void
    }

    fn resolve_symbol(name: &[u8]) -> *mut c_void {
        let proc = unsafe { GetProcAddress(host_module(), name.as_ptr().cast()) };
        if proc.is_null() {
            let symbol = String::from_utf8_lossy(&name[..name.len().saturating_sub(1)]);
            eprintln!("vhpi-winlib: failed to resolve symbol {symbol}");
            std::process::abort();
        }
        proc
    }

    type VhpiAssertFn = unsafe extern "C" fn(c_int, *mut c_char) -> c_int;
    type VhpiRegisterCbFn = unsafe extern "C" fn(*mut c_void, c_int) -> VhpiHandleT;
    type VhpiRemoveCbFn = unsafe extern "C" fn(VhpiHandleT) -> c_int;
    type VhpiHandleByNameFn = unsafe extern "C" fn(*const c_char, VhpiHandleT) -> VhpiHandleT;
    type VhpiHandleByIndexFn = unsafe extern "C" fn(c_int, VhpiHandleT, c_int) -> VhpiHandleT;
    type VhpiHandleFn = unsafe extern "C" fn(c_int, VhpiHandleT) -> VhpiHandleT;
    type VhpiIteratorFn = unsafe extern "C" fn(c_int, VhpiHandleT) -> VhpiHandleT;
    type VhpiScanFn = unsafe extern "C" fn(VhpiHandleT) -> VhpiHandleT;
    type VhpiGetFn = unsafe extern "C" fn(c_int, VhpiHandleT) -> c_int;
    type VhpiGetStrFn = unsafe extern "C" fn(c_int, VhpiHandleT) -> *const u8;
    type VhpiGetRealFn = unsafe extern "C" fn(c_int, VhpiHandleT) -> f64;
    type VhpiGetPhysFn = unsafe extern "C" fn(c_int, VhpiHandleT) -> VhpiPhysS;
    type VhpiGetValueFn = unsafe extern "C" fn(VhpiHandleT, *mut c_void) -> c_int;
    type VhpiPutValueFn = unsafe extern "C" fn(VhpiHandleT, *mut c_void, c_int) -> c_int;
    type VhpiGetTimeFn = unsafe extern "C" fn(*mut VhpiTimeS, *mut c_long);
    type VhpiGetNextTimeFn = unsafe extern "C" fn(*mut VhpiTimeS) -> c_int;
    type VhpiControlFn = unsafe extern "C" fn(c_int) -> c_int;
    type VhpiPrintfFn = unsafe extern "C" fn(*const c_char, *const c_char) -> c_int;
    type VhpiCompareHandlesFn = unsafe extern "C" fn(VhpiHandleT, VhpiHandleT) -> c_int;
    type VhpiCheckErrorFn = unsafe extern "C" fn(*mut c_void) -> c_int;
    type VhpiReleaseHandleFn = unsafe extern "C" fn(VhpiHandleT) -> c_int;

    static VHPI_ASSERT_FN: OnceLock<VhpiAssertFn> = OnceLock::new();
    static VHPI_REGISTER_CB_FN: OnceLock<VhpiRegisterCbFn> = OnceLock::new();
    static VHPI_REMOVE_CB_FN: OnceLock<VhpiRemoveCbFn> = OnceLock::new();
    static VHPI_HANDLE_BY_NAME_FN: OnceLock<VhpiHandleByNameFn> = OnceLock::new();
    static VHPI_HANDLE_BY_INDEX_FN: OnceLock<VhpiHandleByIndexFn> = OnceLock::new();
    static VHPI_HANDLE_FN: OnceLock<VhpiHandleFn> = OnceLock::new();
    static VHPI_ITERATOR_FN: OnceLock<VhpiIteratorFn> = OnceLock::new();
    static VHPI_SCAN_FN: OnceLock<VhpiScanFn> = OnceLock::new();
    static VHPI_GET_FN: OnceLock<VhpiGetFn> = OnceLock::new();
    static VHPI_GET_STR_FN: OnceLock<VhpiGetStrFn> = OnceLock::new();
    static VHPI_GET_REAL_FN: OnceLock<VhpiGetRealFn> = OnceLock::new();
    static VHPI_GET_PHYS_FN: OnceLock<VhpiGetPhysFn> = OnceLock::new();
    static VHPI_GET_VALUE_FN: OnceLock<VhpiGetValueFn> = OnceLock::new();
    static VHPI_PUT_VALUE_FN: OnceLock<VhpiPutValueFn> = OnceLock::new();
    static VHPI_GET_TIME_FN: OnceLock<VhpiGetTimeFn> = OnceLock::new();
    static VHPI_GET_NEXT_TIME_FN: OnceLock<VhpiGetNextTimeFn> = OnceLock::new();
    static VHPI_CONTROL_FN: OnceLock<VhpiControlFn> = OnceLock::new();
    static VHPI_PRINTF_FN: OnceLock<VhpiPrintfFn> = OnceLock::new();
    static VHPI_COMPARE_HANDLES_FN: OnceLock<VhpiCompareHandlesFn> = OnceLock::new();
    static VHPI_CHECK_ERROR_FN: OnceLock<VhpiCheckErrorFn> = OnceLock::new();
    static VHPI_RELEASE_HANDLE_FN: OnceLock<VhpiReleaseHandleFn> = OnceLock::new();

    macro_rules! resolve_fn {
        ($cell:ident, $name:literal, $ty:ty) => {
            *$cell.get_or_init(|| unsafe {
                std::mem::transmute::<*mut c_void, $ty>(resolve_symbol(
                    concat!($name, "\0").as_bytes(),
                ))
            })
        };
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_assert(severity: c_int, formatmsg: *mut c_char) -> c_int {
        resolve_fn!(VHPI_ASSERT_FN, "vhpi_assert", VhpiAssertFn)(severity, formatmsg)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_register_cb(cb_data_p: *mut c_void, flags: c_int) -> VhpiHandleT {
        resolve_fn!(VHPI_REGISTER_CB_FN, "vhpi_register_cb", VhpiRegisterCbFn)(cb_data_p, flags)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_remove_cb(cb_obj: VhpiHandleT) -> c_int {
        resolve_fn!(VHPI_REMOVE_CB_FN, "vhpi_remove_cb", VhpiRemoveCbFn)(cb_obj)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_handle_by_name(
        name: *const c_char,
        scope: VhpiHandleT,
    ) -> VhpiHandleT {
        resolve_fn!(
            VHPI_HANDLE_BY_NAME_FN,
            "vhpi_handle_by_name",
            VhpiHandleByNameFn
        )(name, scope)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_handle_by_index(
        it_rel: c_int,
        parent: VhpiHandleT,
        indx: c_int,
    ) -> VhpiHandleT {
        resolve_fn!(
            VHPI_HANDLE_BY_INDEX_FN,
            "vhpi_handle_by_index",
            VhpiHandleByIndexFn
        )(it_rel, parent, indx)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_handle(
        kind: c_int,
        reference_handle: VhpiHandleT,
    ) -> VhpiHandleT {
        resolve_fn!(VHPI_HANDLE_FN, "vhpi_handle", VhpiHandleFn)(kind, reference_handle)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_iterator(
        kind: c_int,
        reference_handle: VhpiHandleT,
    ) -> VhpiHandleT {
        resolve_fn!(VHPI_ITERATOR_FN, "vhpi_iterator", VhpiIteratorFn)(kind, reference_handle)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_scan(iterator: VhpiHandleT) -> VhpiHandleT {
        resolve_fn!(VHPI_SCAN_FN, "vhpi_scan", VhpiScanFn)(iterator)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_get(property: c_int, object: VhpiHandleT) -> c_int {
        resolve_fn!(VHPI_GET_FN, "vhpi_get", VhpiGetFn)(property, object)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_get_str(property: c_int, object: VhpiHandleT) -> *const u8 {
        resolve_fn!(VHPI_GET_STR_FN, "vhpi_get_str", VhpiGetStrFn)(property, object)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_get_real(property: c_int, object: VhpiHandleT) -> f64 {
        resolve_fn!(VHPI_GET_REAL_FN, "vhpi_get_real", VhpiGetRealFn)(property, object)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_get_phys(property: c_int, object: VhpiHandleT) -> VhpiPhysS {
        resolve_fn!(VHPI_GET_PHYS_FN, "vhpi_get_phys", VhpiGetPhysFn)(property, object)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_get_value(expr: VhpiHandleT, value_p: *mut c_void) -> c_int {
        resolve_fn!(VHPI_GET_VALUE_FN, "vhpi_get_value", VhpiGetValueFn)(expr, value_p)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_put_value(
        object: VhpiHandleT,
        value_p: *mut c_void,
        mode: c_int,
    ) -> c_int {
        resolve_fn!(VHPI_PUT_VALUE_FN, "vhpi_put_value", VhpiPutValueFn)(object, value_p, mode)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_get_time(time_p: *mut VhpiTimeS, cycles: *mut c_long) {
        resolve_fn!(VHPI_GET_TIME_FN, "vhpi_get_time", VhpiGetTimeFn)(time_p, cycles)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_get_next_time(time_p: *mut VhpiTimeS) -> c_int {
        resolve_fn!(
            VHPI_GET_NEXT_TIME_FN,
            "vhpi_get_next_time",
            VhpiGetNextTimeFn
        )(time_p)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_control(command: c_int) -> c_int {
        resolve_fn!(VHPI_CONTROL_FN, "vhpi_control", VhpiControlFn)(command)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_printf(format: *const c_char, arg: *const c_char) -> c_int {
        resolve_fn!(VHPI_PRINTF_FN, "vhpi_printf", VhpiPrintfFn)(format, arg)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_compare_handles(
        handle1: VhpiHandleT,
        handle2: VhpiHandleT,
    ) -> c_int {
        resolve_fn!(
            VHPI_COMPARE_HANDLES_FN,
            "vhpi_compare_handles",
            VhpiCompareHandlesFn
        )(handle1, handle2)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_check_error(error_info_p: *mut c_void) -> c_int {
        resolve_fn!(VHPI_CHECK_ERROR_FN, "vhpi_check_error", VhpiCheckErrorFn)(error_info_p)
    }

    #[no_mangle]
    pub unsafe extern "C" fn vhpi_release_handle(object: VhpiHandleT) -> c_int {
        resolve_fn!(
            VHPI_RELEASE_HANDLE_FN,
            "vhpi_release_handle",
            VhpiReleaseHandleFn
        )(object)
    }

    pub fn __link_vhpi_winlib() {}
}

#[cfg(windows)]
pub use windows_shim::__link_vhpi_winlib;

#[cfg(not(windows))]
pub fn __link_vhpi_winlib() {}
