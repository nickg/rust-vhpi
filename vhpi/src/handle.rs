use bindings::{vhpiHandleT, vhpi_compare_handles, vhpi_handle, vhpi_handle_by_name,
               vhpi_iterator, vhpi_release_handle, vhpi_scan};
use std::ffi::CString;

#[repr(u32)]
pub enum OneToOne {
    RootInst = bindings::vhpiOneToOneT_vhpiRootInst,
    AbstractLiteral = bindings::vhpiOneToOneT_vhpiAbstractLiteral,
    Actual = bindings::vhpiOneToOneT_vhpiActual,
    All = bindings::vhpiOneToOneT_vhpiAll,
    AttrDecl = bindings::vhpiOneToOneT_vhpiAttrDecl,
    AttrSpec = bindings::vhpiOneToOneT_vhpiAttrSpec,
    BaseType = bindings::vhpiOneToOneT_vhpiBaseType,
    BaseUnit = bindings::vhpiOneToOneT_vhpiBaseUnit,
    BlockConfig = bindings::vhpiOneToOneT_vhpiBlockConfig,
    CaseExpr = bindings::vhpiOneToOneT_vhpiCaseExpr,
    CondExpr = bindings::vhpiOneToOneT_vhpiCondExpr,
    ConfigDecl = bindings::vhpiOneToOneT_vhpiConfigDecl,
    ConfigSpec = bindings::vhpiOneToOneT_vhpiConfigSpec,
    Constraint = bindings::vhpiOneToOneT_vhpiConstraint,
    Contributor = bindings::vhpiOneToOneT_vhpiContributor,
    CurCallback = bindings::vhpiOneToOneT_vhpiCurCallback,
    CurStackFrame = bindings::vhpiOneToOneT_vhpiCurStackFrame,
    DerefObj = bindings::vhpiOneToOneT_vhpiDerefObj,
    DesignUnit = bindings::vhpiOneToOneT_vhpiDesignUnit,
    DownStack = bindings::vhpiOneToOneT_vhpiDownStack,
    EntityAspect = bindings::vhpiOneToOneT_vhpiEntityAspect,
    EntityDecl = bindings::vhpiOneToOneT_vhpiEntityDecl,
    EqProcessStmt = bindings::vhpiOneToOneT_vhpiEqProcessStmt,
    Expr = bindings::vhpiOneToOneT_vhpiExpr,
    Formal = bindings::vhpiOneToOneT_vhpiFormal,
    FuncDecl = bindings::vhpiOneToOneT_vhpiFuncDecl,
    GroupTempDecl = bindings::vhpiOneToOneT_vhpiGroupTempDecl,
    GuardExpr = bindings::vhpiOneToOneT_vhpiGuardExpr,
    GuardSig = bindings::vhpiOneToOneT_vhpiGuardSig,
    ImmRegion = bindings::vhpiOneToOneT_vhpiImmRegion,
    InPort = bindings::vhpiOneToOneT_vhpiInPort,
    InitExpr = bindings::vhpiOneToOneT_vhpiInitExpr,
    LeftExpr = bindings::vhpiOneToOneT_vhpiLeftExpr,
    LexicalScope = bindings::vhpiOneToOneT_vhpiLexicalScope,
    LhsExpr = bindings::vhpiOneToOneT_vhpiLhsExpr,
    Local = bindings::vhpiOneToOneT_vhpiLocal,
    LogicalExpr = bindings::vhpiOneToOneT_vhpiLogicalExpr,
    Others = bindings::vhpiOneToOneT_vhpiOthers,
    OutPort = bindings::vhpiOneToOneT_vhpiOutPort,
    ParamDecl = bindings::vhpiOneToOneT_vhpiParamDecl,
    Parent = bindings::vhpiOneToOneT_vhpiParent,
    PhysLiteral = bindings::vhpiOneToOneT_vhpiPhysLiteral,
    Prefix = bindings::vhpiOneToOneT_vhpiPrefix,
    PrimaryUnit = bindings::vhpiOneToOneT_vhpiPrimaryUnit,
    ProtectedTypeBody = bindings::vhpiOneToOneT_vhpiProtectedTypeBody,
    ProtectedTypeDecl = bindings::vhpiOneToOneT_vhpiProtectedTypeDecl,
    RejectTime = bindings::vhpiOneToOneT_vhpiRejectTime,
    ReportExpr = bindings::vhpiOneToOneT_vhpiReportExpr,
    ResolFunc = bindings::vhpiOneToOneT_vhpiResolFunc,
    ReturnExpr = bindings::vhpiOneToOneT_vhpiReturnExpr,
    RhsExpr = bindings::vhpiOneToOneT_vhpiRhsExpr,
    RightExpr = bindings::vhpiOneToOneT_vhpiRightExpr,
    SelectExpr = bindings::vhpiOneToOneT_vhpiSelectExpr,
    SeverityExpr = bindings::vhpiOneToOneT_vhpiSeverityExpr,
    SimpleName = bindings::vhpiOneToOneT_vhpiSimpleName,
    SubpBody = bindings::vhpiOneToOneT_vhpiSubpBody,
    SubpDecl = bindings::vhpiOneToOneT_vhpiSubpDecl,
    Suffix = bindings::vhpiOneToOneT_vhpiSuffix,
    TimeExpr = bindings::vhpiOneToOneT_vhpiTimeExpr,
    TimeOutExpr = bindings::vhpiOneToOneT_vhpiTimeOutExpr,
    Tool = bindings::vhpiOneToOneT_vhpiTool,
    Type = bindings::vhpiOneToOneT_vhpiType,
    UnitDecl = bindings::vhpiOneToOneT_vhpiUnitDecl,
    UpStack = bindings::vhpiOneToOneT_vhpiUpStack,
    UpperRegion = bindings::vhpiOneToOneT_vhpiUpperRegion,
    Use = bindings::vhpiOneToOneT_vhpiUse,
    ValExpr = bindings::vhpiOneToOneT_vhpiValExpr,
    ElemType = bindings::vhpiOneToOneT_vhpiElemType,
    FirstNamedType = bindings::vhpiOneToOneT_vhpiFirstNamedType,
    ReturnType = bindings::vhpiOneToOneT_vhpiReturnType,
    ValType = bindings::vhpiOneToOneT_vhpiValType,
    CurRegion = bindings::vhpiOneToOneT_vhpiCurRegion,
    Signal = bindings::vhpiOneToOneT_vhpiSignal,
    LibraryDecl = bindings::vhpiOneToOneT_vhpiLibraryDecl,
    SimNet = bindings::vhpiOneToOneT_vhpiSimNet,
    AliasedName = bindings::vhpiOneToOneT_vhpiAliasedName,
    CompDecl = bindings::vhpiOneToOneT_vhpiCompDecl,
    ProtectedTypeInst = bindings::vhpiOneToOneT_vhpiProtectedTypeInst,
    GenIndex = bindings::vhpiOneToOneT_vhpiGenIndex,
}

#[repr(u32)]
pub enum OneToMany {
    Decls = bindings::vhpiOneToManyT_vhpiDecls,
    SigDecls = bindings::vhpiOneToManyT_vhpiSigDecls,
    PortDecls = bindings::vhpiOneToManyT_vhpiPortDecls,
    InternalRegions = bindings::vhpiOneToManyT_vhpiInternalRegions,
    AliasDecls = bindings::vhpiOneToManyT_vhpiAliasDecls,
    Argvs = bindings::vhpiOneToManyT_vhpiArgvs,
    AttrDecls = bindings::vhpiOneToManyT_vhpiAttrDecls,
    AttrSpecs = bindings::vhpiOneToManyT_vhpiAttrSpecs,
    BasicSignals = bindings::vhpiOneToManyT_vhpiBasicSignals,
    BlockStmts = bindings::vhpiOneToManyT_vhpiBlockStmts,
    Branchs = bindings::vhpiOneToManyT_vhpiBranchs,
    Choices = bindings::vhpiOneToManyT_vhpiChoices,
    CompInstStmts = bindings::vhpiOneToManyT_vhpiCompInstStmts,
    CondWaveforms = bindings::vhpiOneToManyT_vhpiCondWaveforms,
    ConfigItems = bindings::vhpiOneToManyT_vhpiConfigItems,
    ConfigSpecs = bindings::vhpiOneToManyT_vhpiConfigSpecs,
    ConstDecls = bindings::vhpiOneToManyT_vhpiConstDecls,
    Constraints = bindings::vhpiOneToManyT_vhpiConstraints,
    DepUnits = bindings::vhpiOneToManyT_vhpiDepUnits,
    DesignUnits = bindings::vhpiOneToManyT_vhpiDesignUnits,
    DrivenSigs = bindings::vhpiOneToManyT_vhpiDrivenSigs,
    Drivers = bindings::vhpiOneToManyT_vhpiDrivers,
    ElemAssocs = bindings::vhpiOneToManyT_vhpiElemAssocs,
    EntityDesignators = bindings::vhpiOneToManyT_vhpiEntityDesignators,
    EnumLiterals = bindings::vhpiOneToManyT_vhpiEnumLiterals,
    Foreignfs = bindings::vhpiOneToManyT_vhpiForeignfs,
    GenericAssocs = bindings::vhpiOneToManyT_vhpiGenericAssocs,
    GenericDecls = bindings::vhpiOneToManyT_vhpiGenericDecls,
    IndexExprs = bindings::vhpiOneToManyT_vhpiIndexExprs,
    IndexedNames = bindings::vhpiOneToManyT_vhpiIndexedNames,
    Members = bindings::vhpiOneToManyT_vhpiMembers,
    PackInsts = bindings::vhpiOneToManyT_vhpiPackInsts,
    ParamAssocs = bindings::vhpiOneToManyT_vhpiParamAssocs,
    ParamDecls = bindings::vhpiOneToManyT_vhpiParamDecls,
    PortAssocs = bindings::vhpiOneToManyT_vhpiPortAssocs,
    RecordElems = bindings::vhpiOneToManyT_vhpiRecordElems,
    SelectWaveforms = bindings::vhpiOneToManyT_vhpiSelectWaveforms,
    SelectedNames = bindings::vhpiOneToManyT_vhpiSelectedNames,
    SeqStmts = bindings::vhpiOneToManyT_vhpiSeqStmts,
    SigAttrs = bindings::vhpiOneToManyT_vhpiSigAttrs,
    SigNames = bindings::vhpiOneToManyT_vhpiSigNames,
    Signals = bindings::vhpiOneToManyT_vhpiSignals,
    Specs = bindings::vhpiOneToManyT_vhpiSpecs,
    Stmts = bindings::vhpiOneToManyT_vhpiStmts,
    Transactions = bindings::vhpiOneToManyT_vhpiTransactions,
    UnitDecls = bindings::vhpiOneToManyT_vhpiUnitDecls,
    Uses = bindings::vhpiOneToManyT_vhpiUses,
    VarDecls = bindings::vhpiOneToManyT_vhpiVarDecls,
    WaveformElems = bindings::vhpiOneToManyT_vhpiWaveformElems,
    LibraryDecls = bindings::vhpiOneToManyT_vhpiLibraryDecls,
    LocalLoads = bindings::vhpiOneToManyT_vhpiLocalLoads,
    OptimizedLoads = bindings::vhpiOneToManyT_vhpiOptimizedLoads,
    Types = bindings::vhpiOneToManyT_vhpiTypes,
    UseClauses = bindings::vhpiOneToManyT_vhpiUseClauses,
    GenerateStmts = bindings::vhpiOneToManyT_vhpiGenerateStmts,
    LocalContributors = bindings::vhpiOneToManyT_vhpiLocalContributors,
    OptimizedContributors = bindings::vhpiOneToManyT_vhpiOptimizedContributors,
    ParamExprs = bindings::vhpiOneToManyT_vhpiParamExprs,
    EqProcessStmts = bindings::vhpiOneToManyT_vhpiEqProcessStmts,
    EntityClassEntries = bindings::vhpiOneToManyT_vhpiEntityClassEntries,
    Sensitivities = bindings::vhpiOneToManyT_vhpiSensitivities,
}

pub struct Handle {
    handle: vhpiHandleT,
}

pub struct HandleIterator {
    iter: Handle,
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe {
                vhpi_release_handle(self.handle);
            }
        }
    }
}

impl Default for Handle {
    fn default() -> Self {
        Self::null()
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { vhpi_compare_handles(self.handle, other.handle) != 0 }
    }
}

impl Eq for Handle {}

impl Handle {
    pub fn null() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    pub(crate) fn as_raw(&self) -> vhpiHandleT {
        self.handle
    }

    pub(crate) fn clear(&mut self) {
        self.handle = std::ptr::null_mut();
    }

    pub fn from_raw(raw: vhpiHandleT) -> Self {
        Self { handle: raw }
    }

    pub fn handle(&self, property: OneToOne) -> Handle {
        Handle::from_raw(unsafe { vhpi_handle(property as u32, self.as_raw()) })
    }

    pub fn handle_by_name(&self, name: &str) -> Handle {
        let c_name = CString::new(name).unwrap();
        Handle::from_raw(unsafe {
            vhpi_handle_by_name(c_name.as_ptr() as *const i8, self.as_raw())
        })
    }

    pub fn iterator(&self, typ: OneToMany) -> HandleIterator {
        let raw = unsafe { vhpi_iterator(typ as u32, self.as_raw()) };
        HandleIterator {
            iter: Handle::from_raw(raw),
        }
    }
}

impl Iterator for HandleIterator {
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_null() {
            return None;
        }

        let next = Handle::from_raw(unsafe { vhpi_scan(self.iter.as_raw()) });

        if next.is_null() {
            // The handle is automatically released when the iterator is exhausted
            self.iter.clear();
            None
        } else {
            Some(next)
        }
    }
}

pub fn handle(property: OneToOne) -> Handle {
    Handle::from_raw(unsafe { vhpi_handle(property as u32, std::ptr::null_mut()) })
}

pub fn handle_by_name(name: &str) -> Handle {
    let c_name = CString::new(name).unwrap();
    Handle::from_raw(unsafe {
        vhpi_handle_by_name(c_name.as_ptr() as *const i8, std::ptr::null_mut())
    })
}
