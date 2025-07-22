use std::{collections::VecDeque, ops::Deref};

use wasmtime::{AsContextMut, ExternRef, Result, Rooted, component::Resource};

use crate::{
    DocumentImpl, ElementImpl, EventTarget, Object, WindowStates, ohim::dom::node::HostNode,
};

use super::{Document, Element};

/// <https://dom.spec.whatwg.org/#node>
#[derive(Clone, Debug)]
pub struct Node(pub(crate) Object<NodeImpl>);

// TODO: This should be NodeMethods traits. Same for a EventTarget traits
impl Node {
    /// <https://dom.spec.whatwg.org/#concept-node-pre-insert>
    pub fn pre_insert(&self, node: Node, child: Option<&Node>, mut store: impl AsContextMut) {
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
        self.insert(node, child, false, store);
    }

    /// <https://dom.spec.whatwg.org/#concept-node-insert>
    pub fn insert(
        &self,
        node: Node,
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
            match child {
                // 7.2 If child is null, then append node to parent’s children.
                None => self.append_child(node, &mut store),
                // 7.3 Otherwise, insert node into parent’s children before child’s index.
                Some(c) => {
                    if let Some(index) = self.data(&store).child_nodes.iter().position(|n| {
                        Rooted::ref_eq(&store, n.as_root(), c.as_root()).unwrap_or_default()
                    }) {
                        self.insert_child(index, node, &mut store);
                    } else {
                        // TODO: log warning!
                    }
                }
            }
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

    /// Append a child node to this node.
    pub fn append_child(&self, node: Node, mut store: impl AsContextMut) {
        if let Some(child) = self.data(&store).child_nodes.back() {
            let child = child.clone();
            child.clone().data_mut(&mut store).next_sibling = Some(node.clone());
            node.clone().data_mut(&mut store).previous_sibling = Some(child);
        }
        self.data_mut(&mut store).child_nodes.push_back(node);
    }

    /// Insert a child node to this node.
    pub fn insert_child(&self, index: usize, node: Node, mut store: impl AsContextMut) {
        if let Some(prev) = self.data(&store).child_nodes.get(index - 1) {
            let prev = prev.clone();
            prev.clone().data_mut(&mut store).next_sibling = Some(node.clone());
            node.clone().data_mut(&mut store).previous_sibling = Some(prev);
        }
        if let Some(next) = self.data(&store).child_nodes.get(index) {
            let next = next.clone();
            node.clone().data_mut(&mut store).next_sibling = Some(next.clone());
            next.data_mut(&mut store).previous_sibling = Some(node.clone());
        }
        self.data_mut(&mut store).child_nodes.insert(index, node);
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

impl From<Document> for Node {
    fn from(value: Document) -> Self {
        Self(value.0)
    }
}

impl From<Element> for Node {
    fn from(value: Element) -> Self {
        Self(value.0)
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
        let child_ = self.table.get(&child)?.clone();
        self_.pre_insert(child_, None, &mut self.store);
        Ok(child)
    }

    fn drop(&mut self, rep: Resource<Node>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
