fn main() {
    println!("cargo:rerun-if-changed=src/aarch64_qemuvirt.ld");
    println!("cargo:rustc-link-arg=-Taarch64_qemuvirt/src/aarch64_qemuvirt.ld");
}
