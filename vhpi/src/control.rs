use crate::Handle;

pub enum Control {
    Stop,
    Finish,
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
            _ => panic!("Unknown control value: {}", raw),
        }
    }
}

pub enum ControlReturn {
    Success,
    Failure,
}

impl From<i32> for ControlReturn {
    fn from(raw: i32) -> Self {
        match raw {
            0 => ControlReturn::Success,
            1 => ControlReturn::Failure,
            _ => panic!("Unknown control return value: {}", raw),
        }
    }
}
impl Handle {
    pub fn control(&self, control: Control) -> ControlReturn {
        let result = unsafe { vhpi_sys::vhpi_control(control.into()) };
        result.into()
    }
}
