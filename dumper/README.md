# Dumper rust-vhpi example project

This is an example project demonstrating use of the library to walk the
design and dump signal changes.

## Run checks

Run integration checks for the dumper plugin with:

``` bash
../scripts/run_dumper_checks.sh
```

This builds `libdumper.so`, runs all VHDL testbenches in `../test_examples/`
with `nvc`, and verifies core VHPI lifecycle markers in the simulation logs.
