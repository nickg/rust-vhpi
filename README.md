# rust-vhpi

Project structure

* `vhpi-sys/` - raw low-level generated bindings to C API.
* `vhpi/` - higher level Rust bindings.
* `vhpi-shim/` - platform shim library used by the `dynamic` feature.
* `dumper/` - example plugin dumping much information about the design.
* `tests/test_simple/` - assertion-based plugin for `tb_simple` checkpoints.
* `tests/stringindexing/` - plugin that checks string indexing.
* `tests/cb_toggle/` - plugin that disables/enables callbacks.
* `tests/foreignf` - plugin that implements foreign functions through VHPI.

Test with the example plugin

```bash
cargo build && nvc --vhpi-trace --load ./target/debug/libdumper.so -r toplevel
```

On Windows, load `./target/debug/dumper.dll` instead of `libdumper.so`.
On macOS, load `./target/debug/libdumper.dylib`.

## Building plugin without linking to simulator

For linux there is usually no need to do anything special, but the plugin will be able to find the VHPI symbols at run time.

For Windows and macOS, one can add the `dynamic` feature which will add a shim for dynamically resolving the VHPI symbols at run time. If using nvc, there is an import library at `$PREFIX/lib/nvc/libnvcimp.a` that can linked to. This may also work for other simulators.
