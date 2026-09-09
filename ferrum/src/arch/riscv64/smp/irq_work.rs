use ferrum_core::linked_list::llist::LlistNode;

pub type InterruptRequestWorkFunction = fn();

#[repr(C)]
pub struct InterruptRequestWork {
    pub node: LlistNode,
    pub function: InterruptRequestWorkFunction,
}

impl InterruptRequestWork {
    pub fn new(function: InterruptRequestWorkFunction) -> Self {
        Self {
            node: LlistNode::new(),
            function,
        }
    }
}
