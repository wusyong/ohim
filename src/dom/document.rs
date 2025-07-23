use std::{
    ops::Deref,
    sync::atomic::{AtomicBool, Ordering},
};

use headers::ContentType;
use wasmtime::{AsContext, AsContextMut, ExternRef, Result, Rooted, component::Resource};

use crate::{
    Element, NodeImpl, NodeTypeData, Object, WindowStates,
    agent::{NameSpace, RELEVANT_REALM, RealmID},
    browsing_context::{BrowsingContext, BrowsingContextID, SandboxingFlag},
    ohim::dom::node::HostDocument,
    url::{DOMUrl, ImmutableOrigin},
};

use super::{ElementLocal, Node};

/// <https://dom.spec.whatwg.org/#document>
#[derive(Clone, Debug)]
pub struct Document(pub(crate) Object<NodeImpl>);

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
        flags: SandboxingFlag,
        time_info: bool,
        is_blank: bool,
        base_url: Option<DOMUrl>,
        realm: RealmID,
        allow_shadow: bool,
        mut store: impl AsContextMut,
    ) -> Result<Self> {
        let document = Document(Object::new(
            &mut store,
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
                realm,
                allow_shadow,
            ))),
        )?);

        document
            .data_mut(&mut store)
            .set_node_document(Some(document.clone()));

        Ok(document)
    }

    /// <https://dom.spec.whatwg.org/#concept-document-origin>
    pub fn origin(&self, store: impl AsContext) -> ImmutableOrigin {
        self.data(&store).as_document().origin.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#concept-document-about-base-url>
    pub fn about_base_url(&self, store: impl AsContext) -> Option<DOMUrl> {
        self.data(&store).as_document().about_base_url.clone()
    }

    /// <https://dom.spec.whatwg.org/#dom-document-url>
    pub fn url(&self, store: impl AsContext) -> DOMUrl {
        self.data(&store).as_document().url.clone()
    }

    /// <https://dom.spec.whatwg.org/#dom-document-documentelement>
    pub fn document_element(&self, store: impl AsContext) -> Option<Element> {
        // The documentElement getter steps are to return this’s document element.
        self.data(&store).as_document().document_element.clone()
    }

    /// <https://html.spec.whatwg.org/multipage/#populate-with-html/head/body>
    pub fn populate_hhb(&self, mut store: impl AsContextMut) -> Result<()> {
        // 1. Let html be the result of creating an element given document, "html", and the HTML namespace.
        let html: Node =
            Element::new(self, ElementLocal::Html, NameSpace::HTML, None, &mut store)?.into();
        // 2. Let head be the result of creating an element given document, "head", and the HTML namespace.
        let head =
            Element::new(self, ElementLocal::Head, NameSpace::HTML, None, &mut store)?.into();
        // 3. Let body be the result of creating an element given document, "body", and the HTML namespace.
        let body =
            Element::new(self, ElementLocal::Body, NameSpace::HTML, None, &mut store)?.into();
        // 4. Append html to document.
        let document: Node = self.clone().into();
        document.pre_insert(html.clone(), None, &mut store);
        // 5. Append head to html.
        html.pre_insert(head, None, &mut store);
        // 6. Append body to html.
        html.pre_insert(body, None, &mut store);
        Ok(())
    }

    /// <https://html.spec.whatwg.org/multipage/#make-active>
    pub fn active(&self, context: &mut BrowsingContext, visibility: bool, store: impl AsContext) {
        let id = self.data(&store).as_document().realm;
        let mut window = None;
        if let Some(realm) = RELEVANT_REALM.lock().unwrap().get_mut(&id) {
            // 1. Let window be document's relevant global object.
            window = realm.global_object.clone();
            // 5. Set window's relevant settings object's execution ready flag.
            if let Some(env) = &mut realm.settings_object {
                env.ready = true;
            }
        };
        // 2. Set document's browsing context's WindowProxy's [[Window]] internal slot value to window.
        context.window = window;
        // 3. Set document's visibility state to document's node navigable's traversable navigable's system visibility state.
        self.data(&store)
            .as_document()
            .visibility
            .store(visibility, Ordering::Relaxed);
        // TODO: 4.Queue a new VisibilityStateEntry whose visibility state is document's visibility state and whose timestamp is zero.
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

    // /// Get `DocumentImpl` exclusive reference.
    // fn as_document_mut(&mut self) -> &mut DocumentImpl {
    //     let NodeTypeData::Document(ref mut doc) = self.data else {
    //         unreachable!()
    //     };
    //     doc
    // }
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
    _is_html: bool,
    /// <https://dom.spec.whatwg.org/#concept-document-content-type>
    _content_type: ContentType,
    /// <https://dom.spec.whatwg.org/#concept-document-mode>
    _mode: DocumentMode,
    /// <https://dom.spec.whatwg.org/#concept-document-origin>
    origin: ImmutableOrigin,
    /// <https://html.spec.whatwg.org/multipage/#concept-document-bc>
    _browsing_context: Option<BrowsingContextID>,
    /// <https://html.spec.whatwg.org/multipage/#concept-document-permissions-policy>
    _policy: bool,
    /// <https://html.spec.whatwg.org/multipage/browsers.html#active-sandboxing-flag-set>
    _flags: SandboxingFlag,
    /// <https://html.spec.whatwg.org/multipage/dom.html#load-timing-info>
    _time_info: bool,
    /// <https://html.spec.whatwg.org/multipage/dom.html#is-initial-about:blank>
    _is_blank: bool,
    /// <https://html.spec.whatwg.org/multipage/#concept-document-about-base-url>
    about_base_url: Option<DOMUrl>,
    /// <https://dom.spec.whatwg.org/#document-allow-declarative-shadow-roots>
    _allow_shadow: bool,
    /// <https://dom.spec.whatwg.org/#document-custom-element-registry>
    _custom_element: Option<bool>,
    /// <https://dom.spec.whatwg.org/#concept-document-url>
    url: DOMUrl,
    realm: RealmID,
    document_element: Option<Element>,
    visibility: AtomicBool,
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
        flags: SandboxingFlag,
        time_info: bool,
        is_blank: bool,
        base_url: Option<DOMUrl>,
        realm: RealmID,
        allow_shadow: bool,
    ) -> Self {
        DocumentImpl {
            _is_html: is_html,
            _content_type: content_type,
            _mode: mode,
            origin,
            _browsing_context: Some(browsing_context),
            _policy: policy,
            _flags: flags,
            _time_info: time_info,
            _is_blank: is_blank,
            about_base_url: base_url,
            _allow_shadow: allow_shadow,
            _custom_element: None,
            url: DOMUrl::parse("about:blank").unwrap(),
            realm,
            document_element: None,
            visibility: Default::default(),
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
        Ok(self_.url(&self.store).to_string())
    }

    fn document_element(&mut self, self_: Resource<Document>) -> Result<Option<Resource<Element>>> {
        let self_ = self.table.get(&self_)?;
        match self_.document_element(&self.store) {
            Some(e) => Ok(Some(self.table.push(e)?)),
            None => Ok(None),
        }
    }
}

/// <https://dom.spec.whatwg.org/#concept-document-mode>
#[derive(Debug, Default)]
pub enum DocumentMode {
    /// "no-quirks"
    #[default]
    NoQuirks,
    /// "quirks"
    Quirks,
    /// "limited-quirks"
    LimitedQuirks,
}
