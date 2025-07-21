use std::ops::Deref;

use wasmtime::{ExternRef, Rooted};

use super::{NodeImpl, Object};

/// <https://html.spec.whatwg.org/multipage/#htmlelement>
#[derive(Clone, Debug)]
pub struct HTMLElement(Object<NodeImpl>);

impl HTMLElement {
    /// Get `Rooted<ExternRef>` reference of the `Node`.
    pub fn as_root(&self) -> &Rooted<ExternRef> {
        self
    }
}

// impl NodeImpl {
//     /// Get `ElementImpl` shared reference.
//     fn as_html_element(&self) -> &HTMLElementImpl {
//         let NodeTypeData::Element(ref element) = self.data else {
//             unreachable!()
//         };
//         element
//     }
//
//     /// Get `ElementImpl` exclusive reference.
//     fn as_element_mut(&mut self) -> &mut HTMLElementImpl {
//         let NodeTypeData::Element(ref mut element) = self.data else {
//             unreachable!()
//         };
//         element
//     }
// }

impl Deref for HTMLElement {
    type Target = Object<NodeImpl>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implementation of acutal `HTMLElement` object. This can be accessed from `NodeImpl`.
#[derive(Debug)]
pub struct HTMLElementImpl {
    _html_type: HTMLElementType,
}

impl HTMLElementImpl {
    /// Create an `HTMLElementImpl` with provided node type data.
    pub fn new(data: HTMLElementType) -> Self {
        Self { _html_type: data }
    }
}

/// The actual implementation of each HTMLElement type
#[derive(Debug, Default)]
pub enum HTMLElementType {
    /// HTMLHtmlElement
    Html,
    /// HTMLHeadElement
    Head,
    /// HTMLBodyElement
    Body,
    /// Similer to `Option::None`.
    #[default]
    None,
}
