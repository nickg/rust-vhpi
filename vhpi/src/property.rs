#![cfg_attr(not(windows), allow(clippy::unnecessary_cast))]

use num_derive::FromPrimitive;
use num_traits::Zero;
use std::ffi::CStr;
use std::os::raw::c_char;
use vhpi_sys::{vhpi_get, vhpi_get_phys, vhpi_get_real, vhpi_get_str, vhpi_iterator, vhpi_scan};

use crate::{iso8859_1_cstr_to_string, Handle, Physical};

#[repr(u32)]
/// String-valued properties that can be queried from a VHPI handle.
pub enum StrProperty {
    /// Simple name of the object.
    Name = vhpi_sys::vhpiStrPropertyT_vhpiNameP as u32,
    /// Hierarchical full name of the object.
    FullName = vhpi_sys::vhpiStrPropertyT_vhpiFullNameP as u32,
    /// Case-preserving simple name.
    CaseName = vhpi_sys::vhpiStrPropertyT_vhpiCaseNameP as u32,
    /// Case-preserving hierarchical full name.
    FullCaseName = vhpi_sys::vhpiStrPropertyT_vhpiFullCaseNameP as u32,
    /// Component name.
    CompName = vhpi_sys::vhpiStrPropertyT_vhpiCompNameP as u32,
    /// Defining name.
    DefName = vhpi_sys::vhpiStrPropertyT_vhpiDefNameP as u32,
    /// Source file name.
    FileName = vhpi_sys::vhpiStrPropertyT_vhpiFileNameP as u32,
    /// String representation of the object's kind.
    KindStr = vhpi_sys::vhpiStrPropertyT_vhpiKindStrP as u32,
    /// Label name.
    LabelName = vhpi_sys::vhpiStrPropertyT_vhpiLabelNameP as u32,
    /// Logical library name.
    LibLogicalName = vhpi_sys::vhpiStrPropertyT_vhpiLibLogicalNameP as u32,
    /// Physical library path/name.
    LibPhysicalName = vhpi_sys::vhpiStrPropertyT_vhpiLibPhysicalNameP as u32,
    /// Logical object name.
    LogicalName = vhpi_sys::vhpiStrPropertyT_vhpiLogicalNameP as u32,
    /// Loop label name.
    LoopLabelName = vhpi_sys::vhpiStrPropertyT_vhpiLoopLabelNameP as u32,
    /// String value of the object.
    StrVal = vhpi_sys::vhpiStrPropertyT_vhpiStrValP as u32,
    /// Simulator/tool version string.
    ToolVersion = vhpi_sys::vhpiStrPropertyT_vhpiToolVersionP as u32,
    /// Unit name.
    UnitName = vhpi_sys::vhpiStrPropertyT_vhpiUnitNameP as u32,
    /// Save/restart storage location.
    SaveRestartLocation = vhpi_sys::vhpiStrPropertyT_vhpiSaveRestartLocationP as u32,
    /// Component instance name.
    CompInstName = vhpi_sys::vhpiStrPropertyT_vhpiCompInstNameP as u32,
    /// Instance-name list representation.
    InstNames = vhpi_sys::vhpiStrPropertyT_vhpiInstNamesP as u32,
    /// Subprogram signature name.
    SignatureName = vhpi_sys::vhpiStrPropertyT_vhpiSignatureNameP as u32,
    /// Specification name.
    SpecName = vhpi_sys::vhpiStrPropertyT_vhpiSpecNameP as u32,
}

#[repr(u32)]
/// Integer-valued properties that can be queried from a VHPI handle.
pub enum IntProperty {
    /// Object kind identifier.
    Kind = vhpi_sys::vhpiIntPropertyT_vhpiKindP as u32,
    /// Access type/category.
    Access = vhpi_sys::vhpiIntPropertyT_vhpiAccessP as u32,
    /// Number of command-line arguments.
    Argc = vhpi_sys::vhpiIntPropertyT_vhpiArgcP as u32,
    /// Attribute kind identifier.
    AttrKind = vhpi_sys::vhpiIntPropertyT_vhpiAttrKindP as u32,
    /// Base index value.
    BaseIndex = vhpi_sys::vhpiIntPropertyT_vhpiBaseIndexP as u32,
    /// Source begin line number.
    BeginLineNo = vhpi_sys::vhpiIntPropertyT_vhpiBeginLineNoP as u32,
    /// Source end line number.
    EndLineNo = vhpi_sys::vhpiIntPropertyT_vhpiEndLineNoP as u32,
    /// Entity class identifier.
    EntityClass = vhpi_sys::vhpiIntPropertyT_vhpiEntityClassP as u32,
    /// Foreign-interface kind.
    ForeignKind = vhpi_sys::vhpiIntPropertyT_vhpiForeignKindP as u32,
    /// Stack frame nesting level.
    FrameLevel = vhpi_sys::vhpiIntPropertyT_vhpiFrameLevelP as u32,
    /// Generate index value.
    GenerateIndex = vhpi_sys::vhpiIntPropertyT_vhpiGenerateIndexP as u32,
    /// Integer value of the object.
    IntVal = vhpi_sys::vhpiIntPropertyT_vhpiIntValP as u32,
    /// Whether the object is anonymous.
    IsAnonymous = vhpi_sys::vhpiIntPropertyT_vhpiIsAnonymousP as u32,
    /// Whether the object is of a basic type.
    IsBasic = vhpi_sys::vhpiIntPropertyT_vhpiIsBasicP as u32,
    /// Whether the object is composite.
    IsComposite = vhpi_sys::vhpiIntPropertyT_vhpiIsCompositeP as u32,
    /// Whether this is a default value/association.
    IsDefault = vhpi_sys::vhpiIntPropertyT_vhpiIsDefaultP as u32,
    /// Whether declaration is deferred.
    IsDeferred = vhpi_sys::vhpiIntPropertyT_vhpiIsDeferredP as u32,
    /// Whether type is discrete.
    IsDiscrete = vhpi_sys::vhpiIntPropertyT_vhpiIsDiscreteP as u32,
    /// Whether the object is currently forced.
    IsForced = vhpi_sys::vhpiIntPropertyT_vhpiIsForcedP as u32,
    /// Whether object/declaration is foreign.
    IsForeign = vhpi_sys::vhpiIntPropertyT_vhpiIsForeignP as u32,
    /// Whether object is guarded.
    IsGuarded = vhpi_sys::vhpiIntPropertyT_vhpiIsGuardedP as u32,
    /// Whether declaration is implicit.
    IsImplicitDecl = vhpi_sys::vhpiIntPropertyT_vhpiIsImplicitDeclP as u32,
    /// Whether object is local.
    IsLocal = vhpi_sys::vhpiIntPropertyT_vhpiIsLocalP as u32,
    /// Whether object has an explicit name.
    IsNamed = vhpi_sys::vhpiIntPropertyT_vhpiIsNamedP as u32,
    /// Whether object/value is null.
    IsNull = vhpi_sys::vhpiIntPropertyT_vhpiIsNullP as u32,
    /// Whether association/open item is open.
    IsOpen = vhpi_sys::vhpiIntPropertyT_vhpiIsOpenP as u32,
    /// Whether object is PLI-originated.
    IsPLI = vhpi_sys::vhpiIntPropertyT_vhpiIsPLIP as u32,
    /// Whether process/component is passive.
    IsPassive = vhpi_sys::vhpiIntPropertyT_vhpiIsPassiveP as u32,
    /// Whether process/statement is postponed.
    IsPostponed = vhpi_sys::vhpiIntPropertyT_vhpiIsPostponedP as u32,
    /// Whether type is protected.
    IsProtectedType = vhpi_sys::vhpiIntPropertyT_vhpiIsProtectedTypeP as u32,
    /// Whether subprogram is pure.
    IsPure = vhpi_sys::vhpiIntPropertyT_vhpiIsPureP as u32,
    /// Whether signal/type is resolved.
    IsResolved = vhpi_sys::vhpiIntPropertyT_vhpiIsResolvedP as u32,
    /// Whether type/object is scalar.
    IsScalar = vhpi_sys::vhpiIntPropertyT_vhpiIsScalarP as u32,
    /// Whether statement is sequential.
    IsSeqStmt = vhpi_sys::vhpiIntPropertyT_vhpiIsSeqStmtP as u32,
    /// Whether object is shared.
    IsShared = vhpi_sys::vhpiIntPropertyT_vhpiIsSharedP as u32,
    /// Whether signal assignment uses transport delay.
    IsTransport = vhpi_sys::vhpiIntPropertyT_vhpiIsTransportP as u32,
    /// Whether assignment includes unaffected choice.
    IsUnaffected = vhpi_sys::vhpiIntPropertyT_vhpiIsUnaffectedP as u32,
    /// Whether type/subtype is unconstrained.
    IsUnconstrained = vhpi_sys::vhpiIntPropertyT_vhpiIsUnconstrainedP as u32,
    /// Whether unit/instance is uninstantiated.
    IsUninstantiated = vhpi_sys::vhpiIntPropertyT_vhpiIsUninstantiatedP as u32,
    /// Whether index/range direction is ascending.
    IsUp = vhpi_sys::vhpiIntPropertyT_vhpiIsUpP as u32,
    /// Whether object relates to VITAL semantics.
    IsVital = vhpi_sys::vhpiIntPropertyT_vhpiIsVitalP as u32,
    /// Iterator element/category type.
    IteratorType = vhpi_sys::vhpiIntPropertyT_vhpiIteratorTypeP as u32,
    /// Left bound of a range.
    LeftBound = vhpi_sys::vhpiIntPropertyT_vhpiLeftBoundP as u32,
    /// Source line number.
    LineNo = vhpi_sys::vhpiIntPropertyT_vhpiLineNoP as u32,
    /// Column/offset in source line.
    LineOffset = vhpi_sys::vhpiIntPropertyT_vhpiLineOffsetP as u32,
    /// Loop index value.
    LoopIndex = vhpi_sys::vhpiIntPropertyT_vhpiLoopIndexP as u32,
    /// Interface mode.
    Mode = vhpi_sys::vhpiIntPropertyT_vhpiModeP as u32,
    /// Number of dimensions.
    NumDimensions = vhpi_sys::vhpiIntPropertyT_vhpiNumDimensionsP as u32,
    /// Number of record/aggregate fields.
    NumFields = vhpi_sys::vhpiIntPropertyT_vhpiNumFieldsP as u32,
    /// Number of generics.
    NumGens = vhpi_sys::vhpiIntPropertyT_vhpiNumGensP as u32,
    /// Number of enumeration literals.
    NumLiterals = vhpi_sys::vhpiIntPropertyT_vhpiNumLiteralsP as u32,
    /// Number of members.
    NumMembers = vhpi_sys::vhpiIntPropertyT_vhpiNumMembersP as u32,
    /// Number of parameters.
    NumParams = vhpi_sys::vhpiIntPropertyT_vhpiNumParamsP as u32,
    /// Number of ports.
    NumPorts = vhpi_sys::vhpiIntPropertyT_vhpiNumPortsP as u32,
    /// File open mode.
    OpenMode = vhpi_sys::vhpiIntPropertyT_vhpiOpenModeP as u32,
    /// Simulation phase identifier.
    Phase = vhpi_sys::vhpiIntPropertyT_vhpiPhaseP as u32,
    /// Positional index value.
    Position = vhpi_sys::vhpiIntPropertyT_vhpiPositionP as u32,
    /// Predefined-attribute identifier.
    PredefAttr = vhpi_sys::vhpiIntPropertyT_vhpiPredefAttrP as u32,
    /// Callback/control reason code.
    Reason = vhpi_sys::vhpiIntPropertyT_vhpiReasonP as u32,
    /// Right bound of a range.
    RightBound = vhpi_sys::vhpiIntPropertyT_vhpiRightBoundP as u32,
    /// Signal kind identifier.
    SigKind = vhpi_sys::vhpiIntPropertyT_vhpiSigKindP as u32,
    /// Size/width value.
    Size = vhpi_sys::vhpiIntPropertyT_vhpiSizeP as u32,
    /// Start line number in source.
    StartLineNo = vhpi_sys::vhpiIntPropertyT_vhpiStartLineNoP as u32,
    /// State code.
    State = vhpi_sys::vhpiIntPropertyT_vhpiStateP as u32,
    /// Staticness classification.
    Staticness = vhpi_sys::vhpiIntPropertyT_vhpiStaticnessP as u32,
    /// VHDL language version identifier.
    VHDLversion = vhpi_sys::vhpiIntPropertyT_vhpiVHDLversionP as u32,
    /// Implementation-defined identifier.
    Id = vhpi_sys::vhpiIntPropertyT_vhpiIdP as u32,
    /// Simulator capability bitmask.
    Capabilities = vhpi_sys::vhpiIntPropertyT_vhpiCapabilitiesP as u32,
    /// Automatic restore setting.
    AutomaticRestore = vhpi_sys::vhpiIntPropertyT_vhpiAutomaticRestoreP as u32,
    /// Component instance kind identifier.
    CompInstKind = vhpi_sys::vhpiIntPropertyT_vhpiCompInstKindP as u32,
    /// Whether object is built in.
    IsBuiltIn = vhpi_sys::vhpiIntPropertyT_vhpiIsBuiltInP as u32,
    /// Whether object is dynamically created/resolved.
    IsDynamic = vhpi_sys::vhpiIntPropertyT_vhpiIsDynamicP as u32,
    /// Whether object denotes an operator.
    IsOperator = vhpi_sys::vhpiIntPropertyT_vhpiIsOperatorP as u32,
    #[cfg(feature = "nvc")]
    /// Simulator random seed. Requires `nvc` feature.
    RandomSeed = vhpi_sys::vhpiIntPropertyT_vhpiRandomSeedP as u32,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
/// Floating-point properties that can be queried from a VHPI handle.
pub enum RealProperty {
    /// Left bound of a floating-point range.
    FloatLeftBound = vhpi_sys::vhpiRealPropertyT_vhpiFloatLeftBoundP as u32,
    /// Right bound of a floating-point range.
    FloatRightBound = vhpi_sys::vhpiRealPropertyT_vhpiFloatRightBoundP as u32,
    /// Floating-point value.
    RealVal = vhpi_sys::vhpiRealPropertyT_vhpiRealValP as u32,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
/// Physical-quantity properties that can be queried from a VHPI handle.
pub enum PhysProperty {
    /// Left bound of a physical range.
    PhysLeftBound = vhpi_sys::vhpiPhysPropertyT_vhpiPhysLeftBoundP as u32,
    /// Position within a physical type.
    PhysPosition = vhpi_sys::vhpiPhysPropertyT_vhpiPhysPositionP as u32,
    /// Right bound of a physical range.
    PhysRightBound = vhpi_sys::vhpiPhysPropertyT_vhpiPhysRightBoundP as u32,
    /// Physical value.
    PhysVal = vhpi_sys::vhpiPhysPropertyT_vhpiPhysValP as u32,
    /// Simulation time value.
    Time = vhpi_sys::vhpiPhysPropertyT_vhpiTimeP as u32,
    /// Simulator time resolution limit.
    ResolutionLimit = vhpi_sys::vhpiPhysPropertyT_vhpiResolutionLimitP as u32,
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
/// VHDL class/kind identifiers returned by `IntProperty::Kind`.
pub enum ClassKind {
    /// Access type declaration.
    AccessTypeDecl = vhpi_sys::vhpiClassKindT_vhpiAccessTypeDeclK as u32,
    /// Aggregate expression.
    Aggregate = vhpi_sys::vhpiClassKindT_vhpiAggregateK as u32,
    /// Alias declaration.
    AliasDecl = vhpi_sys::vhpiClassKindT_vhpiAliasDeclK as u32,
    /// `all` choice/object.
    All = vhpi_sys::vhpiClassKindT_vhpiAllK as u32,
    /// Allocator expression.
    Allocator = vhpi_sys::vhpiClassKindT_vhpiAllocatorK as u32,
    /// Collection of arbitrary objects.
    AnyCollection = vhpi_sys::vhpiClassKindT_vhpiAnyCollectionK as u32,
    /// Architecture body.
    ArchBody = vhpi_sys::vhpiClassKindT_vhpiArchBodyK as u32,
    /// Command-line argument object.
    Argv = vhpi_sys::vhpiClassKindT_vhpiArgvK as u32,
    /// Array type declaration.
    ArrayTypeDecl = vhpi_sys::vhpiClassKindT_vhpiArrayTypeDeclK as u32,
    /// Association element.
    AssocElem = vhpi_sys::vhpiClassKindT_vhpiAssocElemK as u32,
    /// Attribute declaration.
    AttrDecl = vhpi_sys::vhpiClassKindT_vhpiAttrDeclK as u32,
    /// Attribute specification.
    AttrSpec = vhpi_sys::vhpiClassKindT_vhpiAttrSpecK as u32,
    /// Bit-string literal.
    BitStringLiteral = vhpi_sys::vhpiClassKindT_vhpiBitStringLiteralK as u32,
    /// Block configuration.
    BlockConfig = vhpi_sys::vhpiClassKindT_vhpiBlockConfigK as u32,
    /// Block statement.
    BlockStmt = vhpi_sys::vhpiClassKindT_vhpiBlockStmtK as u32,
    /// Branch alternative.
    Branch = vhpi_sys::vhpiClassKindT_vhpiBranchK as u32,
    /// Callback object.
    Callback = vhpi_sys::vhpiClassKindT_vhpiCallbackK as u32,
    /// Case statement.
    CaseStmt = vhpi_sys::vhpiClassKindT_vhpiCaseStmtK as u32,
    /// Character literal.
    CharLiteral = vhpi_sys::vhpiClassKindT_vhpiCharLiteralK as u32,
    /// Component configuration.
    CompConfig = vhpi_sys::vhpiClassKindT_vhpiCompConfigK as u32,
    /// Component declaration.
    CompDecl = vhpi_sys::vhpiClassKindT_vhpiCompDeclK as u32,
    /// Component instantiation statement.
    CompInstStmt = vhpi_sys::vhpiClassKindT_vhpiCompInstStmtK as u32,
    /// Concurrent conditional signal assignment.
    CondSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiCondSigAssignStmtK as u32,
    /// Conditional waveform item.
    CondWaveform = vhpi_sys::vhpiClassKindT_vhpiCondWaveformK as u32,
    /// Configuration declaration.
    ConfigDecl = vhpi_sys::vhpiClassKindT_vhpiConfigDeclK as u32,
    /// Constant declaration.
    ConstDecl = vhpi_sys::vhpiClassKindT_vhpiConstDeclK as u32,
    /// Constant parameter declaration.
    ConstParamDecl = vhpi_sys::vhpiClassKindT_vhpiConstParamDeclK as u32,
    /// Dereferenced object.
    DerefObj = vhpi_sys::vhpiClassKindT_vhpiDerefObjK as u32,
    /// Disconnect specification.
    DisconnectSpec = vhpi_sys::vhpiClassKindT_vhpiDisconnectSpecK as u32,
    /// Driver object.
    Driver = vhpi_sys::vhpiClassKindT_vhpiDriverK as u32,
    /// Collection of drivers.
    DriverCollection = vhpi_sys::vhpiClassKindT_vhpiDriverCollectionK as u32,
    /// Element association.
    ElemAssoc = vhpi_sys::vhpiClassKindT_vhpiElemAssocK as u32,
    /// Element declaration.
    ElemDecl = vhpi_sys::vhpiClassKindT_vhpiElemDeclK as u32,
    /// Entity class entry.
    EntityClassEntry = vhpi_sys::vhpiClassKindT_vhpiEntityClassEntryK as u32,
    /// Entity declaration.
    EntityDecl = vhpi_sys::vhpiClassKindT_vhpiEntityDeclK as u32,
    /// Enumeration literal.
    EnumLiteral = vhpi_sys::vhpiClassKindT_vhpiEnumLiteralK as u32,
    /// Enumeration range.
    EnumRange = vhpi_sys::vhpiClassKindT_vhpiEnumRangeK as u32,
    /// Enumeration type declaration.
    EnumTypeDecl = vhpi_sys::vhpiClassKindT_vhpiEnumTypeDeclK as u32,
    /// Exit statement.
    ExitStmt = vhpi_sys::vhpiClassKindT_vhpiExitStmtK as u32,
    /// File declaration.
    FileDecl = vhpi_sys::vhpiClassKindT_vhpiFileDeclK as u32,
    /// File parameter declaration.
    FileParamDecl = vhpi_sys::vhpiClassKindT_vhpiFileParamDeclK as u32,
    /// File type declaration.
    FileTypeDecl = vhpi_sys::vhpiClassKindT_vhpiFileTypeDeclK as u32,
    /// Floating range.
    FloatRange = vhpi_sys::vhpiClassKindT_vhpiFloatRangeK as u32,
    /// Floating type declaration.
    FloatTypeDecl = vhpi_sys::vhpiClassKindT_vhpiFloatTypeDeclK as u32,
    /// For-generate statement.
    ForGenerate = vhpi_sys::vhpiClassKindT_vhpiForGenerateK as u32,
    /// For-loop statement.
    ForLoop = vhpi_sys::vhpiClassKindT_vhpiForLoopK as u32,
    /// Foreign-interface item.
    Foreignf = vhpi_sys::vhpiClassKindT_vhpiForeignfK as u32,
    /// Function call.
    FuncCall = vhpi_sys::vhpiClassKindT_vhpiFuncCallK as u32,
    /// Function declaration.
    FuncDecl = vhpi_sys::vhpiClassKindT_vhpiFuncDeclK as u32,
    /// Generic declaration.
    GenericDecl = vhpi_sys::vhpiClassKindT_vhpiGenericDeclK as u32,
    /// Group declaration.
    GroupDecl = vhpi_sys::vhpiClassKindT_vhpiGroupDeclK as u32,
    /// Group template declaration.
    GroupTempDecl = vhpi_sys::vhpiClassKindT_vhpiGroupTempDeclK as u32,
    /// If-generate statement.
    IfGenerate = vhpi_sys::vhpiClassKindT_vhpiIfGenerateK as u32,
    /// If statement.
    IfStmt = vhpi_sys::vhpiClassKindT_vhpiIfStmtK as u32,
    /// Input port object.
    InPort = vhpi_sys::vhpiClassKindT_vhpiInPortK as u32,
    /// Indexed-name object.
    IndexedName = vhpi_sys::vhpiClassKindT_vhpiIndexedNameK as u32,
    /// Integer literal.
    IntLiteral = vhpi_sys::vhpiClassKindT_vhpiIntLiteralK as u32,
    /// Integer range.
    IntRange = vhpi_sys::vhpiClassKindT_vhpiIntRangeK as u32,
    /// Integer type declaration.
    IntTypeDecl = vhpi_sys::vhpiClassKindT_vhpiIntTypeDeclK as u32,
    /// Iterator object.
    Iterator = vhpi_sys::vhpiClassKindT_vhpiIteratorK as u32,
    /// Library declaration.
    LibraryDecl = vhpi_sys::vhpiClassKindT_vhpiLibraryDeclK as u32,
    /// Next statement.
    NextStmt = vhpi_sys::vhpiClassKindT_vhpiNextStmtK as u32,
    /// Null literal.
    NullLiteral = vhpi_sys::vhpiClassKindT_vhpiNullLiteralK as u32,
    /// Null statement.
    NullStmt = vhpi_sys::vhpiClassKindT_vhpiNullStmtK as u32,
    /// `others` choice/object.
    Others = vhpi_sys::vhpiClassKindT_vhpiOthersK as u32,
    /// Output port object.
    OutPort = vhpi_sys::vhpiClassKindT_vhpiOutPortK as u32,
    /// Package body.
    PackBody = vhpi_sys::vhpiClassKindT_vhpiPackBodyK as u32,
    /// Package declaration.
    PackDecl = vhpi_sys::vhpiClassKindT_vhpiPackDeclK as u32,
    /// Package instantiation.
    PackInst = vhpi_sys::vhpiClassKindT_vhpiPackInstK as u32,
    /// Parameterized attribute name.
    ParamAttrName = vhpi_sys::vhpiClassKindT_vhpiParamAttrNameK as u32,
    /// Physical literal.
    PhysLiteral = vhpi_sys::vhpiClassKindT_vhpiPhysLiteralK as u32,
    /// Physical range.
    PhysRange = vhpi_sys::vhpiClassKindT_vhpiPhysRangeK as u32,
    /// Physical type declaration.
    PhysTypeDecl = vhpi_sys::vhpiClassKindT_vhpiPhysTypeDeclK as u32,
    /// Port declaration.
    PortDecl = vhpi_sys::vhpiClassKindT_vhpiPortDeclK as u32,
    /// Procedure declaration.
    ProcDecl = vhpi_sys::vhpiClassKindT_vhpiProcDeclK as u32,
    /// Process statement.
    ProcessStmt = vhpi_sys::vhpiClassKindT_vhpiProcessStmtK as u32,
    /// Protected type body.
    ProtectedTypeBody = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeBodyK as u32,
    /// Protected type declaration.
    ProtectedTypeDecl = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeDeclK as u32,
    /// Real literal.
    RealLiteral = vhpi_sys::vhpiClassKindT_vhpiRealLiteralK as u32,
    /// Record type declaration.
    RecordTypeDecl = vhpi_sys::vhpiClassKindT_vhpiRecordTypeDeclK as u32,
    /// Report statement.
    ReportStmt = vhpi_sys::vhpiClassKindT_vhpiReportStmtK as u32,
    /// Return statement.
    ReturnStmt = vhpi_sys::vhpiClassKindT_vhpiReturnStmtK as u32,
    /// Root instance object.
    RootInst = vhpi_sys::vhpiClassKindT_vhpiRootInstK as u32,
    /// Concurrent selected signal assignment.
    SelectSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSelectSigAssignStmtK as u32,
    /// Selected waveform item.
    SelectWaveform = vhpi_sys::vhpiClassKindT_vhpiSelectWaveformK as u32,
    /// Selected-name object.
    SelectedName = vhpi_sys::vhpiClassKindT_vhpiSelectedNameK as u32,
    /// Signal declaration.
    SigDecl = vhpi_sys::vhpiClassKindT_vhpiSigDeclK as u32,
    /// Signal parameter declaration.
    SigParamDecl = vhpi_sys::vhpiClassKindT_vhpiSigParamDeclK as u32,
    /// Simple attribute name.
    SimpAttrName = vhpi_sys::vhpiClassKindT_vhpiSimpAttrNameK as u32,
    /// Concurrent simple signal assignment.
    SimpleSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSimpleSigAssignStmtK as u32,
    /// Slice-name object.
    SliceName = vhpi_sys::vhpiClassKindT_vhpiSliceNameK as u32,
    /// String literal.
    StringLiteral = vhpi_sys::vhpiClassKindT_vhpiStringLiteralK as u32,
    /// Subprogram body.
    SubpBody = vhpi_sys::vhpiClassKindT_vhpiSubpBodyK as u32,
    /// Subtype declaration.
    SubtypeDecl = vhpi_sys::vhpiClassKindT_vhpiSubtypeDeclK as u32,
    /// Simulator/tool object.
    Tool = vhpi_sys::vhpiClassKindT_vhpiToolK as u32,
    /// Transaction object.
    Transaction = vhpi_sys::vhpiClassKindT_vhpiTransactionK as u32,
    /// Type conversion expression.
    TypeConv = vhpi_sys::vhpiClassKindT_vhpiTypeConvK as u32,
    /// Unit declaration.
    UnitDecl = vhpi_sys::vhpiClassKindT_vhpiUnitDeclK as u32,
    /// User-defined attribute name.
    UserAttrName = vhpi_sys::vhpiClassKindT_vhpiUserAttrNameK as u32,
    /// Variable assignment statement.
    VarAssignStmt = vhpi_sys::vhpiClassKindT_vhpiVarAssignStmtK as u32,
    /// Variable declaration.
    VarDecl = vhpi_sys::vhpiClassKindT_vhpiVarDeclK as u32,
    /// Variable parameter declaration.
    VarParamDecl = vhpi_sys::vhpiClassKindT_vhpiVarParamDeclK as u32,
    /// Wait statement.
    WaitStmt = vhpi_sys::vhpiClassKindT_vhpiWaitStmtK as u32,
    /// Waveform element.
    WaveformElem = vhpi_sys::vhpiClassKindT_vhpiWaveformElemK as u32,
    /// While-loop statement.
    WhileLoop = vhpi_sys::vhpiClassKindT_vhpiWhileLoopK as u32,
    /// Qualified expression.
    QualifiedExpr = vhpi_sys::vhpiClassKindT_vhpiQualifiedExprK as u32,
    /// Use clause.
    UseClause = vhpi_sys::vhpiClassKindT_vhpiUseClauseK as u32,
    /// Concurrent assertion statement.
    ConcAssertStmt = vhpi_sys::vhpiClassKindT_vhpiConcAssertStmtK as u32,
    /// Forever-loop statement.
    ForeverLoop = vhpi_sys::vhpiClassKindT_vhpiForeverLoopK as u32,
    /// Sequential assertion statement.
    SeqAssertStmt = vhpi_sys::vhpiClassKindT_vhpiSeqAssertStmtK as u32,
    /// Sequential procedure-call statement.
    SeqProcCallStmt = vhpi_sys::vhpiClassKindT_vhpiSeqProcCallStmtK as u32,
    /// Sequential signal assignment statement.
    SeqSigAssignStmt = vhpi_sys::vhpiClassKindT_vhpiSeqSigAssignStmtK as u32,
    /// Protected type instance.
    ProtectedTypeInst = vhpi_sys::vhpiClassKindT_vhpiProtectedTypeInstK as u32,
    #[cfg(feature = "nvc")]
    /// Verilog module object.
    VerilogModule = vhpi_sys::vhpiClassKindT_vhpiVerilogModuleK as u32,
}

impl ClassKind {
    #[must_use]
    /// Convert a raw simulator value into `ClassKind`.
    ///
    /// Returns `None` when `value` is not a recognized class kind.
    pub fn from_i32(value: i32) -> Option<ClassKind> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
/// Interface mode values returned by `IntProperty::Mode`.
pub enum Mode {
    /// Input mode.
    In = vhpi_sys::vhpiModeT_vhpiInMode as u32,
    /// Output mode.
    Out = vhpi_sys::vhpiModeT_vhpiOutMode as u32,
    /// Bidirectional mode.
    Inout = vhpi_sys::vhpiModeT_vhpiInoutMode as u32,
    /// Buffer mode.
    Buffer = vhpi_sys::vhpiModeT_vhpiBufferMode as u32,
    /// Linkage mode.
    Linkage = vhpi_sys::vhpiModeT_vhpiLinkageMode as u32,
}

impl Mode {
    #[must_use]
    /// Convert a raw simulator value into `Mode`.
    ///
    /// Returns `None` when `value` is not a recognized mode.
    pub fn from_i32(value: i32) -> Option<Mode> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
/// Signal kind values returned by `IntProperty::SigKind`.
pub enum SigKind {
    /// Register signal.
    Register = vhpi_sys::vhpiSigKindT_vhpiRegister as u32,
    /// Bus signal.
    Bus = vhpi_sys::vhpiSigKindT_vhpiBus as u32,
    /// Normal signal.
    Normal = vhpi_sys::vhpiSigKindT_vhpiNormal as u32,
}

impl SigKind {
    #[must_use]
    /// Convert a raw simulator value into `SigKind`.
    ///
    /// Returns `None` when `value` is not a recognized signal kind.
    pub fn from_i32(value: i32) -> Option<SigKind> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
/// Staticness classification returned by `IntProperty::Staticness`.
pub enum Staticness {
    /// Locally static expression/object.
    LocallyStatic = vhpi_sys::vhpiStaticnessT_vhpiLocallyStatic as u32,
    /// Globally static expression/object.
    GloballyStatic = vhpi_sys::vhpiStaticnessT_vhpiGloballyStatic as u32,
    /// Dynamically evaluated expression/object.
    Dynamic = vhpi_sys::vhpiStaticnessT_vhpiDynamic as u32,
}

impl Staticness {
    #[must_use]
    /// Convert a raw simulator value into `Staticness`.
    ///
    /// Returns `None` when `value` is not a recognized staticness value.
    pub fn from_i32(value: i32) -> Option<Staticness> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive, PartialEq)]
/// Predefined VHDL attributes returned by `IntProperty::PredefAttr`.
pub enum PredefAttr {
    /// `'active`.
    Active = vhpi_sys::vhpiPredefAttrT_vhpiActivePA as u32,
    /// `'ascending`.
    Ascending = vhpi_sys::vhpiPredefAttrT_vhpiAscendingPA as u32,
    /// `'base`.
    Base = vhpi_sys::vhpiPredefAttrT_vhpiBasePA as u32,
    /// `'delayed`.
    Delayed = vhpi_sys::vhpiPredefAttrT_vhpiDelayedPA as u32,
    /// `'driving`.
    Driving = vhpi_sys::vhpiPredefAttrT_vhpiDrivingPA as u32,
    /// `'driving_value`.
    DrivingValue = vhpi_sys::vhpiPredefAttrT_vhpiDriving_valuePA as u32,
    /// `'event`.
    Event = vhpi_sys::vhpiPredefAttrT_vhpiEventPA as u32,
    /// `'high`.
    High = vhpi_sys::vhpiPredefAttrT_vhpiHighPA as u32,
    /// `'image`.
    Image = vhpi_sys::vhpiPredefAttrT_vhpiImagePA as u32,
    /// `'instance_name`.
    InstanceName = vhpi_sys::vhpiPredefAttrT_vhpiInstance_namePA as u32,
    /// `'last_active`.
    LastActive = vhpi_sys::vhpiPredefAttrT_vhpiLast_activePA as u32,
    /// `'last_event`.
    LastEvent = vhpi_sys::vhpiPredefAttrT_vhpiLast_eventPA as u32,
    /// `'last_value`.
    LastValue = vhpi_sys::vhpiPredefAttrT_vhpiLast_valuePA as u32,
    /// `'left`.
    Left = vhpi_sys::vhpiPredefAttrT_vhpiLeftPA as u32,
    /// `'leftof`.
    LeftOf = vhpi_sys::vhpiPredefAttrT_vhpiLeftofPA as u32,
    /// `'length`.
    Length = vhpi_sys::vhpiPredefAttrT_vhpiLengthPA as u32,
    /// `'low`.
    Low = vhpi_sys::vhpiPredefAttrT_vhpiLowPA as u32,
    /// `'path_name`.
    PathName = vhpi_sys::vhpiPredefAttrT_vhpiPath_namePA as u32,
    /// `'pos`.
    Pos = vhpi_sys::vhpiPredefAttrT_vhpiPosPA as u32,
    /// `'pred`.
    Pred = vhpi_sys::vhpiPredefAttrT_vhpiPredPA as u32,
    /// `'quiet`.
    Quiet = vhpi_sys::vhpiPredefAttrT_vhpiQuietPA as u32,
    /// `'range`.
    Range = vhpi_sys::vhpiPredefAttrT_vhpiRangePA as u32,
    /// `'reverse_range`.
    ReverseRange = vhpi_sys::vhpiPredefAttrT_vhpiReverse_rangePA as u32,
    /// `'right`.
    Right = vhpi_sys::vhpiPredefAttrT_vhpiRightPA as u32,
    /// `'rightof`.
    RightOf = vhpi_sys::vhpiPredefAttrT_vhpiRightofPA as u32,
    /// `'simple_name`.
    SimpleName = vhpi_sys::vhpiPredefAttrT_vhpiSimple_namePA as u32,
    /// `'stable`.
    Stable = vhpi_sys::vhpiPredefAttrT_vhpiStablePA as u32,
    /// `'succ`.
    Succ = vhpi_sys::vhpiPredefAttrT_vhpiSuccPA as u32,
    /// `'transaction`.
    Transaction = vhpi_sys::vhpiPredefAttrT_vhpiTransactionPA as u32,
    /// `'val`.
    Val = vhpi_sys::vhpiPredefAttrT_vhpiValPA as u32,
    /// `'value`.
    Value = vhpi_sys::vhpiPredefAttrT_vhpiValuePA as u32,
}

impl PredefAttr {
    #[must_use]
    /// Convert a raw simulator value into `PredefAttr`.
    ///
    /// Returns `None` when `value` is not a recognized predefined attribute.
    pub fn from_i32(value: i32) -> Option<PredefAttr> {
        num_traits::FromPrimitive::from_i32(value)
    }
}

impl Handle {
    #[must_use]
    /// Read an integer property from this handle.
    pub fn get(&self, property: IntProperty) -> i32 {
        unsafe { vhpi_get(property as vhpi_sys::vhpiIntPropertyT, self.as_raw()) }
    }

    #[must_use]
    /// Read a string property from this handle.
    ///
    /// Returns `None` when the simulator reports a null pointer for the
    /// requested property.
    pub fn get_str(&self, property: StrProperty) -> Option<String> {
        let ptr = unsafe { vhpi_get_str(property as vhpi_sys::vhpiStrPropertyT, self.as_raw()) };
        if ptr.is_null() {
            return None;
        }

        let cstr = unsafe { CStr::from_ptr(ptr.cast::<c_char>()) };
        Some(iso8859_1_cstr_to_string(cstr))
    }

    #[must_use]
    /// Read a physical-quantity property from this handle.
    pub fn get_phys(&self, property: PhysProperty) -> Physical {
        let result =
            unsafe { vhpi_get_phys(property as vhpi_sys::vhpiPhysPropertyT, self.as_raw()) };
        result.into()
    }

    #[must_use]
    /// Read a floating-point property from this handle.
    pub fn get_real(&self, property: RealProperty) -> f64 {
        unsafe { vhpi_get_real(property as vhpi_sys::vhpiRealPropertyT, self.as_raw()) }
    }

    // The following are convenience functions not defined by VHPI

    #[must_use]
    /// Get this object's class kind.
    ///
    /// Returns `None` if the raw kind value is unknown.
    pub fn get_kind(&self) -> Option<ClassKind> {
        let kind_int = self.get(IntProperty::Kind);
        ClassKind::from_i32(kind_int)
    }

    #[must_use]
    /// Get this object's simple name.
    pub fn get_name(&self) -> Option<String> {
        self.get_str(StrProperty::Name)
    }

    #[must_use]
    /// Get this object's fully qualified name.
    pub fn get_full_name(&self) -> Option<String> {
        self.get_str(StrProperty::FullName)
    }

    #[must_use]
    /// Get this object's interface mode.
    ///
    /// Returns `None` if the raw mode value is unknown.
    pub fn get_mode(&self) -> Option<Mode> {
        let mode_int = self.get(IntProperty::Mode);
        Mode::from_i32(mode_int)
    }

    #[must_use]
    /// Get this object's signal kind.
    ///
    /// Returns `None` if the raw signal kind value is unknown.
    pub fn get_sig_kind(&self) -> Option<SigKind> {
        let sig_kind_int = self.get(IntProperty::SigKind);
        SigKind::from_i32(sig_kind_int)
    }

    #[must_use]
    /// Get this object's staticness classification.
    ///
    /// Returns `None` if the raw staticness value is unknown.
    pub fn get_staticness(&self) -> Option<Staticness> {
        let staticness_int = self.get(IntProperty::Staticness);
        Staticness::from_i32(staticness_int)
    }

    #[must_use]
    /// Get this object's predefined attribute identifier.
    ///
    /// Returns `None` if the raw predefined-attribute value is unknown.
    pub fn get_predef_attr(&self) -> Option<PredefAttr> {
        let predef_attr_int = self.get(IntProperty::PredefAttr);
        PredefAttr::from_i32(predef_attr_int)
    }

    #[must_use]
    /// Build an iterator over this type's discrete index range.
    ///
    /// The direction (`to` vs `downto`) is inferred from the associated
    /// constraint metadata.
    pub fn index_range(&self) -> Box<dyn Iterator<Item = i32>> {
        let raw = unsafe {
            vhpi_iterator(
                crate::OneToMany::Constraints as vhpi_sys::vhpiOneToManyT,
                self.as_raw(),
            )
        };
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
    /// Collect names of enumeration literals for an enumeration type.
    ///
    /// Returns `None` if this handle is not an enum type declaration.
    pub fn enum_literals(&self) -> Option<Vec<String>> {
        if self.get_kind()? != ClassKind::EnumTypeDecl {
            return None;
        }

        self.iterator(crate::OneToMany::EnumLiterals)
            .map(|handle| handle.get_str(StrProperty::Name))
            .collect::<Option<Vec<String>>>()
    }
}
