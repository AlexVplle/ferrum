#[derive(Clone, Copy)]
pub enum CpuState {
    Possible = 0,
    Present = 1,
    Online = 2,
    Active = 3,
}

pub const NR_CPU_STATES: usize = 4;
