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
- [x] Boot / early paging (2026-06-14)
- [x] memblock (2026-06-14)
- [x] memmap_init (2026-07-19)
- [x] Direct map (2026-07-19)
- [x] Buddy allocator (2026-07-06)
- [x] Zone allocator (2026-07-19)
- [x] Slab allocator (2026-07-06)
- [x] NUMA (2026-07-19)
- [x] free_all_to_buddy (2026-07-19)
- [ ] Allocation flags (GFP_KERNEL, GFP_ATOMIC, GFP_THISNODE, ...)
- [ ] kmalloc
- [ ] Memory shrinker
- [ ] Watermarks (WMARK_MIN/LOW/HIGH + kswapd)
- [ ] Memory compaction
- [ ] ZONE_MOVABLE
- [ ] Memory policy
- [ ] Virtual memory areas (mm_struct + VMA)
- [ ] Red-Black tree for VMA lookup
- [ ] vmalloc
- [ ] Page fault handler
- [ ] Guard pages
- [ ] KASAN

### Kernel infrastructure
- [x] Intrusive doubly-linked list with iterator traits (2026-07-25)
- [x] Atomic notifier chain (2026-07-22)
- [x] Panic handler (notifier chain, panic_timeout, PanicInfo) (2026-07-22)
- [x] Die notifier chain (2026-07-22)
- [x] Reboot (kernel_restart, kernel_halt, kernel_power_off) (2026-07-22)
- [ ] Blocking notifier chain (needs rwsem + scheduler)
- [ ] Reboot notifier chain (needs blocking notifier chain)
- [ ] WARN / WARN_ON
- [ ] Tainted mask
- [ ] Syslog (ring buffer + log levels)
- [ ] Kernel loadable modules (KLM)

### Security
- [x] PIE (2026-07-16)
- [ ] KASLR

### Benchmarking
- [ ] Pluggable allocator selection (physical allocator, heap allocator, scheduler)

### Processes
- [ ] Per-CPU data
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
cargo xtask build [--debug]
```

- `--debug` - build with debug info

## Run

```sh
cargo xtask run [--memory <size>] [--window] [--gdb] [--debug] [--numa <n>] [--smp <n>]
```

- `--memory <size>` - QEMU RAM size per NUMA node (default: `128M`)
- `--window` - display output in a window instead of serial console
- `--gdb` - start GDB stub on port 1234
- `--debug` - build and run with debug info
- `--numa <n>` - emulate n NUMA nodes
- `--smp <n>` - number of CPUs (default: 1, or equal to `--numa` count when NUMA is enabled)
