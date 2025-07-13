use bindings::{vhpiHandleT, vhpi_release_handle, vhpi_compare_handles};

pub struct Handle {
    handle: vhpiHandleT,
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe {
                vhpi_release_handle(self.handle);
            }
        }
    }
}

impl Default for Handle {
    fn default() -> Self {
        Self::null()
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { vhpi_compare_handles(self.handle, other.handle) != 0 }
    }
}

impl Eq for Handle {}

impl Handle {
    pub fn null() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    pub fn as_raw(&self) -> vhpiHandleT {
        self.handle
    }

    pub fn from_raw(raw: vhpiHandleT) -> Self {
        Self { handle: raw }
    }
}
