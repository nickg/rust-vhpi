use crate::Error;
use crate::Handle;
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
            Value::BinStr(s) => write!(f, "{s}"),
            Value::Int(n) => write!(f, "{n}"),
            Value::Logic(n) => write!(f, "{n}"),
            Value::LogicVec(v) => {
                for val in v {
                    write!(f, "{val}")?;
                }
                Ok(())
            }
            Value::SmallEnum(n) => write!(f, "{n}"),
            Value::Enum(n) => write!(f, "{n}"),
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

impl From<vhpi_sys::vhpiSeverityT> for Format {
    fn from(raw: vhpi_sys::vhpiSeverityT) -> Self {
        match raw {
            vhpi_sys::vhpiFormatT_vhpiObjTypeVal => Format::ObjType,
            vhpi_sys::vhpiFormatT_vhpiBinStrVal => Format::BinStr,
            vhpi_sys::vhpiFormatT_vhpiIntVal => Format::Int,
            vhpi_sys::vhpiFormatT_vhpiLogicVal => Format::Logic,
            vhpi_sys::vhpiFormatT_vhpiLogicVecVal => Format::LogicVec,
            vhpi_sys::vhpiFormatT_vhpiSmallEnumVal => Format::SmallEnum,
            vhpi_sys::vhpiFormatT_vhpiEnumVal => Format::Enum,
            other => Format::Unknown(other),
        }
    }
}

impl From<Format> for vhpi_sys::vhpiFormatT {
    fn from(format: Format) -> Self {
        match format {
            Format::ObjType => vhpi_sys::vhpiFormatT_vhpiObjTypeVal,
            Format::BinStr => vhpi_sys::vhpiFormatT_vhpiBinStrVal,
            Format::Int => vhpi_sys::vhpiFormatT_vhpiIntVal,
            Format::Logic => vhpi_sys::vhpiFormatT_vhpiLogicVal,
            Format::LogicVec => vhpi_sys::vhpiFormatT_vhpiLogicVecVal,
            Format::SmallEnum => vhpi_sys::vhpiFormatT_vhpiSmallEnumVal,
            Format::Enum => vhpi_sys::vhpiFormatT_vhpiEnumVal,
            Format::Unknown(n) => n,
        }
    }
}

impl Handle {
    pub fn get_value(&self, format: Format) -> Result<Value, Error> {
        let mut val = vhpi_sys::vhpiValueT {
            format: format.into(),
            bufSize: 0,
            numElems: 0,
            unit: vhpi_sys::vhpiPhysS { high: 0, low: 0 },
            value: vhpi_sys::vhpiValueS__bindgen_ty_1 { longintg: 0 },
        };

        let mut rc = unsafe { vhpi_sys::vhpi_get_value(self.as_raw(), &raw mut val) };
        let mut buffer: Vec<u8> = vec![];

        if rc > 0 {
            // Need to allocate buffer space
            let buf_size = match val.format {
                vhpi_sys::vhpiFormatT_vhpiBinStrVal => rc as usize,
                vhpi_sys::vhpiFormatT_vhpiLogicVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiEnumT>()
                }
                _ => {
                    panic!("unsupported vector format {}", val.format);
                }
            };
            buffer = vec![0; buf_size];
            val.bufSize = buf_size;

            match val.format {
                vhpi_sys::vhpiFormatT_vhpiBinStrVal => {
                    val.value.str_ = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiCharT>();
                }
                vhpi_sys::vhpiFormatT_vhpiLogicVecVal => {
                    val.value.enumvs = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiEnumT>();
                }
                _ => {
                    panic!("unsupported vector format {}", val.format);
                }
            }

            rc = unsafe { vhpi_sys::vhpi_get_value(self.as_raw(), &raw mut val) };
        }

        if rc < 0 {
            return Err(
                crate::check_error().unwrap_or_else(|| "Unknown error in vhpi_get_value".into())
            );
        }

        match val.format {
            vhpi_sys::vhpiFormatT_vhpiIntVal => Ok(Value::Int(unsafe { val.value.intg })),
            vhpi_sys::vhpiFormatT_vhpiLogicVal => Ok(Value::Logic(LogicVal::from(unsafe {
                val.value.enumv as u8
            }))),
            vhpi_sys::vhpiFormatT_vhpiEnumVal => Ok(Value::Enum(unsafe { val.value.enumv })),
            vhpi_sys::vhpiFormatT_vhpiSmallEnumVal => {
                Ok(Value::SmallEnum(unsafe { val.value.smallenumv }))
            }
            vhpi_sys::vhpiFormatT_vhpiBinStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::BinStr(rust_str.to_owned()))
            }
            vhpi_sys::vhpiFormatT_vhpiLogicVecVal => {
                let slice =
                    unsafe { std::slice::from_raw_parts(val.value.enumvs, val.numElems as usize) };
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
