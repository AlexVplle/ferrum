ENTRY(_start)

KERNEL_PHYS = 0x80200000;
KERNEL_VIRT = 0xffffffff80200000;

SECTIONS {
    . = KERNEL_VIRT;

    _kernel_start = .;

    .limine_requests : AT(ADDR(.limine_requests) - KERNEL_VIRT + KERNEL_PHYS) {
        KEEP(*(.limine_requests_start))
        KEEP(*(.limine_requests))
        KEEP(*(.limine_requests_end))
    }

    .text : AT(ADDR(.text) - KERNEL_VIRT + KERNEL_PHYS) {
        *(.text.boot)
        *(.text .text.*)
    }

    .rodata : AT(ADDR(.rodata) - KERNEL_VIRT + KERNEL_PHYS) {
        *(.rodata .rodata.*)
    }

    .data : AT(ADDR(.data) - KERNEL_VIRT + KERNEL_PHYS) {
        *(.data .data.*)
    }

    .bss : AT(ADDR(.bss) - KERNEL_VIRT + KERNEL_PHYS) {
        _bss_start = .;
        *(.bss .bss.*)
        _bss_end = .;
    }

    _kernel_end = .;
}
