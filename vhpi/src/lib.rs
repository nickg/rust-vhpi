#[macro_use]
mod macros;

mod handle;
mod callback;
mod property;
mod value;
mod error;

pub use handle::*;
pub use callback::*;
pub use property::*;
pub use value::*;
pub use error::*;

use std::ffi::CString;

extern crate num_derive;

pub fn printf(msg: impl AsRef<str>) {
    let cstr = CString::new(msg.as_ref()).unwrap();
    static FMT: &[u8] = b"%s\n\0";
    unsafe { bindings::vhpi_printf(FMT.as_ptr() as *const i8, cstr.as_ptr()) };
}

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        $crate::printf(&format!($($arg)*));
    }}
}
