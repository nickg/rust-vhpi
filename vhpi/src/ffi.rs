#![cfg_attr(not(windows), allow(dead_code))]

#[cfg(not(windows))]
pub use vhpi_sys::{
    vhpi_assert, vhpi_check_error, vhpi_compare_handles, vhpi_control, vhpi_get,
    vhpi_get_next_time, vhpi_get_phys, vhpi_get_real, vhpi_get_str, vhpi_get_time, vhpi_get_value,
    vhpi_handle, vhpi_handle_by_index, vhpi_handle_by_name, vhpi_iterator, vhpi_put_value,
    vhpi_register_cb, vhpi_release_handle, vhpi_remove_cb, vhpi_scan,
};

#[cfg(windows)]
mod windows {
    use std::ffi::c_void;
    use std::os::raw::{c_char, c_int, c_long};
    use std::sync::OnceLock;
    use vhpi_sys::*;

    type HModule = *mut c_void;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleA(lpModuleName: *const c_char) -> HModule;
        fn GetProcAddress(hModule: HModule, lpProcName: *const c_char) -> *mut c_void;
    }

    fn host_module() -> HModule {
        static HOST: OnceLock<usize> = OnceLock::new();
        let raw = HOST.get_or_init(|| {
            let h = unsafe { GetModuleHandleA(std::ptr::null()) };
            assert!(!h.is_null(), "GetModuleHandleA(NULL) failed");
            h as usize
        });
        *raw as HModule
    }

    fn resolve_symbol(name_with_nul: &'static [u8]) -> *const c_void {
        let proc =
            unsafe { GetProcAddress(host_module(), name_with_nul.as_ptr().cast::<c_char>()) };
        assert!(
            !proc.is_null(),
            "failed to resolve VHPI symbol {} from host process",
            String::from_utf8_lossy(&name_with_nul[..name_with_nul.len() - 1])
        );
        proc.cast::<c_void>()
    }

    macro_rules! resolve_fn {
        ($name:ident, $ty:ty) => {{
            static PTR: OnceLock<$ty> = OnceLock::new();
            *PTR.get_or_init(|| unsafe {
                std::mem::transmute::<*const c_void, $ty>(resolve_symbol(
                    concat!(stringify!($name), "\0").as_bytes(),
                ))
            })
        }};
    }

    pub unsafe fn vhpi_register_cb(cb_data_p: *mut vhpiCbDataT, flags: i32) -> vhpiHandleT {
        let f = resolve_fn!(
            vhpi_register_cb,
            unsafe extern "C" fn(*mut vhpiCbDataT, i32) -> vhpiHandleT
        );
        unsafe { f(cb_data_p, flags) }
    }

    pub unsafe fn vhpi_remove_cb(cb_obj: vhpiHandleT) -> c_int {
        let f = resolve_fn!(vhpi_remove_cb, unsafe extern "C" fn(vhpiHandleT) -> c_int);
        unsafe { f(cb_obj) }
    }

    pub unsafe fn vhpi_control(command: vhpiSimControlT) -> c_int {
        let f = resolve_fn!(
            vhpi_control,
            unsafe extern "C" fn(vhpiSimControlT, ...) -> c_int
        );
        unsafe { f(command) }
    }

    pub unsafe fn vhpi_check_error(error_info_p: *mut vhpiErrorInfoT) -> c_int {
        let f = resolve_fn!(
            vhpi_check_error,
            unsafe extern "C" fn(*mut vhpiErrorInfoT) -> c_int
        );
        unsafe { f(error_info_p) }
    }

    pub unsafe fn vhpi_assert(severity: vhpiSeverityT, formatmsg: *mut c_char) -> c_int {
        let f = resolve_fn!(
            vhpi_assert,
            unsafe extern "C" fn(vhpiSeverityT, *mut c_char, ...) -> c_int
        );
        unsafe { f(severity, formatmsg) }
    }

    pub unsafe fn vhpi_get_phys(property: vhpiPhysPropertyT, object: vhpiHandleT) -> vhpiPhysT {
        let f = resolve_fn!(
            vhpi_get_phys,
            unsafe extern "C" fn(vhpiPhysPropertyT, vhpiHandleT) -> vhpiPhysT
        );
        unsafe { f(property, object) }
    }

    pub unsafe fn vhpi_get_real(property: vhpiRealPropertyT, object: vhpiHandleT) -> vhpiRealT {
        let f = resolve_fn!(
            vhpi_get_real,
            unsafe extern "C" fn(vhpiRealPropertyT, vhpiHandleT) -> vhpiRealT
        );
        unsafe { f(property, object) }
    }

    pub unsafe fn vhpi_handle(type_: vhpiOneToOneT, reference_handle: vhpiHandleT) -> vhpiHandleT {
        let f = resolve_fn!(
            vhpi_handle,
            unsafe extern "C" fn(vhpiOneToOneT, vhpiHandleT) -> vhpiHandleT
        );
        unsafe { f(type_, reference_handle) }
    }

    pub unsafe fn vhpi_get(property: vhpiIntPropertyT, object: vhpiHandleT) -> vhpiIntT {
        let f = resolve_fn!(
            vhpi_get,
            unsafe extern "C" fn(vhpiIntPropertyT, vhpiHandleT) -> vhpiIntT
        );
        unsafe { f(property, object) }
    }

    pub unsafe fn vhpi_get_value(expr: vhpiHandleT, value_p: *mut vhpiValueT) -> c_int {
        let f = resolve_fn!(
            vhpi_get_value,
            unsafe extern "C" fn(vhpiHandleT, *mut vhpiValueT) -> c_int
        );
        unsafe { f(expr, value_p) }
    }

    pub unsafe fn vhpi_put_value(
        object: vhpiHandleT,
        value_p: *mut vhpiValueT,
        mode: vhpiPutValueModeT,
    ) -> c_int {
        let f = resolve_fn!(
            vhpi_put_value,
            unsafe extern "C" fn(vhpiHandleT, *mut vhpiValueT, vhpiPutValueModeT) -> c_int
        );
        unsafe { f(object, value_p, mode) }
    }

    pub unsafe fn vhpi_get_time(time_p: *mut vhpiTimeT, cycles: *mut c_long) {
        let f = resolve_fn!(
            vhpi_get_time,
            unsafe extern "C" fn(*mut vhpiTimeT, *mut c_long)
        );
        unsafe { f(time_p, cycles) }
    }

    pub unsafe fn vhpi_get_next_time(time_p: *mut vhpiTimeT) -> c_int {
        let f = resolve_fn!(
            vhpi_get_next_time,
            unsafe extern "C" fn(*mut vhpiTimeT) -> c_int
        );
        unsafe { f(time_p) }
    }

    pub unsafe fn vhpi_compare_handles(handle1: vhpiHandleT, handle2: vhpiHandleT) -> c_int {
        let f = resolve_fn!(
            vhpi_compare_handles,
            unsafe extern "C" fn(vhpiHandleT, vhpiHandleT) -> c_int
        );
        unsafe { f(handle1, handle2) }
    }

    pub unsafe fn vhpi_handle_by_index(
        it_rel: vhpiOneToManyT,
        parent: vhpiHandleT,
        indx: i32,
    ) -> vhpiHandleT {
        let f = resolve_fn!(
            vhpi_handle_by_index,
            unsafe extern "C" fn(vhpiOneToManyT, vhpiHandleT, i32) -> vhpiHandleT
        );
        unsafe { f(it_rel, parent, indx) }
    }

    pub unsafe fn vhpi_handle_by_name(name: *const c_char, scope: vhpiHandleT) -> vhpiHandleT {
        let f = resolve_fn!(
            vhpi_handle_by_name,
            unsafe extern "C" fn(*const c_char, vhpiHandleT) -> vhpiHandleT
        );
        unsafe { f(name, scope) }
    }

    pub unsafe fn vhpi_iterator(
        type_: vhpiOneToManyT,
        reference_handle: vhpiHandleT,
    ) -> vhpiHandleT {
        let f = resolve_fn!(
            vhpi_iterator,
            unsafe extern "C" fn(vhpiOneToManyT, vhpiHandleT) -> vhpiHandleT
        );
        unsafe { f(type_, reference_handle) }
    }

    pub unsafe fn vhpi_release_handle(object: vhpiHandleT) -> c_int {
        let f = resolve_fn!(
            vhpi_release_handle,
            unsafe extern "C" fn(vhpiHandleT) -> c_int
        );
        unsafe { f(object) }
    }

    pub unsafe fn vhpi_scan(iterator: vhpiHandleT) -> vhpiHandleT {
        let f = resolve_fn!(vhpi_scan, unsafe extern "C" fn(vhpiHandleT) -> vhpiHandleT);
        unsafe { f(iterator) }
    }

    pub unsafe fn vhpi_get_str(
        property: vhpiStrPropertyT,
        object: vhpiHandleT,
    ) -> *const vhpiCharT {
        let f = resolve_fn!(
            vhpi_get_str,
            unsafe extern "C" fn(vhpiStrPropertyT, vhpiHandleT) -> *const vhpiCharT
        );
        unsafe { f(property, object) }
    }
}

#[cfg(windows)]
pub use windows::*;
