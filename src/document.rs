use std::ops::Deref;

use wasmtime::{AsContext, ExternRef, Rooted};

use crate::{Element, NodeImpl, NodeTypeData, object::Object};

/// <https://dom.spec.whatwg.org/#document>
#[derive(Clone, Debug)]
pub struct Document(Object<NodeImpl>);

// TODO: This should be NodeMethods traits. Same for a EventTarget traits
impl Document {
    /// <https://dom.spec.whatwg.org/#dom-document-url>
    pub fn url(&self, store: impl AsContext) -> String {
        self.data(&store).as_document().url.clone()
    }
    /// Get `Rooted<ExternRef>` reference of the `Node`.
    pub fn as_root(&self) -> &Rooted<ExternRef> {
        &***self
    }
}

impl NodeImpl {
    /// Get `DocumentImpl` shared reference.
    fn as_document(&self) -> &DocumentImpl {
        let NodeTypeData::Document(ref doc) = self.data else {
            unreachable!()
        };
        doc
    }

    /// Get `DocumentImpl` exclusive reference.
    fn as_document_mut(&mut self) -> &mut DocumentImpl {
        let NodeTypeData::Document(ref mut doc) = self.data else {
            unreachable!()
        };
        doc
    }
}

impl Deref for Document {
    type Target = Object<NodeImpl>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implementation of acutal `Docuemt` object. This can be accessed from `NodeImpl`.
#[derive(Debug)]
pub struct DocumentImpl {
    url: String,
    parent_element: Option<Element>,
}

impl DocumentImpl {
    /// Create an empty `DocumentImpl`.
    pub fn new() -> Self {
        DocumentImpl {
            url: String::new(),
            parent_element: None,
        }
    }
}
