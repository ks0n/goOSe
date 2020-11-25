use core::convert::TryFrom;
use num_enum::TryFromPrimitive;

/// Enum of the different exceptions. See [Volume 2, Privileged Spec v.
/// 20190608](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMFDQC-and-Priv-v1.11/riscv-privileged-20190608.pdf)
///          table 4.2
#[repr(usize)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
enum Exception {
    IllegalInstruction = 2,
    #[num_enum(default)]
    Unhandled,
}

/// Handle exceptions
pub fn handle(id: usize, pc: usize) {
    let exception = Exception::try_from(id).unwrap(); // Unwrap is safe, as we have a num_enum(default) in Exception enum
    match exception {
        Exception::IllegalInstruction => {
            panic!("Illegal instruction at {:#x}!", pc)
        }
        Exception::Unhandled => panic!("Exception {} not handled yet!. Occured at {:#x}", id, pc),
    }
}
