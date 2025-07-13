Project structure

* `bindings/` - raw low-level generated bindings to C API.
* `vhpi/` - higher level Rust bindings.
* `dumper/` - example plugin.

Test with

```
cargo build && nvc --vhpi-trace --load ./target/debug/libdumper.so -r toplevel
```
