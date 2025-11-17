# CH58x-HAL to CH57x-HAL Migration Plan

This document outlines the plan for migrating the `ch58x-hal` to the new `ch57x-hal`.

## Phases

1.  **Initial Setup:**
    *   Create `plan.md`, `progress.md`, and `learnings.md`.
    *   Update `Cargo.toml` to use the new `ch57x-pac`.
    *   Rename the package to `ch57x-hal`.
    *   Update crate-level documentation and attributes.

2.  **Peripheral Access Crate (PAC) Integration:**
    *   Replace `ch58x` with `ch57x` throughout the codebase.
    *   Address breaking changes from the new PAC: GPIO, ADC, RTC are integrated into SYS register block; I2C not available; interrupt names updated (GPIOA->GPIO_A, etc.); peripheral names changed (SYS->Sys, etc.).

3.  **Peripheral Driver Refactoring:**
    *   Review and update each peripheral driver in `src/` to align with the `ch57x-pac`.
    *   **ADC (`adc.rs`):** Updated to use Sys register block.
    *   **Delay (`delay.rs`):**
    *   **DMA (`dma.rs`):** No DMA in CH57x, keep as NoDma.
    *   **GPIO (`gpio.rs`):** Updated to use Sys register block for PA/PB.
    *   **I2C (`i2c.rs`):** I2C not available in CH57x, commented out.
    *   **Interrupt (`interrupt.rs`):** Updated interrupt names.
    *   **ISP (`isp.rs`):**
    *   **LCD (`lcd.rs`):**
    *   **RTC (`rtc.rs`):** Already uses Sys.
    *   **SPI (`spi.rs`):**
    *   **SysCtl (`sysctl.rs`):**
    *   **Systick (`systick.rs`):**
    *   **Timer (`timer.rs`):**
    *   **UART (`uart.rs`):**
    *   **BLE (`ble/`):**

4.  **Examples Update:**
    *   Update all examples in the `examples/` directory to work with the new HAL.
    *   Ensure all examples compile and run correctly on `ch57x` hardware (if possible).

5.  **Documentation Update:**
    *   Update `README.md` to reflect the changes.
    *   Update all documentation comments in the code.

6.  **Final Review and Cleanup:**
    *   Remove any unused code or files.
    *   Run `cargo fmt` and `cargo clippy` to ensure code quality.
    *   Build the project to ensure everything compiles.
