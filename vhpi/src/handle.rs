use std::ffi::CString;
use vhpi_sys::{
    vhpiHandleT, vhpi_compare_handles, vhpi_handle, vhpi_handle_by_name, vhpi_iterator,
    vhpi_release_handle, vhpi_scan,
};

#[repr(u32)]
pub enum OneToOne {
    RootInst = vhpi_sys::vhpiOneToOneT_vhpiRootInst,
    AbstractLiteral = vhpi_sys::vhpiOneToOneT_vhpiAbstractLiteral,
    Actual = vhpi_sys::vhpiOneToOneT_vhpiActual,
    All = vhpi_sys::vhpiOneToOneT_vhpiAll,
    AttrDecl = vhpi_sys::vhpiOneToOneT_vhpiAttrDecl,
    AttrSpec = vhpi_sys::vhpiOneToOneT_vhpiAttrSpec,
    BaseType = vhpi_sys::vhpiOneToOneT_vhpiBaseType,
    BaseUnit = vhpi_sys::vhpiOneToOneT_vhpiBaseUnit,
    BlockConfig = vhpi_sys::vhpiOneToOneT_vhpiBlockConfig,
    CaseExpr = vhpi_sys::vhpiOneToOneT_vhpiCaseExpr,
    CondExpr = vhpi_sys::vhpiOneToOneT_vhpiCondExpr,
    ConfigDecl = vhpi_sys::vhpiOneToOneT_vhpiConfigDecl,
    ConfigSpec = vhpi_sys::vhpiOneToOneT_vhpiConfigSpec,
    Constraint = vhpi_sys::vhpiOneToOneT_vhpiConstraint,
    Contributor = vhpi_sys::vhpiOneToOneT_vhpiContributor,
    CurCallback = vhpi_sys::vhpiOneToOneT_vhpiCurCallback,
    CurStackFrame = vhpi_sys::vhpiOneToOneT_vhpiCurStackFrame,
    DerefObj = vhpi_sys::vhpiOneToOneT_vhpiDerefObj,
    DesignUnit = vhpi_sys::vhpiOneToOneT_vhpiDesignUnit,
    DownStack = vhpi_sys::vhpiOneToOneT_vhpiDownStack,
    EntityAspect = vhpi_sys::vhpiOneToOneT_vhpiEntityAspect,
    EntityDecl = vhpi_sys::vhpiOneToOneT_vhpiEntityDecl,
    EqProcessStmt = vhpi_sys::vhpiOneToOneT_vhpiEqProcessStmt,
    Expr = vhpi_sys::vhpiOneToOneT_vhpiExpr,
    Formal = vhpi_sys::vhpiOneToOneT_vhpiFormal,
    FuncDecl = vhpi_sys::vhpiOneToOneT_vhpiFuncDecl,
    GroupTempDecl = vhpi_sys::vhpiOneToOneT_vhpiGroupTempDecl,
    GuardExpr = vhpi_sys::vhpiOneToOneT_vhpiGuardExpr,
    GuardSig = vhpi_sys::vhpiOneToOneT_vhpiGuardSig,
    ImmRegion = vhpi_sys::vhpiOneToOneT_vhpiImmRegion,
    InPort = vhpi_sys::vhpiOneToOneT_vhpiInPort,
    InitExpr = vhpi_sys::vhpiOneToOneT_vhpiInitExpr,
    LeftExpr = vhpi_sys::vhpiOneToOneT_vhpiLeftExpr,
    LexicalScope = vhpi_sys::vhpiOneToOneT_vhpiLexicalScope,
    LhsExpr = vhpi_sys::vhpiOneToOneT_vhpiLhsExpr,
    Local = vhpi_sys::vhpiOneToOneT_vhpiLocal,
    LogicalExpr = vhpi_sys::vhpiOneToOneT_vhpiLogicalExpr,
    Others = vhpi_sys::vhpiOneToOneT_vhpiOthers,
    OutPort = vhpi_sys::vhpiOneToOneT_vhpiOutPort,
    ParamDecl = vhpi_sys::vhpiOneToOneT_vhpiParamDecl,
    Parent = vhpi_sys::vhpiOneToOneT_vhpiParent,
    PhysLiteral = vhpi_sys::vhpiOneToOneT_vhpiPhysLiteral,
    Prefix = vhpi_sys::vhpiOneToOneT_vhpiPrefix,
    PrimaryUnit = vhpi_sys::vhpiOneToOneT_vhpiPrimaryUnit,
    ProtectedTypeBody = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeBody,
    ProtectedTypeDecl = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeDecl,
    RejectTime = vhpi_sys::vhpiOneToOneT_vhpiRejectTime,
    ReportExpr = vhpi_sys::vhpiOneToOneT_vhpiReportExpr,
    ResolFunc = vhpi_sys::vhpiOneToOneT_vhpiResolFunc,
    ReturnExpr = vhpi_sys::vhpiOneToOneT_vhpiReturnExpr,
    RhsExpr = vhpi_sys::vhpiOneToOneT_vhpiRhsExpr,
    RightExpr = vhpi_sys::vhpiOneToOneT_vhpiRightExpr,
    SelectExpr = vhpi_sys::vhpiOneToOneT_vhpiSelectExpr,
    SeverityExpr = vhpi_sys::vhpiOneToOneT_vhpiSeverityExpr,
    SimpleName = vhpi_sys::vhpiOneToOneT_vhpiSimpleName,
    SubpBody = vhpi_sys::vhpiOneToOneT_vhpiSubpBody,
    SubpDecl = vhpi_sys::vhpiOneToOneT_vhpiSubpDecl,
    Suffix = vhpi_sys::vhpiOneToOneT_vhpiSuffix,
    TimeExpr = vhpi_sys::vhpiOneToOneT_vhpiTimeExpr,
    TimeOutExpr = vhpi_sys::vhpiOneToOneT_vhpiTimeOutExpr,
    Tool = vhpi_sys::vhpiOneToOneT_vhpiTool,
    Type = vhpi_sys::vhpiOneToOneT_vhpiType,
    UnitDecl = vhpi_sys::vhpiOneToOneT_vhpiUnitDecl,
    UpStack = vhpi_sys::vhpiOneToOneT_vhpiUpStack,
    UpperRegion = vhpi_sys::vhpiOneToOneT_vhpiUpperRegion,
    Use = vhpi_sys::vhpiOneToOneT_vhpiUse,
    ValExpr = vhpi_sys::vhpiOneToOneT_vhpiValExpr,
    ElemType = vhpi_sys::vhpiOneToOneT_vhpiElemType,
    FirstNamedType = vhpi_sys::vhpiOneToOneT_vhpiFirstNamedType,
    ReturnType = vhpi_sys::vhpiOneToOneT_vhpiReturnType,
    ValType = vhpi_sys::vhpiOneToOneT_vhpiValType,
    CurRegion = vhpi_sys::vhpiOneToOneT_vhpiCurRegion,
    Signal = vhpi_sys::vhpiOneToOneT_vhpiSignal,
    LibraryDecl = vhpi_sys::vhpiOneToOneT_vhpiLibraryDecl,
    SimNet = vhpi_sys::vhpiOneToOneT_vhpiSimNet,
    AliasedName = vhpi_sys::vhpiOneToOneT_vhpiAliasedName,
    CompDecl = vhpi_sys::vhpiOneToOneT_vhpiCompDecl,
    ProtectedTypeInst = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeInst,
    GenIndex = vhpi_sys::vhpiOneToOneT_vhpiGenIndex,
}

#[repr(u32)]
pub enum OneToMany {
    Decls = vhpi_sys::vhpiOneToManyT_vhpiDecls,
    SigDecls = vhpi_sys::vhpiOneToManyT_vhpiSigDecls,
    PortDecls = vhpi_sys::vhpiOneToManyT_vhpiPortDecls,
    InternalRegions = vhpi_sys::vhpiOneToManyT_vhpiInternalRegions,
    AliasDecls = vhpi_sys::vhpiOneToManyT_vhpiAliasDecls,
    Argvs = vhpi_sys::vhpiOneToManyT_vhpiArgvs,
    AttrDecls = vhpi_sys::vhpiOneToManyT_vhpiAttrDecls,
    AttrSpecs = vhpi_sys::vhpiOneToManyT_vhpiAttrSpecs,
    BasicSignals = vhpi_sys::vhpiOneToManyT_vhpiBasicSignals,
    BlockStmts = vhpi_sys::vhpiOneToManyT_vhpiBlockStmts,
    Branchs = vhpi_sys::vhpiOneToManyT_vhpiBranchs,
    Choices = vhpi_sys::vhpiOneToManyT_vhpiChoices,
    CompInstStmts = vhpi_sys::vhpiOneToManyT_vhpiCompInstStmts,
    CondWaveforms = vhpi_sys::vhpiOneToManyT_vhpiCondWaveforms,
    ConfigItems = vhpi_sys::vhpiOneToManyT_vhpiConfigItems,
    ConfigSpecs = vhpi_sys::vhpiOneToManyT_vhpiConfigSpecs,
    ConstDecls = vhpi_sys::vhpiOneToManyT_vhpiConstDecls,
    Constraints = vhpi_sys::vhpiOneToManyT_vhpiConstraints,
    DepUnits = vhpi_sys::vhpiOneToManyT_vhpiDepUnits,
    DesignUnits = vhpi_sys::vhpiOneToManyT_vhpiDesignUnits,
    DrivenSigs = vhpi_sys::vhpiOneToManyT_vhpiDrivenSigs,
    Drivers = vhpi_sys::vhpiOneToManyT_vhpiDrivers,
    ElemAssocs = vhpi_sys::vhpiOneToManyT_vhpiElemAssocs,
    EntityDesignators = vhpi_sys::vhpiOneToManyT_vhpiEntityDesignators,
    EnumLiterals = vhpi_sys::vhpiOneToManyT_vhpiEnumLiterals,
    Foreignfs = vhpi_sys::vhpiOneToManyT_vhpiForeignfs,
    GenericAssocs = vhpi_sys::vhpiOneToManyT_vhpiGenericAssocs,
    GenericDecls = vhpi_sys::vhpiOneToManyT_vhpiGenericDecls,
    IndexExprs = vhpi_sys::vhpiOneToManyT_vhpiIndexExprs,
    IndexedNames = vhpi_sys::vhpiOneToManyT_vhpiIndexedNames,
    Members = vhpi_sys::vhpiOneToManyT_vhpiMembers,
    PackInsts = vhpi_sys::vhpiOneToManyT_vhpiPackInsts,
    ParamAssocs = vhpi_sys::vhpiOneToManyT_vhpiParamAssocs,
    ParamDecls = vhpi_sys::vhpiOneToManyT_vhpiParamDecls,
    PortAssocs = vhpi_sys::vhpiOneToManyT_vhpiPortAssocs,
    RecordElems = vhpi_sys::vhpiOneToManyT_vhpiRecordElems,
    SelectWaveforms = vhpi_sys::vhpiOneToManyT_vhpiSelectWaveforms,
    SelectedNames = vhpi_sys::vhpiOneToManyT_vhpiSelectedNames,
    SeqStmts = vhpi_sys::vhpiOneToManyT_vhpiSeqStmts,
    SigAttrs = vhpi_sys::vhpiOneToManyT_vhpiSigAttrs,
    SigNames = vhpi_sys::vhpiOneToManyT_vhpiSigNames,
    Signals = vhpi_sys::vhpiOneToManyT_vhpiSignals,
    Specs = vhpi_sys::vhpiOneToManyT_vhpiSpecs,
    Stmts = vhpi_sys::vhpiOneToManyT_vhpiStmts,
    Transactions = vhpi_sys::vhpiOneToManyT_vhpiTransactions,
    UnitDecls = vhpi_sys::vhpiOneToManyT_vhpiUnitDecls,
    Uses = vhpi_sys::vhpiOneToManyT_vhpiUses,
    VarDecls = vhpi_sys::vhpiOneToManyT_vhpiVarDecls,
    WaveformElems = vhpi_sys::vhpiOneToManyT_vhpiWaveformElems,
    LibraryDecls = vhpi_sys::vhpiOneToManyT_vhpiLibraryDecls,
    LocalLoads = vhpi_sys::vhpiOneToManyT_vhpiLocalLoads,
    OptimizedLoads = vhpi_sys::vhpiOneToManyT_vhpiOptimizedLoads,
    Types = vhpi_sys::vhpiOneToManyT_vhpiTypes,
    UseClauses = vhpi_sys::vhpiOneToManyT_vhpiUseClauses,
    GenerateStmts = vhpi_sys::vhpiOneToManyT_vhpiGenerateStmts,
    LocalContributors = vhpi_sys::vhpiOneToManyT_vhpiLocalContributors,
    OptimizedContributors = vhpi_sys::vhpiOneToManyT_vhpiOptimizedContributors,
    ParamExprs = vhpi_sys::vhpiOneToManyT_vhpiParamExprs,
    EqProcessStmts = vhpi_sys::vhpiOneToManyT_vhpiEqProcessStmts,
    EntityClassEntries = vhpi_sys::vhpiOneToManyT_vhpiEntityClassEntries,
    Sensitivities = vhpi_sys::vhpiOneToManyT_vhpiSensitivities,
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
    #[must_use]
    pub fn null() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    #[must_use]
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

    #[must_use]
    pub fn handle(&self, property: OneToOne) -> Handle {
        Handle::from_raw(unsafe { vhpi_handle(property as u32, self.as_raw()) })
    }

    #[must_use]
    pub fn handle_by_name(&self, name: &str) -> Handle {
        let c_name = CString::new(name).unwrap();
        Handle::from_raw(unsafe { vhpi_handle_by_name(c_name.as_ptr(), self.as_raw()) })
    }

    #[must_use]
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

#[must_use]
pub fn handle(property: OneToOne) -> Handle {
    Handle::from_raw(unsafe { vhpi_handle(property as u32, std::ptr::null_mut()) })
}

#[must_use]
pub fn handle_by_name(name: &str) -> Handle {
    let c_name = CString::new(name).unwrap();
    Handle::from_raw(unsafe { vhpi_handle_by_name(c_name.as_ptr(), std::ptr::null_mut()) })
}
