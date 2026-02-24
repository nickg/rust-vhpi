#[macro_use]
mod macros;

mod callback;
mod control;
mod error;
mod handle;
mod logic;
mod physical;
mod property;
mod simulator;
mod time;
mod value;

pub use callback::*;
pub use control::*;
pub use error::*;
pub use handle::*;
pub use logic::*;
pub use physical::*;
pub use property::*;
pub use simulator::*;
pub use time::*;
pub use value::*;

use std::ffi::{CStr, CString};

extern crate num_derive;

pub fn printf(msg: impl AsRef<str>) {
    // Convert UTF-8 string to ISO-8859-1 bytes
    let iso8859_1_bytes: Vec<u8> = msg
        .as_ref()
        .chars()
        .map(|c| {
            let code = c as u32;
            if code <= 0xFF {
                code as u8
            } else {
                b'?' // Replace characters outside ISO-8859-1 range
            }
        })
        .collect();
    let cstr = CString::new(iso8859_1_bytes).unwrap();
    static FMT: &[u8] = b"%s\n\0";
    unsafe { vhpi_sys::vhpi_printf(FMT.as_ptr().cast::<i8>(), cstr.as_ptr()) };
}

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        $crate::printf(&format!($($arg)*));
    }}
}

/// Convert ISO-8859-1 encoded C string from vhpiValueS to Rust String
fn iso8859_1_val_to_string(value: &vhpi_sys::vhpiValueS) -> String {
    let cstr = unsafe { CStr::from_ptr(value.value.str_ as *const i8) };
    iso8859_1_cstr_to_string(cstr)
}

/// Convert ISO-8859-1 encoded C string to Rust String
fn iso8859_1_cstr_to_string(cstr: &CStr) -> String {
    cstr.to_bytes()
        .iter()
        .map(|&b| char::from_u32(u32::from(b)).unwrap())
        .collect::<String>()
}
