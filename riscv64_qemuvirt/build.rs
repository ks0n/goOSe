fn main() {
    println!("cargo:rerun-if-changed=src/riscv64_qemuvirt.ld");
    println!("cargo:rustc-link-arg=-Triscv64_qemuvirt/src/riscv64_qemuvirt.ld");
}
