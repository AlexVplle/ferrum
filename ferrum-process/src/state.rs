pub enum BlockedKind {
    Interruptible,
    Uninterruptible,
}

pub enum ProcessState {
    Created,
    Waiting,
    Running,
    Blocked(BlockedKind),
    Terminated,
    SwappedOutAndReady,
    SwappedOutAndBlocked,
}
