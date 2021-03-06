# nvml-wrapper

[![Docs.rs docs](https://docs.rs/nvml-wrapper/badge.svg)](https://docs.rs/nvml-wrapper)
[![Crates.io version](https://img.shields.io/crates/v/nvml-wrapper.svg?style=flat-square)](https://crates.io/crates/nvml-wrapper)
[![Crates.io downloads](https://img.shields.io/crates/d/nvml-wrapper.svg?style=flat-square)](https://crates.io/crates/nvml-wrapper)
![CI](https://github.com/Cldfire/nvml-wrapper/workflows/CI/badge.svg)

A safe and ergonomic Rust wrapper for the
[NVIDIA Management Library](https://developer.nvidia.com/nvidia-management-library-nvml)
(NVML), a C-based programmatic interface for monitoring and managing various states within
NVIDIA (primarily Tesla) GPUs.

```rust
extern crate nvml_wrapper as nvml;
use nvml::NVML;

let nvml = NVML::init()?;
// Get the first `Device` (GPU) in the system
let device = nvml.device_by_index(0)?;

let brand = device.brand()?; // GeForce on my system
let fan_speed = device.fan_speed(0)?; // Currently 17% on my system
let power_limit = device.enforced_power_limit()?; // 275k milliwatts on my system
let encoder_util = device.encoder_utilization()?; // Currently 0 on my system; Not encoding anything
let memory_info = device.memory_info()?; // Currently 1.63/6.37 GB used on my system

// ... and there's a whole lot more you can do. Most everything in NVML is wrapped and ready to go
```

NVML is intended to be a platform for building 3rd-party applications, and is
also the underlying library for NVIDIA's nvidia-smi tool.

## Compilation

The NVML library comes with the NVIDIA drivers and is essentially present on any
system with a functioning NVIDIA graphics card. The compilation steps vary
between Windows and Linux, however.

### Windows

I have been able to successfully compile and run this wrapper's tests using
both the GNU and MSVC toolchains. An import library (`nvml.lib`) is included for
compilation with the MSVC toolchain.

The NVML library dll can be found at `%ProgramW6432%\NVIDIA Corporation\NVSMI\`
(which is `C:\Program Files\NVIDIA Corporation\NVSMI\` on my machine). I had to add
this folder to my `PATH` or place a copy of the dll in the same folder as the executable
in order to have everything work properly at runtime with the GNU toolchain. You may
need to do the same; I'm not sure if the MSVC toolchain needs this step or not.

### Linux

The NVML library can be found at `/usr/lib/nvidia-<driver-version>/libnvidia-ml.so`;
on my system with driver version 375.51 installed, this puts the library at
`/usr/lib/nvidia-375/libnvidia-ml.so`.

The `sys` crates' build script will automatically add the appropriate directory to
the paths searched for the library, so you shouldn't have to do anything manually
in theory.

## NVML Support

This wrapper is being developed against and currently supports NVML version
10.1. Each new version of NVML is guaranteed to be backwards-compatible according
to NVIDIA, so this wrapper should continue to work without issue regardless of
NVML version bumps.

## MSRV

The Minimum Supported Rust Version is currently 1.42.0. I will not go out of my
way to avoid bumping this.

## Cargo Features

The `serde` feature can be toggled on in order to `#[derive(Serialize, Deserialize)]`
for every NVML data structure.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
