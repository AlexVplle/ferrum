pub enum Interrupt {
    SupervisorSoftware,
    SupervisorTimer,
    SupervisorExternal,
    Unknown(usize),
}

impl Interrupt {
    pub fn handle(&self) {
        match self {
            Interrupt::SupervisorSoftware => {
                crate::arch::riscv::csr::Sip::clear_bits(
                    crate::arch::riscv::csr::sip::SUPERVISOR_SOFTWARE_INTERRUPT_PENDING,
                );
                let processor_id: usize = ferrum_core::arch::current_processor_id();
                crate::arch::riscv::smp::INTER_PROCESSOR_INTERRUPT_CONTROLLER
                    .handle_inter_processor_interrupt(processor_id);
            }
            Interrupt::SupervisorTimer => {
                crate::timer::on_tick();
            }
            Interrupt::SupervisorExternal => {
                let source: u32 = crate::arch::riscv::plic::claim();
                if source != 0 {
                    crate::arch::riscv::plic::complete(source);
                }
            }
            Interrupt::Unknown(_) => {}
        }
    }
}
