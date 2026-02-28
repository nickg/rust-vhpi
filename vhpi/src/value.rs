use crate::iso8859_1_val_to_string;
use crate::string_to_iso8859_1_cstring;
use crate::Error;
use crate::Handle;
use crate::LogicVal;
use crate::Physical;
use crate::Time;

use std::fmt;
use std::mem::size_of;

#[cfg(feature = "bigint")]
use num_bigint::{BigInt, BigUint};
#[cfg(feature = "bigint")]
use num_traits::One;
use num_traits::Zero;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    BinStr(String),
    OctStr(String),
    HexStr(String),
    DecStr(String),
    Char(char),
    Int(i32),
    IntVec(Vec<i32>),
    Logic(LogicVal),
    LogicVec(Vec<LogicVal>),
    SmallEnum(u8),
    SmallEnumVec(Vec<u8>),
    Enum(u32),
    EnumVec(Vec<u32>),
    Str(String),
    Real(f64),
    RealVec(Vec<f64>),
    Time(Time),
    TimeVec(Vec<Time>),
    LongInt(i64),
    LongIntVec(Vec<i64>),
    SmallPhysical(i32),
    SmallPhysicalVec(Vec<i32>),
    Physical(Physical),
    PhysicalVec(Vec<Physical>),
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
            Value::SmallEnumVec(v) => {
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
            Value::Enum(n) => write!(f, "{n}"),
            Value::EnumVec(v) => {
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
            Value::Time(t) => write!(f, "{t}"),
            Value::TimeVec(v) => {
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
            Value::LongInt(n) => write!(f, "{n}"),
            Value::LongIntVec(v) => {
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
            Value::SmallPhysical(n) => write!(f, "{n}"),
            Value::SmallPhysicalVec(v) => {
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
            Value::Physical(p) => write!(f, "{}", p.to_i64()),
            Value::PhysicalVec(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(|p| p.to_i64().to_string())
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
    SmallEnumVec,
    Enum,
    EnumVec,
    Str,
    Real,
    RealVec,
    IntVec,
    LongInt,
    LongIntVec,
    SmallPhysical,
    SmallPhysicalVec,
    Physical,
    PhysicalVec,
    Time,
    TimeVec,
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
            vhpi_sys::vhpiFormatT_vhpiSmallEnumVecVal => Format::SmallEnumVec,
            vhpi_sys::vhpiFormatT_vhpiEnumVal => Format::Enum,
            vhpi_sys::vhpiFormatT_vhpiEnumVecVal => Format::EnumVec,
            vhpi_sys::vhpiFormatT_vhpiStrVal => Format::Str,
            vhpi_sys::vhpiFormatT_vhpiRealVal => Format::Real,
            vhpi_sys::vhpiFormatT_vhpiRealVecVal => Format::RealVec,
            vhpi_sys::vhpiFormatT_vhpiIntVecVal => Format::IntVec,
            vhpi_sys::vhpiFormatT_vhpiLongIntVal => Format::LongInt,
            vhpi_sys::vhpiFormatT_vhpiLongIntVecVal => Format::LongIntVec,
            vhpi_sys::vhpiFormatT_vhpiSmallPhysVal => Format::SmallPhysical,
            vhpi_sys::vhpiFormatT_vhpiSmallPhysVecVal => Format::SmallPhysicalVec,
            vhpi_sys::vhpiFormatT_vhpiPhysVal => Format::Physical,
            vhpi_sys::vhpiFormatT_vhpiPhysVecVal => Format::PhysicalVec,
            vhpi_sys::vhpiFormatT_vhpiTimeVal => Format::Time,
            vhpi_sys::vhpiFormatT_vhpiTimeVecVal => Format::TimeVec,
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
            Format::SmallEnumVec => vhpi_sys::vhpiFormatT_vhpiSmallEnumVecVal,
            Format::Enum => vhpi_sys::vhpiFormatT_vhpiEnumVal,
            Format::EnumVec => vhpi_sys::vhpiFormatT_vhpiEnumVecVal,
            Format::Str => vhpi_sys::vhpiFormatT_vhpiStrVal,
            Format::Real => vhpi_sys::vhpiFormatT_vhpiRealVal,
            Format::RealVec => vhpi_sys::vhpiFormatT_vhpiRealVecVal,
            Format::IntVec => vhpi_sys::vhpiFormatT_vhpiIntVecVal,
            Format::LongInt => vhpi_sys::vhpiFormatT_vhpiLongIntVal,
            Format::LongIntVec => vhpi_sys::vhpiFormatT_vhpiLongIntVecVal,
            Format::SmallPhysical => vhpi_sys::vhpiFormatT_vhpiSmallPhysVal,
            Format::SmallPhysicalVec => vhpi_sys::vhpiFormatT_vhpiSmallPhysVecVal,
            Format::Physical => vhpi_sys::vhpiFormatT_vhpiPhysVal,
            Format::PhysicalVec => vhpi_sys::vhpiFormatT_vhpiPhysVecVal,
            Format::Time => vhpi_sys::vhpiFormatT_vhpiTimeVal,
            Format::TimeVec => vhpi_sys::vhpiFormatT_vhpiTimeVecVal,
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

enum VectorBox {
    #[allow(dead_code)]
    Enum(Vec<vhpi_sys::vhpiEnumT>),
    #[allow(dead_code)]
    Int(Vec<vhpi_sys::vhpiIntT>),
    #[allow(dead_code)]
    Real(Vec<vhpi_sys::vhpiRealT>),
    #[allow(dead_code)]
    Time(Vec<vhpi_sys::vhpiTimeT>),
    #[allow(dead_code)]
    SmallEnum(Vec<vhpi_sys::vhpiSmallEnumT>),
    #[allow(dead_code)]
    LongInt(Vec<vhpi_sys::vhpiLongIntT>),
    #[allow(dead_code)]
    SmallPhys(Vec<vhpi_sys::vhpiSmallPhysT>),
    #[allow(dead_code)]
    Phys(Vec<vhpi_sys::vhpiPhysT>),
}

impl VectorBox {
    fn len(&self) -> usize {
        match self {
            VectorBox::Enum(values) => values.len(),
            VectorBox::Int(values) => values.len(),
            VectorBox::Real(values) => values.len(),
            VectorBox::Time(values) => values.len(),
            VectorBox::SmallEnum(values) => values.len(),
            VectorBox::LongInt(values) => values.len(),
            VectorBox::SmallPhys(values) => values.len(),
            VectorBox::Phys(values) => values.len(),
        }
    }

    fn byte_len(&self) -> usize {
        match self {
            VectorBox::Enum(values) => values.len() * size_of::<vhpi_sys::vhpiEnumT>(),
            VectorBox::Int(values) => values.len() * size_of::<vhpi_sys::vhpiIntT>(),
            VectorBox::Real(values) => values.len() * size_of::<vhpi_sys::vhpiRealT>(),
            VectorBox::Time(values) => values.len() * size_of::<vhpi_sys::vhpiTimeT>(),
            VectorBox::SmallEnum(values) => values.len() * size_of::<vhpi_sys::vhpiSmallEnumT>(),
            VectorBox::LongInt(values) => values.len() * size_of::<vhpi_sys::vhpiLongIntT>(),
            VectorBox::SmallPhys(values) => values.len() * size_of::<vhpi_sys::vhpiSmallPhysT>(),
            VectorBox::Phys(values) => values.len() * size_of::<vhpi_sys::vhpiPhysT>(),
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
        // Allocate buffer so that it is kept for the whole function
        let mut buffer: Vec<u8> = vec![];
        if rc > 0 {
            // Need to allocate buffer space
            let buf_size = match val.format {
                vhpi_sys::vhpiFormatT_vhpiBinStrVal
                | vhpi_sys::vhpiFormatT_vhpiStrVal
                | vhpi_sys::vhpiFormatT_vhpiOctStrVal
                | vhpi_sys::vhpiFormatT_vhpiHexStrVal
                | vhpi_sys::vhpiFormatT_vhpiDecStrVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiCharT>()
                }
                vhpi_sys::vhpiFormatT_vhpiLogicVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiEnumT>()
                }
                vhpi_sys::vhpiFormatT_vhpiRealVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiRealT>()
                }
                vhpi_sys::vhpiFormatT_vhpiIntVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiIntT>()
                }
                vhpi_sys::vhpiFormatT_vhpiLongIntVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiLongIntT>()
                }
                vhpi_sys::vhpiFormatT_vhpiSmallPhysVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiSmallPhysT>()
                }
                vhpi_sys::vhpiFormatT_vhpiPhysVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiPhysT>()
                }
                vhpi_sys::vhpiFormatT_vhpiTimeVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiTimeT>()
                }
                vhpi_sys::vhpiFormatT_vhpiSmallEnumVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiSmallEnumT>()
                }
                vhpi_sys::vhpiFormatT_vhpiEnumVecVal => {
                    rc as usize * size_of::<vhpi_sys::vhpiEnumT>()
                }
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
                    val.value.reals = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiRealT>();
                }
                vhpi_sys::vhpiFormatT_vhpiIntVecVal => {
                    val.value.intgs = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiIntT>();
                }
                vhpi_sys::vhpiFormatT_vhpiLongIntVecVal => {
                    val.value.longintgs = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiLongIntT>();
                }
                vhpi_sys::vhpiFormatT_vhpiEnumVecVal => {
                    val.value.enumvs = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiEnumT>();
                }
                vhpi_sys::vhpiFormatT_vhpiSmallEnumVecVal => {
                    val.value.smallenumvs = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiSmallEnumT>();
                }
                vhpi_sys::vhpiFormatT_vhpiSmallPhysVecVal => {
                    val.value.smallphyss = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiSmallPhysT>();
                }
                vhpi_sys::vhpiFormatT_vhpiPhysVecVal => {
                    val.value.physs = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiPhysT>();
                }
                vhpi_sys::vhpiFormatT_vhpiTimeVecVal => {
                    val.value.times = buffer.as_mut_ptr().cast::<vhpi_sys::vhpiTimeT>();
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

        let ret = match val.format {
            vhpi_sys::vhpiFormatT_vhpiIntVal => Ok(Value::Int(unsafe { val.value.intg })),
            vhpi_sys::vhpiFormatT_vhpiLogicVal => Ok(Value::Logic(LogicVal::from(unsafe {
                val.value.enumv as u8
            }))),
            vhpi_sys::vhpiFormatT_vhpiEnumVal => Ok(Value::Enum(unsafe { val.value.enumv })),
            vhpi_sys::vhpiFormatT_vhpiSmallEnumVal => {
                Ok(Value::SmallEnum(unsafe { val.value.smallenumv }))
            }
            vhpi_sys::vhpiFormatT_vhpiLongIntVal => {
                Ok(Value::LongInt(unsafe { val.value.longintg }))
            }
            vhpi_sys::vhpiFormatT_vhpiRealVal => Ok(Value::Real(unsafe { val.value.real })),
            vhpi_sys::vhpiFormatT_vhpiCharVal => Ok(Value::Char(unsafe { val.value.ch as char })),
            vhpi_sys::vhpiFormatT_vhpiBinStrVal => Ok(Value::BinStr(iso8859_1_val_to_string(&val))),
            vhpi_sys::vhpiFormatT_vhpiOctStrVal => Ok(Value::OctStr(iso8859_1_val_to_string(&val))),
            vhpi_sys::vhpiFormatT_vhpiHexStrVal => Ok(Value::HexStr(iso8859_1_val_to_string(&val))),
            vhpi_sys::vhpiFormatT_vhpiDecStrVal => Ok(Value::DecStr(iso8859_1_val_to_string(&val))),
            vhpi_sys::vhpiFormatT_vhpiStrVal => Ok(Value::Str(iso8859_1_val_to_string(&val))),
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
                    std::slice::from_raw_parts(val.value.reals.cast::<f64>(), val.numElems as usize)
                };
                Ok(Value::RealVec(slice.to_vec()))
            }
            vhpi_sys::vhpiFormatT_vhpiIntVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(val.value.intgs.cast::<i32>(), val.numElems as usize)
                };
                Ok(Value::IntVec(slice.to_vec()))
            }
            vhpi_sys::vhpiFormatT_vhpiTimeVal => Ok(Value::Time(unsafe { val.value.time.into() })),
            vhpi_sys::vhpiFormatT_vhpiTimeVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(
                        val.value.times.cast::<vhpi_sys::vhpiTimeT>(),
                        val.numElems as usize,
                    )
                };
                let time_vec: Vec<Time> = slice.iter().map(|&t| t.into()).collect();
                Ok(Value::TimeVec(time_vec))
            }
            vhpi_sys::vhpiFormatT_vhpiSmallEnumVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(
                        val.value.smallenumvs.cast::<vhpi_sys::vhpiSmallEnumT>(),
                        val.numElems as usize,
                    )
                };
                Ok(Value::SmallEnumVec(slice.to_vec()))
            }
            vhpi_sys::vhpiFormatT_vhpiEnumVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(
                        val.value.enumvs.cast::<vhpi_sys::vhpiEnumT>(),
                        val.numElems as usize,
                    )
                };
                Ok(Value::EnumVec(slice.to_vec()))
            }
            vhpi_sys::vhpiFormatT_vhpiSmallPhysVal => {
                Ok(Value::SmallPhysical(unsafe { val.value.smallphys }))
            }
            vhpi_sys::vhpiFormatT_vhpiSmallPhysVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(
                        val.value.smallphyss.cast::<vhpi_sys::vhpiSmallPhysT>(),
                        val.numElems as usize,
                    )
                };
                Ok(Value::SmallPhysicalVec(slice.to_vec()))
            }
            vhpi_sys::vhpiFormatT_vhpiPhysVal => {
                Ok(Value::Physical(unsafe { val.value.phys.into() }))
            }
            vhpi_sys::vhpiFormatT_vhpiPhysVecVal => {
                let slice = unsafe {
                    std::slice::from_raw_parts(
                        val.value.physs.cast::<vhpi_sys::vhpiPhysT>(),
                        val.numElems as usize,
                    )
                };
                let phys_vec: Vec<Physical> = slice.iter().map(|&p| p.into()).collect();
                Ok(Value::PhysicalVec(phys_vec))
            }
            _ => Ok(Value::Unknown),
        };

        // Keep buffer alive until after the the pointer is used to be safe
        let _ = buffer;

        ret
    }

    pub fn put_value(&self, value: Value, mode: PutValueMode) -> Result<(), Error> {
        // Create a holder for any allocated buffer
        let mut buffer_holder: Option<VectorBox> = None;

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
            Value::BinStr(s) => {
                let c_string = string_to_iso8859_1_cstring(s);
                let ptr = c_string.into_raw().cast::<vhpi_sys::vhpiCharT>();
                (
                    Format::BinStr,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { str_: ptr },
                )
            }
            Value::OctStr(s) => {
                let c_string = string_to_iso8859_1_cstring(s);
                let ptr = c_string.into_raw().cast::<vhpi_sys::vhpiCharT>();
                (
                    Format::OctStr,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { str_: ptr },
                )
            }
            Value::HexStr(s) => {
                let c_string = string_to_iso8859_1_cstring(s);
                let ptr = c_string.into_raw().cast::<vhpi_sys::vhpiCharT>();
                (
                    Format::HexStr,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { str_: ptr },
                )
            }
            Value::DecStr(s) => {
                let c_string = string_to_iso8859_1_cstring(s);
                let ptr = c_string.into_raw().cast::<vhpi_sys::vhpiCharT>();
                (
                    Format::DecStr,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { str_: ptr },
                )
            }
            Value::Str(s) => {
                let c_string = string_to_iso8859_1_cstring(s);
                let ptr = c_string.into_raw().cast::<vhpi_sys::vhpiCharT>();
                (
                    Format::Str,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { str_: ptr },
                )
            }
            Value::LogicVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiEnumT> =
                    vec.iter().map(|&val| val.into()).collect();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::Enum(buffer));
                (
                    Format::LogicVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { enumvs: ptr },
                )
            }
            Value::IntVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiIntT> = vec.clone();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::Int(buffer));
                (
                    Format::IntVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { intgs: ptr },
                )
            }
            Value::RealVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiRealT> = vec.clone();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::Real(buffer));
                (
                    Format::RealVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { reals: ptr },
                )
            }
            Value::Time(t) => (
                Format::Time,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { time: t.into() },
            ),
            Value::TimeVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiTimeT> =
                    vec.iter().map(|val| val.clone().into()).collect();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::Time(buffer));
                (
                    Format::TimeVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { times: ptr },
                )
            }
            Value::Real(n) => (Format::Real, vhpi_sys::vhpiValueS__bindgen_ty_1 { real: n }),
            Value::Char(c) => (
                Format::Char,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { ch: c as u8 },
            ),
            Value::SmallEnumVec(v) => {
                let mut buffer: Vec<vhpi_sys::vhpiSmallEnumT> = v.clone();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::SmallEnum(buffer));
                (
                    Format::SmallEnumVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { smallenumvs: ptr },
                )
            }
            Value::EnumVec(v) => {
                let mut buffer: Vec<vhpi_sys::vhpiEnumT> = v.clone();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::Enum(buffer));
                (
                    Format::EnumVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { enumvs: ptr },
                )
            }
            Value::LongInt(l) => (
                Format::LongInt,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { longintg: l },
            ),
            Value::LongIntVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiLongIntT> = vec.clone();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::LongInt(buffer));
                (
                    Format::LongIntVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { longintgs: ptr },
                )
            }
            Value::SmallPhysical(s) => (
                Format::SmallPhysical,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { smallphys: s },
            ),
            Value::SmallPhysicalVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiSmallPhysT> = vec.clone();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::SmallPhys(buffer));
                (
                    Format::SmallPhysicalVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { smallphyss: ptr },
                )
            }
            Value::Physical(p) => (
                Format::Physical,
                vhpi_sys::vhpiValueS__bindgen_ty_1 { phys: p.into() },
            ),
            Value::PhysicalVec(vec) => {
                let mut buffer: Vec<vhpi_sys::vhpiPhysT> =
                    vec.iter().map(|val| val.clone().into()).collect();
                let ptr = buffer.as_mut_ptr();
                buffer_holder = Some(VectorBox::Phys(buffer));
                (
                    Format::PhysicalVec,
                    vhpi_sys::vhpiValueS__bindgen_ty_1 { physs: ptr },
                )
            }
            Value::Unknown => return Err("Cannot put unknown value".into()),
        };

        let mut val_struct = vhpi_sys::vhpiValueT {
            format: format.into(),
            bufSize: 0,
            numElems: 0,
            unit: vhpi_sys::vhpiPhysS { high: 0, low: 0 },
            value: val,
        };

        if let Some(buffer) = buffer_holder.as_ref() {
            val_struct.bufSize = buffer.byte_len();
            val_struct.numElems = buffer
                .len()
                .try_into()
                .expect("vector element count does not fit into vhpi element count type");
        }

        let rc =
            unsafe { vhpi_sys::vhpi_put_value(self.as_raw(), &raw mut val_struct, mode.into()) };

        // Keep buffer_holder alive until after vhpi_put_value
        let _ = &buffer_holder;

        if rc.is_zero() {
            Ok(())
        } else {
            Err(crate::check_error().unwrap_or_else(|| "Unknown error in vhpi_put_value".into()))
        }
    }
}

/// Convert a string to a `Value::LogicVec` by mapping each character to a `LogicVal` using the character's byte value.
#[must_use]
pub fn string_to_logic_vec(s: &str) -> Value {
    Value::LogicVec(
        s.chars()
            .map(|c| LogicVal::try_from(c).unwrap_or(LogicVal::Unknown(c as u8)))
            .collect(),
    )
}

#[must_use]
/// Convert an unsigned integer to a `Value::LogicVec` of the specified width,
/// where each bit of the integer is mapped to a `LogicVal`.
/// If the integer cannot fit into the specified width, it will be truncated.
pub fn uint_to_logic_vec(mut value: u64, width: usize) -> Value {
    assert!(width <= 64, "Width must be 64 or less to fit into u64");
    let mut logic_vec = Vec::with_capacity(width);
    for _ in 0..width {
        let bit = (value & 1) as u8;
        logic_vec.push(if bit.is_zero() {
            LogicVal::Zero
        } else {
            LogicVal::One
        });
        value >>= 1;
    }
    logic_vec.reverse();
    Value::LogicVec(logic_vec)
}

#[must_use]
/// Convert a signed integer to a `Value::LogicVec` of the specified width,
/// where each bit of the integer is mapped to a `LogicVal`.
/// If the integer cannot fit into the specified width, it will be truncated.
pub fn int_to_logic_vec(mut value: i64, width: usize) -> Value {
    assert!(width <= 64, "Width must be 64 or less to fit into i64");
    let mut logic_vec = Vec::with_capacity(width);
    for _ in 0..width {
        let bit = (value & 1) as u8;
        logic_vec.push(if bit.is_zero() {
            LogicVal::Zero
        } else {
            LogicVal::One
        });
        value >>= 1;
    }
    logic_vec.reverse();
    Value::LogicVec(logic_vec)
}

#[must_use]
/// Convert a `LogicVec` to an unsigned integer by interpreting the vector as a binary number,
/// where `LogicVal::Zero` represents 0 and `LogicVal::One` represents 1.
/// If any value in the vector is not `Zero` or `One`, return `None`.
pub fn logic_vec_to_uint(logic_vec: &[LogicVal]) -> Option<u64> {
    let mut value = 0u64;
    for &logic_val in logic_vec {
        value <<= 1;
        match logic_val {
            LogicVal::Zero => {}
            LogicVal::One => value |= 1,
            _ => return None, // If any value is not 0 or 1, return None
        }
    }
    Some(value)
}

#[must_use]
/// Convert a `LogicVec` to a signed integer by interpreting the vector as a binary number,
/// where `LogicVal::Zero` represents 0 and `LogicVal::One` represents 1.
/// If any value in the vector is not `Zero` or `One`, return `None`.
pub fn logic_vec_to_int(logic_vec: &[LogicVal]) -> Option<i64> {
    if logic_vec.len() > 64 {
        return None;
    }

    let mut value = 0i64;
    for &logic_val in logic_vec {
        value <<= 1;
        match logic_val {
            LogicVal::Zero => {}
            LogicVal::One => value |= 1,
            _ => return None, // If any value is not 0 or 1, return None
        }
    }

    if matches!(logic_vec.first(), Some(LogicVal::One)) {
        let width = logic_vec.len();
        if width < 64 {
            value |= !0i64 << width;
        }
    }

    Some(value)
}

#[cfg(feature = "bigint")]
#[must_use]
/// Convert a `LogicVec` to a `BigInt` by interpreting the vector as a binary number,
/// where `LogicVal::Zero` represents 0 and `LogicVal::One` represents 1.
/// If any value in the vector is not `Zero` or `One`, return `None`.
pub fn logic_vec_to_bigint(logic_vec: &[LogicVal]) -> Option<BigInt> {
    let mut value = BigInt::ZERO;
    let one = BigInt::one();
    for &logic_val in logic_vec {
        value <<= 1;
        match logic_val {
            LogicVal::Zero => {}
            LogicVal::One => value |= &one,
            _ => return None, // If any value is not 0 or 1, return None
        }
    }

    if matches!(logic_vec.first(), Some(LogicVal::One)) && !logic_vec.is_empty() {
        let sign_base = &one << logic_vec.len();
        value -= sign_base;
    }

    Some(value)
}

#[cfg(feature = "bigint")]
#[must_use]
/// Convert a `LogicVec` to a `BigUint` by interpreting the vector as a binary number,
/// where `LogicVal::Zero` represents 0 and `LogicVal::One` represents 1.
/// If any value in the vector is not `Zero` or `One`, return `None`.
pub fn logic_vec_to_biguint(logic_vec: &[LogicVal]) -> Option<BigUint> {
    let mut value = BigUint::ZERO;
    let one = BigUint::one();
    for &logic_val in logic_vec {
        value <<= 1;
        match logic_val {
            LogicVal::Zero => {}
            LogicVal::One => value |= &one,
            _ => return None, // If any value is not 0 or 1, return None
        }
    }
    Some(value)
}

#[cfg(feature = "bigint")]
#[must_use]
/// Convert a `BigInt` to a `Value::LogicVec` of the specified width, where each bit of the integer is mapped to a `LogicVal`.
/// The most significant bit of the integer corresponds to the first element of the vector.
/// If the integer cannot fit into the specified width, it will be truncated.
pub fn bigint_to_logic_vec(value: &BigInt, width: usize) -> Value {
    let mut logic_vec = Vec::with_capacity(width);
    let mut temp = value.clone();
    let one = BigInt::one();
    for _ in 0..width {
        let bit = &temp & &one;
        logic_vec.push(if bit.is_zero() {
            LogicVal::Zero
        } else {
            LogicVal::One
        });
        temp >>= 1;
    }
    logic_vec.reverse();
    Value::LogicVec(logic_vec)
}

#[cfg(feature = "bigint")]
#[must_use]
/// Convert a `BigUint` to a `Value::LogicVec` of the specified width, where each bit of the integer is mapped to a `LogicVal`.
/// The most significant bit of the integer corresponds to the first element of the vector.
/// If the integer cannot fit into the specified width, it will be truncated.
pub fn biguint_to_logic_vec(value: &BigUint, width: usize) -> Value {
    let mut logic_vec = Vec::with_capacity(width);
    let mut temp = value.clone();
    let one = BigUint::one();
    for _ in 0..width {
        let bit = &temp & &one;
        logic_vec.push(if bit.is_zero() {
            LogicVal::Zero
        } else {
            LogicVal::One
        });
        temp >>= 1;
    }
    logic_vec.reverse();
    Value::LogicVec(logic_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_logic_capital_letters() {
        let input = "UX01ZWLH-";

        let parsed = string_to_logic_vec(input);
        assert_eq!(
            parsed,
            Value::LogicVec(vec![
                LogicVal::U,
                LogicVal::X,
                LogicVal::Zero,
                LogicVal::One,
                LogicVal::Z,
                LogicVal::W,
                LogicVal::L,
                LogicVal::H,
                LogicVal::DontCare,
            ])
        );
    }

    #[test]
    fn string_to_logic_small_letters() {
        let input = "ux01zwlh-";

        let parsed = string_to_logic_vec(input);
        assert_eq!(
            parsed,
            Value::LogicVec(vec![
                LogicVal::U,
                LogicVal::X,
                LogicVal::Zero,
                LogicVal::One,
                LogicVal::Z,
                LogicVal::W,
                LogicVal::L,
                LogicVal::H,
                LogicVal::DontCare,
            ])
        );
    }

    #[test]
    fn string_to_logic_vec_returns_empty_for_empty_string() {
        assert_eq!(string_to_logic_vec(""), Value::LogicVec(vec![]));
    }

    #[test]
    fn string_to_logic_vec_maps_unknown_chars_to_unknown_logic_values() {
        assert_eq!(
            string_to_logic_vec("A"),
            Value::LogicVec(vec![LogicVal::Unknown(b'A')])
        );
    }

    #[test]
    fn uint_to_logic_vec_converts_correctly() {
        assert_eq!(
            uint_to_logic_vec(0b101, 3),
            Value::LogicVec(vec![LogicVal::One, LogicVal::Zero, LogicVal::One])
        );
    }

    #[test]
    fn logic_vec_to_uint_converts_correctly() {
        assert_eq!(
            logic_vec_to_uint(&[LogicVal::One, LogicVal::Zero, LogicVal::One]),
            Some(5)
        );
        assert_eq!(
            logic_vec_to_uint(&[LogicVal::One, LogicVal::Zero, LogicVal::Unknown(0)]),
            None
        );
    }

    #[test]
    fn logic_vec_to_int_converts_correctly() {
        assert_eq!(
            logic_vec_to_int(&[LogicVal::One, LogicVal::Zero, LogicVal::One]),
            Some(-3)
        );
        assert_eq!(
            logic_vec_to_int(&[LogicVal::One, LogicVal::Zero, LogicVal::Zero]),
            Some(-4)
        );
        assert_eq!(
            logic_vec_to_int(&[LogicVal::Zero, LogicVal::Zero, LogicVal::One]),
            Some(1)
        );
        assert_eq!(
            logic_vec_to_int(&[LogicVal::One, LogicVal::Zero, LogicVal::Unknown(0)]),
            None
        );
    }

    #[test]
    fn logic_vec_to_int_rejects_width_over_64_bits() {
        let bits = vec![LogicVal::Zero; 65];
        assert_eq!(logic_vec_to_int(&bits), None);
    }

    #[test]
    fn int_to_logic_vec_converts_correctly() {
        assert_eq!(
            int_to_logic_vec(-3, 3),
            Value::LogicVec(vec![LogicVal::One, LogicVal::Zero, LogicVal::One])
        );
        assert_eq!(
            int_to_logic_vec(-4, 3),
            Value::LogicVec(vec![LogicVal::One, LogicVal::Zero, LogicVal::Zero])
        );
        assert_eq!(
            int_to_logic_vec(1, 3),
            Value::LogicVec(vec![LogicVal::Zero, LogicVal::Zero, LogicVal::One])
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_to_bigint_converts_correctly() {
        assert_eq!(
            logic_vec_to_bigint(&[LogicVal::One, LogicVal::Zero, LogicVal::One]),
            Some(num_bigint::BigInt::from(-3i8))
        );
        assert_eq!(
            logic_vec_to_bigint(&[LogicVal::Zero, LogicVal::Zero, LogicVal::One]),
            Some(num_bigint::BigInt::from(1u8))
        );
        assert_eq!(
            logic_vec_to_bigint(&[LogicVal::One, LogicVal::Zero, LogicVal::Zero]),
            Some(num_bigint::BigInt::from(-4i8))
        );
        assert_eq!(
            logic_vec_to_bigint(&[]),
            Some(num_bigint::BigInt::from(0u8))
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_to_bigint_returns_none_for_non_binary_values() {
        assert_eq!(
            logic_vec_to_bigint(&[LogicVal::One, LogicVal::Unknown(0), LogicVal::Zero]),
            None
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn logic_vec_to_biguint_converts_correctly() {
        assert_eq!(
            logic_vec_to_biguint(&[LogicVal::One, LogicVal::Zero, LogicVal::One]),
            Some(num_bigint::BigUint::from(5u8))
        );
        assert_eq!(
            logic_vec_to_biguint(&[]),
            Some(num_bigint::BigUint::from(0u8))
        );
        assert_eq!(
            logic_vec_to_biguint(&[LogicVal::One, LogicVal::Unknown(0), LogicVal::Zero]),
            None
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn bigint_to_logic_vec_converts_signed_values() {
        assert_eq!(
            bigint_to_logic_vec(&num_bigint::BigInt::from(-3i8), 3),
            Value::LogicVec(vec![LogicVal::One, LogicVal::Zero, LogicVal::One])
        );
        assert_eq!(
            bigint_to_logic_vec(&num_bigint::BigInt::from(2u8), 3),
            Value::LogicVec(vec![LogicVal::Zero, LogicVal::One, LogicVal::Zero])
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn biguint_to_logic_vec_converts_values() {
        assert_eq!(
            biguint_to_logic_vec(&num_bigint::BigUint::from(5u8), 3),
            Value::LogicVec(vec![LogicVal::One, LogicVal::Zero, LogicVal::One])
        );
        assert_eq!(
            biguint_to_logic_vec(&num_bigint::BigUint::from(0u8), 4),
            Value::LogicVec(vec![
                LogicVal::Zero,
                LogicVal::Zero,
                LogicVal::Zero,
                LogicVal::Zero,
            ])
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn bigint_and_biguint_round_trip_with_logic_vec() {
        let signed_bits = vec![LogicVal::One, LogicVal::Zero, LogicVal::One, LogicVal::One];
        let signed_value = logic_vec_to_bigint(&signed_bits).unwrap();
        assert_eq!(
            bigint_to_logic_vec(&signed_value, signed_bits.len()),
            Value::LogicVec(signed_bits)
        );

        let unsigned_bits = vec![LogicVal::One, LogicVal::Zero, LogicVal::One, LogicVal::One];
        let unsigned_value = logic_vec_to_biguint(&unsigned_bits).unwrap();
        assert_eq!(
            biguint_to_logic_vec(&unsigned_value, unsigned_bits.len()),
            Value::LogicVec(unsigned_bits)
        );
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn biguint_large_round_trip_with_logic_vec() {
        use num_bigint::BigUint;
        use num_traits::Num;

        let original =
            BigUint::from_str_radix("1234567890abcdef1234567890abcdef1234567890abcdef", 16)
                .unwrap();
        let width = 192;

        let as_logic = biguint_to_logic_vec(&original, width);
        let round_trip = match as_logic {
            Value::LogicVec(bits) => logic_vec_to_biguint(&bits),
            _ => None,
        };

        assert_eq!(round_trip, Some(original));
    }

    #[cfg(feature = "bigint")]
    #[test]
    fn bigint_large_round_trip_with_logic_vec() {
        use num_bigint::BigInt;
        use num_traits::Num;

        let magnitude =
            BigInt::from_str_radix("1234567890abcdef1234567890abcdef1234567890abcdef", 16).unwrap();
        let original = -magnitude;
        let width = 193;

        let as_logic = bigint_to_logic_vec(&original, width);
        let round_trip = match as_logic {
            Value::LogicVec(bits) => logic_vec_to_bigint(&bits),
            _ => None,
        };

        assert_eq!(round_trip, Some(original));
    }
}
