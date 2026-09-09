SECTIONS {
    .text : { *(.text .text.*) }
    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    .bss : { *(.bss .bss.*) *(COMMON) }

    _kernel_start = ADDR(.text);
    _kernel_end = ADDR(.bss) + SIZEOF(.bss);
}
