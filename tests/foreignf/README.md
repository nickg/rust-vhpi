# foreignf rust-vhpi example project

This plugin registers a VHPI foreign procedure and checks that VHDL calls it
at the expected simulation times.

## Run against tb_foreignf

```bash
cargo build -p foreignf
nvc -a test_examples/tb_foreignf.vhdl
nvc -e tb_foreignf
nvc -r tb_foreignf --load=target/debug/libforeignf.so
```

or

```bash
./scripts/run_foreignf_checks.sh
```

A passing run prints:

```text
foreignf: all 4 foreign procedure calls passed
```
