ENTRY(_start)

KERNEL_PHYS = 0x80200000;
KERNEL_VIRT = 0xC0200000;

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

    .rela.dyn : AT(ADDR(.rela.dyn) - KERNEL_VIRT + KERNEL_PHYS) {
        __rela_dyn_start = .;
        *(.rela.dyn)
        __rela_dyn_end = .;
    }

    .got : AT(ADDR(.got) - KERNEL_VIRT + KERNEL_PHYS) {
        *(.got .got.*)
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
