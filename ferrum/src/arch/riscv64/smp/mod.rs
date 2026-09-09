mod call_single_data;
mod hart_ipi_state;
mod ipi_controller;
mod ipi_message;
mod irq_work;

pub use call_single_data::{CallSingleData, SymmetricMultiprocessingCallFunction, SymmetricMultiprocessingConditionFunction};
pub use ipi_controller::InterProcessorInterruptController;
pub use irq_work::{InterruptRequestWork, InterruptRequestWorkFunction};

pub static INTER_PROCESSOR_INTERRUPT_CONTROLLER: InterProcessorInterruptController =
    InterProcessorInterruptController::new();
