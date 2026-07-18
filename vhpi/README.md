# vhpi

This crate provides a high-level wrapper around VHPI (VHDL Procedural
Interface) allowing the user to inspect and control a VHDL simulation.
See [nvc](https://github.com/nickg/nvc) for a supported simulator.

A full description of VHPI capabilities can be found in the [VHDL
LRM](https://standards.ieee.org/standard/1076-2019.html).

## Coverage Note

The `vhpi` crate covers the most common workflows (handles/iteration,
properties, callbacks, values, time, simulation control, errors, and
foreign model registration), but it does not yet wrap every API entry
in `vhpi_user.h`.

At the time of writing, the following `vhpi_user.h` APIs are not exposed
through a high-level wrapper in this crate:

- `vhpi_protected_call`
- `vhpi_schedule_transaction`
- `vhpi_format_value`
- `vhpi_create`
- `vhpi_get_data`
- `vhpi_put_data`

In addition, `vhpi_vprintf` is not and will not be supported as it should be preferred to use Rust formatting. If you have a use case, open an issue.

If you need one of these APIs today, call it through `vhpi-sys` directly
from your plugin code.

## Usage

Add `vhpi` to your `Cargo.toml`:

```toml
[dependencies]
vhpi = "0.5.0"
```

VHPI programs are usually compiled as plugins and loaded into the
simulator which requires:

```toml
[lib]
crate-type = ["cdylib"]
```

If you get linker errors on Windows and macOS because you are trying to build a stand-alone plugin, use the `dynamic` feature:

```toml
[dependencies]
vhpi = { version = "0.5.0", features = ["dynamic"] }
```

or, if you have nvc installed, link with `$PREFIX/lib/nvc/libnvcimp.a`.
