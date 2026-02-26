use std::cell::RefCell;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::LazyLock;

use vhpi::{
    string_to_logic_vec, CbData, CbReason, Format, LogicVal, OneToOne, PutValueMode, Value,
};

#[derive(Clone)]
struct Checkpoint {
    time_fs: i64,
    s_a: Value,
    s_b: Value,
    v_x: &'static str,
    v_y: &'static str,
}

const CHECKPOINTS: [Checkpoint; 6] = [
    Checkpoint {
        time_fs: 0,
        s_a: Value::Logic(LogicVal::Zero),
        s_b: Value::Logic(LogicVal::One),
        v_x: "0000",
        v_y: "11111111",
    },
    Checkpoint {
        time_fs: 10_000_000,
        s_a: Value::Logic(LogicVal::One),
        s_b: Value::Logic(LogicVal::One),
        v_x: "0001",
        v_y: "10100101",
    },
    Checkpoint {
        time_fs: 20_000_000,
        s_a: Value::Logic(LogicVal::One),
        s_b: Value::Logic(LogicVal::Zero),
        v_x: "0011",
        v_y: "00111100",
    },
    Checkpoint {
        time_fs: 30_000_000,
        s_a: Value::Logic(LogicVal::Zero),
        s_b: Value::Logic(LogicVal::One),
        v_x: "0101",
        v_y: "11111111",
    },
    Checkpoint {
        time_fs: 40_000_000,
        s_a: Value::Logic(LogicVal::One),
        s_b: Value::Logic(LogicVal::One),
        v_x: "1001",
        v_y: "00000000",
    },
    Checkpoint {
        time_fs: 50_000_000,
        s_a: Value::Logic(LogicVal::One),
        s_b: Value::Logic(LogicVal::Zero),
        v_x: "1110",
        v_y: "01011010",
    },
];

static NEXT_CHECKPOINT: AtomicUsize = AtomicUsize::new(0);
static LAST_TIME_FS: AtomicI64 = AtomicI64::new(-1);

struct SignalHandles {
    s_a: vhpi::Handle,
    s_b: vhpi::Handle,
    v_x: vhpi::Handle,
    v_y: vhpi::Handle,
}

#[derive()]
struct InjectedCheck {
    deposit_time_fs: i64,
    check_delay_fs: i64,
    signal_name: &'static str,
    deposit_value: Value,
    expected_value: Value,
}

static INJECTED_CHECKS: LazyLock<[InjectedCheck; 2]> = LazyLock::new(|| {
    [
        InjectedCheck {
            deposit_time_fs: 5_000_000,
            check_delay_fs: 1_000_000,
            signal_name: "s_a",
            deposit_value: Value::Logic(LogicVal::DontCare),
            expected_value: Value::Logic(LogicVal::DontCare),
        },
        InjectedCheck {
            deposit_time_fs: 15_000_000,
            check_delay_fs: 1_000_000,
            signal_name: "v_x",
            deposit_value: string_to_logic_vec("-HUW"),
            expected_value: string_to_logic_vec("-HUW"),
        },
    ]
});

static INJECTED_DEPOSIT_MASK: AtomicUsize = AtomicUsize::new(0);
static INJECTED_CHECK_MASK: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static SIGNAL_HANDLES: RefCell<Option<SignalHandles>> = const { RefCell::new(None) };
}

fn bit(index: usize) -> usize {
    1usize << index
}

fn expected_injected_mask() -> usize {
    if INJECTED_CHECKS.is_empty() {
        0
    } else {
        (1usize << INJECTED_CHECKS.len()) - 1
    }
}

fn with_signal_handles<R>(f: impl FnOnce(&SignalHandles) -> R) -> R {
    SIGNAL_HANDLES.with(|cell| {
        if cell.borrow().is_none() {
            let root = vhpi::handle(OneToOne::RootInst);
            let handles = SignalHandles {
                s_a: root.handle_by_name("s_a").expect("signal s_a not found"),
                s_b: root.handle_by_name("s_b").expect("signal s_b not found"),
                v_x: root.handle_by_name("v_x").expect("signal v_x not found"),
                v_y: root.handle_by_name("v_y").expect("signal v_y not found"),
            };
            *cell.borrow_mut() = Some(handles);
        }

        let borrow = cell.borrow();
        f(borrow.as_ref().expect("signal cache was not initialized"))
    })
}

fn clear_signal_handles() {
    SIGNAL_HANDLES.with(|cell| {
        let _ = cell.borrow_mut().take();
    });
}

fn read_value(sig: &vhpi::Handle, name: &str) -> Value {
    match sig.get_value(Format::ObjType) {
        Ok(v) => v,
        Err(err) => panic!("failed to read {name}: {err}"),
    }
}

fn read_logic_vector(sig: &vhpi::Handle, name: &str) -> String {
    match sig.get_value(Format::ObjType) {
        Ok(Value::LogicVec(bits)) => bits
            .into_iter()
            .map(|bit| bit.to_string())
            .collect::<String>(),
        Ok(Value::BinStr(s)) => s,
        Ok(other) => panic!("{name} expected LogicVec/BinStr, got {other:?}"),
        Err(err) => panic!("failed to read {name}: {err}"),
    }
}

fn assert_checkpoint(checkpoint: &Checkpoint) {
    with_signal_handles(|handles| {
        assert_eq!(
            read_value(&handles.s_a, "s_a"),
            checkpoint.s_a,
            "s_a mismatch at {} fs",
            checkpoint.time_fs
        );
        assert_eq!(
            read_value(&handles.s_b, "s_b"),
            checkpoint.s_b,
            "s_b mismatch at {} fs",
            checkpoint.time_fs
        );
        assert_eq!(
            read_logic_vector(&handles.v_x, "v_x"),
            checkpoint.v_x,
            "v_x mismatch at {} fs",
            checkpoint.time_fs
        );
        assert_eq!(
            read_logic_vector(&handles.v_y, "v_y"),
            checkpoint.v_y,
            "v_y mismatch at {} fs",
            checkpoint.time_fs
        );
    });
}

fn end_of_time_step(_data: &CbData) {
    let now_fs = vhpi::get_time().to_i64();
    let last = LAST_TIME_FS.swap(now_fs, Ordering::SeqCst);
    if last == now_fs {
        return;
    }

    let idx = NEXT_CHECKPOINT.load(Ordering::SeqCst);
    if idx >= CHECKPOINTS.len() {
        return;
    }

    let checkpoint = &CHECKPOINTS[idx];
    if now_fs < checkpoint.time_fs {
        return;
    }

    assert_eq!(
        now_fs, checkpoint.time_fs,
        "expected checkpoint at {} fs, got {} fs",
        checkpoint.time_fs, now_fs
    );

    assert_checkpoint(checkpoint);
    vhpi::printf!(
        "test_simple: checkpoint {} at {} fs passed",
        idx + 1,
        now_fs
    );
    NEXT_CHECKPOINT.fetch_add(1, Ordering::SeqCst);
}

fn run_injected_check(index: usize) {
    let check = &INJECTED_CHECKS[index];
    let expected_time_fs = check.deposit_time_fs + check.check_delay_fs;
    let now_fs = vhpi::get_time().to_i64();
    assert_eq!(
        now_fs, expected_time_fs,
        "check callback expected at {expected_time_fs} fs, got {now_fs} fs"
    );

    assert!(
        (INJECTED_DEPOSIT_MASK.load(Ordering::SeqCst) & bit(index)) != 0,
        "deposit callback did not run before delayed check for index {index}"
    );

    with_signal_handles(|handles| {
        let signal = match check.signal_name {
            "s_a" => &handles.s_a,
            "s_b" => &handles.s_b,
            "v_x" => &handles.v_x,
            "v_y" => &handles.v_y,
            _ => panic!("unsupported cached signal: {}", check.signal_name),
        };
        assert_eq!(
            read_value(signal, check.signal_name),
            check.expected_value,
            "{} mismatch at {} fs after deposit",
            check.signal_name,
            expected_time_fs
        );
    });

    INJECTED_CHECK_MASK.fetch_or(bit(index), Ordering::SeqCst);
    vhpi::printf!(
        "test_simple: injected check {} passed at {} fs",
        index + 1,
        now_fs
    );
}

fn run_injected_deposit(index: usize) {
    let check = &INJECTED_CHECKS[index];
    let now_fs = vhpi::get_time().to_i64();
    assert_eq!(
        now_fs, check.deposit_time_fs,
        "deposit callback expected at {} fs, got {} fs",
        check.deposit_time_fs, now_fs
    );

    with_signal_handles(|handles| {
        let signal = match check.signal_name {
            "s_a" => &handles.s_a,
            "s_b" => &handles.s_b,
            "v_x" => &handles.v_x,
            "v_y" => &handles.v_y,
            _ => panic!("unsupported cached signal: {}", check.signal_name),
        };
        signal
            .put_value(check.deposit_value.clone(), PutValueMode::DepositPropagate)
            .expect("failed to deposit injected value");
    });

    INJECTED_DEPOSIT_MASK.fetch_or(bit(index), Ordering::SeqCst);
    vhpi::printf!(
        "test_simple: deposited {} <= '{}' at {} fs",
        check.signal_name,
        check.deposit_value,
        check.deposit_time_fs
    );

    let reg = vhpi::register_cb_after_delay(vhpi::Time::from(check.check_delay_fs), move |_| {
        run_injected_check(index);
    });
    assert!(
        reg.is_ok(),
        "failed to register delayed injected check callback: {:?}",
        reg.err()
    );
}

fn start_of_sim(_data: &CbData) {
    with_signal_handles(|_| {});

    for index in 0..INJECTED_CHECKS.len() {
        let reg = vhpi::register_cb_after_delay(
            vhpi::Time::from(INJECTED_CHECKS[index].deposit_time_fs),
            move |_| {
                run_injected_deposit(index);
            },
        );
        assert!(
            reg.is_ok(),
            "failed to register injected deposit callback: {:?}",
            reg.err()
        );
    }
}

fn end_of_sim(_data: &CbData) {
    let expected_mask = expected_injected_mask();
    assert!(
        INJECTED_DEPOSIT_MASK.load(Ordering::SeqCst) == expected_mask,
        "test_simple: not all injected deposit callbacks were executed"
    );
    assert!(
        INJECTED_CHECK_MASK.load(Ordering::SeqCst) == expected_mask,
        "test_simple: not all delayed injected checks were executed"
    );

    let idx = NEXT_CHECKPOINT.load(Ordering::SeqCst);
    assert_eq!(
        idx,
        CHECKPOINTS.len(),
        "test_simple: simulation ended after {idx} checkpoints; expected {}",
        CHECKPOINTS.len()
    );

    // Clear signal handle cache
    clear_signal_handles();
    vhpi::printf!("test_simple: all {} checkpoints passed", CHECKPOINTS.len());
}

#[no_mangle]
pub extern "C" fn test_simple_startup() {
    vhpi::printf("test_simple plugin loaded");

    let _ = vhpi::register_cb(CbReason::StartOfSimulation, start_of_sim);
    let _ = vhpi::register_cb(CbReason::RepEndOfTimeStep, end_of_time_step);
    let _ = vhpi::register_cb(CbReason::EndOfSimulation, end_of_sim);
}
