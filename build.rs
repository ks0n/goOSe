fn main() {
    println!("cargo:rerun-if-changed=src/arch/riscv64/link.lds");
    println!("cargo:rerun-if-changed=src/arch/x86_64/link.lds");
}
