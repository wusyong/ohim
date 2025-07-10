use std::ops::Deref;

use wasmtime::{AsContext, AsContextMut, ExternRef, Result, Rooted, component::Resource};

use crate::{NodeImpl, NodeTypeData, WindowStates, object::Object, ohim::dom::node::HostElement};

/// <https://dom.spec.whatwg.org/#element>
#[derive(Clone, Debug)]
pub struct Element(Object<NodeImpl>);

// TODO: This should be NodeMethods traits. Same for a EventTarget traits
impl Element {
    /// Create a `Element` object.
    pub fn new(store: impl AsContextMut) -> Result<Self> {
        Ok(Element(Object::new(
            store,
            NodeImpl::new_with_type(NodeTypeData::Element(ElementImpl::new())),
        )?))
    }

    /// <https://dom.spec.whatwg.org/#dom-element-hasattributes>
    pub fn has_attributes(&self, store: impl AsContext) -> bool {
        !self.data(&store).as_element().attribute_list.is_empty()
    }

    /// Get `Rooted<ExternRef>` reference of the `Node`.
    pub fn as_root(&self) -> &Rooted<ExternRef> {
        self
    }
}

impl NodeImpl {
    /// Get `ElementImpl` shared reference.
    fn as_element(&self) -> &ElementImpl {
        let NodeTypeData::Element(ref element) = self.data else {
            unreachable!()
        };
        element
    }

    /// Get `ElementImpl` exclusive reference.
    fn as_element_mut(&mut self) -> &mut ElementImpl {
        let NodeTypeData::Element(ref mut element) = self.data else {
            unreachable!()
        };
        element
    }
}

impl Deref for Element {
    type Target = Object<NodeImpl>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implementation of acutal `Element` object. This can be accessed from `NodeImpl`.
#[derive(Debug)]
pub struct ElementImpl {
    attribute_list: Vec<u8>,
}

impl ElementImpl {
    /// Create an empty `DocumentImpl`.
    pub fn new() -> Self {
        ElementImpl {
            attribute_list: Vec::new(),
        }
    }
}

impl HostElement for WindowStates {
    fn has_attributes(&mut self, self_: Resource<Element>) -> Result<bool> {
        let self_ = self.table.get(&self_)?;
        Ok(self_.has_attributes(&self.store))
    }

    fn drop(&mut self, rep: Resource<Element>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
