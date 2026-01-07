use std::fmt;
use std::ffi::CStr;

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
            1 => Severity::Note,
            2 => Severity::Warning,
            3 => Severity::Error,
            4 => Severity::System,
            5 => Severity::Internal,
            6 => Severity::Failure,
            other => Severity::Unknown(other),
        }
    }
}

impl From<Severity> for vhpi_sys::vhpiSeverityT {
    fn from(sev: Severity) -> Self {
        match sev {
            Severity::Note => 1,
            Severity::Warning => 2,
            Severity::Error => 3,
            Severity::System => 4,
            Severity::Internal => 5,
            Severity::Failure => 6,
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
            Severity::Unknown(n) => write!(f, "Unknown({})", n),
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

pub fn check_error() -> Option<Error> {
    let mut info = vhpi_sys::vhpiErrorInfoS {
        severity: vhpi_sys::vhpiSeverityT_vhpiNote,
        message: std::ptr::null_mut(),
        str_: std::ptr::null_mut(),
        file: std::ptr::null_mut(),
        line: -1,
    };

    let rc = unsafe { vhpi_sys::vhpi_check_error(&mut info as *mut _) };
    if rc == 0 {
        return None;
    }

    let message = unsafe { CStr::from_ptr(info.message) }.to_string_lossy().into_owned();
    let context = if !info.str_.is_null() {
        Some(unsafe { CStr::from_ptr(info.str_) }.to_string_lossy().into_owned())
    } else {
        None
    };
    let file = if !info.file.is_null() {
        Some(unsafe { CStr::from_ptr(info.file) }.to_string_lossy().into_owned())
    } else {
        None
    };

    Some(Error {
        severity: Severity::from(info.severity),
        message,
        file,
        line: Some(info.line),
        context,
    })
}
