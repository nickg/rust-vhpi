#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[cfg(feature = "dynamic")]
#[doc(hidden)]
pub use vhpi_winlib::__link_vhpi_winlib;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
