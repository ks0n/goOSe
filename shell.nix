with import <nixpkgs> {
  # crossSystem = {
    # config = "aarch64-unknown-linux-gnu";
  # };
};

let
  aarch64cc = pkgsCross.aarch64-multiplatform.buildPackages.gcc;
  riscv64cc = pkgsCross.riscv64-embedded.buildPackages.gcc;
in
mkShell {
  CARGO_TARGET_AARCH64_UNKNOWN_NONE_LINKER = "${aarch64cc.targetPrefix}ld";
  CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_LINKER = "${riscv64cc.targetPrefix}ld";

  depsBuildBuild = [ 
    aarch64cc
    riscv64cc
    qemu
  ];
}
