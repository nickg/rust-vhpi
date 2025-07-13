#[macro_use]
mod macros;

mod handle;
mod callback;
mod property;

pub use handle::*;
pub use callback::*;
pub use property::*;

use std::ffi::CString;

extern crate num_derive;

pub fn printf(msg: &str) {
    let cstr = CString::new(msg).expect("CString::new failed");
    unsafe { bindings::vhpi_printf(cstr.as_ptr()) };
}
