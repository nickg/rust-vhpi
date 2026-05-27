# vhpi-shim

This crate provides a cross-platform shim for `vhpi-sys` (and indirectly `vhpi`) to enable dynamic symbol lookup for VHPI plugins on supported platforms.

Supported platforms:

- Windows (via `GetProcAddress`)
- macOS (via `dlsym`)

**Note:** You probably do not want to use this crate directly.

Please see the [`vhpi` crate documentation](https://crates.io/crates/vhpi) for a safe and ergonomic API to use VHPI.
