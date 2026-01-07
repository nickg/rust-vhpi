# vhpi

This crate provides a high-level wrapper around VHPI (VHDL Procedural
Interface) allowing the user to inspect and control a VHDL simulation.
See [nvc](https://github.com/nickg/nvc) for a supported simulator.

A full description of VHPI capabilities can be found in the [VHDL
LRM](https://standards.ieee.org/standard/1076-2019.html).

## Usage

Add `vhpi` to your `Cargo.toml`:

```toml
[dependencies]
vhpi = "0.1.0"
```

VHPI programs are usually compiled as plugins and loaded into the
simulator which requires:

```toml
[lib]
crate-type = ["cdylib"]
```
