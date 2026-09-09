use ferrum_core::linked_list::llist::LlistNode;

pub type IrqWorkFunc = fn();

#[repr(C)]
pub struct IrqWork {
    pub node: LlistNode,
    pub func: IrqWorkFunc,
}

impl IrqWork {
    pub fn new(func: IrqWorkFunc) -> Self {
        Self {
            node: LlistNode::new(),
            func,
        }
    }
}
