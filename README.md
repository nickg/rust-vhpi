# rust-vhpi

Project structure

* `vhpi-sys/` - raw low-level generated bindings to C API.
* `vhpi/` - higher level Rust bindings.
* `dumper/` - example plugin.
* `tests/test_simple/` - assertion-based plugin for `tb_simple` checkpoints.

Test with

```bash
cargo build && nvc --vhpi-trace --load ./target/debug/libdumper.so -r toplevel
```
