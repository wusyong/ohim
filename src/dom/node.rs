use std::{collections::VecDeque, ops::Deref};

use wasmtime::{AsContextMut, ExternRef, Result, Rooted, component::Resource};

use crate::{
    DocumentImpl, ElementImpl, EventTarget, Object, WindowStates, ohim::dom::node::HostNode,
};

use super::Document;

/// <https://dom.spec.whatwg.org/#node>
#[derive(Clone, Debug)]
pub struct Node(Object<NodeImpl>);

// TODO: This should be NodeMethods traits. Same for a EventTarget traits
impl Node {
    /// <https://dom.spec.whatwg.org/#concept-node-pre-insert>
    pub fn pre_insert(&self, node: &Node, child: Option<&Node>, mut store: impl AsContextMut) {
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
    }

    /// <https://dom.spec.whatwg.org/#concept-node-insert>
    pub fn insert(
        &self,
        node: &Node,
        child: Option<&Node>,
        _suppress: bool,
        mut store: impl AsContextMut,
    ) {
        // 1. TODO: Let nodes be node’s children, if node is a DocumentFragment node; otherwise « node ».
        // This should implement a node iterator in tree order.
        let nodes = [node];
        // 4. TODO: If node is a DocumentFragment node:
        // 5. TODO: If child is non-null:
        // 6. Let previousSibling be child’s previous sibling or parent’s last child if child is null.
        let _previous_sibling = match child {
            Some(c) => c.data(&store).previous_sibling.as_ref(),
            None => self.data(&store).last_child(),
        };
        // 7. For each node in nodes, in tree order:
        for node in nodes {
            // 7.1 Adopt node into parent’s node document.
            node.adopt(self.data(&store).node_document.clone(), &mut store);
            // TODO: Step 7.4 ~ 7.7
        }
        // TODO: Step 8 ~ 12
    }

    /// <https://dom.spec.whatwg.org/#concept-node-adopt>
    pub fn adopt(&self, document: Option<Document>, mut store: impl AsContextMut) {
        // 1. Let oldDocument be node’s node document.
        let old_document = self.data(&store).node_document.as_ref();
        // 2. TODO: If node’s parent is non-null, then remove node.
        // 3. If document is not oldDocument:
        let not_same = match (&document, old_document) {
            (Some(d), Some(od)) => {
                Rooted::ref_eq(&store, d.as_root(), od.as_root()).unwrap_or_default()
            }
            _ => false,
        };
        if not_same {
            // TODO: Step 3.1, 3.2, 3.3
            self.data_mut(&mut store).set_node_document(document);
        }
    }

    /// Get `Rooted<ExternRef>` reference of the `Node`.
    pub fn as_root(&self) -> &Rooted<ExternRef> {
        self
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
    _event_target: EventTarget,
    _parent_node: Option<Node>,
    child_nodes: VecDeque<Node>,
    previous_sibling: Option<Node>,
    next_sibling: Option<Node>,
    node_document: Option<Document>,
    pub(crate) data: NodeTypeData,
}

impl NodeImpl {
    /// Create an `NodeImpl` with provided node type data.
    pub fn new_with_type(data: NodeTypeData) -> Self {
        NodeImpl {
            _event_target: EventTarget::new(),
            _parent_node: None,
            child_nodes: VecDeque::new(),
            previous_sibling: None,
            next_sibling: None,
            node_document: None,
            data,
        }
    }

    /// Set Node's node document.
    pub fn set_node_document(&mut self, document: Option<Document>) {
        self.node_document = document;
    }

    /// Get last child of node's child nodes.
    pub fn last_child(&self) -> Option<&Node> {
        self.child_nodes.back()
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
    fn append_child(
        &mut self,
        self_: Resource<Node>,
        child: Resource<Node>,
    ) -> Result<Resource<Node>> {
        // TODO: properly handle error for all host traits
        let self_ = self.table.get(&self_)?;
        let child_ = self.table.get(&child)?;
        self_.pre_insert(child_, None, &mut self.store);
        Ok(child)
    }

    fn drop(&mut self, rep: Resource<Node>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
