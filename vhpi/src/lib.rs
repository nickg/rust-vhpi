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
    static FMT: &[u8] = b"%s\n\0";
    let cstr = string_to_iso8859_1_cstring(msg);
    unsafe { vhpi_sys::vhpi_printf(FMT.as_ptr().cast::<i8>(), cstr.as_ptr()) };
}

/// Convert Rust string to ISO-8859-1 encoded C string
/// Characters outside of ISO-8859-1 range are replaced with ?
pub fn string_to_iso8859_1_cstring(msg: impl AsRef<str>) -> CString {
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
    CString::new(iso8859_1_bytes).unwrap()
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
