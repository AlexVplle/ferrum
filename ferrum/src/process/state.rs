pub enum ProcessState {
    Created,
    Waiting,
    Running,
    Blocked,
    Terminated,
    SwappedOutAndReady,
    SwappedOutAndBlocked,
}
