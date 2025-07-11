use std::ops::Deref;

use headers::ContentType;
use wasmtime::{AsContext, AsContextMut, ExternRef, Result, Rooted, component::Resource};

use crate::{
    Element, NodeImpl, NodeTypeData, WindowStates,
    browsing_context::BrowsingContextID,
    object::Object,
    ohim::dom::node::HostDocument,
    url::{DOMUrl, ImmutableOrigin},
};

/// <https://dom.spec.whatwg.org/#document>
#[derive(Clone, Debug)]
pub struct Document(Object<NodeImpl>);

impl Document {
    /// Create a `Document` object.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        is_html: bool,
        content_type: ContentType,
        mode: DocumentMode,
        origin: ImmutableOrigin,
        browsing_context: BrowsingContextID,
        policy: bool,
        flags: bool,
        time_info: bool,
        is_blank: bool,
        base_url: Option<DOMUrl>,
        allow_shadow: bool,
        store: impl AsContextMut,
    ) -> Result<Self> {
        Ok(Document(Object::new(
            store,
            NodeImpl::new_with_type(NodeTypeData::Document(DocumentImpl::new(
                is_html,
                content_type,
                mode,
                origin,
                browsing_context,
                policy,
                flags,
                time_info,
                is_blank,
                base_url,
                allow_shadow,
            ))),
        )?))
    }

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
        self
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
    /// <https://dom.spec.whatwg.org/#concept-document-type>
    is_html: bool,
    /// <https://dom.spec.whatwg.org/#concept-document-content-type>
    content_type: ContentType,
    /// <https://dom.spec.whatwg.org/#concept-document-mode>
    mode: DocumentMode,
    /// <https://dom.spec.whatwg.org/#concept-document-origin>
    origin: ImmutableOrigin,
    /// <https://html.spec.whatwg.org/multipage/#concept-document-bc>
    browsing_context: Option<BrowsingContextID>,
    /// <https://html.spec.whatwg.org/multipage/#concept-document-permissions-policy>
    policy: bool,
    /// <https://html.spec.whatwg.org/multipage/browsers.html#active-sandboxing-flag-set>
    flags: bool,
    /// <https://html.spec.whatwg.org/multipage/dom.html#load-timing-info>
    time_info: bool,
    /// <https://html.spec.whatwg.org/multipage/dom.html#is-initial-about:blank>
    is_blank: bool,
    /// <https://html.spec.whatwg.org/multipage/#concept-document-about-base-url>
    base_url: Option<DOMUrl>,
    /// <https://dom.spec.whatwg.org/#document-allow-declarative-shadow-roots>
    allow_shadow: bool,
    /// <https://dom.spec.whatwg.org/#document-custom-element-registry>
    custom_element: Option<bool>,
    url: String,
    document_element: Option<Element>,
}

impl DocumentImpl {
    /// Create an empty `DocumentImpl`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        is_html: bool,
        content_type: ContentType,
        mode: DocumentMode,
        origin: ImmutableOrigin,
        browsing_context: BrowsingContextID,
        policy: bool,
        flags: bool,
        time_info: bool,
        is_blank: bool,
        base_url: Option<DOMUrl>,
        allow_shadow: bool,
    ) -> Self {
        DocumentImpl {
            is_html,
            content_type,
            mode,
            origin,
            browsing_context: Some(browsing_context),
            policy,
            flags,
            time_info,
            is_blank,
            base_url,
            allow_shadow,
            custom_element: None,
            // FIXME: This is only for demo purpose
            url: String::from("https://example.com"),
            document_element: None,
        }
    }
}

impl HostDocument for WindowStates {
    fn new(&mut self) -> Result<Resource<Document>> {
        // FIXME: This is only for demo purpose
        // let element = Element::new(&mut self.store)?;
        // let document = Document::new(&mut self.store)?;
        // document
        //     .data_mut(&mut self.store)
        //     .as_document_mut()
        //     .document_element = Some(element);
        //
        // Ok(self.table.push(document)?)
        todo!()
    }

    fn drop(&mut self, rep: Resource<Document>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn url(&mut self, self_: Resource<Document>) -> Result<String> {
        let self_ = self.table.get(&self_)?;
        Ok(self_.url(&self.store))
    }

    fn document_element(&mut self, self_: Resource<Document>) -> Result<Option<Resource<Element>>> {
        let self_ = self.table.get(&self_)?;
        match self_.document_element(&self.store) {
            Some(e) => Ok(Some(self.table.push(e)?)),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Default)]
pub enum DocumentMode {
    #[default]
    NoQuirks,
    Quirks,
    LimitedQuirks,
}
