use num_traits::Zero;

use crate::{check_error, Error, Physical};

/// 1 femtosecond — the base VHDL time unit (`vhpiFS`).
pub const FS: Time = Time { high: 0, low: 1 };
/// 1 picosecond (`vhpiPS`).
pub const PS: Time = Time {
    high: 0,
    low: 1_000,
};
/// 1 nanosecond (`vhpiNS`).
pub const NS: Time = Time {
    high: 0,
    low: 1_000_000,
};
/// 1 microsecond (`vhpiUS`).
pub const US: Time = Time {
    high: 0,
    low: 1_000_000_000,
};
/// 1 millisecond (`vhpiMS`).
pub const MS: Time = Time {
    high: 232,
    low: 3_567_587_328,
};
/// 1 second (`vhpiS`).
pub const S: Time = Time {
    high: 232_830,
    low: 2_764_472_320,
};
/// 1 minute (`vhpiMN`).
pub const MN: Time = Time {
    high: 13_969_838,
    low: 2_659_581_952,
};
/// 1 hour (`vhpiHR`).
pub const HR: Time = Time {
    high: 838_190_317,
    low: 661_127_168,
};

const PS_I64: i64 = PS.to_i64();
const NS_I64: i64 = NS.to_i64();
const US_I64: i64 = US.to_i64();
const MS_I64: i64 = MS.to_i64();
const S_I64: i64 = S.to_i64();
const MN_I64: i64 = MN.to_i64();
const HR_I64: i64 = HR.to_i64();

#[derive(Debug, Clone, PartialEq)]
/// Simulation time represented as a split 64-bit value.
pub struct Time {
    /// Low 32 bits of the time value.
    pub low: u32,
    /// High 32 bits of the time value (signed for compatibility with VHPI).
    pub high: i32,
}

impl From<i64> for Time {
    fn from(value: i64) -> Self {
        Time {
            low: value as u32,
            high: (value >> 32) as i32,
        }
    }
}

impl From<u32> for Time {
    fn from(value: u32) -> Self {
        Time {
            low: value,
            high: 0_i32,
        }
    }
}

impl From<vhpi_sys::vhpiTimeT> for Time {
    fn from(raw: vhpi_sys::vhpiTimeT) -> Self {
        Time {
            low: raw.low,
            high: raw.high,
        }
    }
}

impl From<Time> for vhpi_sys::vhpiTimeT {
    fn from(time: Time) -> Self {
        vhpi_sys::vhpiTimeT {
            low: time.low,
            high: time.high,
        }
    }
}

impl From<Physical> for Time {
    fn from(phys: Physical) -> Self {
        Time {
            low: phys.low,
            high: phys.high,
        }
    }
}

impl std::ops::Mul<Time> for Time {
    type Output = Time;

    fn mul(self, rhs: Time) -> Self::Output {
        let total = self.to_i64() * rhs.to_i64();
        Time::from(total)
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = self.to_i64();
        if time.is_zero() {
            write!(f, "0")
        } else if !(time % PS_I64).is_zero() {
            write!(f, "{time} fs")
        } else if !(time % NS_I64).is_zero() {
            write!(f, "{} ps", time / PS_I64)
        } else if !(time % US_I64).is_zero() {
            write!(f, "{} ns", time / NS_I64)
        } else if !(time % MS_I64).is_zero() {
            write!(f, "{} µs", time / US_I64)
        } else if !(time % S_I64).is_zero() {
            write!(f, "{} ms", time / MS_I64)
        } else if !(time % MN_I64).is_zero() {
            write!(f, "{} s", time / S_I64)
        } else if !(time % HR_I64).is_zero() {
            write!(f, "{} min", time / MN_I64)
        } else {
            write!(f, "{} hr", time / HR_I64)
        }
    }
}
impl Time {
    #[must_use]
    /// Convert this split representation into a single `i64` value.
    pub const fn to_i64(&self) -> i64 {
        (self.high as i64) << 32 | (self.low as i64)
    }
}

#[must_use]
/// Get the current simulator time.
pub fn get_time() -> Time {
    let mut time = vhpi_sys::vhpiTimeT { low: 0, high: 0 };
    unsafe { vhpi_sys::vhpi_get_time(&raw mut time, std::ptr::null_mut()) };

    time.into()
}

#[must_use]
/// Get the current simulator cycle count.
pub fn get_cycles() -> i64 {
    let mut cycles: std::os::raw::c_long = 0;
    unsafe { vhpi_sys::vhpi_get_time(std::ptr::null_mut(), &raw mut cycles) };

    cycles as i64
}

#[derive(Debug, Clone, PartialEq)]
/// Status value from [`get_next_time`].
pub enum NextTimeStatus {
    /// If the next simulation cycle time value is TIME'HIGH and there are no active drivers, or process resumptions, and the following callbacks: `vhpiCbAfterDelay`, `vhpiCbRepAfterDelay`, `vhpiCbTimeOut`, or `vhpiCbRepTimeOut` are not to occur in the next simulation time.
    NoActivity,
    OK,
    Error(Option<Error>),
}

impl std::fmt::Display for NextTimeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NextTimeStatus::NoActivity => Ok(write!(f, "No activity")?),
            NextTimeStatus::OK => Ok(write!(f, "OK")?),
            NextTimeStatus::Error(Some(error)) => Ok(write!(f, "{error}")?),
            NextTimeStatus::Error(None) => Ok(write!(f, "Unknown error")?),
        }
    }
}

#[must_use]
/// Get the next scheduled simulator time and status code.
pub fn get_next_time() -> (Time, NextTimeStatus) {
    let mut time = vhpi_sys::vhpiTimeT { low: 0, high: 0 };
    let result = unsafe { vhpi_sys::vhpi_get_next_time(&raw mut time) };

    let status = match result {
        vhpi_sys::vhpiNoActivity => NextTimeStatus::NoActivity,
        0 => NextTimeStatus::OK,
        _ => NextTimeStatus::Error(check_error()),
    };

    (time.into(), status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_from_i64_round_trips_positive_and_negative_values() {
        let positive = 0x1234_5678_9ABC_DEF0_i64;
        let negative = -42_i64;

        assert_eq!(Time::from(positive).to_i64(), positive);
        assert_eq!(Time::from(negative).to_i64(), negative);
    }

    #[test]
    fn time_from_u32_sets_high_to_zero() {
        let time = Time::from(0xDEAD_BEEF_u32);

        assert_eq!(time.low, 0xDEAD_BEEF);
        assert_eq!(time.high, 0);
        assert_eq!(time.to_i64(), 0x0000_0000_DEAD_BEEF_i64);
    }

    #[test]
    fn time_converts_to_and_from_raw_vhpi_time() {
        let raw = vhpi_sys::vhpiTimeT {
            low: 0x89AB_CDEF,
            high: 0x0123_4567,
        };

        let time = Time::from(raw);
        assert_eq!(time.low, 0x89AB_CDEF);
        assert_eq!(time.high, 0x0123_4567);

        let raw_round_trip: vhpi_sys::vhpiTimeT = time.into();
        assert_eq!(raw_round_trip.low, 0x89AB_CDEF);
        assert_eq!(raw_round_trip.high, 0x0123_4567);
    }

    #[test]
    fn time_mul_uses_full_i64_value() {
        let lhs = Time::from(2_000_i64);
        let rhs = Time::from(3_i64);

        assert_eq!((lhs * rhs).to_i64(), 6_000_i64);
    }

    #[test]
    fn time_display_uses_expected_units() {
        assert_eq!(Time::from(0_i64).to_string(), "0");
        assert_eq!(Time::from(123_i64).to_string(), "123 fs");
        assert_eq!(Time::from(123_000_i64).to_string(), "123 ps");
        assert_eq!(Time::from(123_000_000_i64).to_string(), "123 ns");
        assert_eq!(Time::from(123_000_000_000_i64).to_string(), "123 µs");
        assert_eq!(Time::from(123_000_000_000_000_i64).to_string(), "123 ms");
        assert_eq!(Time::from(123_000_000_000_000_000_i64).to_string(), "123 s");
        assert_eq!(
            Time::from(1_000_000_000_000_000_000_i64).to_string(),
            "1000 s"
        );
        assert_eq!(MN.to_string(), "1 min");
        assert_eq!(HR.to_string(), "1 hr");
        assert_eq!((MN * Time::from(90_i64)).to_string(), "90 min");
    }
}
