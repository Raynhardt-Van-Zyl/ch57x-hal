# Migration Progress

This document tracks the progress of the `ch58x-hal` to `ch57x-hal` migration.

## Completed Tasks

*   **Initial Setup:**
    *   [x] Create `plan.md`, `progress.md`, and `learnings.md`.
    *   [x] Update `Cargo.toml` to use the new `ch57x-pac`.
    *   [x] Rename the package to `ch57x-hal`.
    * [x] Update crate-level documentation and attributes.
*   **PAC Integration:**
    *   [x] Replace `ch58x` with `ch57x` throughout the codebase.
    *   [x] Update peripheral names and API to match `ch57x-pac` (e.g., SYS -> Sys, GPIO integrated into Sys).
    *   [x] Update GPIO code to use Sys register block for PA/PB registers.
    *   [x] Update ADC code to use Sys register block.
    *   [x] Update interrupt definitions to match CH57x interrupts.
    *   [x] Update peripherals definitions.
*   **Peripheral Driver Refactoring:**
    *   [x] Update GPIO, ADC, UART, SPI, BLE drivers to compile with new PAC.
    *   [x] Remove I2C support (not available in CH57x).
    *   [x] Run cargo fmt and cargo clippy for code quality.
*   **Examples Update:**
    *   [x] Remove I2C example (i2c-mpu6050.rs) since I2C not available.
*   **Documentation Update:**
    *   [x] Update README.md to reflect changes.

## To Do

*   **Fix Remaining Compilation Errors:**
    *   SPI register method names (fdiv, fifo, fifo_dir, total_cnt, if_cnt_end)
    *   SysCtl register method names (ck32k_config, pll_config, hfck_pwr_ctrl, clk_sys_cfg, flash_cfg)
    *   UART register method names (tfc, thr, lsr, rfc, rbr, fcr, lcr, div, dl, ier)
    *   Type annotation issues in closures
*   **Final Review and Cleanup**
