use crate::Handle;
use crate::Error;

use std::ffi::CStr;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    BinStr(String),
    Int(i32),
    Unknown,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::BinStr(s) => write!(f, "{}", s),
            Value::Int(n) => write!(f, "{}", n),
            Value::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    ObjType,
    BinStr,
    Int,
    Unknown(u32),
}

impl From<bindings::vhpiSeverityT> for Format {
    fn from(raw: bindings::vhpiSeverityT) -> Self {
        match raw {
            bindings::vhpiFormatT_vhpiObjTypeVal => Format::ObjType,
            bindings::vhpiFormatT_vhpiBinStrVal => Format::BinStr,
            bindings::vhpiFormatT_vhpiIntVal => Format::Int,
            other => Format::Unknown(other),
        }
    }
}

impl From<Format> for bindings::vhpiFormatT {
    fn from(format: Format) -> Self {
        match format {
            Format::ObjType => bindings::vhpiFormatT_vhpiObjTypeVal,
            Format::BinStr => bindings::vhpiFormatT_vhpiBinStrVal,
            Format::Int => bindings::vhpiFormatT_vhpiIntVal,
            Format::Unknown(n) => n,
        }
    }
}

impl Handle {
    pub fn get_value(&self, format: Format) -> Result<Value, Error> {
        const BUF_SIZE: usize = 1024;
        let mut buf = [0; BUF_SIZE];

        let mut val = bindings::vhpiValueT {
            format: format.into(),
            bufSize: match format {
                Format::BinStr => BUF_SIZE,
                _ => 0,
            },
            numElems: 0,
            unit: bindings::vhpiPhysS { high: 0, low: 0 },
            value: bindings::vhpiValueS__bindgen_ty_1 {
                str_: buf.as_mut_ptr() as *mut bindings::vhpiCharT,
            },
        };

        let rc = unsafe { bindings::vhpi_get_value(self.as_raw(), &mut val as *mut _) };
        if rc != 0 {
            return Err(crate::check_error().unwrap_or_else(
                || "Unknown error in vhpi_get_value".into()));
        }

        match val.format {
            bindings::vhpiFormatT_vhpiIntVal =>
                Ok(Value::Int(unsafe { val.value.intg })),
            bindings::vhpiFormatT_vhpiBinStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::BinStr(rust_str.to_owned()))
            }
            _ => Ok(Value::Unknown),
        }
    }
}
