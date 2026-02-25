#[derive(Debug, Clone)]
pub struct Physical {
    pub low: u32,
    pub high: i32,
}

impl From<i64> for Physical {
    fn from(value: i64) -> Self {
        Physical {
            low: value as u32,
            high: (value >> 32) as i32,
        }
    }
}

impl From<u32> for Physical {
    fn from(value: u32) -> Self {
        Physical {
            low: value,
            high: 0_i32,
        }
    }
}

impl From<vhpi_sys::vhpiPhysT> for Physical {
    fn from(raw: vhpi_sys::vhpiPhysT) -> Self {
        Physical {
            low: raw.low,
            high: raw.high,
        }
    }
}

impl From<Physical> for vhpi_sys::vhpiPhysT {
    fn from(phys: Physical) -> Self {
        vhpi_sys::vhpiPhysT {
            low: phys.low,
            high: phys.high,
        }
    }
}

impl Physical {
    #[must_use]
    pub fn to_i64(&self) -> i64 {
        i64::from(self.high) << 32 | i64::from(self.low)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_from_i64_round_trips_positive_and_negative_values() {
        let positive = 0x1234_5678_9ABC_DEF0_i64;
        let negative = -123_456_789_i64;

        assert_eq!(Physical::from(positive).to_i64(), positive);
        assert_eq!(Physical::from(negative).to_i64(), negative);
    }

    #[test]
    fn physical_from_u32_sets_high_to_zero() {
        let phys = Physical::from(0xDEAD_BEEF_u32);

        assert_eq!(phys.low, 0xDEAD_BEEF);
        assert_eq!(phys.high, 0);
        assert_eq!(phys.to_i64(), 0x0000_0000_DEAD_BEEF_i64);
    }

    #[test]
    fn physical_converts_to_and_from_raw_vhpi_phys() {
        let raw = vhpi_sys::vhpiPhysT {
            low: 0x89AB_CDEF,
            high: 0x0123_4567,
        };

        let phys = Physical::from(raw);
        assert_eq!(phys.low, 0x89AB_CDEF);
        assert_eq!(phys.high, 0x0123_4567);

        let raw_round_trip: vhpi_sys::vhpiPhysT = phys.into();
        assert_eq!(raw_round_trip.low, 0x89AB_CDEF);
        assert_eq!(raw_round_trip.high, 0x0123_4567);
    }
}
