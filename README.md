# goose

Goose is a minimal Kernel written in Rust. The aim of this project is to end up
with a functional, albeit simple kernel.

# Design

Goose is focused on modularity. Each feature should be a submodule in the
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

# Origin

[Philipp Opperman's Blog](https://os.phil-opp.com/)
