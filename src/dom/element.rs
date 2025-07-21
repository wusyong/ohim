use std::ops::Deref;

use wasmtime::{AsContext, AsContextMut, ExternRef, Result, Rooted, component::Resource};

use crate::{
    NodeImpl, NodeTypeData, Object, WindowStates, agent::NameSpace, ohim::dom::node::HostElement,
    string::DOMString,
};

use super::{Document, HTMLElementImpl, HTMLElementType};

/// <https://dom.spec.whatwg.org/#element>
#[derive(Clone, Debug)]
pub struct Element(Object<NodeImpl>);

// TODO: This should be NodeMethods traits. Same for a EventTarget traits
impl Element {
    /// <https://dom.spec.whatwg.org/#concept-create-element>
    /// TODO: synchronousCustomElements, registry
    pub fn new(
        document: &Document,
        local: ElementLocal,
        name_space: NameSpace,
        is: Option<DOMString>,
        store: impl AsContextMut,
    ) -> Result<Self> {
        // TODO: Step 1 ~ 5 implement CustomElementRegistry
        // 6.1 Let interface be the element interface for localName and namespace.
        // This is done by new_internal.
        // 6.2 Set result to the result of creating an element internal given document, interface, localName,
        // namespace, prefix, "uncustomized", is, and registry.
        Ok(Element(Object::new(
            store,
            NodeImpl::new_with_type(NodeTypeData::Element(ElementImpl::new(
                document,
                local,
                name_space,
                CustomElementState::Uncustomized,
                is,
            ))),
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

    // /// Get `ElementImpl` exclusive reference.
    // fn as_element_mut(&mut self) -> &mut ElementImpl {
    //     let NodeTypeData::Element(ref mut element) = self.data else {
    //         unreachable!()
    //     };
    //     element
    // }
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
    _name_space: NameSpace,
    _local_name: ElementLocal,
    _state: CustomElementState,
    _is: Option<DOMString>,
    _node_document: Document,
    attribute_list: Vec<u8>,
    _element_type: ElementType,
}

impl ElementImpl {
    /// <https://dom.spec.whatwg.org/#create-an-element-internal>
    /// TODO: registry, custom element definition
    fn new(
        document: &Document,
        local: ElementLocal,
        name_space: NameSpace,
        state: CustomElementState,
        is: Option<DOMString>,
    ) -> Self {
        let element_type = ElementType::get(&local, &name_space);
        Self {
            _node_document: document.clone(),
            attribute_list: Vec::new(),
            _name_space: name_space,
            _local_name: local,
            _state: state,
            _is: is,
            _element_type: element_type,
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

/// <https://dom.spec.whatwg.org/#concept-element-custom-element-state>
#[derive(Clone, Copy, Debug)]
pub enum CustomElementState {
    /// "undefined"
    Undefined,
    /// "failed"
    Failed,
    /// "uncustomized"
    Uncustomized,
    /// "precustomized"
    Precustomized,
    /// "custom"
    Custom,
}

/// The actual implementation of each element type
#[derive(Debug, Default)]
pub enum ElementType {
    /// HTMLElement
    HTMLElement(HTMLElementImpl),
    /// Similer to `Option::None`.
    #[default]
    None,
}

impl ElementType {
    fn get(local: &ElementLocal, name_space: &NameSpace) -> Self {
        match name_space {
            NameSpace::HTML => match local {
                ElementLocal::Html => {
                    ElementType::HTMLElement(HTMLElementImpl::new(HTMLElementType::Html))
                }
                ElementLocal::Head => {
                    ElementType::HTMLElement(HTMLElementImpl::new(HTMLElementType::Head))
                }
                ElementLocal::Body => {
                    ElementType::HTMLElement(HTMLElementImpl::new(HTMLElementType::Body))
                }
                _ => ElementType::None,
            },
            NameSpace::None => ElementType::None,
        }
    }
}

/// Collection of local name to create an element.
#[derive(Debug, Clone)]
pub enum ElementLocal {
    /// "html"
    Html,
    /// "head"
    Head,
    /// "body"
    Body,
    /// "custom"
    Custom(DOMString),
}
