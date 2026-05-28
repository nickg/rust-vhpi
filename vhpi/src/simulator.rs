#![cfg_attr(not(windows), allow(clippy::unnecessary_cast))]

use crate::{IntProperty, OneToOne, PhysProperty, StrProperty, Time};

bitflags::bitflags! {
#[derive(Debug)]
    /// Capability flags reported by the active simulator.
    pub struct Provides: u32 {
        /// Simulator can provide hierarchy information.
        const HIERARCHY  = vhpi_sys::vhpiCapabibilityT_vhpiProvidesHierarchy as u32;
        /// Simulator supports static-object access APIs.
        const STATIC_ACCESS = vhpi_sys::vhpiCapabibilityT_vhpiProvidesStaticAccess as u32;
        /// Simulator provides connectivity information.
        const CONNECTIVITY = vhpi_sys::vhpiCapabibilityT_vhpiProvidesConnectivity as u32;
        /// Simulator supports post-analysis operations.
        const POST_ANALYSIS = vhpi_sys::vhpiCapabibilityT_vhpiProvidesPostAnalysis as u32;
        /// Simulator supports foreign-model integration.
        const FOREIGN_MODEL = vhpi_sys::vhpiCapabibilityT_vhpiProvidesForeignModel as u32;
        /// Simulator supports advanced foreign-model integration.
        const ADVANCED_FOREIGN_MODEL =
            vhpi_sys::vhpiCapabibilityT_vhpiProvidesAdvancedForeignModel as u32;
        /// Simulator supports save/restart operations.
        const SAVE_RESTART = vhpi_sys::vhpiCapabibilityT_vhpiProvidesSaveRestart as u32;
        /// Simulator supports reset operations.
        const RESET = vhpi_sys::vhpiCapabibilityT_vhpiProvidesReset as u32;
        /// Simulator provides runtime debug support.
        const DEBUG_RUNTIME = vhpi_sys::vhpiCapabibilityT_vhpiProvidesDebugRuntime as u32;
        /// Simulator provides advanced runtime debug support.
        const ADVANCED_DEBUG_RUNTIME =
            vhpi_sys::vhpiCapabibilityT_vhpiProvidesAdvancedDebugRuntime as u32;
        /// Simulator supports dynamic elaboration.
        const DYNAMIC_ELAB = vhpi_sys::vhpiCapabibilityT_vhpiProvidesDynamicElab as u32;
    }
}

#[must_use]
/// Query capability flags supported by the active simulator.
pub fn simulator_capabilities() -> Provides {
    let tool_handle = unsafe {
        vhpi_sys::vhpi_handle(
            OneToOne::Tool as vhpi_sys::vhpiOneToOneT,
            std::ptr::null_mut(),
        )
    };
    let caps = unsafe {
        vhpi_sys::vhpi_get(
            IntProperty::Capabilities as vhpi_sys::vhpiIntPropertyT,
            tool_handle,
        )
    };
    Provides::from_bits(caps as u32)
        .unwrap_or_else(|| panic!("Invalid capabilities bitmask: {caps:#010x}",))
}

#[must_use]
/// Get the simulator/tool name.
pub fn simulator_name() -> Option<String> {
    crate::handle(OneToOne::Tool).get_name()
}

#[must_use]
/// Get the simulator/tool version string.
pub fn simulator_version() -> Option<String> {
    crate::handle(OneToOne::Tool).get_str(StrProperty::ToolVersion)
}

#[must_use]
/// Get the simulator time resolution.
pub fn simulator_time_resolution() -> Time {
    crate::handle(OneToOne::Tool)
        .get_phys(PhysProperty::ResolutionLimit)
        .into()
}

#[must_use]
#[cfg(feature = "nvc")]
/// Get the current simulator random seed.
pub fn simulator_random_seed() -> i32 {
    crate::handle(OneToOne::Tool).get(IntProperty::RandomSeed)
}
