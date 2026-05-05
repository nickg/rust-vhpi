use crate::{check_error, Error, Handle};
use vhpi_sys::{vhpiCbDataS, vhpi_register_cb};

#[repr(u32)]
pub enum CbReason {
    StartOfSimulation = vhpi_sys::vhpiCbStartOfSimulation as u32,
    EndOfSimulation = vhpi_sys::vhpiCbEndOfSimulation as u32,
    NextTimeStep = vhpi_sys::vhpiCbNextTimeStep as u32,
    RepNextTimeStep = vhpi_sys::vhpiCbRepNextTimeStep as u32,
    ValueChange = vhpi_sys::vhpiCbValueChange as u32,
    Force = vhpi_sys::vhpiCbForce as u32,
    Release = vhpi_sys::vhpiCbRelease as u32,
    Transaction = vhpi_sys::vhpiCbTransaction as u32,
    Stmt = vhpi_sys::vhpiCbStmt as u32,
    Resume = vhpi_sys::vhpiCbResume as u32,
    Suspend = vhpi_sys::vhpiCbSuspend as u32,
    StartOfSubpCall = vhpi_sys::vhpiCbStartOfSubpCall as u32,
    EndOfSubpCall = vhpi_sys::vhpiCbEndOfSubpCall as u32,
    AfterDelay = vhpi_sys::vhpiCbAfterDelay as u32,
    RepAfterDelay = vhpi_sys::vhpiCbRepAfterDelay as u32,
    StartOfNextCycle = vhpi_sys::vhpiCbStartOfNextCycle as u32,
    RepStartOfNextCycle = vhpi_sys::vhpiCbRepStartOfNextCycle as u32,
    StartOfProcesses = vhpi_sys::vhpiCbStartOfProcesses as u32,
    RepStartOfProcesses = vhpi_sys::vhpiCbRepStartOfProcesses as u32,
    EndOfProcesses = vhpi_sys::vhpiCbEndOfProcesses as u32,
    RepEndOfProcesses = vhpi_sys::vhpiCbRepEndOfProcesses as u32,
    LastKnownDeltaCycle = vhpi_sys::vhpiCbLastKnownDeltaCycle as u32,
    RepLastKnownDeltaCycle = vhpi_sys::vhpiCbRepLastKnownDeltaCycle as u32,
    StartOfPostponed = vhpi_sys::vhpiCbStartOfPostponed as u32,
    RepStartOfPostponed = vhpi_sys::vhpiCbRepStartOfPostponed as u32,
    EndOfTimeStep = vhpi_sys::vhpiCbEndOfTimeStep as u32,
    RepEndOfTimeStep = vhpi_sys::vhpiCbRepEndOfTimeStep as u32,
    StartOfTool = vhpi_sys::vhpiCbStartOfTool as u32,
    EndOfTool = vhpi_sys::vhpiCbEndOfTool as u32,
    StartOfAnalysis = vhpi_sys::vhpiCbStartOfAnalysis as u32,
    EndOfAnalysis = vhpi_sys::vhpiCbEndOfAnalysis as u32,
    StartOfElaboration = vhpi_sys::vhpiCbStartOfElaboration as u32,
    EndOfElaboration = vhpi_sys::vhpiCbEndOfElaboration as u32,
    StartOfInitialization = vhpi_sys::vhpiCbStartOfInitialization as u32,
    EndOfInitialization = vhpi_sys::vhpiCbEndOfInitialization as u32,
    Quiescense = vhpi_sys::vhpiCbQuiescense as u32,
    PLIError = vhpi_sys::vhpiCbPLIError as u32,
    StartOfSave = vhpi_sys::vhpiCbStartOfSave as u32,
    EndOfSave = vhpi_sys::vhpiCbEndOfSave as u32,
    StartOfRestart = vhpi_sys::vhpiCbStartOfRestart as u32,
    EndOfRestart = vhpi_sys::vhpiCbEndOfRestart as u32,
    StartOfReset = vhpi_sys::vhpiCbStartOfReset as u32,
    EndOfReset = vhpi_sys::vhpiCbEndOfReset as u32,
    EnterInteractive = vhpi_sys::vhpiCbEnterInteractive as u32,
    ExitInteractive = vhpi_sys::vhpiCbExitInteractive as u32,
    SigInterrupt = vhpi_sys::vhpiCbSigInterrupt as u32,
    TimeOut = vhpi_sys::vhpiCbTimeOut as u32,
    RepTimeOut = vhpi_sys::vhpiCbRepTimeOut as u32,
    Sensitivity = vhpi_sys::vhpiCbSensitivity as u32,
}

pub struct CbData {
    pub obj: Handle,
}

struct AfterDelayCbState<F>
where
    F: Fn(&CbData),
{
    callback: F,
    time: vhpi_sys::vhpiTimeT,
}

#[derive(Debug)]
pub enum RegisterCbError {
    UnknownReason,
    Error(Error),
}

unsafe extern "C" fn trampoline<F>(cb_data: *const vhpi_sys::vhpiCbDataS)
where
    F: Fn(&CbData),
{
    if cb_data.is_null() {
        return;
    }

    let user_data = (*cb_data).user_data.cast::<F>();
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

unsafe extern "C" fn after_delay_trampoline<F>(cb_data: *const vhpi_sys::vhpiCbDataS)
where
    F: Fn(&CbData),
{
    if cb_data.is_null() {
        return;
    }

    let user_data = (*cb_data).user_data.cast::<AfterDelayCbState<F>>();
    if user_data.is_null() {
        return;
    }

    let mut data = CbData {
        obj: Handle::from_raw((*cb_data).obj),
    };

    ((*user_data).callback)(&data);

    data.obj.clear(); // We do not own this handle

    drop(Box::from_raw(user_data));
}

pub fn register_cb<F>(reason: CbReason, callback: F) -> Result<Handle, RegisterCbError>
where
    F: Fn(&CbData) + 'static,
{
    let boxed: Box<F> = Box::new(callback);
    let user_data = Box::into_raw(boxed).cast::<std::os::raw::c_void>();

    let mut cb_data = vhpiCbDataS {
        reason: reason as i32,
        cb_rtn: Some(trampoline::<F>),
        obj: std::ptr::null_mut(),
        time: std::ptr::null_mut(),
        value: std::ptr::null_mut(),
        user_data,
    };
    let ret = unsafe { vhpi_register_cb(&raw mut cb_data, vhpi_sys::vhpiReturnCb as i32) };
    match check_error() {
        Some(err) => {
            unsafe {
                drop(Box::from_raw(user_data.cast::<F>()));
            }
            Err(RegisterCbError::Error(err))
        }
        None => Ok(Handle::from_raw(ret)),
    }
}

pub fn register_cb_after_delay<F>(
    delay: crate::Time,
    callback: F,
) -> Result<Handle, RegisterCbError>
where
    F: Fn(&CbData) + 'static,
{
    let boxed = Box::new(AfterDelayCbState {
        callback,
        time: delay.into(),
    });
    let user_data = Box::into_raw(boxed);
    let mut cb_data = vhpiCbDataS {
        reason: CbReason::AfterDelay as i32,
        cb_rtn: Some(after_delay_trampoline::<F>),
        obj: std::ptr::null_mut(),
        time: unsafe { &raw mut (*user_data).time },
        value: std::ptr::null_mut(),
        user_data: user_data.cast::<std::os::raw::c_void>(),
    };
    let ret = unsafe { vhpi_register_cb(&raw mut cb_data, vhpi_sys::vhpiReturnCb as i32) };
    match check_error() {
        Some(err) => {
            unsafe {
                drop(Box::from_raw(user_data.cast::<AfterDelayCbState<F>>()));
            }
            Err(RegisterCbError::Error(err))
        }
        None => Ok(Handle::from_raw(ret)),
    }
}

pub fn remove_cb(handle: &Handle) {
    unsafe { vhpi_sys::vhpi_remove_cb(handle.as_raw()) };
}

impl Handle {
    pub fn register_cb<F>(&self, reason: CbReason, callback: F) -> Result<Handle, RegisterCbError>
    where
        F: Fn(&CbData) + 'static,
    {
        let boxed: Box<F> = Box::new(callback);
        let user_data = Box::into_raw(boxed).cast::<std::os::raw::c_void>();

        let mut cb_data = vhpiCbDataS {
            reason: reason as i32,
            cb_rtn: Some(trampoline::<F>),
            obj: self.as_raw(),
            time: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            user_data,
        };
        let ret = unsafe { vhpi_register_cb(&raw mut cb_data, vhpi_sys::vhpiReturnCb as i32) };
        match check_error() {
            Some(err) => {
                unsafe {
                    drop(Box::from_raw(user_data.cast::<F>()));
                }
                Err(RegisterCbError::Error(err))
            }
            None => Ok(Handle::from_raw(ret)),
        }
    }

    pub fn remove_cb(&self) {
        unsafe { vhpi_sys::vhpi_remove_cb(self.as_raw()) };
    }
}
