fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let linker_scripts = [
        ("boards/riscv64_qemuvirt.ld", ["riscv64_qemuvirt"]),
        ("boards/aarch64_qemuvirt.ld", ["aarch64_qemuvirt"]),
    ];

    for (linker_script, binaries) in linker_scripts.into_iter() {
        let linker_script = format!("{}/{}", "kernel", linker_script);
        for binary in binaries {
            println!("cargo:rustc-link-arg-bin={}={}", binary, linker_script);
            println!("cargo:rerun-if-changed={}", linker_script);
        }
    }
}
