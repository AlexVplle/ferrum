use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(usize)]
pub enum SystemState {
    Booting = 0,
    Scheduling = 1,
    FreeingInitMem = 2,
    Running = 3,
    Halt = 4,
    PowerOff = 5,
    Restart = 6,
    Suspend = 7,
}

static SYSTEM_STATE: AtomicUsize = AtomicUsize::new(SystemState::Booting as usize);

pub fn system_state() -> SystemState {
    match SYSTEM_STATE.load(Ordering::Acquire) {
        1 => SystemState::Scheduling,
        2 => SystemState::FreeingInitMem,
        3 => SystemState::Running,
        4 => SystemState::Halt,
        5 => SystemState::PowerOff,
        6 => SystemState::Restart,
        7 => SystemState::Suspend,
        _ => SystemState::Booting,
    }
}

pub fn set_system_state(state: SystemState) {
    SYSTEM_STATE.store(state as usize, Ordering::Release);
}
