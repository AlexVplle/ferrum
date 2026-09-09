pub mod cpu_state;
pub mod cpu_states;

pub use cpu_state::CpuState;
pub use cpu_states::CpuStates;

pub static CPU_STATES: CpuStates = CpuStates::new();
