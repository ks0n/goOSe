fn main() {
    println!("cargo:rerun-if-changed=src/arch/riscv64/link.lds");
}
