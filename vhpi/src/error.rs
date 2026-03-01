use std::ffi::CStr;
use std::fmt;

use num_traits::Zero;

use crate::string_to_iso8859_1_cstring;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Note,
    Warning,
    Error,
    System,
    Internal,
    Failure,
    Unknown(u32),
}

impl From<vhpi_sys::vhpiSeverityT> for Severity {
    fn from(raw: vhpi_sys::vhpiSeverityT) -> Self {
        match raw {
            vhpi_sys::vhpiSeverityT_vhpiNote => Severity::Note,
            vhpi_sys::vhpiSeverityT_vhpiWarning => Severity::Warning,
            vhpi_sys::vhpiSeverityT_vhpiError => Severity::Error,
            vhpi_sys::vhpiSeverityT_vhpiSystem => Severity::System,
            vhpi_sys::vhpiSeverityT_vhpiInternal => Severity::Internal,
            vhpi_sys::vhpiSeverityT_vhpiFailure => Severity::Failure,
            other => Severity::Unknown(other),
        }
    }
}

impl From<Severity> for vhpi_sys::vhpiSeverityT {
    fn from(sev: Severity) -> Self {
        match sev {
            Severity::Note => vhpi_sys::vhpiSeverityT_vhpiNote,
            Severity::Warning => vhpi_sys::vhpiSeverityT_vhpiWarning,
            Severity::Error => vhpi_sys::vhpiSeverityT_vhpiError,
            Severity::System => vhpi_sys::vhpiSeverityT_vhpiSystem,
            Severity::Internal => vhpi_sys::vhpiSeverityT_vhpiInternal,
            Severity::Failure => vhpi_sys::vhpiSeverityT_vhpiFailure,
            Severity::Unknown(n) => n,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Note => write!(f, "Note"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
            Severity::System => write!(f, "System"),
            Severity::Internal => write!(f, "Internal"),
            Severity::Failure => write!(f, "Failure"),
            Severity::Unknown(n) => write!(f, "Unknown({n})"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub severity: Severity,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<i32>,
    pub context: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.severity, self.message)?;
        if let Some(file) = &self.file {
            write!(f, " at {}:{}", file, self.line.unwrap_or(0))?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error {
            severity: Severity::Note,
            message: msg.to_string(),
            file: None,
            line: None,
            context: None,
        }
    }
}

#[must_use]
pub fn check_error() -> Option<Error> {
    let mut info = vhpi_sys::vhpiErrorInfoS {
        severity: vhpi_sys::vhpiSeverityT_vhpiNote,
        message: std::ptr::null_mut(),
        str_: std::ptr::null_mut(),
        file: std::ptr::null_mut(),
        line: -1,
    };

    let rc = unsafe { vhpi_sys::vhpi_check_error(&raw mut info) };
    if rc.is_zero() {
        return None;
    }

    let message = unsafe { CStr::from_ptr(info.message) }
        .to_string_lossy()
        .into_owned();
    let context = if info.str_.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(info.str_) }
                .to_string_lossy()
                .into_owned(),
        )
    };
    let file = if info.file.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(info.file) }
                .to_string_lossy()
                .into_owned(),
        )
    };

    Some(Error {
        severity: Severity::from(info.severity),
        message,
        file,
        line: Some(info.line),
        context,
    })
}

pub fn assert(severity: Severity, message: impl AsRef<str>) {
    let c_message = string_to_iso8859_1_cstring(message);
    unsafe { vhpi_sys::vhpi_assert(severity.into(), c_message.as_ptr().cast_mut()) };
}

#[macro_export]
/// Assert a condition with a severity and print a message through the simulator if the assertion fails
macro_rules! assert {
    ($cond:expr, $severity:expr, $($arg:tt)*) => {{
        if !($cond) {
            $crate::assert($severity, &format!($($arg)*));
        }
    }}
}
