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

- [x] Boot / early paging
- [x] memblock
- [x] memmap_init
- [x] Direct map
- [x] Buddy allocator
- [x] Zone allocator
- [x] Slab allocator
- [x] NUMA
- [ ] Scheduler
- [ ] Threads
- [ ] Syscalls
- [ ] Process isolation
- [ ] IPC
- [ ] Userspace
- [ ] Memory server
- [ ] VFS server
- [ ] Driver model
- [ ] SMP
- [ ] Doom

## Maybe

- [ ] Network server
- [ ] Sockets

## Requirements

- [Rust nightly](https://rustup.rs/)

## Build

```sh
cargo xtask build
```
