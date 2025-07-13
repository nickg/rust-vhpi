use bindings::{vhpi_register_cb, vhpiCbDataS};
use crate::Handle;

#[repr(u32)]
pub enum CbReason {
    StartOfSimulation = bindings::vhpiCbStartOfSimulation,
    EndOfSimulation = bindings::vhpiCbEndOfSimulation,
    NextTimeStep = bindings::vhpiCbNextTimeStep,
    RepNextTimeStep = bindings::vhpiCbRepNextTimeStep,
    ValueChange = bindings::vhpiCbValueChange,
}

pub struct CbData {
    pub obj: Handle,
}

unsafe extern "C" fn trampoline<F>(cb_data: *const bindings::vhpiCbDataS)
where
    F: Fn(&CbData),
{
    if cb_data.is_null() {
        return;
    }

    let user_data = (*cb_data).user_data as *mut F;
    if user_data.is_null() {
        return;
    }

    let mut data = CbData {
        obj: Handle::from_raw((*cb_data).obj),
    };

    let callback = &*user_data;
    callback(&data);

    data.obj.clear(); // We do not own this handle
}

pub fn register_cb<F>(reason: CbReason, callback: F) -> Handle
where
    F: Fn(&CbData) + 'static,
{
    let boxed: Box<F> = Box::new(callback);
    let user_data = Box::into_raw(boxed) as *mut std::os::raw::c_void;

    let mut cb_data = vhpiCbDataS {
        reason: reason as i32,
        cb_rtn: Some(trampoline::<F>),
        obj: std::ptr::null_mut(),
        time: std::ptr::null_mut(),
        value: std::ptr::null_mut(),
        user_data,
    };

    Handle::from_raw(unsafe { vhpi_register_cb(&mut cb_data, 0) })
}

impl Handle {
    pub fn register_cb<F>(&self, reason: CbReason, callback: F) -> Handle
        where
        F: Fn(&CbData) + 'static,
    {
        let boxed: Box<F> = Box::new(callback);
        let user_data = Box::into_raw(boxed) as *mut std::os::raw::c_void;

        let mut cb_data = vhpiCbDataS {
            reason: reason as i32,
            cb_rtn: Some(trampoline::<F>),
            obj: self.as_raw(),
            time: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            user_data,
        };

        Handle::from_raw(unsafe { vhpi_register_cb(&mut cb_data, 0) })
    }
}
