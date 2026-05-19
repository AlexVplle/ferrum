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
            Interrupt::SupervisorTimer => {}
            Interrupt::SupervisorExternal => {}
            Interrupt::Unknown(_) => {}
        }
    }
}
