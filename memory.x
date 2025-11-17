MEMORY {
    FLASH : ORIGIN = 0x00000000, LENGTH = 448K
    RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

SECTIONS {
    .text : {
        *(.text .text.*);
    } > FLASH

    .rodata : {
        *(.rodata .rodata.*);
    } > FLASH

    .highcode : {
        _highcode_vma_start = .;
        *(.highcode .highcode.*);
        _highcode_vma_end = .;
    } > FLASH AT > FLASH

    .data : {
        _data_start = .;
        *(.data .data.*);
        _data_end = .;
    } > RAM AT > FLASH

    .bss : {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > RAM

    .stack : {
        _stack_bottom = .;
        . = . + 4K;
        _stack_top = .;
    } > RAM
}

_stack_top = ORIGIN(RAM) + LENGTH(RAM);

_highcode_lma = LOADADDR(.highcode);
_data_lma = LOADADDR(.data);
_data_vma = ADDR(.data);

ExceptionHandler = DefaultHandler;
__EXTERNAL_INTERRUPTS = 0;
DefaultHandler = 0;

TMR0 = 0;
GPIO_A = 0;
GPIO_B = 0;
SPI0 = 0;
BLEL = 0;
BLEB = 0;
USB = 0;
TMR2 = 0;
UART0 = 0;