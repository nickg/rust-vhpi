use std::ffi::CStr;
use bindings::{vhpi_handle, vhpi_get_str, vhpi_get};
use num_derive::FromPrimitive;

use crate::Handle;

#[repr(u32)]
pub enum OneToOne {
    RootInst = bindings::vhpiOneToOneT_vhpiRootInst,
}

#[repr(u32)]
pub enum StrProperty {
    Name = bindings::vhpiStrPropertyT_vhpiNameP,
    FullName = bindings::vhpiStrPropertyT_vhpiFullNameP,
    CaseName = bindings::vhpiStrPropertyT_vhpiCaseNameP,
    FullCaseName = bindings::vhpiStrPropertyT_vhpiFullCaseNameP,
}

#[repr(u32)]
pub enum IntProperty {
    Kind = bindings::vhpiIntPropertyT_vhpiKindP,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum ClassKind {
    RootInst = bindings::vhpiClassKindT_vhpiRootInstK,
}

impl ClassKind {
    pub fn from_i32(value: i32) -> Option<ClassKind> {
        num::FromPrimitive::from_i32(value)
    }
}

impl Handle {
    pub fn get(&self, property: IntProperty) -> i32 {
        unsafe { vhpi_get(property as u32, self.as_raw()) }
    }

    pub fn get_str(&self, property: StrProperty) -> Option<String> {
        let ptr = unsafe { vhpi_get_str(property as u32, self.as_raw()) };
        if ptr.is_null() {
            return None
        }

        unsafe {
            CStr::from_ptr(ptr as *const i8)
                .to_str()
                .ok()
                .map(|s| s.to_owned())
        }
    }

    pub fn handle(&self, property: OneToOne) -> Handle {
        Handle::from_raw(unsafe { vhpi_handle(property as u32, self.as_raw()) })
    }

    // The following are convenience functions not defined by VHPI

    pub fn get_kind(&self) -> ClassKind {
        ClassKind::from_i32(self.get(IntProperty::Kind)).unwrap()
    }

    pub fn get_name(&self) -> String {
        self.get_str(StrProperty::Name).unwrap()
    }
}
