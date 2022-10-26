# goose

A micro Kernel written in Rust. The goal is to have a small number of syscall 
with a strong emphasis on IPC

## Roadmap

- [ ] Virtual Memory Manager (in progress)
- [ ] Basic in-kernel filesystem
- [ ] Device-tree handling
- [x] In-kernel ELF loader
- [ ] Userland process (in progress)
- [ ] IPC implementation
- [ ] Drivers
    - [ ] Driver API
    - [ ] Kernel API

## Project structure

The project is divided in 2 components, the kernel and the drivers. Most of the 
drivers runs in userland, but some are required to run in kernel just to 
provide the basic kernel functionalities (UART, interrupt hardware, ...)
:warning: **At the moment userland drivers are not implemented**

## Try it out
When using cargo, you need to specify which target triplet to use with 
`--target <triplet_here>`. Here is the list of triplet to use depending on the 
targeted architecture:
- RISC-V --> `riscv64gc-unknown-none-elf`
- AArch64 --> `aarch64-unknown-none`

If not installed yet, use `rustup target add <triple_here>`

You also need to select which board to use with `--bin`:
- Qemu RISC-V (virt) --> `riscv_qemuvirt`
- Qemu AArch64 (virt) --> `aarch64_qemuvirt`

### Requirement
- A rust nightly toolchain
- Clang compiler (for tests)

### Build
```console
$ cargo build --bin <bin_here> --target <triplet_here>
```

### Run
Each platform configuration might contains a custom runner setup, meaning that
depending on your use-case `cargo run` will be able to launch an emulator or
flash a connected microcontroller.

```console
$ cargo run --bin <bin_here> --target <triplet_here>
```

### Tests
GoOSe also comes with unit tests that run directly on hardware and output the 
result over serial. When using Qemu, you can also have an exit code != 0 on 
failure.

```console
$ make -C kernel/fixtures <triplet_here>
...
$ cargo tests --bin <bin_here> --target <triplet_here>
...
```
:warning: **Tests might be slow to run as GoOSe is not really optimized. You can
append `--release` to the previous cargo command line to boost performance but 
please be aware that some test might pass in debug and not in release. Feel 
free to open an issue if you encounter such a case**
