#[derive(Clone, Copy)]
pub enum NodeState {
    Possible = 0,
    Online = 1,
    NormalMemory = 2,
    HighMemory = 3,
    Memory = 4,
    Cpu = 5,
    GenericInitiator = 6,
}

pub const NR_NODE_STATES: usize = 7;
