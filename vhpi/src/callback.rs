use crate::{check_error, Error, Handle};
use vhpi_sys::{vhpiCbDataS, vhpi_register_cb};

#[repr(u32)]
pub enum CbReason {
    StartOfSimulation = vhpi_sys::vhpiCbStartOfSimulation,
    EndOfSimulation = vhpi_sys::vhpiCbEndOfSimulation,
    NextTimeStep = vhpi_sys::vhpiCbNextTimeStep,
    RepNextTimeStep = vhpi_sys::vhpiCbRepNextTimeStep,
    ValueChange = vhpi_sys::vhpiCbValueChange,
    Force = vhpi_sys::vhpiCbForce,
    Release = vhpi_sys::vhpiCbRelease,
    Transaction = vhpi_sys::vhpiCbTransaction,
    Stmt = vhpi_sys::vhpiCbStmt,
    Resume = vhpi_sys::vhpiCbResume,
    Suspend = vhpi_sys::vhpiCbSuspend,
    StartOfSubpCall = vhpi_sys::vhpiCbStartOfSubpCall,
    EndOfSubpCall = vhpi_sys::vhpiCbEndOfSubpCall,
    AfterDelay = vhpi_sys::vhpiCbAfterDelay,
    RepAfterDelay = vhpi_sys::vhpiCbRepAfterDelay,
    StartOfNextCycle = vhpi_sys::vhpiCbStartOfNextCycle,
    RepStartOfNextCycle = vhpi_sys::vhpiCbRepStartOfNextCycle,
    StartOfProcesses = vhpi_sys::vhpiCbStartOfProcesses,
    RepStartOfProcesses = vhpi_sys::vhpiCbRepStartOfProcesses,
    EndOfProcesses = vhpi_sys::vhpiCbEndOfProcesses,
    RepEndOfProcesses = vhpi_sys::vhpiCbRepEndOfProcesses,
    LastKnownDeltaCycle = vhpi_sys::vhpiCbLastKnownDeltaCycle,
    RepLastKnownDeltaCycle = vhpi_sys::vhpiCbRepLastKnownDeltaCycle,
    StartOfPostponed = vhpi_sys::vhpiCbStartOfPostponed,
    RepStartOfPostponed = vhpi_sys::vhpiCbRepStartOfPostponed,
    EndOfTimeStep = vhpi_sys::vhpiCbEndOfTimeStep,
    RepEndOfTimeStep = vhpi_sys::vhpiCbRepEndOfTimeStep,
    StartOfTool = vhpi_sys::vhpiCbStartOfTool,
    EndOfTool = vhpi_sys::vhpiCbEndOfTool,
    StartOfAnalysis = vhpi_sys::vhpiCbStartOfAnalysis,
    EndOfAnalysis = vhpi_sys::vhpiCbEndOfAnalysis,
    StartOfElaboration = vhpi_sys::vhpiCbStartOfElaboration,
    EndOfElaboration = vhpi_sys::vhpiCbEndOfElaboration,
    StartOfInitialization = vhpi_sys::vhpiCbStartOfInitialization,
    EndOfInitialization = vhpi_sys::vhpiCbEndOfInitialization,
    Quiescense = vhpi_sys::vhpiCbQuiescense,
    PLIError = vhpi_sys::vhpiCbPLIError,
    StartOfSave = vhpi_sys::vhpiCbStartOfSave,
    EndOfSave = vhpi_sys::vhpiCbEndOfSave,
    StartOfRestart = vhpi_sys::vhpiCbStartOfRestart,
    EndOfRestart = vhpi_sys::vhpiCbEndOfRestart,
    StartOfReset = vhpi_sys::vhpiCbStartOfReset,
    EndOfReset = vhpi_sys::vhpiCbEndOfReset,
    EnterInteractive = vhpi_sys::vhpiCbEnterInteractive,
    ExitInteractive = vhpi_sys::vhpiCbExitInteractive,
    SigInterrupt = vhpi_sys::vhpiCbSigInterrupt,
    TimeOut = vhpi_sys::vhpiCbTimeOut,
    RepTimeOut = vhpi_sys::vhpiCbRepTimeOut,
    Sensitivity = vhpi_sys::vhpiCbSensitivity,
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
        let ret = unsafe { vhpi_register_cb(&raw mut cb_data, 0) };
        if ret.is_null() {
            unsafe {
                drop(Box::from_raw(user_data.cast::<F>()));
            }
            check_error().map_or_else(
                || Err(RegisterCbError::UnknownReason),
                |err| Err(RegisterCbError::Error(err)),
            )
        } else {
            Ok(Handle::from_raw(ret))
        }
    }
}
