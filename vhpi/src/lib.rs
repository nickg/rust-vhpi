#[macro_use]
mod macros;

mod callback;
mod control;
mod error;
mod handle;
mod logic;
mod property;
mod time;
mod value;

pub use callback::*;
pub use control::*;
pub use error::*;
pub use handle::*;
pub use logic::*;
pub use property::*;
pub use time::*;
pub use value::*;

use std::ffi::CString;

extern crate num_derive;

pub fn printf(msg: impl AsRef<str>) {
    let cstr = CString::new(msg.as_ref()).unwrap();
    static FMT: &[u8] = b"%s\n\0";
    unsafe { vhpi_sys::vhpi_printf(FMT.as_ptr().cast::<i8>(), cstr.as_ptr()) };
}

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        $crate::printf(&format!($($arg)*));
    }}
}
