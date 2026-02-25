# test_simple rust-vhpi example project

This plugin checks expected `tb_simple` signal values at fixed simulation
checkpoints using Rust assertions in VHPI callbacks.

## Run against tb_simple

```bash
cargo build -p test_simple
nvc -a test_examples/tb_simple.vhdl
nvc -e tb_simple
nvc -r tb_simple --load=target/debug/libtest_simple.so
```

or

```bash
./source/run_test_simple.sh
```

A passing run prints checkpoint messages ending with:

```text
test_simple: all 6 checkpoints passed
```
