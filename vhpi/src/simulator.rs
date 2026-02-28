use crate::{IntProperty, OneToOne, PhysProperty, StrProperty, Time};

bitflags::bitflags! {
#[derive(Debug)]
    pub struct Provides: u32 {
        const HIERARCHY  = vhpi_sys::vhpiCapabibilityT_vhpiProvidesHierarchy;
        const STATIC_ACCESS = vhpi_sys::vhpiCapabibilityT_vhpiProvidesStaticAccess;
        const CONNECTIVITY = vhpi_sys::vhpiCapabibilityT_vhpiProvidesConnectivity;
        const POST_ANALYSIS = vhpi_sys::vhpiCapabibilityT_vhpiProvidesPostAnalysis;
        const FOREIGN_MODEL = vhpi_sys::vhpiCapabibilityT_vhpiProvidesForeignModel;
        const ADVANCED_FOREIGN_MODEL = vhpi_sys::vhpiCapabibilityT_vhpiProvidesAdvancedForeignModel;
        const SAVE_RESTART = vhpi_sys::vhpiCapabibilityT_vhpiProvidesSaveRestart;
        const RESET = vhpi_sys::vhpiCapabibilityT_vhpiProvidesReset;
        const DEBUG_RUNTIME = vhpi_sys::vhpiCapabibilityT_vhpiProvidesDebugRuntime;
        const ADVANCED_DEBUG_RUNTIME = vhpi_sys::vhpiCapabibilityT_vhpiProvidesAdvancedDebugRuntime;
        const DYNAMIC_ELAB = vhpi_sys::vhpiCapabibilityT_vhpiProvidesDynamicElab;
    }
}

#[must_use]
pub fn simulator_capabilities() -> Provides {
    let tool_handle = unsafe { vhpi_sys::vhpi_handle(OneToOne::Tool as u32, std::ptr::null_mut()) };
    let caps = unsafe { vhpi_sys::vhpi_get(IntProperty::Capabilities as u32, tool_handle) };
    Provides::from_bits(caps as u32)
        .unwrap_or_else(|| panic!("Invalid capabilities bitmask: {caps:#010x}",))
}

#[must_use]
pub fn simulator_name() -> String {
    crate::handle(OneToOne::Tool).get_name()
}

#[must_use]
pub fn simulator_version() -> String {
    crate::handle(OneToOne::Tool)
        .get_str(StrProperty::ToolVersion)
        .unwrap_or_else(|| "unknown".to_string())
}

#[must_use]
pub fn simulator_time_resolution() -> Time {
    crate::handle(OneToOne::Tool)
        .get_phys(PhysProperty::ResolutionLimit)
        .into()
}
