# rust-vhpi

Project structure

* `vhpi-sys/` - raw low-level generated bindings to C API.
* `vhpi/` - higher level Rust bindings.
* `vhpi-winlib/` - Windows shim library used by the `dynamic` feature.
* `dumper/` - example plugin.
* `tests/test_simple/` - assertion-based plugin for `tb_simple` checkpoints.
* `tests/stringindexing/` - plugin that checks string indexing.

Test with the example plugin

```bash
cargo build && nvc --vhpi-trace --load ./target/debug/libdumper.so -r toplevel
```

On Windows, load `./target/debug/dumper.dll` instead of `libdumper.so`.
On macOS, load `./target/debug/libdumper.dylib`.

## Platform-specific linker flags for plugins

The plugin examples are platform-agnostic except for macOS, where the following linker flag is set in each plugin's build.rs:  `-Wl,-undefined,dynamic_lookup`

Linux and Windows do not require any additional linker flags in the plugin examples.

### Windows note (`dynamic` feature vs import library)

On Windows, this repository enables the `vhpi` crate `dynamic` feature in example
plugins. That feature routes VHPI calls through the `vhpi-winlib` shim, which
resolves symbols at runtime (for example via `GetProcAddress`) instead of relying
on unresolved linker symbols in Rust code.

An alternative approach (for nvc at least) is to link against the nvc import
library at `$PREFIX/lib/nvc/libnvcimp.a`.

This works well for nvc-specific builds, but it may reduce portability if
you want the same plugin binary to load in other simulators.
