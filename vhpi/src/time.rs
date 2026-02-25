use crate::Physical;

#[derive(Debug, Clone)]
pub struct Time {
    pub low: u32,
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
        if time % 1000 != 0 {
            write!(f, "{time} fs")
        } else if time % 1_000_000 != 0 {
            write!(f, "{} ps", time / 1000)
        } else if time % 1_000_000_000 != 0 {
            write!(f, "{} ns", time / 1_000_000)
        } else if time % 1_000_000_000_000 != 0 {
            write!(f, "{} µs", time / 1_000_000_000)
        } else if time % 1_000_000_000_000_000 != 0 {
            write!(f, "{} ms", time / 1_000_000_000_000)
        } else {
            write!(f, "{} s", time / 1_000_000_000_000_000)
        }
    }
}
impl Time {
    #[must_use]
    pub fn to_i64(&self) -> i64 {
        i64::from(self.high) << 32 | i64::from(self.low)
    }
}

#[must_use]
pub fn get_time() -> Time {
    let mut time = vhpi_sys::vhpiTimeT { low: 0, high: 0 };
    unsafe { vhpi_sys::vhpi_get_time(&raw mut time, std::ptr::null_mut()) };

    time.into()
}

#[must_use]
pub fn get_cycles() -> i64 {
    let mut cycles = 0;
    unsafe { vhpi_sys::vhpi_get_time(std::ptr::null_mut(), &raw mut cycles) };

    cycles
}

#[must_use]
pub fn get_next_time() -> (Time, i32) {
    let mut time = vhpi_sys::vhpiTimeT { low: 0, high: 0 };
    let result = unsafe { vhpi_sys::vhpi_get_next_time(&raw mut time) };

    (time.into(), result)
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
        assert_eq!(Time::from(123_i64).to_string(), "123 fs");
        assert_eq!(Time::from(123_000_i64).to_string(), "123 ps");
        assert_eq!(Time::from(123_000_000_i64).to_string(), "123 ns");
        assert_eq!(Time::from(123_000_000_000_i64).to_string(), "123 µs");
        assert_eq!(Time::from(123_000_000_000_000_i64).to_string(), "123 ms");
        assert_eq!(Time::from(123_000_000_000_000_000_i64).to_string(), "123 s");
    }
}
