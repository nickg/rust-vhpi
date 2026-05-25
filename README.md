# rust-vhpi

Project structure

* `vhpi-sys/` - raw low-level generated bindings to C API.
* `vhpi/` - higher level Rust bindings.
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

When writing VHPI plugin that are loaded by the simulator, which provides VHPI symbols at runtime. The
`build.rs` files therefore use platform-specific linker flags to allow unresolved VHPI symbols at link time:

* Windows MSVC: `/FORCE:UNRESOLVED`
* Windows GNU (MinGW): `-Wl,--allow-shlib-undefined`
* macOS: `-Wl,-undefined,dynamic_lookup`

Linux typically does not require an additional linker flag for this workflow.

See `build.rs` for the provided example plugins.

### Windows note (`dynamic` feature vs import library)

On Windows, this repository enables the `vhpi` crate `dynamic` feature in example
plugins. That feature uses runtime symbol resolution (for example via
`GetProcAddress`) instead of relying on unresolved linker symbols.

An alternative approach (for nvc at least) is to link against the nvc import
library at `$PREFIX/lib/nvc/libnvcimp.a`.

This can work well for nvc-specific builds, but it may reduce portability if
you want the same plugin binary to load in other simulators.
