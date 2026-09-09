#[derive(Clone, Copy)]
pub enum IpiMessage {
    Reschedule = 0,
    CallFunc = 1,
    CpuStop = 2,
    CpuCrashStop = 3,
    IrqWork = 4,
    Timer = 5,
    CpuBacktrace = 6,
    KgdbRoundup = 7,
}

impl IpiMessage {
    pub const fn bit(self) -> usize {
        1 << (self as usize)
    }
}
