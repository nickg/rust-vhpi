#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::c_char;

#[cfg(feature = "dynamic")]
#[doc(hidden)]
pub use vhpi_shim::__link_vhpi_shim;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(feature = "dynamic", any(windows, target_os = "macos")))]
pub unsafe fn vhpi_printf_cstr(format: *const c_char, arg: *const c_char) -> ::std::os::raw::c_int {
    vhpi_shim::vhpi_printf_cstr(format, arg)
}

#[cfg(not(all(feature = "dynamic", any(windows, target_os = "macos"))))]
pub unsafe fn vhpi_printf_cstr(format: *const c_char, arg: *const c_char) -> ::std::os::raw::c_int {
    vhpi_printf(format, arg)
}

#[cfg(all(feature = "dynamic", any(windows, target_os = "macos")))]
pub unsafe fn vhpi_assert_cstr(severity: vhpiSeverityT, msg: *mut c_char) -> ::std::os::raw::c_int {
    vhpi_shim::vhpi_assert_cstr(severity as ::std::os::raw::c_int, msg)
}

#[cfg(not(all(feature = "dynamic", any(windows, target_os = "macos"))))]
pub unsafe fn vhpi_assert_cstr(severity: vhpiSeverityT, msg: *mut c_char) -> ::std::os::raw::c_int {
    vhpi_assert(severity, msg)
}

#[cfg(all(feature = "dynamic", any(windows, target_os = "macos")))]
pub unsafe fn vhpi_control1(command: vhpiSimControlT) -> ::std::os::raw::c_int {
    vhpi_shim::vhpi_control1(command as ::std::os::raw::c_int)
}

#[cfg(not(all(feature = "dynamic", any(windows, target_os = "macos"))))]
pub unsafe fn vhpi_control1(command: vhpiSimControlT) -> ::std::os::raw::c_int {
    vhpi_control(command)
}
