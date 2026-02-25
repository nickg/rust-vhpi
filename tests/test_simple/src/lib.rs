use vhpi::startup_routines;

mod test_simple;

startup_routines! {
    test_simple::test_simple_startup,
}
