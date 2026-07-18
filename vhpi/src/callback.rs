#![cfg_attr(not(windows), allow(clippy::unnecessary_cast))]

use crate::{check_error, Error, Handle, Time};
use num_derive::{FromPrimitive, ToPrimitive};
use std::mem::ManuallyDrop;
use vhpi_sys::{vhpiCbDataS, vhpi_register_cb};

bitflags::bitflags! {
    /// Bitmask of callback flags.
    pub struct CallbackFlag: i32 {
        const Disable = vhpi_sys::vhpiDisableCb as i32;
        const Return = vhpi_sys::vhpiReturnCb as i32;
    }
}

#[repr(u32)]
/// Callback reasons.
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum CbReason {
    /// Triggered at the start of simulation.
    StartOfSimulation = vhpi_sys::vhpiCbStartOfSimulation as u32,
    /// Triggered at the end of simulation.
    EndOfSimulation = vhpi_sys::vhpiCbEndOfSimulation as u32,
    /// Triggered on the next simulation time step.
    NextTimeStep = vhpi_sys::vhpiCbNextTimeStep as u32,
    /// Repeating trigger for each next simulation time step.
    RepNextTimeStep = vhpi_sys::vhpiCbRepNextTimeStep as u32,
    /// Triggered when a watched value changes.
    ValueChange = vhpi_sys::vhpiCbValueChange as u32,
    /// Triggered when a force operation is applied.
    Force = vhpi_sys::vhpiCbForce as u32,
    /// Triggered when a force operation is released.
    Release = vhpi_sys::vhpiCbRelease as u32,
    /// Triggered when a transaction is scheduled.
    Transaction = vhpi_sys::vhpiCbTransaction as u32,
    /// Triggered on statement execution.
    Stmt = vhpi_sys::vhpiCbStmt as u32,
    /// Triggered when simulation resumes.
    Resume = vhpi_sys::vhpiCbResume as u32,
    /// Triggered when simulation suspends.
    Suspend = vhpi_sys::vhpiCbSuspend as u32,
    /// Triggered at the start of a subprogram call.
    StartOfSubpCall = vhpi_sys::vhpiCbStartOfSubpCall as u32,
    /// Triggered at the end of a subprogram call.
    EndOfSubpCall = vhpi_sys::vhpiCbEndOfSubpCall as u32,
    /// Triggered once after a specified delay.
    AfterDelay = vhpi_sys::vhpiCbAfterDelay as u32,
    /// Repeating trigger after each specified delay interval.
    RepAfterDelay = vhpi_sys::vhpiCbRepAfterDelay as u32,
    /// Triggered at the start of the next simulation cycle.
    StartOfNextCycle = vhpi_sys::vhpiCbStartOfNextCycle as u32,
    /// Repeating trigger at the start of each next cycle.
    RepStartOfNextCycle = vhpi_sys::vhpiCbRepStartOfNextCycle as u32,
    /// Triggered at the start of process execution.
    StartOfProcesses = vhpi_sys::vhpiCbStartOfProcesses as u32,
    /// Repeating trigger at the start of process execution.
    RepStartOfProcesses = vhpi_sys::vhpiCbRepStartOfProcesses as u32,
    /// Triggered at the end of process execution.
    EndOfProcesses = vhpi_sys::vhpiCbEndOfProcesses as u32,
    /// Repeating trigger at the end of process execution.
    RepEndOfProcesses = vhpi_sys::vhpiCbRepEndOfProcesses as u32,
    /// Triggered at the last known delta cycle in a time step.
    LastKnownDeltaCycle = vhpi_sys::vhpiCbLastKnownDeltaCycle as u32,
    /// Repeating trigger at the last known delta cycle.
    RepLastKnownDeltaCycle = vhpi_sys::vhpiCbRepLastKnownDeltaCycle as u32,
    /// Triggered at the start of postponed process execution.
    StartOfPostponed = vhpi_sys::vhpiCbStartOfPostponed as u32,
    /// Repeating trigger at the start of postponed execution.
    RepStartOfPostponed = vhpi_sys::vhpiCbRepStartOfPostponed as u32,
    /// Triggered at the end of the current time step.
    EndOfTimeStep = vhpi_sys::vhpiCbEndOfTimeStep as u32,
    /// Repeating trigger at the end of each time step.
    RepEndOfTimeStep = vhpi_sys::vhpiCbRepEndOfTimeStep as u32,
    /// Triggered when tool-specific execution starts.
    StartOfTool = vhpi_sys::vhpiCbStartOfTool as u32,
    /// Triggered when tool-specific execution ends.
    EndOfTool = vhpi_sys::vhpiCbEndOfTool as u32,
    /// Triggered when analysis starts.
    StartOfAnalysis = vhpi_sys::vhpiCbStartOfAnalysis as u32,
    /// Triggered when analysis ends.
    EndOfAnalysis = vhpi_sys::vhpiCbEndOfAnalysis as u32,
    /// Triggered when elaboration starts.
    StartOfElaboration = vhpi_sys::vhpiCbStartOfElaboration as u32,
    /// Triggered when elaboration ends.
    EndOfElaboration = vhpi_sys::vhpiCbEndOfElaboration as u32,
    /// Triggered when initialization starts.
    StartOfInitialization = vhpi_sys::vhpiCbStartOfInitialization as u32,
    /// Triggered when initialization ends.
    EndOfInitialization = vhpi_sys::vhpiCbEndOfInitialization as u32,
    /// Triggered when the simulator reaches quiescence.
    Quiescense = vhpi_sys::vhpiCbQuiescense as u32,
    /// Triggered when a PLI error is reported.
    PLIError = vhpi_sys::vhpiCbPLIError as u32,
    /// Triggered when save operation starts.
    StartOfSave = vhpi_sys::vhpiCbStartOfSave as u32,
    /// Triggered when save operation ends.
    EndOfSave = vhpi_sys::vhpiCbEndOfSave as u32,
    /// Triggered when restart operation starts.
    StartOfRestart = vhpi_sys::vhpiCbStartOfRestart as u32,
    /// Triggered when restart operation ends.
    EndOfRestart = vhpi_sys::vhpiCbEndOfRestart as u32,
    /// Triggered when reset operation starts.
    StartOfReset = vhpi_sys::vhpiCbStartOfReset as u32,
    /// Triggered when reset operation ends.
    EndOfReset = vhpi_sys::vhpiCbEndOfReset as u32,
    /// Triggered when entering interactive mode.
    EnterInteractive = vhpi_sys::vhpiCbEnterInteractive as u32,
    /// Triggered when leaving interactive mode.
    ExitInteractive = vhpi_sys::vhpiCbExitInteractive as u32,
    /// Triggered on signal interrupt.
    SigInterrupt = vhpi_sys::vhpiCbSigInterrupt as u32,
    /// Triggered when a timeout expires.
    TimeOut = vhpi_sys::vhpiCbTimeOut as u32,
    /// Repeating trigger for timeout expiry.
    RepTimeOut = vhpi_sys::vhpiCbRepTimeOut as u32,
    /// Triggered when sensitivity conditions are met.
    Sensitivity = vhpi_sys::vhpiCbSensitivity as u32,
    Unknown,
}

impl CbReason {
    pub fn from_u32(value: u32) -> Self {
        num_traits::FromPrimitive::from_u32(value).unwrap_or(CbReason::Unknown)
    }
}

/// Data passed to callback functions.
pub struct CbData {
    obj: ManuallyDrop<Handle>,
}

impl CbData {
    #[inline]
    unsafe fn from_raw(raw: *const vhpiCbDataS) -> Self {
        Self {
            // vhpiCbDataS::obj is the callback trigger object handle.
            // Keep it borrowed by avoiding Drop with ManuallyDrop.
            obj: ManuallyDrop::new(Handle::from_raw((*raw).obj)),
        }
    }

    #[must_use]
    pub fn obj(&self) -> &Handle {
        // ManuallyDrop has the same layout as Handle; only Drop behavior differs.
        unsafe { &*(&raw const self.obj).cast::<Handle>() }
    }
}

/// Information about a registered callback returned by [`get_cb_info`] and
/// [`Handle::get_cb_info`].
#[derive(Debug, Clone, PartialEq)]
pub struct CbInfo {
    /// Raw callback reason as returned by the simulator. Compare against
    /// [`CbReason`] discriminant values to identify the reason.
    pub reason: CbReason,
    /// The trigger object this callback is attached to.
    ///
    /// The handle is borrowed from the simulator and must not be released;
    /// it is valid only as long as the callback handle is alive.
    obj: ManuallyDrop<Handle>,
    /// The scheduled simulation time for time-based callbacks.
    pub time: Option<Time>,
}

impl CbInfo {
    #[must_use]
    /// Return a reference to the trigger object handle.
    pub fn obj(&self) -> &Handle {
        unsafe { &*(&raw const self.obj).cast::<Handle>() }
    }
}

struct AfterDelayCbState<F>
where
    F: Fn(&CbData),
{
    callback: F,
    time: vhpi_sys::vhpiTimeT,
}

#[derive(Debug, Clone, PartialEq)]
/// Errors returned when registering a VHPI callback.
pub enum RegisterCbError {
    /// A callback reason value was not recognized.
    UnknownReason,
    /// The simulator reported an error while registering the callback.
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

    let data = CbData::from_raw(cb_data);

    let callback = &*user_data;
    callback(&data);
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

    let data = CbData::from_raw(cb_data);

    ((*user_data).callback)(&data);

    drop(Box::from_raw(user_data));
}

/// Register a global callback for a simulator event.
///
/// The callback is retained by the simulator and can be removed later using
/// [`remove_cb`]. The returned handle represents the registered callback.
///
/// # Errors
///
/// Returns [`RegisterCbError::Error`] when the simulator reports an error while
/// registering the callback.
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
    let ret = unsafe { vhpi_register_cb(&raw mut cb_data, CallbackFlag::Return.bits()) };
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

/// Register a callback that fires once after the specified simulation delay.
///
/// The callback state is released after the callback runs. If you need to
/// cancel it before it fires, call [`remove_cb`] with the returned handle.
///
/// # Errors
///
/// Returns [`RegisterCbError::Error`] when the simulator reports an error while
/// registering the callback.
pub fn register_cb_after_delay<F>(delay: Time, callback: F) -> Result<Handle, RegisterCbError>
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
    let ret = unsafe { vhpi_register_cb(&raw mut cb_data, CallbackFlag::Return.bits()) };
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

/// Remove a previously registered callback.
///
/// # Errors
///
/// Returns an [`Error`] if the simulator reports a failure.
pub fn remove_cb(handle: &Handle) -> Result<(), Error> {
    let rc = unsafe { vhpi_sys::vhpi_remove_cb(handle.as_raw()) };
    if rc != 0 {
        Err(check_error().unwrap_or_else(|| "vhpi_remove_cb failed".into()))
    } else {
        Ok(())
    }
}

/// Disable a previously registered callback without removing it.
///
/// The callback remains registered but will not fire until re-enabled with
/// [`enable_cb`]. Returns an error if the simulator reports a failure.
///
/// # Errors
///
/// Returns an [`Error`] if the simulator reports a failure.
pub fn disable_cb(handle: &Handle) -> Result<(), Error> {
    let rc = unsafe { vhpi_sys::vhpi_disable_cb(handle.as_raw()) };
    if rc != 0 {
        Err(check_error().unwrap_or_else(|| "vhpi_disable_cb failed".into()))
    } else {
        Ok(())
    }
}

/// Re-enable a callback that was previously disabled with [`disable_cb`].
///
/// # Errors
///
/// Returns an [`Error`] if the simulator reports a failure.
pub fn enable_cb(handle: &Handle) -> Result<(), Error> {
    let rc = unsafe { vhpi_sys::vhpi_enable_cb(handle.as_raw()) };
    if rc != 0 {
        Err(check_error().unwrap_or_else(|| "vhpi_enable_cb failed".into()))
    } else {
        Ok(())
    }
}

/// Retrieve information about a registered callback.
///
/// # Errors
///
/// Returns an [`Error`] if the simulator reports a failure.
pub fn get_cb_info(handle: &Handle) -> Result<CbInfo, Error> {
    let mut raw: vhpi_sys::vhpiCbDataT = unsafe { std::mem::zeroed() };
    let rc = unsafe { vhpi_sys::vhpi_get_cb_info(handle.as_raw(), &raw mut raw) };
    if rc != 0 {
        return Err(check_error().unwrap_or_else(|| "vhpi_get_cb_info failed".into()));
    }
    let time = if raw.time.is_null() {
        None
    } else {
        Some(Time::from(unsafe { *raw.time }))
    };
    Ok(CbInfo {
        reason: CbReason::from_u32(raw.reason as u32),
        obj: ManuallyDrop::new(Handle::from_raw(raw.obj)),
        time,
    })
}

impl Handle {
    /// Register a callback scoped to this object handle.
    ///
    /// # Errors
    ///
    /// Returns [`RegisterCbError::Error`] when the simulator reports an error
    /// while registering the callback.
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
        let ret = unsafe { vhpi_register_cb(&raw mut cb_data, CallbackFlag::Return.bits()) };
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

    /// Remove the callback represented by this handle.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the simulator reports a failure.
    pub fn remove_cb(&self) -> Result<(), Error> {
        remove_cb(self)
    }

    /// Disable this callback without removing it.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the simulator reports a failure.
    pub fn disable_cb(&self) -> Result<(), Error> {
        disable_cb(self)
    }

    /// Re-enable this callback after it was disabled.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the simulator reports a failure.
    pub fn enable_cb(&self) -> Result<(), Error> {
        enable_cb(self)
    }

    /// Retrieve information about this callback.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the simulator reports a failure.
    pub fn get_cb_info(&self) -> Result<CbInfo, Error> {
        get_cb_info(self)
    }
}
