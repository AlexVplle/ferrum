use ferrum_core::linked_list::llist::LlistNode;

pub type SymmetricMultiprocessingCallFunction = fn(usize);
pub type SymmetricMultiprocessingConditionFunction = fn(usize, usize) -> bool;

#[repr(C)]
pub struct CallSingleData {
    pub node: LlistNode,
    pub function: SymmetricMultiprocessingCallFunction,
    pub data: usize,
}

impl CallSingleData {
    pub fn new(function: SymmetricMultiprocessingCallFunction, data: usize) -> Self {
        Self {
            node: LlistNode::new(),
            function,
            data,
        }
    }
}
