use crate::Handle;
use crate::Error;
use crate::LogicVal;

use std::ffi::CStr;
use std::fmt;
use std::mem::size_of;

#[derive(Debug)]
pub enum Value {
    BinStr(String),
    Int(i32),
    Logic(LogicVal),
    LogicVec(Vec<LogicVal>),
    SmallEnum(u8),
    Enum(u32),
    Unknown,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::BinStr(s) => write!(f, "{}", s),
            Value::Int(n) => write!(f, "{}", n),
            Value::Logic(n) => write!(f, "{}", n),
            Value::LogicVec(v) => {
                for (_, val) in v.iter().enumerate() {
                    write!(f, "{}", val)?;
                }
                Ok(())
            }
            Value::SmallEnum(n) => write!(f, "{}", n),
            Value::Enum(n) => write!(f, "{}", n),
            Value::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    ObjType,
    BinStr,
    Int,
    Logic,
    LogicVec,
    SmallEnum,
    Enum,
    Unknown(u32),
}

impl From<bindings::vhpiSeverityT> for Format {
    fn from(raw: bindings::vhpiSeverityT) -> Self {
        match raw {
            bindings::vhpiFormatT_vhpiObjTypeVal => Format::ObjType,
            bindings::vhpiFormatT_vhpiBinStrVal => Format::BinStr,
            bindings::vhpiFormatT_vhpiIntVal => Format::Int,
            bindings::vhpiFormatT_vhpiLogicVal => Format::Logic,
            bindings::vhpiFormatT_vhpiLogicVecVal => Format::LogicVec,
            bindings::vhpiFormatT_vhpiSmallEnumVal => Format::SmallEnum,
            bindings::vhpiFormatT_vhpiEnumVal => Format::Enum,
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
            Format::Logic => bindings::vhpiFormatT_vhpiLogicVal,
            Format::LogicVec => bindings::vhpiFormatT_vhpiLogicVecVal,
            Format::SmallEnum => bindings::vhpiFormatT_vhpiSmallEnumVal,
            Format::Enum => bindings::vhpiFormatT_vhpiEnumVal,
            Format::Unknown(n) => n,
        }
    }
}

impl Handle {
    pub fn get_value(&self, format: Format) -> Result<Value, Error> {
        let mut val = bindings::vhpiValueT {
            format: format.into(),
            bufSize: 0,
            numElems: 0,
            unit: bindings::vhpiPhysS { high: 0, low: 0 },
            value: bindings::vhpiValueS__bindgen_ty_1 {
                longintg: 0,
            },
        };

        let mut rc = unsafe { bindings::vhpi_get_value(self.as_raw(), &mut val as *mut _) };
        let mut buffer: Vec<u8> = vec![];

        if rc > 0 {
            // Need to allocate buffer space
            let buf_size = match val.format {
                bindings::vhpiFormatT_vhpiBinStrVal => rc as usize,
                bindings::vhpiFormatT_vhpiLogicVecVal => {
                    rc as usize * size_of::<bindings::vhpiEnumT>()
                }
                _ => {
                    panic!("unsupported vector format {}", val.format);
                }
            };
            buffer = vec![0; buf_size];
            val.bufSize = buf_size;

            match val.format {
                bindings::vhpiFormatT_vhpiBinStrVal => {
                    val.value.str_ = buffer.as_mut_ptr() as *mut bindings::vhpiCharT;
                }
                bindings::vhpiFormatT_vhpiLogicVecVal => {
                    val.value.enumvs = buffer.as_mut_ptr() as *mut bindings::vhpiEnumT;
                }
                _ => {
                    panic!("unsupported vector format {}", val.format);
                }
            }

            rc = unsafe { bindings::vhpi_get_value(self.as_raw(), &mut val as *mut _) };
        }

        if rc < 0 {
            return Err(crate::check_error().unwrap_or_else(
                || "Unknown error in vhpi_get_value".into()));
        }

        match val.format {
            bindings::vhpiFormatT_vhpiIntVal =>
                Ok(Value::Int(unsafe { val.value.intg })),
            bindings::vhpiFormatT_vhpiLogicVal =>
                Ok(Value::Logic(LogicVal::from(unsafe { val.value.enumv as u8 }))),
            bindings::vhpiFormatT_vhpiEnumVal =>
                Ok(Value::Enum(unsafe { val.value.enumv })),
            bindings::vhpiFormatT_vhpiSmallEnumVal =>
                Ok(Value::SmallEnum(unsafe { val.value.smallenumv })),
            bindings::vhpiFormatT_vhpiBinStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::BinStr(rust_str.to_owned()))
            }
            bindings::vhpiFormatT_vhpiLogicVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(val.value.enumvs, val.numElems as usize)
                };
                let logic_vec: Vec<LogicVal> = slice
                    .iter()
                    .map(|&enumv| LogicVal::from(enumv as u8))
                    .collect();
                Ok(Value::LogicVec(logic_vec))
            }
            _ => Ok(Value::Unknown),
        }
    }
}
