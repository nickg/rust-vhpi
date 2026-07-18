//! Registration of VHPI foreign models via [`register_foreignf`].
//!
//! A *foreign model* lets a VHDL design call out to a compiled Rust (or C)
//! function at simulation time.  The VHDL side marks the subprogram with the
//! `foreign` attribute; the Rust side registers a matching record during the
//! startup callback using [`register_foreignf`].
//!
//! # Kinds
//!
//! | [`ForeignKind`] | VHDL construct                                  |
//! |-----------------|-------------------------------------------------|
//! | `Func`          | `function` — reads parameters, writes result    |
//! | `Proc`          | `procedure` — reads/writes parameters           |
//! | `Arch`          | architecture body replacement                   |
//! | `Lib`           | library initialisation                          |
//! | `App`           | application initialisation                      |
//!
//! # Example: function that adds 1 to an integer
//!
//! ## VHDL
//!
//! ```vhdl
//! package math_foreign is
//!     -- Declare the function normally …
//!     function increment(x : integer) return integer;
//!
//!     -- … then attach the VHPI foreign attribute.
//!     -- Format: "VHPI <library_name> <model_name>"
//!     attribute foreign of increment : function is "VHPI my_lib increment";
//! end package;
//! ```
//!
//! ## Rust
//!
//! ```rust,no_run
//! use std::mem::ManuallyDrop;
//! use vhpi::{
//!     register_foreignf, ForeignCallback, ForeignData, ForeignExecData, ForeignKind,
//!     Format, Handle, OneToOne, PutValueMode, Value,
//! };
//!
//! /// Called by the simulator when `increment` is invoked in VHDL.
//! ///
//! /// # Safety
//! /// This is an `extern "C"` callback driven by the simulator; `cb_data`
//! /// is guaranteed non-null by the VHPI specification.
//! unsafe extern "C" fn increment_exec(call_data: &ForeignExecData) {
//!     let func_handle = call_data.obj();
//!
//!     // The first formal parameter.
//!     let param = func_handle.handle(OneToOne::ParamDecl);
//!     let Value::Int(x) = param
//!         .get_value(Format::Int)
//!         .expect("failed to read parameter")
//!     else {
//!         return;
//!     };
//!
//!     // Write the return value back through the function handle itself.
//!     func_handle
//!         .put_value(Value::Int(x + 1), PutValueMode::Deposit)
//!         .expect("failed to write return value");
//! }
//!
//! #[no_mangle]
//! pub extern "C" fn vhpi_startup() {
//!     register_foreignf(
//!         &ForeignData::new(ForeignKind::Func, "my_lib", "increment")
//!             .exec(increment_exec),
//!     )
//!     .expect("failed to register foreign function `increment`");
//! }
//! ```

use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;

use crate::{check_error, Error, Format, Handle, LogicVal, OneToMany, Value};

/// The kind of VHPI foreign model, corresponding to `vhpiForeignKindT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForeignKind {
    /// Architecture body replacement.
    Arch,
    /// Foreign function (reads parameters, produces a return value).
    Func,
    /// Foreign procedure (reads and/or writes parameters).
    Proc,
    /// Library initialisation model.
    Lib,
    /// Application initialisation model.
    App,
}

impl From<ForeignKind> for vhpi_sys::vhpiForeignKindT {
    fn from(kind: ForeignKind) -> Self {
        match kind {
            ForeignKind::Arch => vhpi_sys::vhpiForeignKindT_vhpiArchF,
            ForeignKind::Func => vhpi_sys::vhpiForeignKindT_vhpiFuncF,
            ForeignKind::Proc => vhpi_sys::vhpiForeignKindT_vhpiProcF,
            ForeignKind::Lib => vhpi_sys::vhpiForeignKindT_vhpiLibF,
            ForeignKind::App => vhpi_sys::vhpiForeignKindT_vhpiAppF,
        }
    }
}

impl TryFrom<vhpi_sys::vhpiForeignKindT> for ForeignKind {
    type Error = vhpi_sys::vhpiForeignKindT;

    fn try_from(raw: vhpi_sys::vhpiForeignKindT) -> Result<Self, Self::Error> {
        match raw {
            vhpi_sys::vhpiForeignKindT_vhpiArchF => Ok(ForeignKind::Arch),
            vhpi_sys::vhpiForeignKindT_vhpiFuncF => Ok(ForeignKind::Func),
            vhpi_sys::vhpiForeignKindT_vhpiProcF => Ok(ForeignKind::Proc),
            vhpi_sys::vhpiForeignKindT_vhpiLibF => Ok(ForeignKind::Lib),
            vhpi_sys::vhpiForeignKindT_vhpiAppF => Ok(ForeignKind::App),
            other => Err(other),
        }
    }
}

/// Data passed to foreign model elaboration and execution callbacks.
#[repr(transparent)]
pub struct ForeignExecData(vhpi_sys::vhpiCbDataS);

impl ForeignExecData {
    /// Return a non-owning reference to the foreign model instance handle.
    ///
    /// For `Func` and `Proc` models this is the subprogram handle through
    /// which parameter values are read and the return / output values are
    /// written.
    #[must_use]
    pub fn obj(&self) -> ManuallyDrop<Handle> {
        ManuallyDrop::new(Handle::from_raw(self.0.obj))
    }

    /// Raw callback reason code as provided by the simulator.
    #[must_use]
    pub fn reason(&self) -> i32 {
        self.0.reason
    }

    /// Read a foreign subprogram argument by index using the requested format.
    ///
    /// Returns `None` when the index is out of range, does not fit into VHPI's
    /// signed index type, or the simulator rejects value retrieval.
    #[must_use]
    pub fn get_foreignf_arg(&self, index: u32, format: Format) -> Option<Value> {
        let index: i32 = index.try_into().ok()?;
        let handle = self.obj().handle_by_index(OneToMany::ParamDecls, index)?;
        handle.get_value(format).ok()
    }

    /// Read multiple foreign subprogram arguments using per-argument formats.
    ///
    /// For each entry in `formats`, retrieves the argument at the same index.
    /// Returns one `Option<Value>` per requested format.
    #[must_use]
    pub fn get_foreignf_args(&self, formats: impl AsRef<[Format]>) -> Vec<Option<Value>> {
        formats
            .as_ref()
            .iter()
            .enumerate()
            .map(|(index, format)| self.get_foreignf_arg(index as u32, *format))
            .collect()
    }

    /// Return `true` when the simulator provided a writable return-value buffer.
    ///
    /// This is most commonly available for foreign functions. For procedures,
    /// simulators typically leave this as null.
    #[must_use]
    pub fn has_return_value_buffer(&self) -> bool {
        !self.0.value.is_null()
    }

    /// Return the simulator-provided return-value format, if available.
    #[must_use]
    pub fn return_value_format(&self) -> Option<Format> {
        let raw = self.0.value;
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Format::from((*raw).format) })
        }
    }

    /// Return the simulator-provided return-value element count, if available.
    ///
    /// Scalars usually report `0`. Vector return buffers report the element count.
    #[must_use]
    pub fn return_value_num_elems(&self) -> Option<i32> {
        let raw = self.0.value;
        if raw.is_null() {
            None
        } else {
            Some(unsafe { (*raw).numElems })
        }
    }

    /// Try to write a return value directly into the simulator-provided value buffer.
    ///
    /// This path is simulator-dependent. Some simulators do not provide a writable
    /// buffer for certain return types (for example unconstrained vectors).
    /// Callers should be prepared to fall back to other mechanisms.
    ///
    /// Supported value kinds currently are:
    /// - `Value::Int`
    /// - `Value::LongInt`
    /// - `Value::Real`
    /// - `Value::Char`
    /// - `Value::Logic`
    /// - `Value::LogicVec`
    ///
    /// # Errors
    ///
    /// Returns an error when the simulator does not provide a value buffer, the
    /// format/size is incompatible with `value`, or the value kind is unsupported.
    pub fn try_put_return_value(&self, value: Value) -> Result<(), Error> {
        let raw = self.0.value;
        if raw.is_null() {
            return Err("foreignf: simulator did not provide return-value buffer".into());
        }

        // SAFETY: `raw` is checked non-null above and points to the callback-owned
        // vhpiValueT for the duration of this callback.
        let raw = unsafe { &mut *raw };
        let format = Format::from(raw.format);

        match value {
            Value::Int(v) => {
                if !matches!(format, Format::Int | Format::ObjType) {
                    return Err(format!(
                        "foreignf: return-value format mismatch for Int: got {format:?}"
                    )
                    .as_str()
                    .into());
                }
                raw.value.intg = v;
                Ok(())
            }
            Value::LongInt(v) => {
                if !matches!(format, Format::LongInt | Format::ObjType) {
                    return Err(format!(
                        "foreignf: return-value format mismatch for LongInt: got {format:?}"
                    )
                    .as_str()
                    .into());
                }
                raw.value.longintg = v;
                Ok(())
            }
            Value::Real(v) => {
                if !matches!(format, Format::Real | Format::ObjType) {
                    return Err(format!(
                        "foreignf: return-value format mismatch for Real: got {format:?}"
                    )
                    .as_str()
                    .into());
                }
                raw.value.real = v;
                Ok(())
            }
            Value::Char(v) => {
                if !matches!(format, Format::Char | Format::ObjType) {
                    return Err(format!(
                        "foreignf: return-value format mismatch for Char: got {format:?}"
                    )
                    .as_str()
                    .into());
                }
                raw.value.ch = v as u8;
                Ok(())
            }
            Value::Logic(v) => {
                if !matches!(format, Format::Logic | Format::ObjType) {
                    return Err(format!(
                        "foreignf: return-value format mismatch for Logic: got {format:?}"
                    )
                    .as_str()
                    .into());
                }
                raw.value.enumv = v.into();
                Ok(())
            }
            Value::LogicVec(values) => {
                if !matches!(format, Format::LogicVec | Format::ObjType) {
                    return Err(format!(
                        "foreignf: return-value format mismatch for LogicVec: got {format:?}"
                    )
                    .as_str()
                    .into());
                }

                if raw.numElems < 0 {
                    return Err("foreignf: simulator reported negative return vector length".into());
                }

                let expected_len = raw.numElems as usize;
                if expected_len != values.len() {
                    return Err(format!(
                        "foreignf: return vector length mismatch: simulator expects {}, got {}",
                        expected_len,
                        values.len()
                    )
                    .as_str()
                    .into());
                }

                // SAFETY: Accessing a union field is unsafe; for LogicVec we
                // expect `enumvs` to be the active field.
                let out_ptr = unsafe { raw.value.enumvs };
                if out_ptr.is_null() {
                    return Err(
                        "foreignf: simulator provided null return vector buffer pointer".into(),
                    );
                }

                // SAFETY: `enumvs` points to a simulator-owned buffer with at least
                // `numElems` elements for this callback.
                let out = unsafe { std::slice::from_raw_parts_mut(out_ptr, expected_len) };
                for (dst, src) in out.iter_mut().zip(values.iter()) {
                    *dst = <vhpi_sys::vhpiEnumT as From<LogicVal>>::from(*src);
                }
                Ok(())
            }
            other => Err(format!(
                "foreignf: unsupported return-value kind for direct buffer write: {other:?}"
            )
            .as_str()
            .into()),
        }
    }
}

/// `extern "C"` function pointer type for foreign model elaboration and
/// execution callbacks.
///
/// Users write `unsafe extern "C" fn my_fn(data: &ForeignExecData)`.
pub type ForeignCallback = unsafe extern "C" fn(&ForeignExecData);

/// Builder for a VHPI foreign-model registration record.
///
/// Construct with [`ForeignData::new`], attach optional callbacks with
/// [`elab`](ForeignData::elab) and [`exec`](ForeignData::exec), then pass a
/// reference to [`register_foreignf`].
#[derive(Debug, Clone)]
pub struct ForeignData {
    kind: ForeignKind,
    library_name: CString,
    model_name: CString,
    elab: Option<ForeignCallback>,
    exec: Option<ForeignCallback>,
}

impl ForeignData {
    /// Create a new registration record.
    ///
    /// `library_name` and `model_name` must match the values declared in the
    /// VHDL `foreign` attribute exactly.  Any interior NUL bytes are stripped.
    #[must_use]
    pub fn new(kind: ForeignKind, library_name: &str, model_name: &str) -> Self {
        Self {
            kind,
            library_name: CString::new(library_name).unwrap_or_default(),
            model_name: CString::new(model_name).unwrap_or_default(),
            elab: None,
            exec: None,
        }
    }

    /// Set the elaboration callback, called once when the design is elaborated.
    #[must_use]
    pub fn elab(mut self, f: ForeignCallback) -> Self {
        self.elab = Some(f);
        self
    }

    /// Set the execution callback, called each time the foreign model is
    /// invoked during simulation.
    #[must_use]
    pub fn exec(mut self, f: ForeignCallback) -> Self {
        self.exec = Some(f);
        self
    }
}

/// Information about a registered foreign model returned by
/// [`get_foreignf_info`] and [`Handle::get_foreignf_info`].
#[derive(Debug, Clone)]
pub struct ForeignInfo {
    /// The kind of foreign model.
    pub kind: ForeignKind,
    /// Library name as registered with the simulator, if available.
    pub library_name: Option<String>,
    /// Model name as registered with the simulator, if available.
    pub model_name: Option<String>,
    /// Elaboration callback, if one was registered.
    pub elab: Option<ForeignCallback>,
    /// Execution callback, if one was registered.
    pub exec: Option<ForeignCallback>,
}

/// Register a VHPI foreign model with the simulator.
///
/// Call this from the startup routine (the function registered with
/// [`startup_routines!`](crate::startup_routines)) before simulation begins.
/// The returned [`Handle`] can be used to query the registered record later.
///
/// # Errors
///
/// Returns an [`Error`] if the simulator rejects the registration.
pub fn register_foreignf(data: &ForeignData) -> Result<Handle, Error> {
    let mut raw = vhpi_sys::vhpiForeignDataT {
        kind: data.kind.into(),
        libraryName: data.library_name.as_ptr().cast_mut(),
        modelName: data.model_name.as_ptr().cast_mut(),
        // SAFETY: ForeignExecData is #[repr(transparent)] over vhpiCbDataS,
        // so fn(&ForeignExecData) and fn(*const vhpiCbDataS) are ABI-identical.
        elabf: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C" fn(&ForeignExecData)>,
                Option<unsafe extern "C" fn(*const vhpi_sys::vhpiCbDataS)>,
            >(data.elab)
        },
        execf: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C" fn(&ForeignExecData)>,
                Option<unsafe extern "C" fn(*const vhpi_sys::vhpiCbDataS)>,
            >(data.exec)
        },
    };

    let handle = unsafe { vhpi_sys::vhpi_register_foreignf(&raw mut raw) };

    if handle.is_null() {
        Err(check_error().unwrap_or_else(|| "vhpi_register_foreignf failed".into()))
    } else {
        Ok(Handle::from_raw(handle))
    }
}

/// Retrieve the registration record for a foreign model handle.
///
/// # Errors
///
/// Returns an [`Error`] if the simulator reports a failure or returns an
/// unrecognised foreign-kind discriminant.
pub fn get_foreignf_info(handle: &Handle) -> Result<ForeignInfo, Error> {
    let mut raw: vhpi_sys::vhpiForeignDataT = unsafe { std::mem::zeroed() };
    let rc = unsafe { vhpi_sys::vhpi_get_foreignf_info(handle.as_raw(), &raw mut raw) };
    if rc != 0 {
        return Err(check_error().unwrap_or_else(|| "vhpi_get_foreignf_info failed".into()));
    }

    let kind = ForeignKind::try_from(raw.kind)
        .map_err(|v| Error::from(format!("unknown vhpiForeignKindT value: {v}").as_str()))?;

    let library_name = unsafe {
        raw.libraryName
            .as_ref()
            .map(|p| CStr::from_ptr(p).to_string_lossy().into_owned())
    };
    let model_name = unsafe {
        raw.modelName
            .as_ref()
            .map(|p| CStr::from_ptr(p).to_string_lossy().into_owned())
    };

    Ok(ForeignInfo {
        kind,
        library_name,
        model_name,
        // SAFETY: ForeignExecData is #[repr(transparent)] over vhpiCbDataS,
        // so fn(*const vhpiCbDataS) and fn(&ForeignExecData) are ABI-identical.
        elab: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C" fn(*const vhpi_sys::vhpiCbDataS)>,
                Option<unsafe extern "C" fn(&ForeignExecData)>,
            >(raw.elabf)
        },
        exec: unsafe {
            std::mem::transmute::<
                Option<unsafe extern "C" fn(*const vhpi_sys::vhpiCbDataS)>,
                Option<unsafe extern "C" fn(&ForeignExecData)>,
            >(raw.execf)
        },
    })
}

impl Handle {
    /// Retrieve the registration record for this foreign model handle.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the simulator reports a failure.
    pub fn get_foreignf_info(&self) -> Result<ForeignInfo, Error> {
        get_foreignf_info(self)
    }
}
