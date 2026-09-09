use super::block::NotifierBlock;
use super::result::NotifierResult;

pub type NotifierFn = fn(&NotifierBlock, event: usize, data: *const ()) -> NotifierResult;
