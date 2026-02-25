use crate::iso8859_1_val_to_string;
use crate::string_to_iso8859_1_cstring;
use crate::Error;
use crate::Handle;
use crate::LogicVal;
use crate::Physical;
use crate::Time;

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

        let rc =
            unsafe { vhpi_sys::vhpi_put_value(self.as_raw(), &raw mut val_struct, mode.into()) };

        // Keep buffer_holder alive until after vhpi_put_value
        let _ = &buffer_holder;

        if rc < 0 {
            return Err(
                crate::check_error().unwrap_or_else(|| "Unknown error in vhpi_put_value".into())
            );
        }

        Ok(())
    }
}
