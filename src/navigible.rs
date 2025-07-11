//! A navigable presents a Document to the user via its active session history entry.

use std::{
    collections::HashMap,
    ops::Deref,
    sync::{
        LazyLock,
        atomic::{AtomicUsize, Ordering},
    },
    usize,
};

use wasmtime::AsContextMut;

use crate::{
    Document, Element,
    browsing_context::{
        BrowsingContext, BrowsingContextGroup, BrowsingContextGroupID, BrowsingContextID,
    },
    url::{DOMUrl, ImmutableOrigin},
};

#[derive(Debug, Default)]
pub struct Traversable {
    history_entries: HashMap<SessionHistoryID, SessionHistory>,
}

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#navigable>
#[derive(Debug, Default)]
pub struct Navigable {
    id: NavigableID,
    parent: Option<NavigableID>,
    current_entry: Option<SessionHistoryID>,
    active_entry: Option<SessionHistoryID>,
    traversable: Option<Traversable>,
    // FIXME: These should go to user agent
    browsing_context: HashMap<BrowsingContextID, BrowsingContext>,
    browsing_context_group: HashMap<BrowsingContextGroupID, BrowsingContextGroup>,
}

impl Navigable {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#create-a-fresh-top-level-traversable>
    /// TODO: implement POST resource
    pub fn create_fresh_top_traversable(
        url: DOMUrl,
        resource: Option<bool>,
        store: impl AsContextMut,
    ) -> Self {
        // 1. Let traversable be the result of creating a new top-level traversable given null and the empty string.
        let traversable = Navigable::create_top_traversable(None, String::from(""), None, store);
        // 2. Navigate traversable to initialNavigationURL using traversable's active document,
        // with documentResource set to initialNavigationPostResource.
        todo!()
    }

    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-top-level-traversable>
    /// TODO: implement BrowsingContext
    pub fn create_top_traversable(
        opener: Option<bool>,
        target: String,
        navigable: Option<Navigable>,
        mut store: impl AsContextMut,
    ) -> Self {
        // 5. Let traversable be a new traversable navigable.
        let mut traversable = Self::default();
        // 1. Let document be null.
        let document = match opener {
            // 2. If opener is null, then set document to the second return value of creating a new top-level browsing
            // context and document.
            None => {
                let (group, context, document) =
                    BrowsingContext::new_top_browsing_context(&mut store);
                traversable.browsing_context_group.insert(group.id(), group);
                traversable.browsing_context.insert(context.id(), context);
                document
            }
            // 3. Otherwise, set document to the second return value of creating a new auxiliary browsing context and
            // document given opener.
            Some(_) => {
                todo!()
            }
        };
        // 4. Let documentState be a new document state
        let url = document.url(&store);
        let state = DocumentState {
            // TODO: null if opener is null; otherwise, document's origin
            initiator_origin: None,
            origin: Some(document.origin(&store)),
            target,
            about_base_url: document.about_base_url(&store),
            document: Some(document),
        };
        // 6. Initialize the navigable traversable given documentState.
        // 7. Let initialHistoryEntry be traversable's active session history entry.
        let mut initial_entry = traversable.initialize(state, url, None);
        // 8. Set initialHistoryEntry's step to 0.
        initial_entry.step = Some(0);
        // 9. Append initialHistoryEntry to traversable's session history entries.
        traversable.traversable = Some(Traversable::default());
        traversable
            .traversable
            .as_mut()
            .unwrap()
            .history_entries
            .insert(initial_entry.id, initial_entry);
        // 10. TODO: If opener is non-null, then legacy-clone a traversable storage shed given opener's
        // top-level traversable and traversable.
        // 11. TODO: Append traversable to the user agent's top-level traversable set.
        // 12. TODO: Invoke WebDriver BiDi navigable created with traversable and openerNavigableForWebDriver.

        // 13. Return traversable.
        traversable
    }

    /// <https://html.spec.whatwg.org/multipage/#initialize-the-navigable>
    fn initialize(
        &mut self,
        state: DocumentState,
        url: DOMUrl,
        parent: Option<NavigableID>,
    ) -> SessionHistory {
        // 2. Let entry be a new session history entry
        let entry = SessionHistory {
            id: SessionHistoryID::default(),
            step: None,
            url,
            state,
        };
        // 3. Set navigable's current session history entry to entry.
        self.current_entry = Some(entry.id);
        // 4. Set navigable's active session history entry to entry.
        self.active_entry = Some(entry.id);
        // 5. Set navigable's parent to parent.
        self.parent = parent;
        entry
    }

    /// <https://html.spec.whatwg.org/multipage/#navigate>
    /// TODO: response, navigationAPIState, formDataEntryList, userInvolvement
    #[allow(clippy::too_many_arguments)]
    pub fn navigate(
        &self,
        url: DOMUrl,
        documet: Option<Document>,
        resource: Option<bool>,
        response: Option<bool>,
        exception: bool,
        history_handling: NavigationHistoryBehavior,
        api_state: Option<bool>,
        entry_list: Option<bool>,
        referer_policy: ReferrerPolicy,
        involvement: Option<bool>,
        element: Option<Element>,
        initialInsertion: bool,
    ) {
    }
}

/// ID of `Navigable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NavigableID(pub usize);

impl Default for NavigableID {
    fn default() -> Self {
        static COUNT: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
        let id = Self(COUNT.load(Ordering::Relaxed));
        COUNT.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl Deref for NavigableID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// <https://html.spec.whatwg.org/multipage/#session-history-entry>
#[derive(Clone, Debug)]
pub struct SessionHistory {
    pub id: SessionHistoryID,
    /// <https://html.spec.whatwg.org/multipage/#she-step>
    pub step: Option<usize>,
    /// <https://html.spec.whatwg.org/multipage/#she-url>
    pub url: DOMUrl,
    /// <https://html.spec.whatwg.org/multipage/#she-document-state>
    pub state: DocumentState,
}

/// ID of `SessionHistory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionHistoryID(pub usize);

impl Default for SessionHistoryID {
    fn default() -> Self {
        static COUNT: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
        let id = Self(COUNT.load(Ordering::Relaxed));
        COUNT.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl Deref for SessionHistoryID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// <https://html.spec.whatwg.org/multipage/browsing-the-web.html#document-state-2>
#[derive(Clone, Debug, Default)]
pub struct DocumentState {
    /// <https://html.spec.whatwg.org/multipage/#document-state-document>
    pub document: Option<Document>,
    /// <https://html.spec.whatwg.org/multipage/#document-state-initiator-origin>
    pub initiator_origin: Option<ImmutableOrigin>,
    /// <https://html.spec.whatwg.org/multipage/#document-state-origin>
    pub origin: Option<ImmutableOrigin>,
    /// <https://html.spec.whatwg.org/multipage/#document-state-nav-target-name>
    pub target: String,
    /// <https://html.spec.whatwg.org/multipage/#document-state-about-base-url>
    pub about_base_url: Option<DOMUrl>,
}

/// <https://html.spec.whatwg.org/multipage/#navigationhistorybehavior>
/// FIXME: Should move to related DOM module
#[derive(Clone, Copy, Debug, Default)]
pub enum NavigationHistoryBehavior {
    /// "auto"
    #[default]
    Auto,
    /// "push"
    Push,
    /// "replace"
    Replace,
}

/// <https://w3c.github.io/webappsec-referrer-policy/#referrer-policy>
/// FIXME: Should move to related DOM module
#[derive(Clone, Copy, Debug, Default)]
pub enum ReferrerPolicy {
    /// ""
    None,
    /// "no-referrer"
    NoReferrer,
    /// "no-referrer-when-downgrade"
    NoReferrerWhenDowngrade,
    /// "same-origin"
    SameOrigin,
    /// "origin"
    Origin,
    /// "strict-origin"
    StrictOrigin,
    /// "origin-when-cross-origin"
    OriginWhenCrossOrigin,
    /// "strict-origin-when-cross-origin"
    #[default]
    StrictOriginWhenCrossOrigin,
    /// "unsafe-url"
    UnsafeUrl,
}
