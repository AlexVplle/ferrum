use super::frame::TrapFrame;

pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EcallFromUMode,
    EcallFromSMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown(usize),
}

impl Exception {
    pub fn handle(&self, frame: &TrapFrame) {
        let name: &str = match self {
            Exception::InstructionAddressMisaligned => "instruction address misaligned",
            Exception::InstructionAccessFault => "instruction access fault",
            Exception::IllegalInstruction => "illegal instruction",
            Exception::Breakpoint => "breakpoint",
            Exception::LoadAddressMisaligned => "load address misaligned",
            Exception::LoadAccessFault => "load access fault",
            Exception::StoreAddressMisaligned => "store address misaligned",
            Exception::StoreAccessFault => "store access fault",
            Exception::EcallFromUMode => "ecall from U-mode",
            Exception::EcallFromSMode => "ecall from S-mode",
            Exception::InstructionPageFault => "instruction page fault",
            Exception::LoadPageFault => "load page fault",
            Exception::StorePageFault => "store page fault",
            Exception::Unknown(_) => "unknown exception",
        };

        crate::printkln!("[trap] unhandled exception: {} sepc={:#x} stval={:#x}",
            name, frame.sepc.bits(), frame.stval.bits());

        loop {}
    }
}
