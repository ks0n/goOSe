use core::convert::TryFrom;
use num_enum::TryFromPrimitive;

/// Enum of the different exceptions. See [Volume 2, Privileged Spec v.
/// 20190608](https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMFDQC-and-Priv-v1.11/riscv-privileged-20190608.pdf)
///          table 4.2
#[repr(usize)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
enum Exception {
    IllegalInstruction = 2,
}

/// Handle exceptions
pub fn handle(id: usize, pc: usize) {
    let exception = Exception::try_from(id).unwrap();
    match exception {
        Exception::IllegalInstruction => {
            panic!("Illegal instruction at {:#x}!", pc)
        }
        _ => panic!("Exception {} not handled yet!. Occured at {:#x}", id, pc),
    }
}
