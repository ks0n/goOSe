# goose

A minimal Kernel written in Rust

# Building

- Generate the compilation configuration for your desired platform using
  [`gen_cargo.sh`](gen_cargo.sh)
- Build the project using `cargo`. You will need to install the compilation
  targets using `rustup target add`
- Each platform configuration might contains a custom runner setup, meaning that
  depending on your use-case `cargo run` will be able to launch an emulator or
  flash a connected microcontroller.

## Roadmap

- [ ] Virtual Memory Manager
    - [ ] Allocator API
    - [ ] Virtual Memory handler/MMU usage
- [ ] Basic in-kernel filesystem
- [ ] Device-tree handling
- [ ] ELF loader
- [ ] IPC implementation
- [ ] Drivers
    - [ ] Driver API
    - [ ] Kernel API
