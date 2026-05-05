use num_traits::Zero;
use std::ffi::CString;
use vhpi_sys::{
    vhpiHandleT, vhpi_compare_handles, vhpi_handle, vhpi_handle_by_index, vhpi_handle_by_name,
    vhpi_iterator, vhpi_release_handle, vhpi_scan,
};

#[repr(u32)]
pub enum OneToOne {
    RootInst = vhpi_sys::vhpiOneToOneT_vhpiRootInst as u32,
    AbstractLiteral = vhpi_sys::vhpiOneToOneT_vhpiAbstractLiteral as u32,
    Actual = vhpi_sys::vhpiOneToOneT_vhpiActual as u32,
    All = vhpi_sys::vhpiOneToOneT_vhpiAll as u32,
    AttrDecl = vhpi_sys::vhpiOneToOneT_vhpiAttrDecl as u32,
    AttrSpec = vhpi_sys::vhpiOneToOneT_vhpiAttrSpec as u32,
    BaseType = vhpi_sys::vhpiOneToOneT_vhpiBaseType as u32,
    BaseUnit = vhpi_sys::vhpiOneToOneT_vhpiBaseUnit as u32,
    BlockConfig = vhpi_sys::vhpiOneToOneT_vhpiBlockConfig as u32,
    CaseExpr = vhpi_sys::vhpiOneToOneT_vhpiCaseExpr as u32,
    CondExpr = vhpi_sys::vhpiOneToOneT_vhpiCondExpr as u32,
    ConfigDecl = vhpi_sys::vhpiOneToOneT_vhpiConfigDecl as u32,
    ConfigSpec = vhpi_sys::vhpiOneToOneT_vhpiConfigSpec as u32,
    Constraint = vhpi_sys::vhpiOneToOneT_vhpiConstraint as u32,
    Contributor = vhpi_sys::vhpiOneToOneT_vhpiContributor as u32,
    CurCallback = vhpi_sys::vhpiOneToOneT_vhpiCurCallback as u32,
    CurStackFrame = vhpi_sys::vhpiOneToOneT_vhpiCurStackFrame as u32,
    DerefObj = vhpi_sys::vhpiOneToOneT_vhpiDerefObj as u32,
    DesignUnit = vhpi_sys::vhpiOneToOneT_vhpiDesignUnit as u32,
    DownStack = vhpi_sys::vhpiOneToOneT_vhpiDownStack as u32,
    EntityAspect = vhpi_sys::vhpiOneToOneT_vhpiEntityAspect as u32,
    EntityDecl = vhpi_sys::vhpiOneToOneT_vhpiEntityDecl as u32,
    EqProcessStmt = vhpi_sys::vhpiOneToOneT_vhpiEqProcessStmt as u32,
    Expr = vhpi_sys::vhpiOneToOneT_vhpiExpr as u32,
    Formal = vhpi_sys::vhpiOneToOneT_vhpiFormal as u32,
    FuncDecl = vhpi_sys::vhpiOneToOneT_vhpiFuncDecl as u32,
    GroupTempDecl = vhpi_sys::vhpiOneToOneT_vhpiGroupTempDecl as u32,
    GuardExpr = vhpi_sys::vhpiOneToOneT_vhpiGuardExpr as u32,
    GuardSig = vhpi_sys::vhpiOneToOneT_vhpiGuardSig as u32,
    ImmRegion = vhpi_sys::vhpiOneToOneT_vhpiImmRegion as u32,
    InPort = vhpi_sys::vhpiOneToOneT_vhpiInPort as u32,
    InitExpr = vhpi_sys::vhpiOneToOneT_vhpiInitExpr as u32,
    LeftExpr = vhpi_sys::vhpiOneToOneT_vhpiLeftExpr as u32,
    LexicalScope = vhpi_sys::vhpiOneToOneT_vhpiLexicalScope as u32,
    LhsExpr = vhpi_sys::vhpiOneToOneT_vhpiLhsExpr as u32,
    Local = vhpi_sys::vhpiOneToOneT_vhpiLocal as u32,
    LogicalExpr = vhpi_sys::vhpiOneToOneT_vhpiLogicalExpr as u32,
    Others = vhpi_sys::vhpiOneToOneT_vhpiOthers as u32,
    OutPort = vhpi_sys::vhpiOneToOneT_vhpiOutPort as u32,
    ParamDecl = vhpi_sys::vhpiOneToOneT_vhpiParamDecl as u32,
    Parent = vhpi_sys::vhpiOneToOneT_vhpiParent as u32,
    PhysLiteral = vhpi_sys::vhpiOneToOneT_vhpiPhysLiteral as u32,
    Prefix = vhpi_sys::vhpiOneToOneT_vhpiPrefix as u32,
    PrimaryUnit = vhpi_sys::vhpiOneToOneT_vhpiPrimaryUnit as u32,
    ProtectedTypeBody = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeBody as u32,
    ProtectedTypeDecl = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeDecl as u32,
    RejectTime = vhpi_sys::vhpiOneToOneT_vhpiRejectTime as u32,
    ReportExpr = vhpi_sys::vhpiOneToOneT_vhpiReportExpr as u32,
    ResolFunc = vhpi_sys::vhpiOneToOneT_vhpiResolFunc as u32,
    ReturnExpr = vhpi_sys::vhpiOneToOneT_vhpiReturnExpr as u32,
    RhsExpr = vhpi_sys::vhpiOneToOneT_vhpiRhsExpr as u32,
    RightExpr = vhpi_sys::vhpiOneToOneT_vhpiRightExpr as u32,
    SelectExpr = vhpi_sys::vhpiOneToOneT_vhpiSelectExpr as u32,
    SeverityExpr = vhpi_sys::vhpiOneToOneT_vhpiSeverityExpr as u32,
    SimpleName = vhpi_sys::vhpiOneToOneT_vhpiSimpleName as u32,
    SubpBody = vhpi_sys::vhpiOneToOneT_vhpiSubpBody as u32,
    SubpDecl = vhpi_sys::vhpiOneToOneT_vhpiSubpDecl as u32,
    Suffix = vhpi_sys::vhpiOneToOneT_vhpiSuffix as u32,
    TimeExpr = vhpi_sys::vhpiOneToOneT_vhpiTimeExpr as u32,
    TimeOutExpr = vhpi_sys::vhpiOneToOneT_vhpiTimeOutExpr as u32,
    Tool = vhpi_sys::vhpiOneToOneT_vhpiTool as u32,
    Type = vhpi_sys::vhpiOneToOneT_vhpiType as u32,
    UnitDecl = vhpi_sys::vhpiOneToOneT_vhpiUnitDecl as u32,
    UpStack = vhpi_sys::vhpiOneToOneT_vhpiUpStack as u32,
    UpperRegion = vhpi_sys::vhpiOneToOneT_vhpiUpperRegion as u32,
    Use = vhpi_sys::vhpiOneToOneT_vhpiUse as u32,
    ValExpr = vhpi_sys::vhpiOneToOneT_vhpiValExpr as u32,
    ElemType = vhpi_sys::vhpiOneToOneT_vhpiElemType as u32,
    FirstNamedType = vhpi_sys::vhpiOneToOneT_vhpiFirstNamedType as u32,
    ReturnType = vhpi_sys::vhpiOneToOneT_vhpiReturnType as u32,
    ValType = vhpi_sys::vhpiOneToOneT_vhpiValType as u32,
    CurRegion = vhpi_sys::vhpiOneToOneT_vhpiCurRegion as u32,
    Signal = vhpi_sys::vhpiOneToOneT_vhpiSignal as u32,
    LibraryDecl = vhpi_sys::vhpiOneToOneT_vhpiLibraryDecl as u32,
    SimNet = vhpi_sys::vhpiOneToOneT_vhpiSimNet as u32,
    AliasedName = vhpi_sys::vhpiOneToOneT_vhpiAliasedName as u32,
    CompDecl = vhpi_sys::vhpiOneToOneT_vhpiCompDecl as u32,
    ProtectedTypeInst = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeInst as u32,
    GenIndex = vhpi_sys::vhpiOneToOneT_vhpiGenIndex as u32,
}

#[repr(u32)]
pub enum OneToMany {
    Decls = vhpi_sys::vhpiOneToManyT_vhpiDecls as u32,
    SigDecls = vhpi_sys::vhpiOneToManyT_vhpiSigDecls as u32,
    PortDecls = vhpi_sys::vhpiOneToManyT_vhpiPortDecls as u32,
    InternalRegions = vhpi_sys::vhpiOneToManyT_vhpiInternalRegions as u32,
    AliasDecls = vhpi_sys::vhpiOneToManyT_vhpiAliasDecls as u32,
    Argvs = vhpi_sys::vhpiOneToManyT_vhpiArgvs as u32,
    AttrDecls = vhpi_sys::vhpiOneToManyT_vhpiAttrDecls as u32,
    AttrSpecs = vhpi_sys::vhpiOneToManyT_vhpiAttrSpecs as u32,
    BasicSignals = vhpi_sys::vhpiOneToManyT_vhpiBasicSignals as u32,
    BlockStmts = vhpi_sys::vhpiOneToManyT_vhpiBlockStmts as u32,
    Branchs = vhpi_sys::vhpiOneToManyT_vhpiBranchs as u32,
    Choices = vhpi_sys::vhpiOneToManyT_vhpiChoices as u32,
    CompInstStmts = vhpi_sys::vhpiOneToManyT_vhpiCompInstStmts as u32,
    CondWaveforms = vhpi_sys::vhpiOneToManyT_vhpiCondWaveforms as u32,
    ConfigItems = vhpi_sys::vhpiOneToManyT_vhpiConfigItems as u32,
    ConfigSpecs = vhpi_sys::vhpiOneToManyT_vhpiConfigSpecs as u32,
    ConstDecls = vhpi_sys::vhpiOneToManyT_vhpiConstDecls as u32,
    Constraints = vhpi_sys::vhpiOneToManyT_vhpiConstraints as u32,
    DepUnits = vhpi_sys::vhpiOneToManyT_vhpiDepUnits as u32,
    DesignUnits = vhpi_sys::vhpiOneToManyT_vhpiDesignUnits as u32,
    DrivenSigs = vhpi_sys::vhpiOneToManyT_vhpiDrivenSigs as u32,
    Drivers = vhpi_sys::vhpiOneToManyT_vhpiDrivers as u32,
    ElemAssocs = vhpi_sys::vhpiOneToManyT_vhpiElemAssocs as u32,
    EntityDesignators = vhpi_sys::vhpiOneToManyT_vhpiEntityDesignators as u32,
    EnumLiterals = vhpi_sys::vhpiOneToManyT_vhpiEnumLiterals as u32,
    Foreignfs = vhpi_sys::vhpiOneToManyT_vhpiForeignfs as u32,
    GenericAssocs = vhpi_sys::vhpiOneToManyT_vhpiGenericAssocs as u32,
    GenericDecls = vhpi_sys::vhpiOneToManyT_vhpiGenericDecls as u32,
    IndexExprs = vhpi_sys::vhpiOneToManyT_vhpiIndexExprs as u32,
    IndexedNames = vhpi_sys::vhpiOneToManyT_vhpiIndexedNames as u32,
    Members = vhpi_sys::vhpiOneToManyT_vhpiMembers as u32,
    PackInsts = vhpi_sys::vhpiOneToManyT_vhpiPackInsts as u32,
    ParamAssocs = vhpi_sys::vhpiOneToManyT_vhpiParamAssocs as u32,
    ParamDecls = vhpi_sys::vhpiOneToManyT_vhpiParamDecls as u32,
    PortAssocs = vhpi_sys::vhpiOneToManyT_vhpiPortAssocs as u32,
    RecordElems = vhpi_sys::vhpiOneToManyT_vhpiRecordElems as u32,
    SelectWaveforms = vhpi_sys::vhpiOneToManyT_vhpiSelectWaveforms as u32,
    SelectedNames = vhpi_sys::vhpiOneToManyT_vhpiSelectedNames as u32,
    SeqStmts = vhpi_sys::vhpiOneToManyT_vhpiSeqStmts as u32,
    SigAttrs = vhpi_sys::vhpiOneToManyT_vhpiSigAttrs as u32,
    SigNames = vhpi_sys::vhpiOneToManyT_vhpiSigNames as u32,
    Signals = vhpi_sys::vhpiOneToManyT_vhpiSignals as u32,
    Specs = vhpi_sys::vhpiOneToManyT_vhpiSpecs as u32,
    Stmts = vhpi_sys::vhpiOneToManyT_vhpiStmts as u32,
    Transactions = vhpi_sys::vhpiOneToManyT_vhpiTransactions as u32,
    UnitDecls = vhpi_sys::vhpiOneToManyT_vhpiUnitDecls as u32,
    Uses = vhpi_sys::vhpiOneToManyT_vhpiUses as u32,
    VarDecls = vhpi_sys::vhpiOneToManyT_vhpiVarDecls as u32,
    WaveformElems = vhpi_sys::vhpiOneToManyT_vhpiWaveformElems as u32,
    LibraryDecls = vhpi_sys::vhpiOneToManyT_vhpiLibraryDecls as u32,
    LocalLoads = vhpi_sys::vhpiOneToManyT_vhpiLocalLoads as u32,
    OptimizedLoads = vhpi_sys::vhpiOneToManyT_vhpiOptimizedLoads as u32,
    Types = vhpi_sys::vhpiOneToManyT_vhpiTypes as u32,
    UseClauses = vhpi_sys::vhpiOneToManyT_vhpiUseClauses as u32,
    GenerateStmts = vhpi_sys::vhpiOneToManyT_vhpiGenerateStmts as u32,
    LocalContributors = vhpi_sys::vhpiOneToManyT_vhpiLocalContributors as u32,
    OptimizedContributors = vhpi_sys::vhpiOneToManyT_vhpiOptimizedContributors as u32,
    ParamExprs = vhpi_sys::vhpiOneToManyT_vhpiParamExprs as u32,
    EqProcessStmts = vhpi_sys::vhpiOneToManyT_vhpiEqProcessStmts as u32,
    EntityClassEntries = vhpi_sys::vhpiOneToManyT_vhpiEntityClassEntries as u32,
    Sensitivities = vhpi_sys::vhpiOneToManyT_vhpiSensitivities as u32,
}

pub struct Handle {
    handle: vhpiHandleT,
}

pub struct HandleIterator {
    pub(crate) iter: Handle,
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
        unsafe { !vhpi_compare_handles(self.handle, other.handle).is_zero() }
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
        Handle::from_raw(unsafe { vhpi_handle(property as vhpi_sys::vhpiOneToOneT, self.as_raw()) })
    }

    #[must_use]
    pub fn handle_by_name(&self, name: &str) -> Option<Handle> {
        let c_name = CString::new(name).unwrap();
        let handle = unsafe { vhpi_handle_by_name(c_name.as_ptr(), self.as_raw()) };
        if handle.is_null() {
            None
        } else {
            Some(Handle::from_raw(handle))
        }
    }

    #[must_use]
    pub fn handle_by_index(&self, property: OneToMany, index: i32) -> Option<Handle> {
        let handle = unsafe {
            vhpi_handle_by_index(property as vhpi_sys::vhpiOneToManyT, self.as_raw(), index)
        };
        if handle.is_null() {
            None
        } else {
            Some(Handle::from_raw(handle))
        }
    }

    #[must_use]
    pub fn iterator(&self, typ: OneToMany) -> HandleIterator {
        let raw = unsafe { vhpi_iterator(typ as vhpi_sys::vhpiOneToManyT, self.as_raw()) };
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
    Handle::from_raw(unsafe {
        vhpi_handle(property as vhpi_sys::vhpiOneToOneT, std::ptr::null_mut())
    })
}

#[must_use]
pub fn handle_by_name(name: &str) -> Option<Handle> {
    let c_name = CString::new(name).unwrap();
    let handle = unsafe { vhpi_handle_by_name(c_name.as_ptr(), std::ptr::null_mut()) };
    if handle.is_null() {
        None
    } else {
        Some(Handle::from_raw(handle))
    }
}
