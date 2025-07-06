use std::ops::Deref;

use wasmtime::{AsContext, ExternRef, Result, Rooted, component::Resource};

use crate::{
    Element, NodeImpl, NodeTypeData, WindowStates, object::Object, ohim::dom::node::HostDocument,
};

/// <https://dom.spec.whatwg.org/#document>
#[derive(Clone, Debug)]
pub struct Document(Object<NodeImpl>);

// TODO: This should be NodeMethods traits. Same for a EventTarget traits
impl Document {
    /// <https://dom.spec.whatwg.org/#dom-document-url>
    pub fn url(&self, store: impl AsContext) -> String {
        // TODO: implement real one
        self.data(&store).as_document().url.clone()
    }

    /// <https://dom.spec.whatwg.org/#dom-document-documentelement>
    pub fn document_element(&self, store: impl AsContext) -> Option<Element> {
        // The documentElement getter steps are to return this’s document element.
        self.data(&store).as_document().document_element.clone()
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
    document_element: Option<Element>,
}

impl DocumentImpl {
    /// Create an empty `DocumentImpl`.
    pub fn new() -> Self {
        DocumentImpl {
            url: String::new(),
            document_element: None,
        }
    }
}

impl HostDocument for WindowStates {
    fn new(&mut self) -> Resource<Document> {
        todo!()
        //     self.table
        //         .push(Node::new(&mut self.store).unwrap())
        //         .unwrap()
    }

    fn drop(&mut self, rep: Resource<Document>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn url(&mut self, self_: Resource<Document>) -> String {
        let self_ = self.table.get(&self_).unwrap();
        self_.url(&self.store)
    }

    fn document_element(&mut self, self_: Resource<Document>) -> Option<Resource<Element>> {
        let self_ = self.table.get(&self_).unwrap();
        self_
            .document_element(&self.store)
            .map(|element| self.table.push(element).unwrap())
    }
}
