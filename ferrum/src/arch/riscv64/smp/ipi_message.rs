#[derive(Clone, Copy)]
pub enum InterProcessorInterruptMessage {
    Reschedule = 0,
    CallFunction = 1,
    CpuStop = 2,
    CpuCrashStop = 3,
    IrqWork = 4,
    Timer = 5,
    CpuBacktrace = 6,
    KgdbRoundup = 7,
}

impl InterProcessorInterruptMessage {
    pub const fn bit(self) -> usize {
        1 << (self as usize)
    }
}
