use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

use vhpi::{startup_routines, CbData, CbReason, OneToOne, Time};

const DISABLE_AT_FS: i64 = 13_000_000;
const CHECK_DISABLED_AT_FS: i64 = 22_000_000;
const ENABLE_AT_FS: i64 = 23_000_000;
const CHECK_ENABLED_AT_FS: i64 = 34_000_000;

const EXPECTED_CALL_TIMES_FS: [i64; 4] = [5_000_000, 10_000_000, 25_000_000, 30_000_000];
const EXPECTED_CALLS_WHILE_DISABLED: usize = 2;

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static OBSERVED_CALL_TIMES_FS: LazyLock<Mutex<Vec<i64>>> = LazyLock::new(|| Mutex::new(Vec::new()));

thread_local! {
    static VALUE_CHANGE_CB: RefCell<Option<vhpi::Handle>> = const { RefCell::new(None) };
}

fn with_value_change_cb<R>(f: impl FnOnce(&vhpi::Handle) -> R) -> R {
    VALUE_CHANGE_CB.with(|cell| {
        let borrow = cell.borrow();
        let handle = borrow
            .as_ref()
            .expect("value-change callback handle was not initialized");
        f(handle)
    })
}

fn value_change(_data: &CbData) {
    let now = vhpi::get_time();
    let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    OBSERVED_CALL_TIMES_FS
        .lock()
        .expect("poisoned callback-times mutex")
        .push(now.to_i64());

    vhpi::printf!("cb_toggle: value change callback fired #{count} at {now}",);
}

fn disable_value_change_cb(_data: &CbData) {
    with_value_change_cb(|cb| {
        cb.disable_cb()
            .expect("failed to disable value-change callback");
    });

    vhpi::printf!("cb_toggle: callback disabled at {}", vhpi::get_time());
}

fn check_disabled_window(_data: &CbData) {
    let call_count = CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(
        call_count, EXPECTED_CALLS_WHILE_DISABLED,
        "cb_toggle: callback fired while disabled; expected {EXPECTED_CALLS_WHILE_DISABLED} calls before enable, got {call_count}"
    );

    vhpi::printf!(
        "cb_toggle: callback remained disabled during off window (count = {})",
        call_count
    );
}

fn enable_value_change_cb(_data: &CbData) {
    with_value_change_cb(|cb| {
        cb.enable_cb()
            .expect("failed to enable value-change callback");
    });

    vhpi::printf!("cb_toggle: callback re-enabled at {}", vhpi::get_time());
}

fn check_enabled_window(_data: &CbData) {
    let call_count = CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(
        call_count,
        EXPECTED_CALL_TIMES_FS.len(),
        "cb_toggle: callback did not resume correctly; expected {} calls, got {}",
        EXPECTED_CALL_TIMES_FS.len(),
        call_count
    );

    let observed_times = OBSERVED_CALL_TIMES_FS
        .lock()
        .expect("poisoned callback-times mutex")
        .clone();
    assert_eq!(
        observed_times, EXPECTED_CALL_TIMES_FS,
        "cb_toggle: unexpected callback fire times"
    );

    vhpi::printf!(
        "cb_toggle: callback re-enable checks passed ({} total firings)",
        call_count
    );
}

fn end_of_sim(data: &CbData) {
    check_enabled_window(data);

    VALUE_CHANGE_CB.with(|cell| {
        let _ = cell.borrow_mut().take();
    });

    vhpi::printf!("cb_toggle: all callback toggle checks passed");
}

#[no_mangle]
pub extern "C" fn cb_toggle_startup() {
    vhpi::printf!("cb_toggle plugin loaded");

    let _ = vhpi::register_cb(CbReason::StartOfSimulation, start_of_sim);
    let _ = vhpi::register_cb(CbReason::EndOfSimulation, end_of_sim);
}

fn start_of_sim(_data: &CbData) {
    let root = vhpi::handle(OneToOne::RootInst);
    let signal = root
        .handle_by_name("s_watch")
        .expect("signal s_watch not found");

    let value_cb = signal
        .register_cb(CbReason::ValueChange, value_change)
        .expect("failed to register value-change callback");
    VALUE_CHANGE_CB.with(|cell| {
        *cell.borrow_mut() = Some(value_cb);
    });

    vhpi::register_cb_after_delay(Time::from(DISABLE_AT_FS), disable_value_change_cb)
        .expect("failed to register disable callback");
    vhpi::register_cb_after_delay(Time::from(CHECK_DISABLED_AT_FS), check_disabled_window)
        .expect("failed to register disabled-window check callback");
    vhpi::register_cb_after_delay(Time::from(ENABLE_AT_FS), enable_value_change_cb)
        .expect("failed to register enable callback");
    vhpi::register_cb_after_delay(Time::from(CHECK_ENABLED_AT_FS), check_enabled_window)
        .expect("failed to register enabled-window check callback");
}

startup_routines! {
    cb_toggle_startup,
}
