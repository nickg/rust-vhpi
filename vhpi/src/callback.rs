use bindings::{vhpi_register_cb, vhpiCbDataS};
use crate::Handle;

#[repr(u32)]
pub enum CbReason {
    StartOfSimulation = bindings::vhpiCbStartOfSimulation,
    EndOfSimulation = bindings::vhpiCbEndOfSimulation,
    NextTimeStep = bindings::vhpiCbNextTimeStep,
    RepNextTimeStep = bindings::vhpiCbRepNextTimeStep,
}

pub struct Callback {
    reason: CbReason,
    cb_rtn: unsafe extern "C" fn(*const vhpiCbDataS),
    obj: Handle,
    time: *mut bindings::vhpiTimeT,
    value: *mut bindings::vhpiValueT,
    user_data: *mut std::os::raw::c_void,
    flags: i32,
}

unsafe extern "C" fn trampoline<F>(cb_data: *const bindings::vhpiCbDataS)
where
    F: Fn(),
{
    if cb_data.is_null() {
        return;
    }

    let user_data = (*cb_data).user_data as *mut F;
    if !user_data.is_null() {
        let callback = &*user_data;
        callback();
    }
}

impl Callback {
    pub fn new<F>(reason: CbReason, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        let boxed: Box<F> = Box::new(callback);
        let user_data = Box::into_raw(boxed) as *mut std::os::raw::c_void;

        Self {
            reason,
            cb_rtn: trampoline::<F>,
            obj: Handle::null(),
            time: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            user_data,
            flags: 0,
        }
    }

    pub fn with_obj(mut self, obj: Handle) -> Self {
        self.obj = obj;
        self
    }

    pub fn with_time(mut self, time: *mut bindings::vhpiTimeT) -> Self {
        self.time = time;
        self
    }

    pub fn with_value(mut self, value: *mut bindings::vhpiValueT) -> Self {
        self.value = value;
        self
    }

    pub fn with_user_data(mut self, data: *mut std::os::raw::c_void) -> Self {
        self.user_data = data;
        self
    }

    pub fn with_flags(mut self, flags: i32) -> Self {
        self.flags = flags;
        self
    }

    pub fn register(self) -> Handle {
        let mut cb_data = vhpiCbDataS {
            reason: self.reason as i32,
            cb_rtn: Some(self.cb_rtn),
            obj: self.obj.as_raw(),
            time: self.time,
            value: self.value,
            user_data: self.user_data,
        };

        Handle::from_raw(unsafe { vhpi_register_cb(&mut cb_data, self.flags) })
    }
}
