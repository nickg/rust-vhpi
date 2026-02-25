use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicVal {
    U,
    X,
    Zero,
    One,
    Z,
    W,
    L,
    H,
    DontCare,
    Unknown(u8),
}

impl From<u8> for LogicVal {
    fn from(val: u8) -> Self {
        match u32::from(val) {
            vhpi_sys::vhpiU => LogicVal::U,
            vhpi_sys::vhpiX => LogicVal::X,
            vhpi_sys::vhpi0 => LogicVal::Zero,
            vhpi_sys::vhpi1 => LogicVal::One,
            vhpi_sys::vhpiZ => LogicVal::Z,
            vhpi_sys::vhpiW => LogicVal::W,
            vhpi_sys::vhpiL => LogicVal::L,
            vhpi_sys::vhpiH => LogicVal::H,
            vhpi_sys::vhpiDontCare => LogicVal::DontCare,
            _ => LogicVal::Unknown(val),
        }
    }
}

impl From<LogicVal> for vhpi_sys::vhpiEnumT {
    fn from(logic: LogicVal) -> Self {
        match logic {
            LogicVal::U => vhpi_sys::vhpiU,
            LogicVal::X => vhpi_sys::vhpiX,
            LogicVal::Zero => vhpi_sys::vhpi0,
            LogicVal::One => vhpi_sys::vhpi1,
            LogicVal::Z => vhpi_sys::vhpiZ,
            LogicVal::W => vhpi_sys::vhpiW,
            LogicVal::L => vhpi_sys::vhpiL,
            LogicVal::H => vhpi_sys::vhpiH,
            LogicVal::DontCare => vhpi_sys::vhpiDontCare,
            LogicVal::Unknown(v) => vhpi_sys::vhpiEnumT::from(v),
        }
    }
}

impl TryFrom<char> for LogicVal {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' | 'u' => Ok(LogicVal::U),
            'X' | 'x' => Ok(LogicVal::X),
            '0' => Ok(LogicVal::Zero),
            '1' => Ok(LogicVal::One),
            'Z' | 'z' => Ok(LogicVal::Z),
            'W' | 'w' => Ok(LogicVal::W),
            'L' | 'l' => Ok(LogicVal::L),
            'H' | 'h' => Ok(LogicVal::H),
            '-' => Ok(LogicVal::DontCare),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LogicVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogicVal::U => "U",
                LogicVal::X => "X",
                LogicVal::Zero => "0",
                LogicVal::One => "1",
                LogicVal::Z => "Z",
                LogicVal::W => "W",
                LogicVal::L => "L",
                LogicVal::H => "H",
                LogicVal::DontCare => "-",
                LogicVal::Unknown(v) => return write!(f, "?({v})"),
            }
        )
    }
}
