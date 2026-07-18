#![cfg_attr(not(windows), allow(clippy::unnecessary_cast))]

use num_traits::Zero;
use std::ffi::CString;
use vhpi_sys::{
    vhpiHandleT, vhpi_compare_handles, vhpi_handle, vhpi_handle_by_index, vhpi_handle_by_name,
    vhpi_iterator, vhpi_release_handle, vhpi_scan,
};

#[repr(u32)]
/// VHPI one-to-one relationship selectors used with `vhpi_handle`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneToOne {
    /// Root instance of the elaborated design.
    RootInst = vhpi_sys::vhpiOneToOneT_vhpiRootInst as u32,
    /// Abstract numeric literal associated with an object.
    AbstractLiteral = vhpi_sys::vhpiOneToOneT_vhpiAbstractLiteral as u32,
    /// Actual side of an association element.
    Actual = vhpi_sys::vhpiOneToOneT_vhpiActual as u32,
    /// Represents an `all` choice.
    All = vhpi_sys::vhpiOneToOneT_vhpiAll as u32,
    /// Attribute declaration object.
    AttrDecl = vhpi_sys::vhpiOneToOneT_vhpiAttrDecl as u32,
    /// Attribute specification object.
    AttrSpec = vhpi_sys::vhpiOneToOneT_vhpiAttrSpec as u32,
    /// Base type of a subtype or derived type.
    BaseType = vhpi_sys::vhpiOneToOneT_vhpiBaseType as u32,
    /// Base unit of a physical type.
    BaseUnit = vhpi_sys::vhpiOneToOneT_vhpiBaseUnit as u32,
    /// Block configuration object.
    BlockConfig = vhpi_sys::vhpiOneToOneT_vhpiBlockConfig as u32,
    /// Case expression for a case statement.
    CaseExpr = vhpi_sys::vhpiOneToOneT_vhpiCaseExpr as u32,
    /// Condition expression for a conditional construct.
    CondExpr = vhpi_sys::vhpiOneToOneT_vhpiCondExpr as u32,
    /// Configuration declaration object.
    ConfigDecl = vhpi_sys::vhpiOneToOneT_vhpiConfigDecl as u32,
    /// Configuration specification object.
    ConfigSpec = vhpi_sys::vhpiOneToOneT_vhpiConfigSpec as u32,
    /// Constraint associated with a type or object.
    Constraint = vhpi_sys::vhpiOneToOneT_vhpiConstraint as u32,
    /// Contributor object in a resolved signal context.
    Contributor = vhpi_sys::vhpiOneToOneT_vhpiContributor as u32,
    /// Callback currently being executed.
    CurCallback = vhpi_sys::vhpiOneToOneT_vhpiCurCallback as u32,
    /// Current stack frame during subprogram execution.
    CurStackFrame = vhpi_sys::vhpiOneToOneT_vhpiCurStackFrame as u32,
    /// Dereferenced object for an access value.
    DerefObj = vhpi_sys::vhpiOneToOneT_vhpiDerefObj as u32,
    /// Design unit containing the queried object.
    DesignUnit = vhpi_sys::vhpiOneToOneT_vhpiDesignUnit as u32,
    /// Next stack frame down the call stack.
    DownStack = vhpi_sys::vhpiOneToOneT_vhpiDownStack as u32,
    /// Entity aspect in a configuration or binding.
    EntityAspect = vhpi_sys::vhpiOneToOneT_vhpiEntityAspect as u32,
    /// Entity declaration object.
    EntityDecl = vhpi_sys::vhpiOneToOneT_vhpiEntityDecl as u32,
    /// Equivalent process statement.
    EqProcessStmt = vhpi_sys::vhpiOneToOneT_vhpiEqProcessStmt as u32,
    /// Generic expression node.
    Expr = vhpi_sys::vhpiOneToOneT_vhpiExpr as u32,
    /// Formal side of an association element.
    Formal = vhpi_sys::vhpiOneToOneT_vhpiFormal as u32,
    /// Function declaration object.
    FuncDecl = vhpi_sys::vhpiOneToOneT_vhpiFuncDecl as u32,
    /// Group template declaration object.
    GroupTempDecl = vhpi_sys::vhpiOneToOneT_vhpiGroupTempDecl as u32,
    /// Guard expression for guarded constructs.
    GuardExpr = vhpi_sys::vhpiOneToOneT_vhpiGuardExpr as u32,
    /// Guard signal for guarded assignments.
    GuardSig = vhpi_sys::vhpiOneToOneT_vhpiGuardSig as u32,
    /// Immediate region containing the object.
    ImmRegion = vhpi_sys::vhpiOneToOneT_vhpiImmRegion as u32,
    /// Input port view of an interface object.
    InPort = vhpi_sys::vhpiOneToOneT_vhpiInPort as u32,
    /// Initialization expression of a declaration.
    InitExpr = vhpi_sys::vhpiOneToOneT_vhpiInitExpr as u32,
    /// Left expression in a binary context.
    LeftExpr = vhpi_sys::vhpiOneToOneT_vhpiLeftExpr as u32,
    /// Lexical scope containing the object.
    LexicalScope = vhpi_sys::vhpiOneToOneT_vhpiLexicalScope as u32,
    /// Left-hand-side expression.
    LhsExpr = vhpi_sys::vhpiOneToOneT_vhpiLhsExpr as u32,
    /// Locally declared object.
    Local = vhpi_sys::vhpiOneToOneT_vhpiLocal as u32,
    /// Logical expression object.
    LogicalExpr = vhpi_sys::vhpiOneToOneT_vhpiLogicalExpr as u32,
    /// Represents an `others` choice.
    Others = vhpi_sys::vhpiOneToOneT_vhpiOthers as u32,
    /// Output port view of an interface object.
    OutPort = vhpi_sys::vhpiOneToOneT_vhpiOutPort as u32,
    /// Parameter declaration object.
    ParamDecl = vhpi_sys::vhpiOneToOneT_vhpiParamDecl as u32,
    /// Parent object in the design hierarchy.
    Parent = vhpi_sys::vhpiOneToOneT_vhpiParent as u32,
    /// Physical literal object.
    PhysLiteral = vhpi_sys::vhpiOneToOneT_vhpiPhysLiteral as u32,
    /// Prefix object for selected or indexed names.
    Prefix = vhpi_sys::vhpiOneToOneT_vhpiPrefix as u32,
    /// Primary design unit for a secondary unit.
    PrimaryUnit = vhpi_sys::vhpiOneToOneT_vhpiPrimaryUnit as u32,
    /// Body of a protected type.
    ProtectedTypeBody = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeBody as u32,
    /// Declaration of a protected type.
    ProtectedTypeDecl = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeDecl as u32,
    /// Reject time expression in waveform assignments.
    RejectTime = vhpi_sys::vhpiOneToOneT_vhpiRejectTime as u32,
    /// Report expression in a report statement.
    ReportExpr = vhpi_sys::vhpiOneToOneT_vhpiReportExpr as u32,
    /// Resolution function associated with a type.
    ResolFunc = vhpi_sys::vhpiOneToOneT_vhpiResolFunc as u32,
    /// Return expression in a return statement.
    ReturnExpr = vhpi_sys::vhpiOneToOneT_vhpiReturnExpr as u32,
    /// Right-hand-side expression.
    RhsExpr = vhpi_sys::vhpiOneToOneT_vhpiRhsExpr as u32,
    /// Right expression in a binary context.
    RightExpr = vhpi_sys::vhpiOneToOneT_vhpiRightExpr as u32,
    /// Selector expression of a selected assignment.
    SelectExpr = vhpi_sys::vhpiOneToOneT_vhpiSelectExpr as u32,
    /// Severity expression for assertions or reports.
    SeverityExpr = vhpi_sys::vhpiOneToOneT_vhpiSeverityExpr as u32,
    /// Simple name object.
    SimpleName = vhpi_sys::vhpiOneToOneT_vhpiSimpleName as u32,
    /// Subprogram body object.
    SubpBody = vhpi_sys::vhpiOneToOneT_vhpiSubpBody as u32,
    /// Subprogram declaration object.
    SubpDecl = vhpi_sys::vhpiOneToOneT_vhpiSubpDecl as u32,
    /// Suffix object for selected names.
    Suffix = vhpi_sys::vhpiOneToOneT_vhpiSuffix as u32,
    /// Time expression object.
    TimeExpr = vhpi_sys::vhpiOneToOneT_vhpiTimeExpr as u32,
    /// Timeout expression object.
    TimeOutExpr = vhpi_sys::vhpiOneToOneT_vhpiTimeOutExpr as u32,
    /// Simulator tool object.
    Tool = vhpi_sys::vhpiOneToOneT_vhpiTool as u32,
    /// Type object.
    Type = vhpi_sys::vhpiOneToOneT_vhpiType as u32,
    /// Unit declaration object.
    UnitDecl = vhpi_sys::vhpiOneToOneT_vhpiUnitDecl as u32,
    /// Next stack frame up the call stack.
    UpStack = vhpi_sys::vhpiOneToOneT_vhpiUpStack as u32,
    /// Enclosing region above the current one.
    UpperRegion = vhpi_sys::vhpiOneToOneT_vhpiUpperRegion as u32,
    /// Use clause object.
    Use = vhpi_sys::vhpiOneToOneT_vhpiUse as u32,
    /// Value expression object.
    ValExpr = vhpi_sys::vhpiOneToOneT_vhpiValExpr as u32,
    /// Element type of an array or collection.
    ElemType = vhpi_sys::vhpiOneToOneT_vhpiElemType as u32,
    /// First named type associated with a declaration chain.
    FirstNamedType = vhpi_sys::vhpiOneToOneT_vhpiFirstNamedType as u32,
    /// Return type of a subprogram.
    ReturnType = vhpi_sys::vhpiOneToOneT_vhpiReturnType as u32,
    /// Type of a value object.
    ValType = vhpi_sys::vhpiOneToOneT_vhpiValType as u32,
    /// Current region in simulator context.
    CurRegion = vhpi_sys::vhpiOneToOneT_vhpiCurRegion as u32,
    /// Signal object.
    Signal = vhpi_sys::vhpiOneToOneT_vhpiSignal as u32,
    /// Library declaration object.
    LibraryDecl = vhpi_sys::vhpiOneToOneT_vhpiLibraryDecl as u32,
    /// Simulated net object.
    SimNet = vhpi_sys::vhpiOneToOneT_vhpiSimNet as u32,
    /// Aliased name object.
    AliasedName = vhpi_sys::vhpiOneToOneT_vhpiAliasedName as u32,
    /// Component declaration object.
    CompDecl = vhpi_sys::vhpiOneToOneT_vhpiCompDecl as u32,
    /// Instance of a protected type.
    ProtectedTypeInst = vhpi_sys::vhpiOneToOneT_vhpiProtectedTypeInst as u32,
    /// Generate index object.
    GenIndex = vhpi_sys::vhpiOneToOneT_vhpiGenIndex as u32,
}

#[repr(u32)]
/// VHPI one-to-many relationship selectors used for indexed and iterator access.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneToMany {
    /// Declarations contained in a scope.
    Decls = vhpi_sys::vhpiOneToManyT_vhpiDecls as u32,
    /// Signal declarations in a scope.
    SigDecls = vhpi_sys::vhpiOneToManyT_vhpiSigDecls as u32,
    /// Port declarations for an interface.
    PortDecls = vhpi_sys::vhpiOneToManyT_vhpiPortDecls as u32,
    /// Internal regions nested in the current region.
    InternalRegions = vhpi_sys::vhpiOneToManyT_vhpiInternalRegions as u32,
    /// Alias declarations.
    AliasDecls = vhpi_sys::vhpiOneToManyT_vhpiAliasDecls as u32,
    /// Command-line argument values.
    Argvs = vhpi_sys::vhpiOneToManyT_vhpiArgvs as u32,
    /// Attribute declarations.
    AttrDecls = vhpi_sys::vhpiOneToManyT_vhpiAttrDecls as u32,
    /// Attribute specifications.
    AttrSpecs = vhpi_sys::vhpiOneToManyT_vhpiAttrSpecs as u32,
    /// Basic signals associated with an object.
    BasicSignals = vhpi_sys::vhpiOneToManyT_vhpiBasicSignals as u32,
    /// Block statements in a region.
    BlockStmts = vhpi_sys::vhpiOneToManyT_vhpiBlockStmts as u32,
    /// Branch alternatives.
    Branchs = vhpi_sys::vhpiOneToManyT_vhpiBranchs as u32,
    /// Choice alternatives.
    Choices = vhpi_sys::vhpiOneToManyT_vhpiChoices as u32,
    /// Component instantiation statements.
    CompInstStmts = vhpi_sys::vhpiOneToManyT_vhpiCompInstStmts as u32,
    /// Conditional waveform elements.
    CondWaveforms = vhpi_sys::vhpiOneToManyT_vhpiCondWaveforms as u32,
    /// Configuration items.
    ConfigItems = vhpi_sys::vhpiOneToManyT_vhpiConfigItems as u32,
    /// Configuration specifications.
    ConfigSpecs = vhpi_sys::vhpiOneToManyT_vhpiConfigSpecs as u32,
    /// Constant declarations.
    ConstDecls = vhpi_sys::vhpiOneToManyT_vhpiConstDecls as u32,
    /// Constraint objects.
    Constraints = vhpi_sys::vhpiOneToManyT_vhpiConstraints as u32,
    /// Dependent units.
    DepUnits = vhpi_sys::vhpiOneToManyT_vhpiDepUnits as u32,
    /// Design units in a library.
    DesignUnits = vhpi_sys::vhpiOneToManyT_vhpiDesignUnits as u32,
    /// Driven signals.
    DrivenSigs = vhpi_sys::vhpiOneToManyT_vhpiDrivenSigs as u32,
    /// Driver objects.
    Drivers = vhpi_sys::vhpiOneToManyT_vhpiDrivers as u32,
    /// Element associations.
    ElemAssocs = vhpi_sys::vhpiOneToManyT_vhpiElemAssocs as u32,
    /// Entity designators.
    EntityDesignators = vhpi_sys::vhpiOneToManyT_vhpiEntityDesignators as u32,
    /// Enumeration literals.
    EnumLiterals = vhpi_sys::vhpiOneToManyT_vhpiEnumLiterals as u32,
    /// Foreign interface specifications.
    Foreignfs = vhpi_sys::vhpiOneToManyT_vhpiForeignfs as u32,
    /// Generic associations.
    GenericAssocs = vhpi_sys::vhpiOneToManyT_vhpiGenericAssocs as u32,
    /// Generic declarations.
    GenericDecls = vhpi_sys::vhpiOneToManyT_vhpiGenericDecls as u32,
    /// Index expressions.
    IndexExprs = vhpi_sys::vhpiOneToManyT_vhpiIndexExprs as u32,
    /// Indexed-name objects.
    IndexedNames = vhpi_sys::vhpiOneToManyT_vhpiIndexedNames as u32,
    /// Member declarations.
    Members = vhpi_sys::vhpiOneToManyT_vhpiMembers as u32,
    /// Package instantiations.
    PackInsts = vhpi_sys::vhpiOneToManyT_vhpiPackInsts as u32,
    /// Parameter associations.
    ParamAssocs = vhpi_sys::vhpiOneToManyT_vhpiParamAssocs as u32,
    /// Parameter declarations.
    ParamDecls = vhpi_sys::vhpiOneToManyT_vhpiParamDecls as u32,
    /// Port associations.
    PortAssocs = vhpi_sys::vhpiOneToManyT_vhpiPortAssocs as u32,
    /// Record elements.
    RecordElems = vhpi_sys::vhpiOneToManyT_vhpiRecordElems as u32,
    /// Selected waveform elements.
    SelectWaveforms = vhpi_sys::vhpiOneToManyT_vhpiSelectWaveforms as u32,
    /// Selected-name objects.
    SelectedNames = vhpi_sys::vhpiOneToManyT_vhpiSelectedNames as u32,
    /// Sequential statements.
    SeqStmts = vhpi_sys::vhpiOneToManyT_vhpiSeqStmts as u32,
    /// Signal attributes.
    SigAttrs = vhpi_sys::vhpiOneToManyT_vhpiSigAttrs as u32,
    /// Signal names.
    SigNames = vhpi_sys::vhpiOneToManyT_vhpiSigNames as u32,
    /// Signal objects.
    Signals = vhpi_sys::vhpiOneToManyT_vhpiSignals as u32,
    /// Specification objects.
    Specs = vhpi_sys::vhpiOneToManyT_vhpiSpecs as u32,
    /// Statement objects.
    Stmts = vhpi_sys::vhpiOneToManyT_vhpiStmts as u32,
    /// Scheduled transactions.
    Transactions = vhpi_sys::vhpiOneToManyT_vhpiTransactions as u32,
    /// Unit declarations.
    UnitDecls = vhpi_sys::vhpiOneToManyT_vhpiUnitDecls as u32,
    /// Use-clause objects.
    Uses = vhpi_sys::vhpiOneToManyT_vhpiUses as u32,
    /// Variable declarations.
    VarDecls = vhpi_sys::vhpiOneToManyT_vhpiVarDecls as u32,
    /// Waveform elements.
    WaveformElems = vhpi_sys::vhpiOneToManyT_vhpiWaveformElems as u32,
    /// Library declarations.
    LibraryDecls = vhpi_sys::vhpiOneToManyT_vhpiLibraryDecls as u32,
    /// Locally computed load contributors.
    LocalLoads = vhpi_sys::vhpiOneToManyT_vhpiLocalLoads as u32,
    /// Optimized load contributors.
    OptimizedLoads = vhpi_sys::vhpiOneToManyT_vhpiOptimizedLoads as u32,
    /// Type declarations.
    Types = vhpi_sys::vhpiOneToManyT_vhpiTypes as u32,
    /// Use clauses in a design unit.
    UseClauses = vhpi_sys::vhpiOneToManyT_vhpiUseClauses as u32,
    /// Generate statements.
    GenerateStmts = vhpi_sys::vhpiOneToManyT_vhpiGenerateStmts as u32,
    /// Locally computed contributors.
    LocalContributors = vhpi_sys::vhpiOneToManyT_vhpiLocalContributors as u32,
    /// Optimized contributors.
    OptimizedContributors = vhpi_sys::vhpiOneToManyT_vhpiOptimizedContributors as u32,
    /// Parameter expressions.
    ParamExprs = vhpi_sys::vhpiOneToManyT_vhpiParamExprs as u32,
    /// Equivalent process statements.
    EqProcessStmts = vhpi_sys::vhpiOneToManyT_vhpiEqProcessStmts as u32,
    /// Entity class entries.
    EntityClassEntries = vhpi_sys::vhpiOneToManyT_vhpiEntityClassEntries as u32,
    /// Sensitivity-list entries.
    Sensitivities = vhpi_sys::vhpiOneToManyT_vhpiSensitivities as u32,
}

/// Owned wrapper around a `vhpiHandleT`.
///
/// A non-null handle is automatically released on drop.
#[derive(Debug, Clone)]
pub struct Handle {
    handle: vhpiHandleT,
}

/// Iterator over VHPI handles produced by `vhpi_iterator`/`vhpi_scan`.
#[derive(Debug)]
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
    /// Create a null handle.
    pub fn null() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    #[must_use]
    /// Return `true` when this handle is null.
    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    pub fn as_raw(&self) -> vhpiHandleT {
        self.handle
    }

    pub(crate) fn clear(&mut self) {
        self.handle = std::ptr::null_mut();
    }

    /// Construct a `Handle` from a raw VHPI handle.
    ///
    /// The returned wrapper takes ownership and will release the handle on drop
    /// when it is non-null.
    pub fn from_raw(raw: vhpiHandleT) -> Self {
        Self { handle: raw }
    }

    #[must_use]
    /// Look up a related object through a one-to-one relationship.
    pub fn handle(&self, property: OneToOne) -> Handle {
        Handle::from_raw(unsafe { vhpi_handle(property as vhpi_sys::vhpiOneToOneT, self.as_raw()) })
    }

    #[must_use]
    /// Look up a related object by name relative to this handle.
    ///
    /// Returns `None` when no object with `name` is found.
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
    /// Look up a related object by index for a one-to-many relationship.
    ///
    /// Returns `None` when the index is out of range or the relationship is not
    /// available.
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
    /// Create an iterator for a one-to-many relationship.
    ///
    /// The underlying iterator handle is released automatically when exhausted
    /// or dropped.
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
/// Look up a top-level object through a one-to-one relationship.
pub fn handle(property: OneToOne) -> Handle {
    Handle::from_raw(unsafe {
        vhpi_handle(property as vhpi_sys::vhpiOneToOneT, std::ptr::null_mut())
    })
}

#[must_use]
/// Look up a top-level object by name.
///
/// Returns `None` when no object with `name` is found.
pub fn handle_by_name(name: &str) -> Option<Handle> {
    let c_name = CString::new(name).unwrap();
    let handle = unsafe { vhpi_handle_by_name(c_name.as_ptr(), std::ptr::null_mut()) };
    if handle.is_null() {
        None
    } else {
        Some(Handle::from_raw(handle))
    }
}
