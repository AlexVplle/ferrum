use core::ops::Deref;

use ferrum_core::bitmask_iter::BitmaskIter;
use ferrum_core::state_map::StateMap;

use super::cpu_state::{CpuState, NR_CPU_STATES};

pub struct CpuStates(StateMap<NR_CPU_STATES>);

impl Deref for CpuStates {
    type Target = StateMap<NR_CPU_STATES>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CpuStates {
    pub const fn new() -> Self {
        Self(StateMap::new())
    }

    pub fn cpu_set(&self, cpu_id: usize, state: CpuState) {
        self.set(cpu_id, state as usize);
    }

    pub fn cpu_clear(&self, cpu_id: usize, state: CpuState) {
        self.clear(cpu_id, state as usize);
    }

    pub fn cpu_is_set(&self, cpu_id: usize, state: CpuState) -> bool {
        self.is_set(cpu_id, state as usize)
    }

    pub fn for_each_cpu(&self, state: CpuState) -> BitmaskIter {
        self.iter(state as usize)
    }

    pub fn for_each_online_cpu(&self) -> BitmaskIter {
        self.iter(CpuState::Online as usize)
    }
}
