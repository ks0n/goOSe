# goOSe

goOSe is a minimal Kernel written in Rust. The aim of this project is to end up
with a functional, albeit simple kernel.

## Design

goOSe is focused on modularity. Each feature should be a submodule in the
`src` directory, or its own crate.

## Objectives

goOSe aims to become a tiny micro kernel. For more information, have a look at
the [roadmap](ROADMAP.md)

## Dependencies

Enter in the provided nix-shell:
`nix-shell goose.nix`

To specifiy a different shell like `zsh`, `nix-shell goose.nix --command zsh`

This will install the required, non-cargo dependencies, such as `qemu`, locally.

## Building

To build, use `make build`.

To run in qemu, use `make run`.

To run the tests, use `make test`.

## Contributing

### Adding unit tests

Adding unit tests is done through the `kassert*` macros. Here's an example:

```rust
use crate::kassert_eq;
use crate::kassert;

#[test_case]
fn new_test() {
    kassert_eq!(1, 1); // Will assert with 'Unknown test' as the test name
    kassert_eq!(1, 1, "new_test"); // Will assert with 'new_test' as the test name
    kassert!(true);
}
```

## Origins

* [Philipp Opperman's Blog](https://os.phil-opp.com/)
* [The Adventures of OS: RISC-V OS using Rust](https://osblog.stephenmarz.com/index.html)
* [LSE's K Project](https://k.lse.epita.fr)

## License and copyright

Copyright 2020 Arthur Cohen and Esteban Blanc

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE-2](LICENSE-APACHE-2) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

See the [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for more information about utilized third
party projects and their respective licenses.
