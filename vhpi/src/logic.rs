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
        match val as u32 {
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

impl TryFrom<char> for LogicVal {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(LogicVal::U),
            'X' => Ok(LogicVal::X),
            '0' => Ok(LogicVal::Zero),
            '1' => Ok(LogicVal::One),
            'Z' => Ok(LogicVal::Z),
            'W' => Ok(LogicVal::W),
            'L' => Ok(LogicVal::L),
            'H' => Ok(LogicVal::H),
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
                LogicVal::Unknown(v) => return write!(f, "?({})", v),
            }
        )
    }
}
