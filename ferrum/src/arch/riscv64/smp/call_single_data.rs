use ferrum_core::linked_list::llist::LlistNode;

pub type SmpCallFunc = fn(usize);
pub type SmpCondFunc = fn(usize, usize) -> bool;

#[repr(C)]
pub struct CallSingleData {
    pub node: LlistNode,
    pub func: SmpCallFunc,
    pub data: usize,
}

impl CallSingleData {
    pub fn new(func: SmpCallFunc, data: usize) -> Self {
        Self {
            node: LlistNode::new(),
            func,
            data,
        }
    }
}
