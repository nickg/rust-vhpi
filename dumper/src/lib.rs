use vhpi::startup_routines;

mod dumper;

startup_routines!{
    dumper::dumper_startup,
}
