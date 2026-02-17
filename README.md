# ch57x-hal

[![Github Actions][github-workflow]][homepage]
[![Crates.io][badge-license]][crates]
[![Crates.io][badge-version]][crates]
[![docs.rs][badge-docsrs]][docsrs]

[github-workflow]: https://img.shields.io/github/actions/workflow/status/Raynhardt-Van-Zyl/ch57x-hal/ci.yml?style=for-the-badge
[badge-license]: https://img.shields.io/crates/l/ch57x-hal-raynhardt?style=for-the-badge
[badge-version]: https://img.shields.io/crates/v/ch57x-hal-raynhardt?style=for-the-badge
[badge-docsrs]: https://img.shields.io/docsrs/ch57x-hal-raynhardt?style=for-the-badge
[crates]: https://crates.io/crates/ch57x-hal-raynhardt
[docsrs]: https://docs.rs/ch57x-hal-raynhardt
[homepage]: https://github.com/Raynhardt-Van-Zyl/ch57x-hal

HAL for CH57x RISC-V BLE microcontrollers from WCH.

This crate is under active development and should currently be treated as experimental.

## What This Crate Provides

- CH57x peripheral abstractions built on top of `ch57x-pac`
- Runtime integration via `qingke-rt`
- Optional Embassy support (`embassy` feature)
- Optional BLE bindings (`ble` feature, backed by vendor libraries in `vendor/`)

## Development Setup

1. Install Rust target:
`rustup target add riscv32imac-unknown-none-elf`

2. Optional nightly toolchain (needed by several examples using `#![feature(type_alias_impl_trait)]`):
`rustup toolchain install nightly`

3. Install a flasher tool used by `.cargo/config.toml`:
- default runner: `wlink -v flash`
- alternatives are commented in `.cargo/config.toml`

4. Validate baseline:
- library: `cargo check`
- one example at a time: `cargo check --example blinky`
- flash one example: `cargo run --release --example blinky`

## Current Status

- Library crate compiles on stable with warnings.
- Many examples are still mid-migration from CH58x style APIs to CH57x and do not all compile yet.
- Migration tracking docs:
  - `progress.md`
  - `plan.md`
  - `learnings.md`
  - `docs/ch572ds1-mapping.md`

## Features

- Basic: clock init, delay, interrupt, etc.
- Dedicated runtime: interrupt table, hardware stack push, highcode support, critical section implementation
- embassy support
  - time driver with SysTick, defaults to 32KHz tick
  - about 7k flash rom overhead
- GPIO, with async support
- UART, basic blocking tx, rx
- RTC, with datetime support
- SysTick delay (conflicts with embassy time driver)

- ADC, with Temperature sensor, VBAT sensor
- SPI
- libISP ROM functions
- Basic BLE library support

## Usage

```toml
[dependencies]
ch57x-hal = { package = "ch57x-hal-raynhardt", version = "0.0.5" }
```

Feature examples:

```toml
[dependencies]
ch57x-hal = { package = "ch57x-hal-raynhardt", version = "0.0.5", default-features = false, features = ["embassy", "ble"] }
```

See the `examples/` directory for end-to-end usage.

## Crates.io

Add dependency:

```toml
[dependencies]
ch57x-hal = { package = "ch57x-hal-raynhardt", version = "0.0.5" }
```

Release process is tracked in `RELEASING.md`.

## Notes

- `UNDOCUMENTED:` tags in code comments means the information is not from official documents.

## References

- [ch32-rs/ch58x-hal](https://github.com/ch32-rs/ch58x-hal) (original)
- [Slappy2022/ch58x-ble-rt](https://github.com/Slappy2022/ch58x-ble-rt)
