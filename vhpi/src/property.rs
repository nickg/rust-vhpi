use num_derive::FromPrimitive;
use std::ffi::CStr;
use vhpi_sys::{vhpi_get, vhpi_get_str};

use crate::Handle;

#[repr(u32)]
pub enum StrProperty {
    Name = vhpi_sys::vhpiStrPropertyT_vhpiNameP,
    FullName = vhpi_sys::vhpiStrPropertyT_vhpiFullNameP,
    CaseName = vhpi_sys::vhpiStrPropertyT_vhpiCaseNameP,
    FullCaseName = vhpi_sys::vhpiStrPropertyT_vhpiFullCaseNameP,
    CompName = vhpi_sys::vhpiStrPropertyT_vhpiCompNameP,
    DefName = vhpi_sys::vhpiStrPropertyT_vhpiDefNameP,
    FileName = vhpi_sys::vhpiStrPropertyT_vhpiFileNameP,
    KindStr = vhpi_sys::vhpiStrPropertyT_vhpiKindStrP,
    LabelName = vhpi_sys::vhpiStrPropertyT_vhpiLabelNameP,
    LibLogicalName = vhpi_sys::vhpiStrPropertyT_vhpiLibLogicalNameP,
    LibPhysicalName = vhpi_sys::vhpiStrPropertyT_vhpiLibPhysicalNameP,
    LogicalName = vhpi_sys::vhpiStrPropertyT_vhpiLogicalNameP,
    LoopLabelName = vhpi_sys::vhpiStrPropertyT_vhpiLoopLabelNameP,
    StrVal = vhpi_sys::vhpiStrPropertyT_vhpiStrValP,
    ToolVersion = vhpi_sys::vhpiStrPropertyT_vhpiToolVersionP,
    UnitName = vhpi_sys::vhpiStrPropertyT_vhpiUnitNameP,
    SaveRestartLocation = vhpi_sys::vhpiStrPropertyT_vhpiSaveRestartLocationP,
    CompInstName = vhpi_sys::vhpiStrPropertyT_vhpiCompInstNameP,
    InstNames = vhpi_sys::vhpiStrPropertyT_vhpiInstNamesP,
    SignatureName = vhpi_sys::vhpiStrPropertyT_vhpiSignatureNameP,
    SpecName = vhpi_sys::vhpiStrPropertyT_vhpiSpecNameP,
}

#[repr(u32)]
pub enum IntProperty {
    Kind = vhpi_sys::vhpiIntPropertyT_vhpiKindP,
    Access = vhpi_sys::vhpiIntPropertyT_vhpiAccessP,
    Argc = vhpi_sys::vhpiIntPropertyT_vhpiArgcP,
    AttrKind = vhpi_sys::vhpiIntPropertyT_vhpiAttrKindP,
    BaseIndex = vhpi_sys::vhpiIntPropertyT_vhpiBaseIndexP,
    BeginLineNo = vhpi_sys::vhpiIntPropertyT_vhpiBeginLineNoP,
    EndLineNo = vhpi_sys::vhpiIntPropertyT_vhpiEndLineNoP,
    EntityClass = vhpi_sys::vhpiIntPropertyT_vhpiEntityClassP,
    ForeignKind = vhpi_sys::vhpiIntPropertyT_vhpiForeignKindP,
    FrameLevel = vhpi_sys::vhpiIntPropertyT_vhpiFrameLevelP,
    GenerateIndex = vhpi_sys::vhpiIntPropertyT_vhpiGenerateIndexP,
    IntVal = vhpi_sys::vhpiIntPropertyT_vhpiIntValP,
    IsAnonymous = vhpi_sys::vhpiIntPropertyT_vhpiIsAnonymousP,
    IsBasic = vhpi_sys::vhpiIntPropertyT_vhpiIsBasicP,
    IsComposite = vhpi_sys::vhpiIntPropertyT_vhpiIsCompositeP,
    IsDefault = vhpi_sys::vhpiIntPropertyT_vhpiIsDefaultP,
    IsDeferred = vhpi_sys::vhpiIntPropertyT_vhpiIsDeferredP,
    IsDiscrete = vhpi_sys::vhpiIntPropertyT_vhpiIsDiscreteP,
    IsForced = vhpi_sys::vhpiIntPropertyT_vhpiIsForcedP,
    IsForeign = vhpi_sys::vhpiIntPropertyT_vhpiIsForeignP,
    IsGuarded = vhpi_sys::vhpiIntPropertyT_vhpiIsGuardedP,
    IsImplicitDecl = vhpi_sys::vhpiIntPropertyT_vhpiIsImplicitDeclP,
    LoopIndex = vhpi_sys::vhpiIntPropertyT_vhpiLoopIndexP,
    Mode = vhpi_sys::vhpiIntPropertyT_vhpiModeP,
    NumDimensions = vhpi_sys::vhpiIntPropertyT_vhpiNumDimensionsP,
    NumFields = vhpi_sys::vhpiIntPropertyT_vhpiNumFieldsP,
    NumGens = vhpi_sys::vhpiIntPropertyT_vhpiNumGensP,
    NumLiterals = vhpi_sys::vhpiIntPropertyT_vhpiNumLiteralsP,
    NumMembers = vhpi_sys::vhpiIntPropertyT_vhpiNumMembersP,
    NumParams = vhpi_sys::vhpiIntPropertyT_vhpiNumParamsP,
    NumPorts = vhpi_sys::vhpiIntPropertyT_vhpiNumPortsP,
    OpenMode = vhpi_sys::vhpiIntPropertyT_vhpiOpenModeP,
    Phase = vhpi_sys::vhpiIntPropertyT_vhpiPhaseP,
    Position = vhpi_sys::vhpiIntPropertyT_vhpiPositionP,
    PredefAttr = vhpi_sys::vhpiIntPropertyT_vhpiPredefAttrP,
    Reason = vhpi_sys::vhpiIntPropertyT_vhpiReasonP,
    RightBound = vhpi_sys::vhpiIntPropertyT_vhpiRightBoundP,
    SigKind = vhpi_sys::vhpiIntPropertyT_vhpiSigKindP,
    Size = vhpi_sys::vhpiIntPropertyT_vhpiSizeP,
    StartLineNo = vhpi_sys::vhpiIntPropertyT_vhpiStartLineNoP,
    State = vhpi_sys::vhpiIntPropertyT_vhpiStateP,
    Staticness = vhpi_sys::vhpiIntPropertyT_vhpiStaticnessP,
    VHDLversion = vhpi_sys::vhpiIntPropertyT_vhpiVHDLversionP,
    Id = vhpi_sys::vhpiIntPropertyT_vhpiIdP,
    Capabilities = vhpi_sys::vhpiIntPropertyT_vhpiCapabilitiesP,
    AutomaticRestore = vhpi_sys::vhpiIntPropertyT_vhpiAutomaticRestoreP,
    CompInstKind = vhpi_sys::vhpiIntPropertyT_vhpiCompInstKindP,
    IsBuiltIn = vhpi_sys::vhpiIntPropertyT_vhpiIsBuiltInP,
    IsDynamic = vhpi_sys::vhpiIntPropertyT_vhpiIsDynamicP,
    IsOperator = vhpi_sys::vhpiIntPropertyT_vhpiIsOperatorP,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum ClassKind {
    RootInst = vhpi_sys::vhpiClassKindT_vhpiRootInstK,
}

impl ClassKind {
    #[must_use]
    pub fn from_i32(value: i32) -> Option<ClassKind> {
        num::FromPrimitive::from_i32(value)
    }
}

impl Handle {
    #[must_use]
    pub fn get(&self, property: IntProperty) -> i32 {
        unsafe { vhpi_get(property as u32, self.as_raw()) }
    }

    #[must_use]
    pub fn get_str(&self, property: StrProperty) -> Option<String> {
        let ptr = unsafe { vhpi_get_str(property as u32, self.as_raw()) };
        if ptr.is_null() {
            return None;
        }

        unsafe {
            CStr::from_ptr(ptr.cast::<i8>())
                .to_str()
                .ok()
                .map(std::borrow::ToOwned::to_owned)
        }
    }

    // The following are convenience functions not defined by VHPI

    #[must_use]
    pub fn get_kind(&self) -> ClassKind {
        ClassKind::from_i32(self.get(IntProperty::Kind)).unwrap()
    }

    #[must_use]
    pub fn get_name(&self) -> String {
        self.get_str(StrProperty::Name).unwrap()
    }
}
