use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

use vhpi::{
    startup_routines, CbData, CbReason, ForeignData, ForeignExecData, ForeignKind, Format,
    OneToMany, PutValueMode, Value,
};

const EXPECTED_CALL_TIMES_FS: [i64; 4] = [0, 5_000_000, 10_000_000, 15_000_000];
const EXPECTED_ADD_CALLS: usize = 4;
const EXPECTED_BIT_REVERSE_CALLS: usize = 6;

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static OBSERVED_CALL_TIMES_FS: LazyLock<Mutex<Vec<i64>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static ADD_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static BIT_REVERSE_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn mark_call_exec(_call_data: &ForeignExecData) {
    let call_index = CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    let now = vhpi::get_time();

    OBSERVED_CALL_TIMES_FS
        .lock()
        .expect("poisoned observed-times mutex")
        .push(now.to_i64());

    vhpi::printf!(
        "foreignf: mark_call invocation {} at {}",
        call_index + 1,
        now
    );
}

unsafe extern "C" fn add_ints_exec(call_data: &ForeignExecData) {
    let func_handle = call_data.obj();

    let params: Vec<_> = func_handle.iterator(OneToMany::ParamDecls).collect();
    assert_eq!(
        params.len(),
        2,
        "foreignf: add_ints expected 2 parameters, got {}",
        params.len()
    );

    let args = call_data.get_foreignf_args([Format::Int, Format::Int]);

    let a = match args[0]
        .clone()
        .expect("foreignf: failed to read add_ints first parameter")
    {
        Value::Int(v) => v,
        other => panic!("foreignf: add_ints first parameter is not integer: {other:?}"),
    };

    let b = match args[1]
        .clone()
        .expect("foreignf: failed to read add_ints second parameter")
    {
        Value::Int(v) => v,
        other => panic!("foreignf: add_ints second parameter is not integer: {other:?}"),
    };

    let sum = a + b;
    func_handle
        .put_value(Value::Int(sum), PutValueMode::Deposit)
        .expect("foreignf: failed to write add_ints return value");

    let call_index = ADD_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    vhpi::printf!(
        "foreignf: add_ints invocation {}: {} + {} = {}",
        call_index + 1,
        a,
        b,
        sum
    );
}

unsafe extern "C" fn bit_reverse_exec(call_data: &ForeignExecData) {
    let func_handle = call_data.obj();

    let params: Vec<_> = func_handle.iterator(OneToMany::ParamDecls).collect();
    assert_eq!(
        params.len(),
        2,
        "foreignf: bit_reverse expected 2 parameters, got {}",
        params.len()
    );

    let bits = match call_data
        .get_foreignf_arg(0, Format::LogicVec)
        .expect("foreignf: failed to read bit_reverse parameter")
    {
        Value::LogicVec(v) => v,
        other => panic!("foreignf: bit_reverse parameter is not logic vector: {other:?}"),
    };

    let rev_bits = bits.reverse();
    // Prefer direct return-buffer writes when the simulator provides one.
    // Fall back to the explicit out parameter path for broad compatibility.
    if call_data.try_put_return_value(rev_bits.as_value()).is_err() {
        params[1]
            .put_value(rev_bits.as_value(), PutValueMode::Deposit)
            .expect("foreignf: failed to write bit_reverse output value");
    }

    let call_index = BIT_REVERSE_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    vhpi::printf!("foreignf: bit_reverse invocation {}", call_index + 1);
}

fn end_of_sim(_data: &CbData) {
    let call_count = CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(
        call_count,
        EXPECTED_CALL_TIMES_FS.len(),
        "foreignf: expected {} calls, got {}",
        EXPECTED_CALL_TIMES_FS.len(),
        call_count
    );

    let observed_times = OBSERVED_CALL_TIMES_FS
        .lock()
        .expect("poisoned observed-times mutex")
        .clone();
    assert_eq!(
        observed_times, EXPECTED_CALL_TIMES_FS,
        "foreignf: unexpected call times"
    );

    let add_calls = ADD_CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(
        add_calls, EXPECTED_ADD_CALLS,
        "foreignf: expected {EXPECTED_ADD_CALLS} add_ints calls, got {add_calls}"
    );

    let bit_reverse_calls = BIT_REVERSE_CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(
        bit_reverse_calls, EXPECTED_BIT_REVERSE_CALLS,
        "foreignf: expected {EXPECTED_BIT_REVERSE_CALLS} bit_reverse calls, got {bit_reverse_calls}"
    );

    vhpi::printf!(
        "foreignf: all checks passed ({} mark_call invocations, {} add_ints invocations, {} bit_reverse invocations)",
        EXPECTED_CALL_TIMES_FS.len(),
        EXPECTED_ADD_CALLS,
        EXPECTED_BIT_REVERSE_CALLS
    );
}

#[no_mangle]
pub extern "C" fn foreignf_startup() {
    vhpi::printf!("foreignf plugin loaded");

    let registration = vhpi::register_foreignf(
        &ForeignData::new(ForeignKind::Proc, "rust_vhpi_tests", "mark_call").exec(mark_call_exec),
    )
    .expect("failed to register foreign procedure mark_call");

    let info = registration
        .get_foreignf_info()
        .expect("failed to query foreign procedure registration info");
    assert_eq!(info.kind, ForeignKind::Proc, "foreignf: wrong kind");
    assert_eq!(
        info.library_name.as_deref(),
        Some("rust_vhpi_tests"),
        "foreignf: wrong library name"
    );
    assert_eq!(
        info.model_name.as_deref(),
        Some("mark_call"),
        "foreignf: wrong model name"
    );
    assert!(info.exec.is_some(), "foreignf: missing exec callback");

    let add_registration = vhpi::register_foreignf(
        &ForeignData::new(ForeignKind::Func, "rust_vhpi_tests", "add_ints").exec(add_ints_exec),
    )
    .expect("failed to register foreign function add_ints");

    let add_info = add_registration
        .get_foreignf_info()
        .expect("failed to query foreign function registration info");
    assert_eq!(
        add_info.kind,
        ForeignKind::Func,
        "foreignf: wrong add_ints kind"
    );
    assert_eq!(
        add_info.library_name.as_deref(),
        Some("rust_vhpi_tests"),
        "foreignf: wrong add_ints library name"
    );
    assert_eq!(
        add_info.model_name.as_deref(),
        Some("add_ints"),
        "foreignf: wrong add_ints model name"
    );
    assert!(
        add_info.exec.is_some(),
        "foreignf: missing add_ints exec callback"
    );

    let bit_reverse_registration = vhpi::register_foreignf(
        &ForeignData::new(ForeignKind::Proc, "rust_vhpi_tests", "bit_reverse")
            .exec(bit_reverse_exec),
    )
    .expect("failed to register foreign function bit_reverse");

    let bit_reverse_info = bit_reverse_registration
        .get_foreignf_info()
        .expect("failed to query foreign function bit_reverse info");
    assert_eq!(
        bit_reverse_info.kind,
        ForeignKind::Proc,
        "foreignf: wrong bit_reverse kind"
    );
    assert_eq!(
        bit_reverse_info.library_name.as_deref(),
        Some("rust_vhpi_tests"),
        "foreignf: wrong bit_reverse library name"
    );
    assert_eq!(
        bit_reverse_info.model_name.as_deref(),
        Some("bit_reverse"),
        "foreignf: wrong bit_reverse model name"
    );
    assert!(
        bit_reverse_info.exec.is_some(),
        "foreignf: missing bit_reverse exec callback"
    );

    let _ = vhpi::register_cb(CbReason::EndOfSimulation, end_of_sim);
}

startup_routines! {
    foreignf_startup,
}
