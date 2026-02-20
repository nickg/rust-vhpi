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
