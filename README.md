# goose
A minimal Kernel written in Rust

# Building

- Generate the compilation configuration for your desired platform using
  [`gen_cargo.sh`](gen_cargo.sh)
- Build the project using `cargo`. You will need to install the compilation
  targets using `rustup target add`
- Each platform configuration contains a custom runner setup, meaning that
  depending on your use-case `cargo run` will be able to launch an emulator or
  flash a connected microcontroller.
