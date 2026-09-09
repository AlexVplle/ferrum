use crate::arch::riscv64::csr::Scause;
use super::exception::Exception;
use super::interrupt::Interrupt;

pub enum Trap {
    Exception(Exception),
    Interrupt(Interrupt),
}

impl From<Scause> for Trap {
    fn from(scause: Scause) -> Trap {
        let code: usize = scause.code();
        if scause.is_interrupt() {
            Trap::Interrupt(match code {
                1 => Interrupt::SupervisorSoftware,
                5 => Interrupt::SupervisorTimer,
                9 => Interrupt::SupervisorExternal,
                _ => Interrupt::Unknown(code),
            })
        } else {
            Trap::Exception(match code {
                0 => Exception::InstructionAddressMisaligned,
                1 => Exception::InstructionAccessFault,
                2 => Exception::IllegalInstruction,
                3 => Exception::Breakpoint,
                4 => Exception::LoadAddressMisaligned,
                5 => Exception::LoadAccessFault,
                6 => Exception::StoreAddressMisaligned,
                7 => Exception::StoreAccessFault,
                8 => Exception::EcallFromUMode,
                9 => Exception::EcallFromSMode,
                12 => Exception::InstructionPageFault,
                13 => Exception::LoadPageFault,
                15 => Exception::StorePageFault,
                _ => Exception::Unknown(code),
            })
        }
    }
}
