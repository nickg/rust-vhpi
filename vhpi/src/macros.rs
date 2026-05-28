#[macro_export]
/// Define the `vhpi_startup_routines` table required by VHPI simulators.
///
/// Pass one or more startup function expressions. The macro emits a
/// null-terminated static table where each function is wrapped in `Some(...)`
/// and the final entry is `None`.
macro_rules! startup_routines {
    ($($func:expr),* $(,)?) => {
        #[no_mangle]
        pub static vhpi_startup_routines: [Option<extern "C" fn()>; $crate::count_idents!($($func),*) + 1] = [
            $(Some($func),)*
            None,
        ];
    };
}

#[macro_export]
/// Count the number of comma-separated expressions.
///
/// This helper macro is primarily intended for internal use by
/// [`startup_routines!`].
macro_rules! count_idents {
    () => {0};
    ($_head:expr $(, $tail:expr)*) => {1 + $crate::count_idents!($($tail),*)};
}
