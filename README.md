# Ferrum

A bare-metal kernel written in Rust, built from scratch, targeting RISC-V.

Inspired by:
- [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann
- [Operating Systems: Three Easy Pieces](https://ostep.org/) by Arpaci-Dusseau
- [xv6](https://github.com/mit-pdos/xv6-riscv) by MIT
- [OSDev Wiki](https://wiki.osdev.org/)
- [Understanding the Linux Kernel](https://www.oreilly.com/library/view/understanding-the-linux/0596005652/) by Bovet & Cesati
- [The Linux Memory Manager](https://nostarch.com/linux-memory-manager) by Lorenzo Stoakes

## Goal

Like every good thing that exists, it must run Doom.

## Roadmap

### Memory management
- [x] Boot / early paging
- [x] memblock
- [x] memmap_init
- [x] Direct map
- [x] Buddy allocator
- [x] Zone allocator
- [x] Slab allocator
- [x] NUMA
- [ ] kmalloc
- [ ] free_all_to_buddy
- [ ] Virtual memory areas (mm_struct + VMA)
- [ ] Page fault handler

### Kernel infrastructure
- [ ] Syslog (ring buffer + log levels)

### Security
- [x] PIE
- [ ] KASLR

### Processes
- [ ] Scheduler
- [ ] Threads
- [ ] SMP
- [ ] Syscalls
- [ ] Process isolation
- [ ] IPC

### Userspace
- [ ] Userspace
- [ ] VFS server
- [ ] Driver model
- [ ] Doom

## Maybe

- [ ] Memory server
- [ ] Network server
- [ ] Sockets

## Requirements

- [Rust nightly](https://rustup.rs/)

## Build

```sh
cargo xtask build
```

## Run

```sh
cargo xtask run [--memory <size>] [--window] [--gdb] [--numa]
```

- `--memory <size>` - QEMU RAM size (default: `128M`)
- `--window` - display output in a window instead of serial console
- `--gdb` - start GDB stub on port 1234
- `--numa` - emulate two NUMA nodes
