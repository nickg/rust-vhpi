use vhpi_sys::{vhpi_register_cb, vhpiCbDataS};
use crate::Handle;

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

unsafe extern "C" fn trampoline<F>(cb_data: *const vhpi_sys::vhpiCbDataS)
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
