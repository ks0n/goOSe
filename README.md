# goose

A minimal Kernel written in Rust

## Building

Currently, the following platforms are supports:

- riscv64 on qemu virt
    - `cargo build --bin riscv64_qemuvirt --target riscv64-unknown-none`
- aarch64 on qemu virt
    - `cargo build --bin aarch64_qemuvirt --target aarch64-unknown-none`

- Each platform configuration might contains a custom runner setup, meaning that
  depending on your use-case `cargo run` will be able to launch an emulator or
  flash a connected microcontroller.

## Testing
```console
$ make -C fixtures
...
$ cargo test
...
```

You might whant to build tests with release profile to speed up testing:
```console
$ cargo test --profile release
...
```

## Roadmap

- [ ] Virtual Memory Manager
    - [ ] Allocator API
    - [ ] Virtual Memory handler/MMU usage
- [ ] Basic in-kernel filesystem
- [ ] Device-tree handling
- [x] In-kernel ELF loader
- [ ] IPC implementation
- [ ] Drivers
    - [ ] Driver API
    - [ ] Kernel API
