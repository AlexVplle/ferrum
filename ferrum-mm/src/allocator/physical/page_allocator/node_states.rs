use core::ops::Deref;

use ferrum_core::bitmask_iter::BitmaskIter;
use ferrum_core::state_map::StateMap;

use super::node_state::{NodeState, NR_NODE_STATES};

pub struct NodeStates(StateMap<NR_NODE_STATES>);

impl Deref for NodeStates {
    type Target = StateMap<NR_NODE_STATES>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NodeStates {
    pub const fn new() -> Self {
        Self(StateMap::new())
    }

    pub fn node_set(&self, node_id: usize, state: NodeState) {
        self.set(node_id, state as usize);
    }

    pub fn node_clear(&self, node_id: usize, state: NodeState) {
        self.clear(node_id, state as usize);
    }

    pub fn node_is_set(&self, node_id: usize, state: NodeState) -> bool {
        self.is_set(node_id, state as usize)
    }

    pub fn for_each_node(&self, state: NodeState) -> BitmaskIter {
        self.iter(state as usize)
    }

    pub fn for_each_online_node(&self) -> BitmaskIter {
        self.iter(NodeState::Online as usize)
    }

    pub fn node_set_all(&self, state: NodeState) {
        self.set_all(state as usize);
    }

    pub fn node_is_empty(&self, state: NodeState) -> bool {
        self.is_empty(state as usize)
    }
}
