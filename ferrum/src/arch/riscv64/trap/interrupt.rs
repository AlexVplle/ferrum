pub enum Interrupt {
    SupervisorSoftware,
    SupervisorTimer,
    SupervisorExternal,
    Unknown(usize),
}

impl Interrupt {
    pub fn handle(&self) {
        match self {
            Interrupt::SupervisorSoftware => {}
            Interrupt::SupervisorTimer => {
                crate::timer::on_tick();
            }
            Interrupt::SupervisorExternal => {
                let source: u32 = crate::arch::riscv64::plic::claim();
                if source != 0 {
                    crate::arch::riscv64::plic::complete(source);
                }
            }
            Interrupt::Unknown(_) => {}
        }
    }
}
