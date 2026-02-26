# own_os

A bare-metal x86_64 kernel written in Rust, built from scratch without any OS abstractions.

Developed following [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann.

## Requirements

- [Rust nightly](https://rustup.rs/) (automatically selected via `rust-toolchain.toml`)
- [`bootimage`](https://github.com/rust-osdev/bootimage) — creates a bootable disk image

```sh
cargo install bootimage
```

- QEMU (to run the kernel)

```sh
brew install qemu  # macOS
```

## Build

```sh
cargo build
```

The default target is `x86_64-own_os.json` (bare-metal, no OS, no SSE, redzone disabled).
`core` and `compiler_builtins` are compiled from source via `build-std`.

## Run

```sh
cargo run
```

This uses `bootimage runner` to build a bootable image and launch it in QEMU.
