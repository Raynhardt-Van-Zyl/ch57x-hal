# Learnings

This document records things that were discovered, were incorrect, or required extra effort during the migration.

*   CH57x does not have a separate I2C peripheral; I2C functionality may not be available or implemented differently.
*   GPIO peripherals are integrated into the SYS register block, requiring significant refactoring of GPIO code.
*   ADC is integrated into the SYS register block.
*   RTC is integrated into the SYS register block.
*   The ch57x-pac uses specific naming conventions for register access methods, often prefixed with the register size and peripheral name (e.g., r8_uart0_fcr instead of r8_fcr).
*   Some bit field accessors are prefixed with rb_ for register bits.
*   Many register write operations require .bits() instead of .variant() for enum-like fields.
*   Type annotations are often needed in closure parameters for PAC register operations.
