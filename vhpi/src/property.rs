use std::ffi::CStr;
use bindings::{vhpi_get_str, vhpi_get};
use num_derive::FromPrimitive;

use crate::Handle;

#[repr(u32)]
pub enum StrProperty {
    Name = bindings::vhpiStrPropertyT_vhpiNameP,
    FullName = bindings::vhpiStrPropertyT_vhpiFullNameP,
    CaseName = bindings::vhpiStrPropertyT_vhpiCaseNameP,
    FullCaseName = bindings::vhpiStrPropertyT_vhpiFullCaseNameP,
    CompName = bindings::vhpiStrPropertyT_vhpiCompNameP,
    DefName = bindings::vhpiStrPropertyT_vhpiDefNameP,
    FileName = bindings::vhpiStrPropertyT_vhpiFileNameP,
    KindStr = bindings::vhpiStrPropertyT_vhpiKindStrP,
    LabelName = bindings::vhpiStrPropertyT_vhpiLabelNameP,
    LibLogicalName = bindings::vhpiStrPropertyT_vhpiLibLogicalNameP,
    LibPhysicalName = bindings::vhpiStrPropertyT_vhpiLibPhysicalNameP,
    LogicalName = bindings::vhpiStrPropertyT_vhpiLogicalNameP,
    LoopLabelName = bindings::vhpiStrPropertyT_vhpiLoopLabelNameP,
    StrVal = bindings::vhpiStrPropertyT_vhpiStrValP,
    ToolVersion = bindings::vhpiStrPropertyT_vhpiToolVersionP,
    UnitName = bindings::vhpiStrPropertyT_vhpiUnitNameP,
    SaveRestartLocation = bindings::vhpiStrPropertyT_vhpiSaveRestartLocationP,
    CompInstName = bindings::vhpiStrPropertyT_vhpiCompInstNameP,
    InstNames = bindings::vhpiStrPropertyT_vhpiInstNamesP,
    SignatureName = bindings::vhpiStrPropertyT_vhpiSignatureNameP,
    SpecName = bindings::vhpiStrPropertyT_vhpiSpecNameP,
}

#[repr(u32)]
pub enum IntProperty {
    Kind = bindings::vhpiIntPropertyT_vhpiKindP,
    Access = bindings::vhpiIntPropertyT_vhpiAccessP,
    Argc = bindings::vhpiIntPropertyT_vhpiArgcP,
    AttrKind = bindings::vhpiIntPropertyT_vhpiAttrKindP,
    BaseIndex = bindings::vhpiIntPropertyT_vhpiBaseIndexP,
    BeginLineNo = bindings::vhpiIntPropertyT_vhpiBeginLineNoP,
    EndLineNo = bindings::vhpiIntPropertyT_vhpiEndLineNoP,
    EntityClass = bindings::vhpiIntPropertyT_vhpiEntityClassP,
    ForeignKind = bindings::vhpiIntPropertyT_vhpiForeignKindP,
    FrameLevel = bindings::vhpiIntPropertyT_vhpiFrameLevelP,
    GenerateIndex = bindings::vhpiIntPropertyT_vhpiGenerateIndexP,
    IntVal = bindings::vhpiIntPropertyT_vhpiIntValP,
    IsAnonymous = bindings::vhpiIntPropertyT_vhpiIsAnonymousP,
    IsBasic = bindings::vhpiIntPropertyT_vhpiIsBasicP,
    IsComposite = bindings::vhpiIntPropertyT_vhpiIsCompositeP,
    IsDefault = bindings::vhpiIntPropertyT_vhpiIsDefaultP,
    IsDeferred = bindings::vhpiIntPropertyT_vhpiIsDeferredP,
    IsDiscrete = bindings::vhpiIntPropertyT_vhpiIsDiscreteP,
    IsForced = bindings::vhpiIntPropertyT_vhpiIsForcedP,
    IsForeign = bindings::vhpiIntPropertyT_vhpiIsForeignP,
    IsGuarded = bindings::vhpiIntPropertyT_vhpiIsGuardedP,
    IsImplicitDecl = bindings::vhpiIntPropertyT_vhpiIsImplicitDeclP,
    LoopIndex = bindings::vhpiIntPropertyT_vhpiLoopIndexP,
    Mode = bindings::vhpiIntPropertyT_vhpiModeP,
    NumDimensions = bindings::vhpiIntPropertyT_vhpiNumDimensionsP,
    NumFields = bindings::vhpiIntPropertyT_vhpiNumFieldsP,
    NumGens = bindings::vhpiIntPropertyT_vhpiNumGensP,
    NumLiterals = bindings::vhpiIntPropertyT_vhpiNumLiteralsP,
    NumMembers = bindings::vhpiIntPropertyT_vhpiNumMembersP,
    NumParams = bindings::vhpiIntPropertyT_vhpiNumParamsP,
    NumPorts = bindings::vhpiIntPropertyT_vhpiNumPortsP,
    OpenMode = bindings::vhpiIntPropertyT_vhpiOpenModeP,
    Phase = bindings::vhpiIntPropertyT_vhpiPhaseP,
    Position = bindings::vhpiIntPropertyT_vhpiPositionP,
    PredefAttr = bindings::vhpiIntPropertyT_vhpiPredefAttrP,
    Reason = bindings::vhpiIntPropertyT_vhpiReasonP,
    RightBound = bindings::vhpiIntPropertyT_vhpiRightBoundP,
    SigKind = bindings::vhpiIntPropertyT_vhpiSigKindP,
    Size = bindings::vhpiIntPropertyT_vhpiSizeP,
    StartLineNo = bindings::vhpiIntPropertyT_vhpiStartLineNoP,
    State = bindings::vhpiIntPropertyT_vhpiStateP,
    Staticness = bindings::vhpiIntPropertyT_vhpiStaticnessP,
    VHDLversion = bindings::vhpiIntPropertyT_vhpiVHDLversionP,
    Id = bindings::vhpiIntPropertyT_vhpiIdP,
    Capabilities = bindings::vhpiIntPropertyT_vhpiCapabilitiesP,
    AutomaticRestore = bindings::vhpiIntPropertyT_vhpiAutomaticRestoreP,
    CompInstKind = bindings::vhpiIntPropertyT_vhpiCompInstKindP,
    IsBuiltIn = bindings::vhpiIntPropertyT_vhpiIsBuiltInP,
    IsDynamic = bindings::vhpiIntPropertyT_vhpiIsDynamicP,
    IsOperator = bindings::vhpiIntPropertyT_vhpiIsOperatorP,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum ClassKind {
    RootInst = bindings::vhpiClassKindT_vhpiRootInstK,
}

impl ClassKind {
    pub fn from_i32(value: i32) -> Option<ClassKind> {
        num::FromPrimitive::from_i32(value)
    }
}

impl Handle {
    pub fn get(&self, property: IntProperty) -> i32 {
        unsafe { vhpi_get(property as u32, self.as_raw()) }
    }

    pub fn get_str(&self, property: StrProperty) -> Option<String> {
        let ptr = unsafe { vhpi_get_str(property as u32, self.as_raw()) };
        if ptr.is_null() {
            return None
        }

        unsafe {
            CStr::from_ptr(ptr as *const i8)
                .to_str()
                .ok()
                .map(|s| s.to_owned())
        }
    }

    // The following are convenience functions not defined by VHPI

    pub fn get_kind(&self) -> ClassKind {
        ClassKind::from_i32(self.get(IntProperty::Kind)).unwrap()
    }

    pub fn get_name(&self) -> String {
        self.get_str(StrProperty::Name).unwrap()
    }
}
