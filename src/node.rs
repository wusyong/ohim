use wasmtime::{
    AsContextMut, ExternRef, Result, Rooted, StoreContext, StoreContextMut, component::Resource,
};

use crate::{
    EventTarget, WindowStates, event_target::EventTargetMethods, object::Object,
    ohim::dom::node::HostNode,
};

#[derive(Clone)]
pub struct Node(Object<NodeImpl>);

pub type NodeObject = Rooted<ExternRef>;

impl Node {
    pub fn new(context: impl AsContextMut) -> Result<Self> {
        let node = NodeImpl::new();
        Ok(Self(Object::new(context, node)?))
    }
}

pub struct NodeImpl {
    event_target: EventTarget,
    child_node: Option<NodeObject>,
}

impl NodeImpl {
    pub fn new() -> Self {
        NodeImpl {
            event_target: EventTarget::new(),
            child_node: None,
        }
    }
}

impl Node {
    fn append_child(&mut self, child: Self, mut store: impl AsContextMut) {
        let node = self.0.data_mut(&mut store).unwrap();
        node.child_node = Some(child.0.to_externref());
    }
}

impl HostNode for WindowStates {
    fn new(&mut self) -> Resource<Node> {
        self.table
            .push(Node::new(&mut self.store).unwrap())
            .unwrap()
    }

    fn append_child(&mut self, self_: Resource<Node>, child: Resource<Node>) -> Resource<Node> {
        let mut self_ = self.table.get(&self_).unwrap().clone();
        let child_ = self.table.get(&child).unwrap();
        Node::append_child(&mut self_, child_.clone(), &mut self.store);
        child
    }

    fn drop(&mut self, rep: Resource<Node>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
