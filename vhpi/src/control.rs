use crate::Handle;

/// Simulation control operations that can be requested via VHPI.
pub enum Control {
    /// Pause simulation execution.
    Stop,
    /// Terminate simulation.
    Finish,
    /// Reset simulation state.
    Reset,
}

impl From<Control> for vhpi_sys::vhpiSimControlT {
    fn from(control: Control) -> Self {
        match control {
            Control::Stop => vhpi_sys::vhpiSimControlT_vhpiStop,
            Control::Finish => vhpi_sys::vhpiSimControlT_vhpiFinish,
            Control::Reset => vhpi_sys::vhpiSimControlT_vhpiReset,
        }
    }
}

impl From<vhpi_sys::vhpiSimControlT> for Control {
    fn from(raw: vhpi_sys::vhpiSimControlT) -> Self {
        match raw {
            vhpi_sys::vhpiSimControlT_vhpiStop => Control::Stop,
            vhpi_sys::vhpiSimControlT_vhpiFinish => Control::Finish,
            vhpi_sys::vhpiSimControlT_vhpiReset => Control::Reset,
            _ => panic!("Unknown control value: {raw}"),
        }
    }
}

/// Result returned by simulator control operations.
pub enum ControlReturn {
    /// The operation completed successfully.
    Success,
    /// The operation failed.
    Failure,
}

impl From<i32> for ControlReturn {
    fn from(raw: i32) -> Self {
        match raw {
            0 => ControlReturn::Success,
            1 => ControlReturn::Failure,
            _ => panic!("Unknown control return value: {raw}"),
        }
    }
}
impl Handle {
    #[must_use]
    #[deprecated(
        since = "0.4.0",
        note = "use the standalone `control` function instead"
    )]
    /// Request a simulation control operation.
    pub fn control(&self, control: Control) -> ControlReturn {
        let result = unsafe { vhpi_sys::vhpi_control1(control.into()) };
        result.into()
    }
}

#[must_use]
/// Request a simulation control operation.
pub fn control(control: Control) -> ControlReturn {
    let result = unsafe { vhpi_sys::vhpi_control1(control.into()) };
    result.into()
}
