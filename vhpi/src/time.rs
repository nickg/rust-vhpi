use vhpi_sys::vhpi_get_time;

use crate::Handle;

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
            low: value as u32,
            high: 0 as i32,
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

impl Time {
    pub fn to_i64(&self) -> i64 {
        (self.high as i64) << 32 | (self.low as i64)
    }
}

impl Handle {
    pub fn get_time(&self) -> Time {
        let mut time = vhpi_sys::vhpiTimeT { low: 0, high: 0 };
        unsafe { vhpi_get_time(&mut time, std::ptr::null_mut()) };

        time.into()
    }

    pub fn get_cycles(&self) -> i64 {
        let mut cycles = 0;
        unsafe { vhpi_get_time(std::ptr::null_mut(), &mut cycles) };

        cycles
    }

    pub fn get_next_time(&self) -> (Time, i32) {
        let mut time = vhpi_sys::vhpiTimeT { low: 0, high: 0 };
        let result = unsafe { vhpi_sys::vhpi_get_next_time(&mut time) };

        (time.into(), result)
    }
}
