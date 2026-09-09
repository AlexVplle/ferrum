# Ferrum

A bare-metal kernel written in Rust, built from scratch, targeting RISC-V.

## References

### Operating Systems
- [Operating Systems: Three Easy Pieces](https://ostep.org/) by Arpaci-Dusseau
- [xv6](https://github.com/mit-pdos/xv6-riscv) by MIT
- [OSDev Wiki](https://wiki.osdev.org/)

### Linux Kernel
- [Understanding the Linux Kernel](https://www.oreilly.com/library/view/understanding-the-linux/0596005652/) by Bovet & Cesati
- [Linux Kernel Internals](https://kernel-internals.org)

### Memory Management
- [The Linux Memory Manager](https://nostarch.com/linux-memory-manager) by Lorenzo Stoakes
- [The Slab Allocator: An Object-Caching Kernel Memory Allocator](https://people.eecs.berkeley.edu/~kubitron/courses/cs194-24-S14/hand-outs/bonwick_slab.pdf) by Bonwick

### Rust
- [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)

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
- [ ] Memory shrinker
- [x] free_all_to_buddy
- [ ] Virtual memory areas (mm_struct + VMA)
- [ ] Red-Black tree for VMA lookup
- [ ] vmalloc
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
