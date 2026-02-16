# CH57x-HAL Plan (CH572 + Embassy)

This plan is updated to match current repository state.

## Phase 1: Stabilize Baseline

1. Keep `cargo check` green for the library on stable.
2. Resolve or explicitly document current warnings in `src/`.
3. Keep README setup instructions aligned with actual toolchain and runner behavior.

## Phase 2: Datasheet-Driven Mapping

1. Use `docs/ch572ds1-mapping.md` as the source of truth for mapping progress.
2. For each CH572 peripheral:
   - verify register/bit semantics against `CH572DS1.PDF`
   - verify `ch57x-pac` naming and behavior
   - verify HAL behavior in `src/`
3. Track unresolved ambiguities as explicit TODOs with chapter/page references.

## Phase 3: Example Triage and Recovery

1. Classify examples into `green`/`yellow`/`red`.
2. Recover a minimum green set first:
   - `blinky`
   - `uart-echo`
   - `adc_temp`
3. For Embassy:
   - migrate one async GPIO/timer example end-to-end
   - confirm nightly requirements are clearly documented
4. For BLE:
   - migrate one peripheral role example end-to-end before broad BLE cleanup

## Phase 4: Validation and CI

1. Define check matrix:
   - `cargo check` (required)
   - selected `cargo check --example ...` for green examples (required)
2. Add a simple script or CI workflow once green examples are identified.
3. Hardware smoke test each green example on CH572 and capture outcomes in `progress.md`.

## Exit Criteria

- Library builds cleanly (or with documented intentional warnings).
- At least one green example each for:
  - basic GPIO/UART
  - Embassy async
  - BLE
- Mapping table has concrete entries for all targeted peripherals.
