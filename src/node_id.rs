pub type NodeId = i64;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(super) struct NodeIdGenerator {
    node_id: NodeId,
}

impl NodeIdGenerator {
    pub fn new() -> NodeIdGenerator {
        NodeIdGenerator {
            node_id: 0,
        }
    }

    pub fn next(&mut self) -> NodeId {
        self.node_id += 1;
        self.node_id
    }
}

impl Default for NodeIdGenerator {
    fn default() -> NodeIdGenerator {
        NodeIdGenerator {
            node_id: 0,
        }
    }
}
