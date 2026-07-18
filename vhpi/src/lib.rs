//! Safe Rust wrappers around VHPI APIs.
//!
//! | Feature   | Default | Description |
//! |-----------|---------|-------------|
//! | `nvc`     | Yes     | Include NVC-specific VHPI-extensions |
//! | `bigint`  | No      | Functions that return `BigInt`/`BigUint` |
//! | `dynamic` | No      | Enable runtime name resolution. |
//!
//! The `dynamic` feature is required if you want to build a dynamic library (dylib on macOS and DLL on Windows).
//! If you link directly to the simulator it is not required.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

mod callback;
mod control;
mod error;
mod foreignf;
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
pub use foreignf::*;
pub use handle::*;
pub use logic::*;
pub use physical::*;
pub use property::*;
pub use simulator::*;
pub use time::*;
pub use value::*;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern crate num_derive;

/// Print a message to the simulator console using `vhpi_printf`
pub fn printf(msg: impl AsRef<str>) {
    static FMT: &[u8] = b"%s\n\0";
    let cstr = string_to_iso8859_1_cstring(msg);
    unsafe { vhpi_sys::vhpi_printf_cstr(FMT.as_ptr().cast::<c_char>(), cstr.as_ptr()) };
}

/// Convert Rust string to ISO-8859-1 encoded C string.
///
/// Characters outside of ISO-8859-1 range are replaced with ?
pub fn string_to_iso8859_1_cstring(msg: impl AsRef<str>) -> CString {
    // Convert UTF-8 string to ISO-8859-1 bytes
    let iso8859_1_bytes: Vec<u8> = msg
        .as_ref()
        .chars()
        .map(|c| {
            let code = c as u32;
            u8::try_from(code).unwrap_or(b'?')
        })
        .collect();
    CString::new(iso8859_1_bytes).unwrap()
}

/// Test whether a character is printable by the simulator.
///
/// Returns `true` if the character can be safely printed to the simulator console,
/// `false` otherwise.
#[must_use]
pub fn is_printable(ch: u8) -> bool {
    unsafe { vhpi_sys::vhpi_is_printable(ch as std::os::raw::c_char) != 0 }
}

#[macro_export]
/// Print a formatted message to the simulator console.
///
/// This macro mirrors `println!`-style formatting and forwards to
/// [`printf()`].
macro_rules! printf {
    ($($arg:tt)*) => {{
        $crate::printf(&format!($($arg)*));
    }}
}

/// Convert ISO-8859-1 encoded C string from vhpiValueS to Rust String
fn iso8859_1_val_to_string(value: &vhpi_sys::vhpiValueS) -> String {
    let cstr = unsafe { CStr::from_ptr(value.value.str_.cast::<c_char>()) };
    iso8859_1_cstr_to_string(cstr)
}

/// Convert ISO-8859-1 encoded C string to Rust String
fn iso8859_1_cstr_to_string(cstr: &CStr) -> String {
    cstr.to_bytes()
        .iter()
        .map(|&b| char::from_u32(u32::from(b)).unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_iso8859_1_cstring_preserves_ascii_and_latin1() {
        let input = "Hello Àÿ";
        let cstr = string_to_iso8859_1_cstring(input);

        assert_eq!(cstr.as_bytes(), &[72, 101, 108, 108, 111, 32, 192, 255]);
    }

    #[test]
    fn string_to_iso8859_1_cstring_replaces_out_of_range_chars() {
        let input = "A€𝄞";
        let cstr = string_to_iso8859_1_cstring(input);

        assert_eq!(cstr.as_bytes(), b"A??");
    }

    #[test]
    fn iso8859_1_cstr_to_string_decodes_latin1_bytes() {
        let cstr =
            CStr::from_bytes_with_nul(&[0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0xC0, 0xFF, 0x00])
                .unwrap();

        assert_eq!(iso8859_1_cstr_to_string(cstr), "Hello Àÿ");
    }
}
