fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let linker_scripts = [
        ("boards/generic_qemuvirt.ld",
            ["riscv64_qemuvirt", "aarch64_qemuvirt"]),
    ];

    for (linker_script, binaries) in linker_scripts.into_iter() {
        for binary in binaries {
            println!("cargo:rustc-link-arg-bin={}={}", binary, linker_script);
            println!("cargo:rerun-if-changed={}", linker_script);
        }
    }
}
