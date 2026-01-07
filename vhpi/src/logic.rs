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
            bindings::vhpiU => LogicVal::U,
            bindings::vhpiX => LogicVal::X,
            bindings::vhpi0 => LogicVal::Zero,
            bindings::vhpi1 => LogicVal::One,
            bindings::vhpiZ => LogicVal::Z,
            bindings::vhpiW => LogicVal::W,
            bindings::vhpiL => LogicVal::L,
            bindings::vhpiH => LogicVal::H,
            bindings::vhpiDontCare => LogicVal::DontCare,
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
