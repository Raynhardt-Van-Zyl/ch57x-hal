# Migration Progress

This document tracks migration status for `ch57x-hal` focused on CH572 + Embassy.

## Completed

- Library crate builds successfully: `cargo check`.
- Core migration docs exist (`plan.md`, `progress.md`, `learnings.md`).
- README now includes explicit setup and validation commands.
- Added CH572 datasheet mapping tracker: `docs/ch572ds1-mapping.md`.
- Examples updated to reference `ch57x_hal` (not `ch58x_hal`).

## Current Reality

- The crate-level migration is farther along than this file previously indicated.
- Main blockers are now in examples and validation coverage, not base library compile.

## Open Blockers

- `cargo check --examples` still fails on multiple examples due to:
  - old peripheral naming/usage patterns (for example `UART1` field/type mismatches)
  - legacy assumptions from CH58 examples
  - nightly-only example attributes (`#![feature(type_alias_impl_trait)]`)
  - type inference issues in example-local logging macros
  - some BLE example API/import mismatches

## Next Focus

- Split examples into support tiers:
  - `green`: compile + tested on CH572
  - `yellow`: compile only
  - `red`: migration in progress
- Bring one non-BLE example set to `green` first (`blinky`, `uart-echo`, `adc_temp`).
- Then migrate one Embassy example and one BLE example end-to-end.
