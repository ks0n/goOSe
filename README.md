# goOSe

goOSe is a minimal Kernel written in Rust. The aim of this project is to end up
with a functional, albeit simple kernel.

# Design

goOSe is focused on modularity. Each feature should be a submodule in the
`src` directory, or its own crate.

# Cargo dependencies

* `cargo-xbuild`
* `bootimage`

# Building

`cargo bootimage` will build `.bin` file in the target directory. You can
launch it with qemu with the following command:

`qemu -drive format=raw,file=target/x86_64_goose/debug/bootimage-goose.bin`

Alternatively, `cargo xrun` will build and run said binary kernel.

To run the tests, use `cargo xtest`.

# Contributing

## Adding unit tests

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

# Origins

* [Philipp Opperman's Blog](https://os.phil-opp.com/)
* [LSE's K Project](https://k.lse.epita.fr)

## License

See the [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for more information about utilized third
party projects and their respective licenses.
