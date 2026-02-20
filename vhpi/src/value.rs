use crate::Error;
use crate::Handle;
use crate::LogicVal;

use std::ffi::CStr;
use std::fmt;
use std::mem::size_of;

#[derive(Debug)]
pub enum Value {
    BinStr(String),
    OctStr(String),
    HexStr(String),
    DecStr(String),
    Char(char),
    Int(i32),
    Logic(LogicVal),
    LogicVec(Vec<LogicVal>),
    SmallEnum(u8),
    Enum(u32),
    Str(String),
    Real(f64),
    RealVec(Vec<f64>),
    IntVec(Vec<i32>),
    Unknown,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::BinStr(s) => write!(f, "{s}"),
            Value::OctStr(s) => write!(f, "{s}"),
            Value::HexStr(s) => write!(f, "{s}"),
            Value::DecStr(s) => write!(f, "{s}"),
            Value::Int(n) => write!(f, "{n}"),
            Value::Char(c) => write!(f, "{c}"),
            Value::Logic(n) => write!(f, "{n}"),
            Value::LogicVec(v) => {
                for val in v {
                    write!(f, "{val}")?;
                }
                Ok(())
            }
            Value::SmallEnum(n) => write!(f, "{n}"),
            Value::Enum(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "{s}"),
            Value::Real(n) => write!(f, "{n}"),
            Value::RealVec(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
                Ok(())
            }
            Value::IntVec(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
                Ok(())
            }
            Value::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    ObjType,
    BinStr,
    OctStr,
    HexStr,
    DecStr,
    Char,
    Int,
    Logic,
    LogicVec,
    SmallEnum,
    Enum,
    Str,
    Real,
    RealVec,
    IntVec,
    Unknown(u32),
}

impl From<vhpi_sys::vhpiSeverityT> for Format {
    fn from(raw: vhpi_sys::vhpiSeverityT) -> Self {
        match raw {
            vhpi_sys::vhpiFormatT_vhpiObjTypeVal => Format::ObjType,
            vhpi_sys::vhpiFormatT_vhpiBinStrVal => Format::BinStr,
            vhpi_sys::vhpiFormatT_vhpiOctStrVal => Format::OctStr,
            vhpi_sys::vhpiFormatT_vhpiHexStrVal => Format::HexStr,
            vhpi_sys::vhpiFormatT_vhpiDecStrVal => Format::DecStr,
            vhpi_sys::vhpiFormatT_vhpiIntVal => Format::Int,
            vhpi_sys::vhpiFormatT_vhpiCharVal => Format::Char,
            vhpi_sys::vhpiFormatT_vhpiLogicVal => Format::Logic,
            vhpi_sys::vhpiFormatT_vhpiLogicVecVal => Format::LogicVec,
            vhpi_sys::vhpiFormatT_vhpiSmallEnumVal => Format::SmallEnum,
            vhpi_sys::vhpiFormatT_vhpiEnumVal => Format::Enum,
            vhpi_sys::vhpiFormatT_vhpiStrVal => Format::Str,
            vhpi_sys::vhpiFormatT_vhpiRealVal => Format::Real,
            vhpi_sys::vhpiFormatT_vhpiRealVecVal => Format::RealVec,
            vhpi_sys::vhpiFormatT_vhpiIntVecVal => Format::IntVec,
            other => Format::Unknown(other),
        }
    }
}

impl From<Format> for vhpi_sys::vhpiFormatT {
    fn from(format: Format) -> Self {
        match format {
            Format::ObjType => vhpi_sys::vhpiFormatT_vhpiObjTypeVal,
            Format::BinStr => vhpi_sys::vhpiFormatT_vhpiBinStrVal,
            Format::OctStr => vhpi_sys::vhpiFormatT_vhpiOctStrVal,
            Format::HexStr => vhpi_sys::vhpiFormatT_vhpiHexStrVal,
            Format::DecStr => vhpi_sys::vhpiFormatT_vhpiDecStrVal,
            Format::Int => vhpi_sys::vhpiFormatT_vhpiIntVal,
            Format::Char => vhpi_sys::vhpiFormatT_vhpiCharVal,
            Format::Logic => vhpi_sys::vhpiFormatT_vhpiLogicVal,
            Format::LogicVec => vhpi_sys::vhpiFormatT_vhpiLogicVecVal,
            Format::SmallEnum => vhpi_sys::vhpiFormatT_vhpiSmallEnumVal,
            Format::Enum => vhpi_sys::vhpiFormatT_vhpiEnumVal,
            Format::Str => vhpi_sys::vhpiFormatT_vhpiStrVal,
            Format::Real => vhpi_sys::vhpiFormatT_vhpiRealVal,
            Format::RealVec => vhpi_sys::vhpiFormatT_vhpiRealVecVal,
            Format::IntVec => vhpi_sys::vhpiFormatT_vhpiIntVecVal,
            Format::Unknown(n) => n,
        }
    }
}

pub enum PutValueMode {
    Deposit,
    DepositPropagate,
    Force,
    ForcePropagate,
    Release,
    SizeConstraint,
}

impl From<PutValueMode> for vhpi_sys::vhpiPutValueModeT {
    fn from(mode: PutValueMode) -> Self {
        match mode {
            PutValueMode::Deposit => vhpi_sys::vhpiPutValueModeT_vhpiDeposit,
            PutValueMode::DepositPropagate => vhpi_sys::vhpiPutValueModeT_vhpiDepositPropagate,
            PutValueMode::Force => vhpi_sys::vhpiPutValueModeT_vhpiForce,
            PutValueMode::ForcePropagate => vhpi_sys::vhpiPutValueModeT_vhpiForcePropagate,
            PutValueMode::Release => vhpi_sys::vhpiPutValueModeT_vhpiRelease,
            PutValueMode::SizeConstraint => vhpi_sys::vhpiPutValueModeT_vhpiSizeConstraint,
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
                vhpi_sys::vhpiFormatT_vhpiBinStrVal
                | vhpi_sys::vhpiFormatT_vhpiStrVal
                | vhpi_sys::vhpiFormatT_vhpiOctStrVal
                | vhpi_sys::vhpiFormatT_vhpiHexStrVal
                | vhpi_sys::vhpiFormatT_vhpiDecStrVal => rc as usize,
                vhpi_sys::vhpiFormatT_vhpiLogicVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiEnumT>()
                }
                vhpi_sys::vhpiFormatT_vhpiRealVecVal => rc as usize * size_of::<f64>(),
                vhpi_sys::vhpiFormatT_vhpiIntVecVal => rc as usize * size_of::<i32>(),
                _ => {
                    panic!("unsupported vector format {}", val.format);
                }
            };
            buffer = vec![0; buf_size];
            val.bufSize = buf_size;

            match val.format {
                vhpi_sys::vhpiFormatT_vhpiBinStrVal
                | vhpi_sys::vhpiFormatT_vhpiStrVal
                | vhpi_sys::vhpiFormatT_vhpiOctStrVal
                | vhpi_sys::vhpiFormatT_vhpiHexStrVal
                | vhpi_sys::vhpiFormatT_vhpiDecStrVal => {
                    val.value.str_ = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiCharT>();
                }
                vhpi_sys::vhpiFormatT_vhpiLogicVecVal => {
                    val.value.enumvs = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiEnumT>();
                }
                vhpi_sys::vhpiFormatT_vhpiRealVecVal => {
                    val.value.ptr = buffer.as_mut_ptr().cast::<std::ffi::c_void>();
                }
                vhpi_sys::vhpiFormatT_vhpiIntVecVal => {
                    val.value.ptr = buffer.as_mut_ptr().cast::<std::ffi::c_void>();
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
            vhpi_sys::vhpiFormatT_vhpiRealVal => Ok(Value::Real(unsafe { val.value.real })),
            vhpi_sys::vhpiFormatT_vhpiCharVal => Ok(Value::Char(unsafe { val.value.ch as char })),
            vhpi_sys::vhpiFormatT_vhpiBinStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::BinStr(rust_str.to_owned()))
            }
            vhpi_sys::vhpiFormatT_vhpiOctStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::OctStr(rust_str.to_owned()))
            }
            vhpi_sys::vhpiFormatT_vhpiHexStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::HexStr(rust_str.to_owned()))
            }
            vhpi_sys::vhpiFormatT_vhpiDecStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::DecStr(rust_str.to_owned()))
            }
            vhpi_sys::vhpiFormatT_vhpiStrVal => {
                let cstr = unsafe { CStr::from_ptr(val.value.str_ as *const i8) };
                let rust_str = cstr.to_str().map_err(|_| "Invalid UTF-8 in VHPI string")?;
                Ok(Value::Str(rust_str.to_owned()))
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
            vhpi_sys::vhpiFormatT_vhpiRealVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(val.value.ptr.cast::<f64>(), val.numElems as usize)
                };
                Ok(Value::RealVec(slice.to_vec()))
            }
            vhpi_sys::vhpiFormatT_vhpiIntVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(val.value.ptr.cast::<i32>(), val.numElems as usize)
                };
                Ok(Value::IntVec(slice.to_vec()))
            }
            _ => Ok(Value::Unknown),
        }
    }

    pub fn put_value(&self, value: Value, mode: PutValueMode) -> Result<(), Error> {
        let (format, val) = match value {
            Value::Int(n) => (Format::Int, vhpi_sys::vhpiValueS__bindgen_ty_1 { intg: n }),
            Value::Logic(n) => (
                Format::Logic,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { enumv: n.into() },
            ),
            Value::Enum(n) => (
                Format::Enum,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { enumv: n },
            ),
            Value::SmallEnum(n) => (
                Format::SmallEnum,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { smallenumv: n },
            ),
            Value::BinStr(s)
            | Value::OctStr(s)
            | Value::HexStr(s)
            | Value::DecStr(s)
            | Value::Str(s) => {
                let c_string = std::ffi::CString::new(s)
                    .map_err(|_| "Failed to convert string to C string")?;
                let ptr = c_string.into_raw().cast::<vhpi_sys::vhpiCharT>();
                (
                    Format::BinStr,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { str_: ptr },
                )
            }
            Value::LogicVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiEnumT> =
                    vec.iter().map(|&val| val.into()).collect();
                let ptr = buffer.as_mut_ptr();
                std::mem::forget(buffer); // Prevent Rust from freeing the buffer
                (
                    Format::LogicVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { enumvs: ptr },
                )
            }
            Value::IntVec(vec) => {
                let mut buffer: Vec<i32> = vec.clone();
                let ptr = buffer.as_mut_ptr();
                std::mem::forget(buffer); // Prevent Rust from freeing the buffer
                (
                    Format::IntVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { ptr: ptr.cast() },
                )
            }
            Value::RealVec(vec) => {
                let mut buffer: Vec<f64> = vec.clone();
                let ptr = buffer.as_mut_ptr();
                std::mem::forget(buffer); // Prevent Rust from freeing the buffer
                (
                    Format::RealVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { ptr: ptr.cast() },
                )
            }
            Value::Real(n) => (Format::Real, vhpi_sys::vhpiValueS__bindgen_ty_1 { real: n }),
            Value::Char(c) => (
                Format::Char,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { ch: c as u8 },
            ),
            Value::Unknown => return Err("Cannot put unknown value".into()),
        };

        let mut val_struct = vhpi_sys::vhpiValueT {
            format: format.into(),
            bufSize: 0,
            numElems: 0,
            unit: vhpi_sys::vhpiPhysS { high: 0, low: 0 },
            value: val,
        };

        let rc =
            unsafe { vhpi_sys::vhpi_put_value(self.as_raw(), &raw mut val_struct, mode.into()) };
        if rc < 0 {
            return Err(
                crate::check_error().unwrap_or_else(|| "Unknown error in vhpi_put_value".into())
            );
        }

        Ok(())
    }
}
