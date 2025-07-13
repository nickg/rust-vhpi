use bindings::{vhpiHandleT, vhpi_compare_handles, vhpi_handle, vhpi_handle_by_name,
               vhpi_iterator, vhpi_release_handle, vhpi_scan};
use std::ffi::CString;

#[repr(u32)]
pub enum OneToOne {
    RootInst = bindings::vhpiOneToOneT_vhpiRootInst,
}

#[repr(u32)]
pub enum OneToMany {
    Decls = bindings::vhpiOneToManyT_vhpiDecls,
    SigDecls = bindings::vhpiOneToManyT_vhpiSigDecls,
    PortDecls = bindings::vhpiOneToManyT_vhpiPortDecls,
    InternalRegions = bindings::vhpiOneToManyT_vhpiInternalRegions
}

pub struct Handle {
    handle: vhpiHandleT,
}

pub struct HandleIterator {
    iter: Handle,
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

    pub(crate) fn as_raw(&self) -> vhpiHandleT {
        self.handle
    }

    pub(crate) fn clear(&mut self) {
        self.handle = std::ptr::null_mut();
    }

    pub fn from_raw(raw: vhpiHandleT) -> Self {
        Self { handle: raw }
    }

    pub fn handle(&self, property: OneToOne) -> Handle {
        Handle::from_raw(unsafe { vhpi_handle(property as u32, self.as_raw()) })
    }

    pub fn handle_by_name(&self, name: &str) -> Handle {
        let c_name = CString::new(name).unwrap();
        Handle::from_raw(unsafe {
            vhpi_handle_by_name(c_name.as_ptr() as *const i8, self.as_raw())
        })
    }

    pub fn iterator(&self, typ: OneToMany) -> HandleIterator {
        let raw = unsafe { vhpi_iterator(typ as u32, self.as_raw()) };
        HandleIterator {
            iter: Handle::from_raw(raw),
        }
    }
}

impl Iterator for HandleIterator {
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_null() {
            return None;
        }

        let next = Handle::from_raw(unsafe { vhpi_scan(self.iter.as_raw()) });

        if next.is_null() {
            // The handle is automatically released when the iterator is exhausted
            self.iter.clear();
            None
        } else {
            Some(next)
        }
    }
}

pub fn handle(property: OneToOne) -> Handle {
    Handle::from_raw(unsafe { vhpi_handle(property as u32, std::ptr::null_mut()) })
}

pub fn handle_by_name(name: &str) -> Handle {
    let c_name = CString::new(name).unwrap();
    Handle::from_raw(unsafe {
        vhpi_handle_by_name(c_name.as_ptr() as *const i8, std::ptr::null_mut())
    })
}
