use num_derive::FromPrimitive;
use num_traits::Zero;
use std::ffi::CStr;
use vhpi_sys::{vhpi_get, vhpi_get_str, vhpi_iterator, vhpi_scan};

use crate::{iso8859_1_cstr_to_string, Handle, Physical};

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
    IsLocal = vhpi_sys::vhpiIntPropertyT_vhpiIsLocalP,
    IsNamed = vhpi_sys::vhpiIntPropertyT_vhpiIsNamedP,
    IsNull = vhpi_sys::vhpiIntPropertyT_vhpiIsNullP,
    IsOpen = vhpi_sys::vhpiIntPropertyT_vhpiIsOpenP,
    IsPLI = vhpi_sys::vhpiIntPropertyT_vhpiIsPLIP,
    IsPassive = vhpi_sys::vhpiIntPropertyT_vhpiIsPassiveP,
    IsPostponed = vhpi_sys::vhpiIntPropertyT_vhpiIsPostponedP,
    IsProtectedType = vhpi_sys::vhpiIntPropertyT_vhpiIsProtectedTypeP,
    IsPure = vhpi_sys::vhpiIntPropertyT_vhpiIsPureP,
    IsResolved = vhpi_sys::vhpiIntPropertyT_vhpiIsResolvedP,
    IsScalar = vhpi_sys::vhpiIntPropertyT_vhpiIsScalarP,
    IsSeqStmt = vhpi_sys::vhpiIntPropertyT_vhpiIsSeqStmtP,
    IsShared = vhpi_sys::vhpiIntPropertyT_vhpiIsSharedP,
    IsTransport = vhpi_sys::vhpiIntPropertyT_vhpiIsTransportP,
    IsUnaffected = vhpi_sys::vhpiIntPropertyT_vhpiIsUnaffectedP,
    IsUnconstrained = vhpi_sys::vhpiIntPropertyT_vhpiIsUnconstrainedP,
    IsUninstantiated = vhpi_sys::vhpiIntPropertyT_vhpiIsUninstantiatedP,
    IsUp = vhpi_sys::vhpiIntPropertyT_vhpiIsUpP,
    IsVital = vhpi_sys::vhpiIntPropertyT_vhpiIsVitalP,
    IteratorType = vhpi_sys::vhpiIntPropertyT_vhpiIteratorTypeP,
    LeftBound = vhpi_sys::vhpiIntPropertyT_vhpiLeftBoundP,
    LineNo = vhpi_sys::vhpiIntPropertyT_vhpiLineNoP,
    LineOffset = vhpi_sys::vhpiIntPropertyT_vhpiLineOffsetP,
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
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum RealProperty {
    FloatLeftBound = vhpi_sys::vhpiRealPropertyT_vhpiFloatLeftBoundP,
    FloatRightBound = vhpi_sys::vhpiRealPropertyT_vhpiFloatRightBoundP,
    RealVal = vhpi_sys::vhpiRealPropertyT_vhpiRealValP,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum PhysProperty {
    PhysLeftBound = vhpi_sys::vhpiPhysPropertyT_vhpiPhysLeftBoundP,
    PhysPosition = vhpi_sys::vhpiPhysPropertyT_vhpiPhysPositionP,
    PhysRightBound = vhpi_sys::vhpiPhysPropertyT_vhpiPhysRightBoundP,
    PhysVal = vhpi_sys::vhpiPhysPropertyT_vhpiPhysValP,
    Time = vhpi_sys::vhpiPhysPropertyT_vhpiTimeP,
    ResolutionLimit = vhpi_sys::vhpiPhysPropertyT_vhpiResolutionLimitP,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum ClassKind {
    AccessTypeDecl = vhpi_sys::vhpiClassKindT_vhpiAccessTypeDeclK,
    Aggregate = vhpi_sys::vhpiClassKindT_vhpiAggregateK,
    AliasDecl = vhpi_sys::vhpiClassKindT_vhpiAliasDeclK,
    All = vhpi_sys::vhpiClassKindT_vhpiAllK,
    Allocator = vhpi_sys::vhpiClassKindT_vhpiAllocatorK,
    AnyCollection = vhpi_sys::vhpiClassKindT_vhpiAnyCollectionK,
    ArchBody = vhpi_sys::vhpiClassKindT_vhpiArchBodyK,
    Argv = vhpi_sys::vhpiClassKindT_vhpiArgvK,
    ArrayTypeDecl = vhpi_sys::vhpiClassKindT_vhpiArrayTypeDeclK,
    AssocElem = vhpi_sys::vhpiClassKindT_vhpiAssocElemK,
    AttrDecl = vhpi_sys::vhpiClassKindT_vhpiAttrDeclK,
    AttrSpec = vhpi_sys::vhpiClassKindT_vhpiAttrSpecK,
    BitStringLiteral = vhpi_sys::vhpiClassKindT_vhpiBitStringLiteralK,
    BlockConfig = vhpi_sys::vhpiClassKindT_vhpiBlockConfigK,
    BlockStmt = vhpi_sys::vhpiClassKindT_vhpiBlockStmtK,
    Branch = vhpi_sys::vhpiClassKindT_vhpiBranchK,
    Callback = vhpi_sys::vhpiClassKindT_vhpiCallbackK,
    CaseStmt = vhpi_sys::vhpiClassKindT_vhpiCaseStmtK,
    CharLiteral = vhpi_sys::vhpiClassKindT_vhpiCharLiteralK,
    CompConfig = vhpi_sys::vhpiClassKindT_vhpiCompConfigK,
    CompDecl = vhpi_sys::vhpiClassKindT_vhpiCompDeclK,
    CompInstStmt = vhpi_sys::vhpiClassKindT_vhpiCompInstStmtK,
    CondSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiCondSigAssignStmtK,
    CondWaveform = vhpi_sys::vhpiClassKindT_vhpiCondWaveformK,
    ConfigDecl = vhpi_sys::vhpiClassKindT_vhpiConfigDeclK,
    ConstDecl = vhpi_sys::vhpiClassKindT_vhpiConstDeclK,
    ConstParamDecl = vhpi_sys::vhpiClassKindT_vhpiConstParamDeclK,
    DerefObj = vhpi_sys::vhpiClassKindT_vhpiDerefObjK,
    DisconnectSpec = vhpi_sys::vhpiClassKindT_vhpiDisconnectSpecK,
    Driver = vhpi_sys::vhpiClassKindT_vhpiDriverK,
    DriverCollection = vhpi_sys::vhpiClassKindT_vhpiDriverCollectionK,
    ElemAssoc = vhpi_sys::vhpiClassKindT_vhpiElemAssocK,
    ElemDecl = vhpi_sys::vhpiClassKindT_vhpiElemDeclK,
    EntityClassEntry = vhpi_sys::vhpiClassKindT_vhpiEntityClassEntryK,
    EntityDecl = vhpi_sys::vhpiClassKindT_vhpiEntityDeclK,
    EnumLiteral = vhpi_sys::vhpiClassKindT_vhpiEnumLiteralK,
    EnumRange = vhpi_sys::vhpiClassKindT_vhpiEnumRangeK,
    EnumTypeDecl = vhpi_sys::vhpiClassKindT_vhpiEnumTypeDeclK,
    ExitStmt = vhpi_sys::vhpiClassKindT_vhpiExitStmtK,
    FileDecl = vhpi_sys::vhpiClassKindT_vhpiFileDeclK,
    FileParamDecl = vhpi_sys::vhpiClassKindT_vhpiFileParamDeclK,
    FileTypeDecl = vhpi_sys::vhpiClassKindT_vhpiFileTypeDeclK,
    FloatRange = vhpi_sys::vhpiClassKindT_vhpiFloatRangeK,
    FloatTypeDecl = vhpi_sys::vhpiClassKindT_vhpiFloatTypeDeclK,
    ForGenerate = vhpi_sys::vhpiClassKindT_vhpiForGenerateK,
    ForLoop = vhpi_sys::vhpiClassKindT_vhpiForLoopK,
    Foreignf = vhpi_sys::vhpiClassKindT_vhpiForeignfK,
    FuncCall = vhpi_sys::vhpiClassKindT_vhpiFuncCallK,
    FuncDecl = vhpi_sys::vhpiClassKindT_vhpiFuncDeclK,
    GenericDecl = vhpi_sys::vhpiClassKindT_vhpiGenericDeclK,
    GroupDecl = vhpi_sys::vhpiClassKindT_vhpiGroupDeclK,
    GroupTempDecl = vhpi_sys::vhpiClassKindT_vhpiGroupTempDeclK,
    IfGenerate = vhpi_sys::vhpiClassKindT_vhpiIfGenerateK,
    IfStmt = vhpi_sys::vhpiClassKindT_vhpiIfStmtK,
    InPort = vhpi_sys::vhpiClassKindT_vhpiInPortK,
    IndexedName = vhpi_sys::vhpiClassKindT_vhpiIndexedNameK,
    IntLiteral = vhpi_sys::vhpiClassKindT_vhpiIntLiteralK,
    IntRange = vhpi_sys::vhpiClassKindT_vhpiIntRangeK,
    IntTypeDecl = vhpi_sys::vhpiClassKindT_vhpiIntTypeDeclK,
    Iterator = vhpi_sys::vhpiClassKindT_vhpiIteratorK,
    LibraryDecl = vhpi_sys::vhpiClassKindT_vhpiLibraryDeclK,
    NextStmt = vhpi_sys::vhpiClassKindT_vhpiNextStmtK,
    NullLiteral = vhpi_sys::vhpiClassKindT_vhpiNullLiteralK,
    NullStmt = vhpi_sys::vhpiClassKindT_vhpiNullStmtK,
    Others = vhpi_sys::vhpiClassKindT_vhpiOthersK,
    OutPort = vhpi_sys::vhpiClassKindT_vhpiOutPortK,
    PackBody = vhpi_sys::vhpiClassKindT_vhpiPackBodyK,
    PackDecl = vhpi_sys::vhpiClassKindT_vhpiPackDeclK,
    PackInst = vhpi_sys::vhpiClassKindT_vhpiPackInstK,
    ParamAttrName = vhpi_sys::vhpiClassKindT_vhpiParamAttrNameK,
    PhysLiteral = vhpi_sys::vhpiClassKindT_vhpiPhysLiteralK,
    PhysRange = vhpi_sys::vhpiClassKindT_vhpiPhysRangeK,
    PhysTypeDecl = vhpi_sys::vhpiClassKindT_vhpiPhysTypeDeclK,
    PortDecl = vhpi_sys::vhpiClassKindT_vhpiPortDeclK,
    ProcDecl = vhpi_sys::vhpiClassKindT_vhpiProcDeclK,
    ProcessStmt = vhpi_sys::vhpiClassKindT_vhpiProcessStmtK,
    ProtectedTypeBody = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeBodyK,
    ProtectedTypeDecl = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeDeclK,
    RealLiteral = vhpi_sys::vhpiClassKindT_vhpiRealLiteralK,
    RecordTypeDecl = vhpi_sys::vhpiClassKindT_vhpiRecordTypeDeclK,
    ReportStmt = vhpi_sys::vhpiClassKindT_vhpiReportStmtK,
    ReturnStmt = vhpi_sys::vhpiClassKindT_vhpiReturnStmtK,
    RootInst = vhpi_sys::vhpiClassKindT_vhpiRootInstK,
    SelectSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSelectSigAssignStmtK,
    SelectWaveform = vhpi_sys::vhpiClassKindT_vhpiSelectWaveformK,
    SelectedName = vhpi_sys::vhpiClassKindT_vhpiSelectedNameK,
    SigDecl = vhpi_sys::vhpiClassKindT_vhpiSigDeclK,
    SigParamDecl = vhpi_sys::vhpiClassKindT_vhpiSigParamDeclK,
    SimpAttrName = vhpi_sys::vhpiClassKindT_vhpiSimpAttrNameK,
    SimpleSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSimpleSigAssignStmtK,
    SliceName = vhpi_sys::vhpiClassKindT_vhpiSliceNameK,
    StringLiteral = vhpi_sys::vhpiClassKindT_vhpiStringLiteralK,
    SubpBody = vhpi_sys::vhpiClassKindT_vhpiSubpBodyK,
    SubtypeDecl = vhpi_sys::vhpiClassKindT_vhpiSubtypeDeclK,
    Tool = vhpi_sys::vhpiClassKindT_vhpiToolK,
    Transaction = vhpi_sys::vhpiClassKindT_vhpiTransactionK,
    TypeConv = vhpi_sys::vhpiClassKindT_vhpiTypeConvK,
    UnitDecl = vhpi_sys::vhpiClassKindT_vhpiUnitDeclK,
    UserAttrName = vhpi_sys::vhpiClassKindT_vhpiUserAttrNameK,
    VarAssignStmt = vhpi_sys::vhpiClassKindT_vhpiVarAssignStmtK,
    VarDecl = vhpi_sys::vhpiClassKindT_vhpiVarDeclK,
    VarParamDecl = vhpi_sys::vhpiClassKindT_vhpiVarParamDeclK,
    WaitStmt = vhpi_sys::vhpiClassKindT_vhpiWaitStmtK,
    WaveformElem = vhpi_sys::vhpiClassKindT_vhpiWaveformElemK,
    WhileLoop = vhpi_sys::vhpiClassKindT_vhpiWhileLoopK,
    QualifiedExpr = vhpi_sys::vhpiClassKindT_vhpiQualifiedExprK,
    UseClause = vhpi_sys::vhpiClassKindT_vhpiUseClauseK,
    ConcAssertStmt = vhpi_sys::vhpiClassKindT_vhpiConcAssertStmtK,
    ForeverLoop = vhpi_sys::vhpiClassKindT_vhpiForeverLoopK,
    SeqAssertStmt = vhpi_sys::vhpiClassKindT_vhpiSeqAssertStmtK,
    SeqProcCallStmt = vhpi_sys::vhpiClassKindT_vhpiSeqProcCallStmtK,
    SeqSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSeqSigAssignStmtK,
    ProtectedTypeInst = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeInstK,
    VerilogModule = vhpi_sys::vhpiClassKindT_vhpiVerilogModuleK,
}

impl ClassKind {
    #[must_use]
    pub fn from_i32(value: i32) -> Option<ClassKind> {
        num_traits::FromPrimitive::from_i32(value)
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

        let cstr = unsafe { CStr::from_ptr(ptr.cast::<i8>()) };
        Some(iso8859_1_cstr_to_string(cstr))
    }

    #[must_use]
    pub fn get_phys(&self, property: PhysProperty) -> Physical {
        let result = unsafe { vhpi_sys::vhpi_get_phys(property as u32, self.as_raw()) };
        result.into()
    }

    #[must_use]
    pub fn get_real(&self, property: RealProperty) -> f64 {
        unsafe { vhpi_sys::vhpi_get_real(property as u32, self.as_raw()) }
    }

    // The following are convenience functions not defined by VHPI

    #[must_use]
    pub fn get_kind(&self) -> Option<ClassKind> {
        let kind_int = self.get(IntProperty::Kind);
        ClassKind::from_i32(kind_int)
    }

    #[must_use]
    pub fn get_name(&self) -> String {
        self.get_str(StrProperty::Name).unwrap()
    }

    #[must_use]
    pub fn get_full_name(&self) -> String {
        self.get_str(StrProperty::FullName).unwrap()
    }

    #[must_use]
    pub fn index_range(&self) -> Box<dyn Iterator<Item = i32>> {
        let raw = unsafe { vhpi_iterator(crate::OneToMany::Constraints as u32, self.as_raw()) };
        let handle = Handle::from_raw(unsafe { vhpi_scan(raw) });
        let is_up = handle.get(IntProperty::IsUp);
        let left = handle.get(IntProperty::LeftBound);
        let right = handle.get(IntProperty::RightBound);
        if is_up.is_zero() {
            Box::new((right..=left).rev())
        } else {
            Box::new(left..=right)
        }
    }

    #[must_use]
    pub fn enum_literals(&self) -> Option<Vec<String>> {
        if self.get_kind()? != ClassKind::EnumTypeDecl {
            return None;
        }

        self.iterator(crate::OneToMany::EnumLiterals)
            .map(|handle| handle.get_str(StrProperty::Name))
            .collect::<Option<Vec<String>>>()
    }
}
