//! A shim library to enable dynamic name resolution for VHPI plugins on macOS and Windows.
#![allow(clippy::missing_safety_doc)]

#[cfg(any(windows, target_os = "macos"))]
macro_rules! define_vhpi_forwarders {
    () => {
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

        type VhpiAssertFn =
            unsafe extern "C" fn(std::ffi::c_int, *mut std::ffi::c_char) -> std::ffi::c_int;
        type VhpiRegisterCbFn =
            unsafe extern "C" fn(*mut std::ffi::c_void, std::ffi::c_int) -> VhpiHandleT;
        type VhpiRemoveCbFn = unsafe extern "C" fn(VhpiHandleT) -> std::ffi::c_int;
        type VhpiHandleByNameFn =
            unsafe extern "C" fn(*const std::ffi::c_char, VhpiHandleT) -> VhpiHandleT;
        type VhpiHandleByIndexFn =
            unsafe extern "C" fn(std::ffi::c_int, VhpiHandleT, std::ffi::c_int) -> VhpiHandleT;
        type VhpiHandleFn = unsafe extern "C" fn(std::ffi::c_int, VhpiHandleT) -> VhpiHandleT;
        type VhpiIteratorFn = unsafe extern "C" fn(std::ffi::c_int, VhpiHandleT) -> VhpiHandleT;
        type VhpiScanFn = unsafe extern "C" fn(VhpiHandleT) -> VhpiHandleT;
        type VhpiGetFn = unsafe extern "C" fn(std::ffi::c_int, VhpiHandleT) -> std::ffi::c_int;
        type VhpiGetStrFn = unsafe extern "C" fn(std::ffi::c_int, VhpiHandleT) -> *const u8;
        type VhpiGetRealFn = unsafe extern "C" fn(std::ffi::c_int, VhpiHandleT) -> f64;
        type VhpiGetPhysFn = unsafe extern "C" fn(std::ffi::c_int, VhpiHandleT) -> VhpiPhysS;
        type VhpiGetValueFn =
            unsafe extern "C" fn(VhpiHandleT, *mut std::ffi::c_void) -> std::ffi::c_int;
        type VhpiPutValueFn = unsafe extern "C" fn(
            VhpiHandleT,
            *mut std::ffi::c_void,
            std::ffi::c_int,
        ) -> std::ffi::c_int;
        type VhpiGetTimeFn = unsafe extern "C" fn(*mut VhpiTimeS, *mut std::ffi::c_long);
        type VhpiGetNextTimeFn = unsafe extern "C" fn(*mut VhpiTimeS) -> std::ffi::c_int;
        type VhpiControlFn = unsafe extern "C" fn(std::ffi::c_int) -> std::ffi::c_int;
        type VhpiPrintfFn = unsafe extern "C" fn(
            *const std::ffi::c_char,
            *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        type VhpiCompareHandlesFn =
            unsafe extern "C" fn(VhpiHandleT, VhpiHandleT) -> std::ffi::c_int;
        type VhpiCheckErrorFn = unsafe extern "C" fn(*mut std::ffi::c_void) -> std::ffi::c_int;
        type VhpiReleaseHandleFn = unsafe extern "C" fn(VhpiHandleT) -> std::ffi::c_int;

        static VHPI_ASSERT_FN: std::sync::OnceLock<VhpiAssertFn> = std::sync::OnceLock::new();
        static VHPI_REGISTER_CB_FN: std::sync::OnceLock<VhpiRegisterCbFn> =
            std::sync::OnceLock::new();
        static VHPI_REMOVE_CB_FN: std::sync::OnceLock<VhpiRemoveCbFn> = std::sync::OnceLock::new();
        static VHPI_HANDLE_BY_NAME_FN: std::sync::OnceLock<VhpiHandleByNameFn> =
            std::sync::OnceLock::new();
        static VHPI_HANDLE_BY_INDEX_FN: std::sync::OnceLock<VhpiHandleByIndexFn> =
            std::sync::OnceLock::new();
        static VHPI_HANDLE_FN: std::sync::OnceLock<VhpiHandleFn> = std::sync::OnceLock::new();
        static VHPI_ITERATOR_FN: std::sync::OnceLock<VhpiIteratorFn> = std::sync::OnceLock::new();
        static VHPI_SCAN_FN: std::sync::OnceLock<VhpiScanFn> = std::sync::OnceLock::new();
        static VHPI_GET_FN: std::sync::OnceLock<VhpiGetFn> = std::sync::OnceLock::new();
        static VHPI_GET_STR_FN: std::sync::OnceLock<VhpiGetStrFn> = std::sync::OnceLock::new();
        static VHPI_GET_REAL_FN: std::sync::OnceLock<VhpiGetRealFn> = std::sync::OnceLock::new();
        static VHPI_GET_PHYS_FN: std::sync::OnceLock<VhpiGetPhysFn> = std::sync::OnceLock::new();
        static VHPI_GET_VALUE_FN: std::sync::OnceLock<VhpiGetValueFn> = std::sync::OnceLock::new();
        static VHPI_PUT_VALUE_FN: std::sync::OnceLock<VhpiPutValueFn> = std::sync::OnceLock::new();
        static VHPI_GET_TIME_FN: std::sync::OnceLock<VhpiGetTimeFn> = std::sync::OnceLock::new();
        static VHPI_GET_NEXT_TIME_FN: std::sync::OnceLock<VhpiGetNextTimeFn> =
            std::sync::OnceLock::new();
        static VHPI_CONTROL_FN: std::sync::OnceLock<VhpiControlFn> = std::sync::OnceLock::new();
        static VHPI_PRINTF_FN: std::sync::OnceLock<VhpiPrintfFn> = std::sync::OnceLock::new();
        static VHPI_COMPARE_HANDLES_FN: std::sync::OnceLock<VhpiCompareHandlesFn> =
            std::sync::OnceLock::new();
        static VHPI_CHECK_ERROR_FN: std::sync::OnceLock<VhpiCheckErrorFn> =
            std::sync::OnceLock::new();
        static VHPI_RELEASE_HANDLE_FN: std::sync::OnceLock<VhpiReleaseHandleFn> =
            std::sync::OnceLock::new();

        macro_rules! resolve_fn {
            ($cell:ident, $name:literal, $ty:ty) => {
                *$cell.get_or_init(|| unsafe {
                    std::mem::transmute::<*mut std::ffi::c_void, $ty>(resolve_symbol(
                        concat!($name, "\0").as_bytes(),
                    ))
                })
            };
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_assert(
            severity: std::ffi::c_int,
            formatmsg: *mut std::ffi::c_char,
        ) -> std::ffi::c_int {
            vhpi_assert_cstr(severity, formatmsg)
        }

        pub unsafe fn vhpi_assert_cstr(
            severity: std::ffi::c_int,
            formatmsg: *mut std::ffi::c_char,
        ) -> std::ffi::c_int {
            resolve_fn!(VHPI_ASSERT_FN, "vhpi_assert", VhpiAssertFn)(severity, formatmsg)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_register_cb(
            cb_data_p: *mut std::ffi::c_void,
            flags: std::ffi::c_int,
        ) -> VhpiHandleT {
            resolve_fn!(VHPI_REGISTER_CB_FN, "vhpi_register_cb", VhpiRegisterCbFn)(cb_data_p, flags)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_remove_cb(cb_obj: VhpiHandleT) -> std::ffi::c_int {
            resolve_fn!(VHPI_REMOVE_CB_FN, "vhpi_remove_cb", VhpiRemoveCbFn)(cb_obj)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_handle_by_name(
            name: *const std::ffi::c_char,
            scope: VhpiHandleT,
        ) -> VhpiHandleT {
            resolve_fn!(
                VHPI_HANDLE_BY_NAME_FN,
                "vhpi_handle_by_name",
                VhpiHandleByNameFn
            )(name, scope)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_handle_by_index(
            it_rel: std::ffi::c_int,
            parent: VhpiHandleT,
            indx: std::ffi::c_int,
        ) -> VhpiHandleT {
            resolve_fn!(
                VHPI_HANDLE_BY_INDEX_FN,
                "vhpi_handle_by_index",
                VhpiHandleByIndexFn
            )(it_rel, parent, indx)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_handle(
            kind: std::ffi::c_int,
            reference_handle: VhpiHandleT,
        ) -> VhpiHandleT {
            resolve_fn!(VHPI_HANDLE_FN, "vhpi_handle", VhpiHandleFn)(kind, reference_handle)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_iterator(
            kind: std::ffi::c_int,
            reference_handle: VhpiHandleT,
        ) -> VhpiHandleT {
            resolve_fn!(VHPI_ITERATOR_FN, "vhpi_iterator", VhpiIteratorFn)(kind, reference_handle)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_scan(iterator: VhpiHandleT) -> VhpiHandleT {
            resolve_fn!(VHPI_SCAN_FN, "vhpi_scan", VhpiScanFn)(iterator)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_get(
            property: std::ffi::c_int,
            object: VhpiHandleT,
        ) -> std::ffi::c_int {
            resolve_fn!(VHPI_GET_FN, "vhpi_get", VhpiGetFn)(property, object)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_get_str(
            property: std::ffi::c_int,
            object: VhpiHandleT,
        ) -> *const u8 {
            resolve_fn!(VHPI_GET_STR_FN, "vhpi_get_str", VhpiGetStrFn)(property, object)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_get_real(
            property: std::ffi::c_int,
            object: VhpiHandleT,
        ) -> f64 {
            resolve_fn!(VHPI_GET_REAL_FN, "vhpi_get_real", VhpiGetRealFn)(property, object)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_get_phys(
            property: std::ffi::c_int,
            object: VhpiHandleT,
        ) -> VhpiPhysS {
            resolve_fn!(VHPI_GET_PHYS_FN, "vhpi_get_phys", VhpiGetPhysFn)(property, object)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_get_value(
            expr: VhpiHandleT,
            value_p: *mut std::ffi::c_void,
        ) -> std::ffi::c_int {
            resolve_fn!(VHPI_GET_VALUE_FN, "vhpi_get_value", VhpiGetValueFn)(expr, value_p)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_put_value(
            object: VhpiHandleT,
            value_p: *mut std::ffi::c_void,
            mode: std::ffi::c_int,
        ) -> std::ffi::c_int {
            resolve_fn!(VHPI_PUT_VALUE_FN, "vhpi_put_value", VhpiPutValueFn)(object, value_p, mode)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_get_time(
            time_p: *mut VhpiTimeS,
            cycles: *mut std::ffi::c_long,
        ) {
            resolve_fn!(VHPI_GET_TIME_FN, "vhpi_get_time", VhpiGetTimeFn)(time_p, cycles)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_get_next_time(time_p: *mut VhpiTimeS) -> std::ffi::c_int {
            resolve_fn!(
                VHPI_GET_NEXT_TIME_FN,
                "vhpi_get_next_time",
                VhpiGetNextTimeFn
            )(time_p)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_control(command: std::ffi::c_int) -> std::ffi::c_int {
            vhpi_control1(command)
        }

        pub unsafe fn vhpi_control1(command: std::ffi::c_int) -> std::ffi::c_int {
            resolve_fn!(VHPI_CONTROL_FN, "vhpi_control", VhpiControlFn)(command)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_printf(
            format: *const std::ffi::c_char,
            arg: *const std::ffi::c_char,
        ) -> std::ffi::c_int {
            vhpi_printf_cstr(format, arg)
        }

        pub unsafe fn vhpi_printf_cstr(
            format: *const std::ffi::c_char,
            arg: *const std::ffi::c_char,
        ) -> std::ffi::c_int {
            resolve_fn!(VHPI_PRINTF_FN, "vhpi_printf", VhpiPrintfFn)(format, arg)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_compare_handles(
            handle1: VhpiHandleT,
            handle2: VhpiHandleT,
        ) -> std::ffi::c_int {
            resolve_fn!(
                VHPI_COMPARE_HANDLES_FN,
                "vhpi_compare_handles",
                VhpiCompareHandlesFn
            )(handle1, handle2)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_check_error(
            error_info_p: *mut std::ffi::c_void,
        ) -> std::ffi::c_int {
            resolve_fn!(VHPI_CHECK_ERROR_FN, "vhpi_check_error", VhpiCheckErrorFn)(error_info_p)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn vhpi_release_handle(object: VhpiHandleT) -> std::ffi::c_int {
            resolve_fn!(
                VHPI_RELEASE_HANDLE_FN,
                "vhpi_release_handle",
                VhpiReleaseHandleFn
            )(object)
        }

        pub fn __link_vhpi_shim() {}
    };
}

#[cfg(windows)]
mod platform {
    use std::ffi::{c_char, c_void};
    use std::sync::OnceLock;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleA(module_name: *const c_char) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, proc_name: *const c_char) -> *mut c_void;
    }

    static HOST_MODULE: OnceLock<usize> = OnceLock::new();

    fn host_module() -> *mut c_void {
        let module = *HOST_MODULE.get_or_init(|| unsafe {
            let module = GetModuleHandleA(std::ptr::null());
            if module.is_null() {
                eprintln!("vhpi-shim: GetModuleHandleA(NULL) failed");
                std::process::abort();
            }
            module as usize
        });
        module as *mut c_void
    }

    pub fn resolve_symbol(name: &[u8]) -> *mut c_void {
        let proc = unsafe { GetProcAddress(host_module(), name.as_ptr().cast()) };
        if proc.is_null() {
            let symbol = String::from_utf8_lossy(&name[..name.len().saturating_sub(1)]);
            eprintln!("vhpi-shim: failed to resolve symbol {symbol}");
            std::process::abort();
        }
        proc
    }

    define_vhpi_forwarders!();
}

#[cfg(target_os = "macos")]
mod platform {
    use std::ffi::{c_char, c_int, c_void};
    use std::sync::OnceLock;

    unsafe extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }

    const RTLD_NOW: c_int = 0x2;
    const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

    static HOST_MODULE: OnceLock<usize> = OnceLock::new();

    fn host_module() -> *mut c_void {
        let module = *HOST_MODULE.get_or_init(|| unsafe {
            let module = dlopen(std::ptr::null(), RTLD_NOW);
            if module.is_null() {
                eprintln!("vhpi-shim: dlopen(NULL, RTLD_NOW) failed");
                std::process::abort();
            }
            module as usize
        });
        module as *mut c_void
    }

    pub fn resolve_symbol(name: &[u8]) -> *mut c_void {
        let mut proc = unsafe { dlsym(RTLD_DEFAULT, name.as_ptr().cast()) };
        if proc.is_null() {
            proc = unsafe { dlsym(host_module(), name.as_ptr().cast()) };
        }
        if proc.is_null() {
            let symbol = String::from_utf8_lossy(&name[..name.len().saturating_sub(1)]);
            eprintln!("vhpi-shim: failed to resolve symbol {symbol}");
            std::process::abort();
        }
        proc
    }

    define_vhpi_forwarders!();
}

#[cfg(any(windows, target_os = "macos"))]
pub use platform::__link_vhpi_shim;

#[cfg(any(windows, target_os = "macos"))]
pub use platform::vhpi_assert_cstr;

#[cfg(any(windows, target_os = "macos"))]
pub use platform::vhpi_control1;

#[cfg(any(windows, target_os = "macos"))]
pub use platform::vhpi_printf_cstr;

#[cfg(not(any(windows, target_os = "macos")))]
pub fn __link_vhpi_shim() {}
