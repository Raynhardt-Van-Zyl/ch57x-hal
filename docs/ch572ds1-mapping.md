# CH572DS1 Mapping Checklist

This file tracks mapping from `CH572DS1.PDF` into:
- `ch57x-pac` register and field definitions
- `ch57x-hal` driver behavior and APIs

Use it to keep datasheet work explicit and reviewable.

## Workflow

1. Pick one peripheral chapter from `CH572DS1.PDF`.
2. Record register names, offsets, reset values, and bit definitions.
3. Confirm each item exists in `ch57x-pac` with matching meaning.
4. Confirm HAL driver code uses the same semantic behavior.
5. Add a minimal compile-time or runtime example that exercises the path.
6. Mark status and open questions in the table below.

## Mapping Table

| Peripheral | Datasheet chapter/page | PAC path | HAL path | Status | Notes |
|---|---|---|---|---|---|
| SYSCTL/Clock | TODO | TODO | `src/sysctl.rs` | TODO | |
| GPIO | TODO | TODO | `src/gpio.rs` | TODO | |
| UART | TODO | TODO | `src/uart.rs` | TODO | |
| SPI | TODO | TODO | `src/spi.rs` | TODO | |
| RTC | TODO | TODO | `src/rtc.rs` | TODO | |
| ADC | TODO | TODO | `src/adc.rs` | TODO | |
| TIMER | TODO | TODO | `src/timer.rs` | TODO | |
| BLE | TODO | TODO | `src/ble/` | TODO | |

## Example Status Buckets

- `green`: compiles and flash-tested on CH572 hardware
- `yellow`: compiles, not yet hardware-validated
- `red`: does not compile or known incorrect behavior

Keep a per-example status in `progress.md`.
