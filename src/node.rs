use std::ops::Deref;

use wasmtime::{AsContextMut, ExternRef, Result, Rooted, component::Resource};

use crate::{
    EventTarget, WindowStates, document::DocumentImpl, element::ElementImpl, object::Object,
    ohim::dom::node::HostNode,
};

/// <https://dom.spec.whatwg.org/#node>
#[derive(Clone, Debug)]
pub struct Node(Object<NodeImpl>);

// TODO: This should be NodeMethods traits. Same for a EventTarget traits
impl Node {
    /// <https://dom.spec.whatwg.org/#concept-node-append>
    pub fn append(&self, node: &Node, store: impl AsContextMut) -> Node {
        // To append a node to a parent, pre-insert node into parent before null.
        self.pre_insert(node, None, store)
    }

    /// <https://dom.spec.whatwg.org/#concept-node-pre-insert>
    pub fn pre_insert(
        &self,
        node: &Node,
        child: Option<&Node>,
        mut store: impl AsContextMut,
    ) -> Node {
        // To pre-insert a node into a parent before a child, run these steps:
        // TODO:
        // 1. Ensure pre-insert validity of node into parent before child.

        // 2. Let referenceChild be child.
        // 3. If referenceChild is node, then set referenceChild to node’s next sibling.
        if let Some(reference) = child {
            if let Ok(true) = Rooted::ref_eq(&store, reference.as_root(), node.as_root()) {
                let node = node.data_mut(&mut store);
                node.next_sibling = Some(reference.clone());
            }
        }

        // TODO:
        // 4. Insert node into parent before referenceChild.

        // 5. Return node.
        node.clone()
    }

    /// Get `Rooted<ExternRef>` reference of the `Node`.
    pub fn as_root(&self) -> &Rooted<ExternRef> {
        &***self
    }
}

impl Deref for Node {
    type Target = Object<NodeImpl>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implementation of acutal `Node` object. It also contains data of types that inherent `Node`
/// like `Document`, `Element`, `Attr`... etc. So it can also present as these types.
#[derive(Debug)]
pub struct NodeImpl {
    event_target: EventTarget,
    parent_node: Option<Node>,
    first_child: Option<Node>,
    last_child: Option<Node>,
    previous_sibling: Option<Node>,
    next_sibling: Option<Node>,
    pub(crate) data: NodeTypeData,
}

impl NodeImpl {
    /// Create an empty `NodeImpl`.
    pub fn new() -> Self {
        NodeImpl {
            event_target: EventTarget::new(),
            parent_node: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            data: NodeTypeData::None,
        }
    }
}

/// The actual implementation of each node type
#[derive(Debug, Default)]
pub enum NodeTypeData {
    /// `ELEMENT_NODE`
    Element(ElementImpl),
    /// `DOCUMENT_NODE`
    Document(DocumentImpl),
    /// Similer to `Option::None`.
    #[default]
    None,
}

impl HostNode for WindowStates {
    fn append_child(&mut self, self_: Resource<Node>, child: Resource<Node>) -> Resource<Node> {
        // TODO: properly handle error for all host traits
        let self_ = self.table.get(&self_).unwrap();
        let child_ = self.table.get(&child).unwrap();
        self_.append(&child_, &mut self.store);
        child
    }

    fn drop(&mut self, rep: Resource<Node>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
