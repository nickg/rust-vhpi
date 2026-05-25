#![cfg_attr(not(windows), allow(clippy::unnecessary_cast))]

use num_derive::FromPrimitive;
use num_traits::Zero;
use std::ffi::CStr;

use crate::{iso8859_1_cstr_to_string, Handle, Physical};

#[repr(u32)]
pub enum StrProperty {
    Name = vhpi_sys::vhpiStrPropertyT_vhpiNameP as u32,
    FullName = vhpi_sys::vhpiStrPropertyT_vhpiFullNameP as u32,
    CaseName = vhpi_sys::vhpiStrPropertyT_vhpiCaseNameP as u32,
    FullCaseName = vhpi_sys::vhpiStrPropertyT_vhpiFullCaseNameP as u32,
    CompName = vhpi_sys::vhpiStrPropertyT_vhpiCompNameP as u32,
    DefName = vhpi_sys::vhpiStrPropertyT_vhpiDefNameP as u32,
    FileName = vhpi_sys::vhpiStrPropertyT_vhpiFileNameP as u32,
    KindStr = vhpi_sys::vhpiStrPropertyT_vhpiKindStrP as u32,
    LabelName = vhpi_sys::vhpiStrPropertyT_vhpiLabelNameP as u32,
    LibLogicalName = vhpi_sys::vhpiStrPropertyT_vhpiLibLogicalNameP as u32,
    LibPhysicalName = vhpi_sys::vhpiStrPropertyT_vhpiLibPhysicalNameP as u32,
    LogicalName = vhpi_sys::vhpiStrPropertyT_vhpiLogicalNameP as u32,
    LoopLabelName = vhpi_sys::vhpiStrPropertyT_vhpiLoopLabelNameP as u32,
    StrVal = vhpi_sys::vhpiStrPropertyT_vhpiStrValP as u32,
    ToolVersion = vhpi_sys::vhpiStrPropertyT_vhpiToolVersionP as u32,
    UnitName = vhpi_sys::vhpiStrPropertyT_vhpiUnitNameP as u32,
    SaveRestartLocation = vhpi_sys::vhpiStrPropertyT_vhpiSaveRestartLocationP as u32,
    CompInstName = vhpi_sys::vhpiStrPropertyT_vhpiCompInstNameP as u32,
    InstNames = vhpi_sys::vhpiStrPropertyT_vhpiInstNamesP as u32,
    SignatureName = vhpi_sys::vhpiStrPropertyT_vhpiSignatureNameP as u32,
    SpecName = vhpi_sys::vhpiStrPropertyT_vhpiSpecNameP as u32,
}

#[repr(u32)]
pub enum IntProperty {
    Kind = vhpi_sys::vhpiIntPropertyT_vhpiKindP as u32,
    Access = vhpi_sys::vhpiIntPropertyT_vhpiAccessP as u32,
    Argc = vhpi_sys::vhpiIntPropertyT_vhpiArgcP as u32,
    AttrKind = vhpi_sys::vhpiIntPropertyT_vhpiAttrKindP as u32,
    BaseIndex = vhpi_sys::vhpiIntPropertyT_vhpiBaseIndexP as u32,
    BeginLineNo = vhpi_sys::vhpiIntPropertyT_vhpiBeginLineNoP as u32,
    EndLineNo = vhpi_sys::vhpiIntPropertyT_vhpiEndLineNoP as u32,
    EntityClass = vhpi_sys::vhpiIntPropertyT_vhpiEntityClassP as u32,
    ForeignKind = vhpi_sys::vhpiIntPropertyT_vhpiForeignKindP as u32,
    FrameLevel = vhpi_sys::vhpiIntPropertyT_vhpiFrameLevelP as u32,
    GenerateIndex = vhpi_sys::vhpiIntPropertyT_vhpiGenerateIndexP as u32,
    IntVal = vhpi_sys::vhpiIntPropertyT_vhpiIntValP as u32,
    IsAnonymous = vhpi_sys::vhpiIntPropertyT_vhpiIsAnonymousP as u32,
    IsBasic = vhpi_sys::vhpiIntPropertyT_vhpiIsBasicP as u32,
    IsComposite = vhpi_sys::vhpiIntPropertyT_vhpiIsCompositeP as u32,
    IsDefault = vhpi_sys::vhpiIntPropertyT_vhpiIsDefaultP as u32,
    IsDeferred = vhpi_sys::vhpiIntPropertyT_vhpiIsDeferredP as u32,
    IsDiscrete = vhpi_sys::vhpiIntPropertyT_vhpiIsDiscreteP as u32,
    IsForced = vhpi_sys::vhpiIntPropertyT_vhpiIsForcedP as u32,
    IsForeign = vhpi_sys::vhpiIntPropertyT_vhpiIsForeignP as u32,
    IsGuarded = vhpi_sys::vhpiIntPropertyT_vhpiIsGuardedP as u32,
    IsImplicitDecl = vhpi_sys::vhpiIntPropertyT_vhpiIsImplicitDeclP as u32,
    IsLocal = vhpi_sys::vhpiIntPropertyT_vhpiIsLocalP as u32,
    IsNamed = vhpi_sys::vhpiIntPropertyT_vhpiIsNamedP as u32,
    IsNull = vhpi_sys::vhpiIntPropertyT_vhpiIsNullP as u32,
    IsOpen = vhpi_sys::vhpiIntPropertyT_vhpiIsOpenP as u32,
    IsPLI = vhpi_sys::vhpiIntPropertyT_vhpiIsPLIP as u32,
    IsPassive = vhpi_sys::vhpiIntPropertyT_vhpiIsPassiveP as u32,
    IsPostponed = vhpi_sys::vhpiIntPropertyT_vhpiIsPostponedP as u32,
    IsProtectedType = vhpi_sys::vhpiIntPropertyT_vhpiIsProtectedTypeP as u32,
    IsPure = vhpi_sys::vhpiIntPropertyT_vhpiIsPureP as u32,
    IsResolved = vhpi_sys::vhpiIntPropertyT_vhpiIsResolvedP as u32,
    IsScalar = vhpi_sys::vhpiIntPropertyT_vhpiIsScalarP as u32,
    IsSeqStmt = vhpi_sys::vhpiIntPropertyT_vhpiIsSeqStmtP as u32,
    IsShared = vhpi_sys::vhpiIntPropertyT_vhpiIsSharedP as u32,
    IsTransport = vhpi_sys::vhpiIntPropertyT_vhpiIsTransportP as u32,
    IsUnaffected = vhpi_sys::vhpiIntPropertyT_vhpiIsUnaffectedP as u32,
    IsUnconstrained = vhpi_sys::vhpiIntPropertyT_vhpiIsUnconstrainedP as u32,
    IsUninstantiated = vhpi_sys::vhpiIntPropertyT_vhpiIsUninstantiatedP as u32,
    IsUp = vhpi_sys::vhpiIntPropertyT_vhpiIsUpP as u32,
    IsVital = vhpi_sys::vhpiIntPropertyT_vhpiIsVitalP as u32,
    IteratorType = vhpi_sys::vhpiIntPropertyT_vhpiIteratorTypeP as u32,
    LeftBound = vhpi_sys::vhpiIntPropertyT_vhpiLeftBoundP as u32,
    LineNo = vhpi_sys::vhpiIntPropertyT_vhpiLineNoP as u32,
    LineOffset = vhpi_sys::vhpiIntPropertyT_vhpiLineOffsetP as u32,
    LoopIndex = vhpi_sys::vhpiIntPropertyT_vhpiLoopIndexP as u32,
    Mode = vhpi_sys::vhpiIntPropertyT_vhpiModeP as u32,
    NumDimensions = vhpi_sys::vhpiIntPropertyT_vhpiNumDimensionsP as u32,
    NumFields = vhpi_sys::vhpiIntPropertyT_vhpiNumFieldsP as u32,
    NumGens = vhpi_sys::vhpiIntPropertyT_vhpiNumGensP as u32,
    NumLiterals = vhpi_sys::vhpiIntPropertyT_vhpiNumLiteralsP as u32,
    NumMembers = vhpi_sys::vhpiIntPropertyT_vhpiNumMembersP as u32,
    NumParams = vhpi_sys::vhpiIntPropertyT_vhpiNumParamsP as u32,
    NumPorts = vhpi_sys::vhpiIntPropertyT_vhpiNumPortsP as u32,
    OpenMode = vhpi_sys::vhpiIntPropertyT_vhpiOpenModeP as u32,
    Phase = vhpi_sys::vhpiIntPropertyT_vhpiPhaseP as u32,
    Position = vhpi_sys::vhpiIntPropertyT_vhpiPositionP as u32,
    PredefAttr = vhpi_sys::vhpiIntPropertyT_vhpiPredefAttrP as u32,
    Reason = vhpi_sys::vhpiIntPropertyT_vhpiReasonP as u32,
    RightBound = vhpi_sys::vhpiIntPropertyT_vhpiRightBoundP as u32,
    SigKind = vhpi_sys::vhpiIntPropertyT_vhpiSigKindP as u32,
    Size = vhpi_sys::vhpiIntPropertyT_vhpiSizeP as u32,
    StartLineNo = vhpi_sys::vhpiIntPropertyT_vhpiStartLineNoP as u32,
    State = vhpi_sys::vhpiIntPropertyT_vhpiStateP as u32,
    Staticness = vhpi_sys::vhpiIntPropertyT_vhpiStaticnessP as u32,
    VHDLversion = vhpi_sys::vhpiIntPropertyT_vhpiVHDLversionP as u32,
    Id = vhpi_sys::vhpiIntPropertyT_vhpiIdP as u32,
    Capabilities = vhpi_sys::vhpiIntPropertyT_vhpiCapabilitiesP as u32,
    AutomaticRestore = vhpi_sys::vhpiIntPropertyT_vhpiAutomaticRestoreP as u32,
    CompInstKind = vhpi_sys::vhpiIntPropertyT_vhpiCompInstKindP as u32,
    IsBuiltIn = vhpi_sys::vhpiIntPropertyT_vhpiIsBuiltInP as u32,
    IsDynamic = vhpi_sys::vhpiIntPropertyT_vhpiIsDynamicP as u32,
    IsOperator = vhpi_sys::vhpiIntPropertyT_vhpiIsOperatorP as u32,
    #[cfg(feature = "nvc")]
    RandomSeed = vhpi_sys::vhpiIntPropertyT_vhpiRandomSeedP as u32,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum RealProperty {
    FloatLeftBound = vhpi_sys::vhpiRealPropertyT_vhpiFloatLeftBoundP as u32,
    FloatRightBound = vhpi_sys::vhpiRealPropertyT_vhpiFloatRightBoundP as u32,
    RealVal = vhpi_sys::vhpiRealPropertyT_vhpiRealValP as u32,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum PhysProperty {
    PhysLeftBound = vhpi_sys::vhpiPhysPropertyT_vhpiPhysLeftBoundP as u32,
    PhysPosition = vhpi_sys::vhpiPhysPropertyT_vhpiPhysPositionP as u32,
    PhysRightBound = vhpi_sys::vhpiPhysPropertyT_vhpiPhysRightBoundP as u32,
    PhysVal = vhpi_sys::vhpiPhysPropertyT_vhpiPhysValP as u32,
    Time = vhpi_sys::vhpiPhysPropertyT_vhpiTimeP as u32,
    ResolutionLimit = vhpi_sys::vhpiPhysPropertyT_vhpiResolutionLimitP as u32,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum ClassKind {
    AccessTypeDecl = vhpi_sys::vhpiClassKindT_vhpiAccessTypeDeclK as u32,
    Aggregate = vhpi_sys::vhpiClassKindT_vhpiAggregateK as u32,
    AliasDecl = vhpi_sys::vhpiClassKindT_vhpiAliasDeclK as u32,
    All = vhpi_sys::vhpiClassKindT_vhpiAllK as u32,
    Allocator = vhpi_sys::vhpiClassKindT_vhpiAllocatorK as u32,
    AnyCollection = vhpi_sys::vhpiClassKindT_vhpiAnyCollectionK as u32,
    ArchBody = vhpi_sys::vhpiClassKindT_vhpiArchBodyK as u32,
    Argv = vhpi_sys::vhpiClassKindT_vhpiArgvK as u32,
    ArrayTypeDecl = vhpi_sys::vhpiClassKindT_vhpiArrayTypeDeclK as u32,
    AssocElem = vhpi_sys::vhpiClassKindT_vhpiAssocElemK as u32,
    AttrDecl = vhpi_sys::vhpiClassKindT_vhpiAttrDeclK as u32,
    AttrSpec = vhpi_sys::vhpiClassKindT_vhpiAttrSpecK as u32,
    BitStringLiteral = vhpi_sys::vhpiClassKindT_vhpiBitStringLiteralK as u32,
    BlockConfig = vhpi_sys::vhpiClassKindT_vhpiBlockConfigK as u32,
    BlockStmt = vhpi_sys::vhpiClassKindT_vhpiBlockStmtK as u32,
    Branch = vhpi_sys::vhpiClassKindT_vhpiBranchK as u32,
    Callback = vhpi_sys::vhpiClassKindT_vhpiCallbackK as u32,
    CaseStmt = vhpi_sys::vhpiClassKindT_vhpiCaseStmtK as u32,
    CharLiteral = vhpi_sys::vhpiClassKindT_vhpiCharLiteralK as u32,
    CompConfig = vhpi_sys::vhpiClassKindT_vhpiCompConfigK as u32,
    CompDecl = vhpi_sys::vhpiClassKindT_vhpiCompDeclK as u32,
    CompInstStmt = vhpi_sys::vhpiClassKindT_vhpiCompInstStmtK as u32,
    CondSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiCondSigAssignStmtK as u32,
    CondWaveform = vhpi_sys::vhpiClassKindT_vhpiCondWaveformK as u32,
    ConfigDecl = vhpi_sys::vhpiClassKindT_vhpiConfigDeclK as u32,
    ConstDecl = vhpi_sys::vhpiClassKindT_vhpiConstDeclK as u32,
    ConstParamDecl = vhpi_sys::vhpiClassKindT_vhpiConstParamDeclK as u32,
    DerefObj = vhpi_sys::vhpiClassKindT_vhpiDerefObjK as u32,
    DisconnectSpec = vhpi_sys::vhpiClassKindT_vhpiDisconnectSpecK as u32,
    Driver = vhpi_sys::vhpiClassKindT_vhpiDriverK as u32,
    DriverCollection = vhpi_sys::vhpiClassKindT_vhpiDriverCollectionK as u32,
    ElemAssoc = vhpi_sys::vhpiClassKindT_vhpiElemAssocK as u32,
    ElemDecl = vhpi_sys::vhpiClassKindT_vhpiElemDeclK as u32,
    EntityClassEntry = vhpi_sys::vhpiClassKindT_vhpiEntityClassEntryK as u32,
    EntityDecl = vhpi_sys::vhpiClassKindT_vhpiEntityDeclK as u32,
    EnumLiteral = vhpi_sys::vhpiClassKindT_vhpiEnumLiteralK as u32,
    EnumRange = vhpi_sys::vhpiClassKindT_vhpiEnumRangeK as u32,
    EnumTypeDecl = vhpi_sys::vhpiClassKindT_vhpiEnumTypeDeclK as u32,
    ExitStmt = vhpi_sys::vhpiClassKindT_vhpiExitStmtK as u32,
    FileDecl = vhpi_sys::vhpiClassKindT_vhpiFileDeclK as u32,
    FileParamDecl = vhpi_sys::vhpiClassKindT_vhpiFileParamDeclK as u32,
    FileTypeDecl = vhpi_sys::vhpiClassKindT_vhpiFileTypeDeclK as u32,
    FloatRange = vhpi_sys::vhpiClassKindT_vhpiFloatRangeK as u32,
    FloatTypeDecl = vhpi_sys::vhpiClassKindT_vhpiFloatTypeDeclK as u32,
    ForGenerate = vhpi_sys::vhpiClassKindT_vhpiForGenerateK as u32,
    ForLoop = vhpi_sys::vhpiClassKindT_vhpiForLoopK as u32,
    Foreignf = vhpi_sys::vhpiClassKindT_vhpiForeignfK as u32,
    FuncCall = vhpi_sys::vhpiClassKindT_vhpiFuncCallK as u32,
    FuncDecl = vhpi_sys::vhpiClassKindT_vhpiFuncDeclK as u32,
    GenericDecl = vhpi_sys::vhpiClassKindT_vhpiGenericDeclK as u32,
    GroupDecl = vhpi_sys::vhpiClassKindT_vhpiGroupDeclK as u32,
    GroupTempDecl = vhpi_sys::vhpiClassKindT_vhpiGroupTempDeclK as u32,
    IfGenerate = vhpi_sys::vhpiClassKindT_vhpiIfGenerateK as u32,
    IfStmt = vhpi_sys::vhpiClassKindT_vhpiIfStmtK as u32,
    InPort = vhpi_sys::vhpiClassKindT_vhpiInPortK as u32,
    IndexedName = vhpi_sys::vhpiClassKindT_vhpiIndexedNameK as u32,
    IntLiteral = vhpi_sys::vhpiClassKindT_vhpiIntLiteralK as u32,
    IntRange = vhpi_sys::vhpiClassKindT_vhpiIntRangeK as u32,
    IntTypeDecl = vhpi_sys::vhpiClassKindT_vhpiIntTypeDeclK as u32,
    Iterator = vhpi_sys::vhpiClassKindT_vhpiIteratorK as u32,
    LibraryDecl = vhpi_sys::vhpiClassKindT_vhpiLibraryDeclK as u32,
    NextStmt = vhpi_sys::vhpiClassKindT_vhpiNextStmtK as u32,
    NullLiteral = vhpi_sys::vhpiClassKindT_vhpiNullLiteralK as u32,
    NullStmt = vhpi_sys::vhpiClassKindT_vhpiNullStmtK as u32,
    Others = vhpi_sys::vhpiClassKindT_vhpiOthersK as u32,
    OutPort = vhpi_sys::vhpiClassKindT_vhpiOutPortK as u32,
    PackBody = vhpi_sys::vhpiClassKindT_vhpiPackBodyK as u32,
    PackDecl = vhpi_sys::vhpiClassKindT_vhpiPackDeclK as u32,
    PackInst = vhpi_sys::vhpiClassKindT_vhpiPackInstK as u32,
    ParamAttrName = vhpi_sys::vhpiClassKindT_vhpiParamAttrNameK as u32,
    PhysLiteral = vhpi_sys::vhpiClassKindT_vhpiPhysLiteralK as u32,
    PhysRange = vhpi_sys::vhpiClassKindT_vhpiPhysRangeK as u32,
    PhysTypeDecl = vhpi_sys::vhpiClassKindT_vhpiPhysTypeDeclK as u32,
    PortDecl = vhpi_sys::vhpiClassKindT_vhpiPortDeclK as u32,
    ProcDecl = vhpi_sys::vhpiClassKindT_vhpiProcDeclK as u32,
    ProcessStmt = vhpi_sys::vhpiClassKindT_vhpiProcessStmtK as u32,
    ProtectedTypeBody = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeBodyK as u32,
    ProtectedTypeDecl = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeDeclK as u32,
    RealLiteral = vhpi_sys::vhpiClassKindT_vhpiRealLiteralK as u32,
    RecordTypeDecl = vhpi_sys::vhpiClassKindT_vhpiRecordTypeDeclK as u32,
    ReportStmt = vhpi_sys::vhpiClassKindT_vhpiReportStmtK as u32,
    ReturnStmt = vhpi_sys::vhpiClassKindT_vhpiReturnStmtK as u32,
    RootInst = vhpi_sys::vhpiClassKindT_vhpiRootInstK as u32,
    SelectSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSelectSigAssignStmtK as u32,
    SelectWaveform = vhpi_sys::vhpiClassKindT_vhpiSelectWaveformK as u32,
    SelectedName = vhpi_sys::vhpiClassKindT_vhpiSelectedNameK as u32,
    SigDecl = vhpi_sys::vhpiClassKindT_vhpiSigDeclK as u32,
    SigParamDecl = vhpi_sys::vhpiClassKindT_vhpiSigParamDeclK as u32,
    SimpAttrName = vhpi_sys::vhpiClassKindT_vhpiSimpAttrNameK as u32,
    SimpleSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSimpleSigAssignStmtK as u32,
    SliceName = vhpi_sys::vhpiClassKindT_vhpiSliceNameK as u32,
    StringLiteral = vhpi_sys::vhpiClassKindT_vhpiStringLiteralK as u32,
    SubpBody = vhpi_sys::vhpiClassKindT_vhpiSubpBodyK as u32,
    SubtypeDecl = vhpi_sys::vhpiClassKindT_vhpiSubtypeDeclK as u32,
    Tool = vhpi_sys::vhpiClassKindT_vhpiToolK as u32,
    Transaction = vhpi_sys::vhpiClassKindT_vhpiTransactionK as u32,
    TypeConv = vhpi_sys::vhpiClassKindT_vhpiTypeConvK as u32,
    UnitDecl = vhpi_sys::vhpiClassKindT_vhpiUnitDeclK as u32,
    UserAttrName = vhpi_sys::vhpiClassKindT_vhpiUserAttrNameK as u32,
    VarAssignStmt = vhpi_sys::vhpiClassKindT_vhpiVarAssignStmtK as u32,
    VarDecl = vhpi_sys::vhpiClassKindT_vhpiVarDeclK as u32,
    VarParamDecl = vhpi_sys::vhpiClassKindT_vhpiVarParamDeclK as u32,
    WaitStmt = vhpi_sys::vhpiClassKindT_vhpiWaitStmtK as u32,
    WaveformElem = vhpi_sys::vhpiClassKindT_vhpiWaveformElemK as u32,
    WhileLoop = vhpi_sys::vhpiClassKindT_vhpiWhileLoopK as u32,
    QualifiedExpr = vhpi_sys::vhpiClassKindT_vhpiQualifiedExprK as u32,
    UseClause = vhpi_sys::vhpiClassKindT_vhpiUseClauseK as u32,
    ConcAssertStmt = vhpi_sys::vhpiClassKindT_vhpiConcAssertStmtK as u32,
    ForeverLoop = vhpi_sys::vhpiClassKindT_vhpiForeverLoopK as u32,
    SeqAssertStmt = vhpi_sys::vhpiClassKindT_vhpiSeqAssertStmtK as u32,
    SeqProcCallStmt = vhpi_sys::vhpiClassKindT_vhpiSeqProcCallStmtK as u32,
    SeqSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSeqSigAssignStmtK as u32,
    ProtectedTypeInst = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeInstK as u32,
    #[cfg(feature = "nvc")]
    VerilogModule = vhpi_sys::vhpiClassKindT_vhpiVerilogModuleK as u32,
}

impl ClassKind {
    #[must_use]
    pub fn from_i32(value: i32) -> Option<ClassKind> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum Mode {
    In = vhpi_sys::vhpiModeT_vhpiInMode as u32,
    Out = vhpi_sys::vhpiModeT_vhpiOutMode as u32,
    Inout = vhpi_sys::vhpiModeT_vhpiInoutMode as u32,
    Buffer = vhpi_sys::vhpiModeT_vhpiBufferMode as u32,
    Linkage = vhpi_sys::vhpiModeT_vhpiLinkageMode as u32,
}

impl Mode {
    #[must_use]
    pub fn from_i32(value: i32) -> Option<Mode> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum SigKind {
    Register = vhpi_sys::vhpiSigKindT_vhpiRegister as u32,
    Bus = vhpi_sys::vhpiSigKindT_vhpiBus as u32,
    Normal = vhpi_sys::vhpiSigKindT_vhpiNormal as u32,
}

impl SigKind {
    #[must_use]
    pub fn from_i32(value: i32) -> Option<SigKind> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum Staticness {
    LocallyStatic = vhpi_sys::vhpiStaticnessT_vhpiLocallyStatic as u32,
    GloballyStatic = vhpi_sys::vhpiStaticnessT_vhpiGloballyStatic as u32,
    Dynamic = vhpi_sys::vhpiStaticnessT_vhpiDynamic as u32,
}

impl Staticness {
    #[must_use]
    pub fn from_i32(value: i32) -> Option<Staticness> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum PredefAttr {
    Active = vhpi_sys::vhpiPredefAttrT_vhpiActivePA as u32,
    Ascending = vhpi_sys::vhpiPredefAttrT_vhpiAscendingPA as u32,
    Base = vhpi_sys::vhpiPredefAttrT_vhpiBasePA as u32,
    Delayed = vhpi_sys::vhpiPredefAttrT_vhpiDelayedPA as u32,
    Driving = vhpi_sys::vhpiPredefAttrT_vhpiDrivingPA as u32,
    DrivingValue = vhpi_sys::vhpiPredefAttrT_vhpiDriving_valuePA as u32,
    Event = vhpi_sys::vhpiPredefAttrT_vhpiEventPA as u32,
    High = vhpi_sys::vhpiPredefAttrT_vhpiHighPA as u32,
    Image = vhpi_sys::vhpiPredefAttrT_vhpiImagePA as u32,
    InstanceName = vhpi_sys::vhpiPredefAttrT_vhpiInstance_namePA as u32,
    LastActive = vhpi_sys::vhpiPredefAttrT_vhpiLast_activePA as u32,
    LastEvent = vhpi_sys::vhpiPredefAttrT_vhpiLast_eventPA as u32,
    LastValue = vhpi_sys::vhpiPredefAttrT_vhpiLast_valuePA as u32,
    Left = vhpi_sys::vhpiPredefAttrT_vhpiLeftPA as u32,
    LeftOf = vhpi_sys::vhpiPredefAttrT_vhpiLeftofPA as u32,
    Length = vhpi_sys::vhpiPredefAttrT_vhpiLengthPA as u32,
    Low = vhpi_sys::vhpiPredefAttrT_vhpiLowPA as u32,
    PathName = vhpi_sys::vhpiPredefAttrT_vhpiPath_namePA as u32,
    Pos = vhpi_sys::vhpiPredefAttrT_vhpiPosPA as u32,
    Pred = vhpi_sys::vhpiPredefAttrT_vhpiPredPA as u32,
    Quiet = vhpi_sys::vhpiPredefAttrT_vhpiQuietPA as u32,
    Range = vhpi_sys::vhpiPredefAttrT_vhpiRangePA as u32,
    ReverseRange = vhpi_sys::vhpiPredefAttrT_vhpiReverse_rangePA as u32,
    Right = vhpi_sys::vhpiPredefAttrT_vhpiRightPA as u32,
    RightOf = vhpi_sys::vhpiPredefAttrT_vhpiRightofPA as u32,
    SimpleName = vhpi_sys::vhpiPredefAttrT_vhpiSimple_namePA as u32,
    Stable = vhpi_sys::vhpiPredefAttrT_vhpiStablePA as u32,
    Succ = vhpi_sys::vhpiPredefAttrT_vhpiSuccPA as u32,
    Transaction = vhpi_sys::vhpiPredefAttrT_vhpiTransactionPA as u32,
    Val = vhpi_sys::vhpiPredefAttrT_vhpiValPA as u32,
    Value = vhpi_sys::vhpiPredefAttrT_vhpiValuePA as u32,
}

impl PredefAttr {
    #[must_use]
    pub fn from_i32(value: i32) -> Option<PredefAttr> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

impl Handle {
    #[must_use]
    pub fn get(&self, property: IntProperty) -> i32 {
        unsafe { crate::ffi::vhpi_get(property as vhpi_sys::vhpiIntPropertyT, self.as_raw()) }
    }

    #[must_use]
    pub fn get_str(&self, property: StrProperty) -> Option<String> {
        let ptr = unsafe {
            crate::ffi::vhpi_get_str(property as vhpi_sys::vhpiStrPropertyT, self.as_raw())
        };
        if ptr.is_null() {
            return None;
        }

        let cstr = unsafe { CStr::from_ptr(ptr.cast::<i8>()) };
        Some(iso8859_1_cstr_to_string(cstr))
    }

    #[must_use]
    pub fn get_phys(&self, property: PhysProperty) -> Physical {
        let result = unsafe {
            crate::ffi::vhpi_get_phys(property as vhpi_sys::vhpiPhysPropertyT, self.as_raw())
        };
        result.into()
    }

    #[must_use]
    pub fn get_real(&self, property: RealProperty) -> f64 {
        unsafe { crate::ffi::vhpi_get_real(property as vhpi_sys::vhpiRealPropertyT, self.as_raw()) }
    }

    // The following are convenience functions not defined by VHPI

    #[must_use]
    pub fn get_kind(&self) -> Option<ClassKind> {
        let kind_int = self.get(IntProperty::Kind);
        ClassKind::from_i32(kind_int)
    }

    #[must_use]
    pub fn get_name(&self) -> Option<String> {
        self.get_str(StrProperty::Name)
    }

    #[must_use]
    pub fn get_full_name(&self) -> Option<String> {
        self.get_str(StrProperty::FullName)
    }

    #[must_use]
    pub fn get_mode(&self) -> Option<Mode> {
        let mode_int = self.get(IntProperty::Mode);
        Mode::from_i32(mode_int)
    }

    #[must_use]
    pub fn get_sig_kind(&self) -> Option<SigKind> {
        let sig_kind_int = self.get(IntProperty::SigKind);
        SigKind::from_i32(sig_kind_int)
    }

    #[must_use]
    pub fn get_staticness(&self) -> Option<Staticness> {
        let staticness_int = self.get(IntProperty::Staticness);
        Staticness::from_i32(staticness_int)
    }

    #[must_use]
    pub fn get_predef_attr(&self) -> Option<PredefAttr> {
        let predef_attr_int = self.get(IntProperty::PredefAttr);
        PredefAttr::from_i32(predef_attr_int)
    }

    #[must_use]
    pub fn index_range(&self) -> Box<dyn Iterator<Item = i32>> {
        let raw = unsafe {
            crate::ffi::vhpi_iterator(
                crate::OneToMany::Constraints as vhpi_sys::vhpiOneToManyT,
                self.as_raw(),
            )
        };
        let handle = Handle::from_raw(unsafe { crate::ffi::vhpi_scan(raw) });
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
