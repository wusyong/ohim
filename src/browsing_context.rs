//! A browsing context is a programmatic representation of a series of documents, multiple of which can live within a single navigable.

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
};

use bitflags::bitflags;
use headers::ContentType;
use wasmtime::AsContextMut;

use crate::{
    Document, DocumentMode, Window, WindowProxy,
    agent::{Agent, AgentCluster, AgentID, Realm},
    url::{DOMUrl, ImmutableOrigin},
};

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#browsing-context>
#[derive(Debug)]
pub struct BrowsingContext {
    id: BrowsingContextID,
    group: Option<BrowsingContextGroupID>,
    /// <https://html.spec.whatwg.org/multipage/#popup-sandboxing-flag-set>
    popup_flag: SandboxingFlag,
}

/// <https://html.spec.whatwg.org/multipage/#browsing-context-set>
static BROWSING_CONTEXT_SET: LazyLock<Arc<Mutex<HashMap<BrowsingContextID, BrowsingContext>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

impl BrowsingContext {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-top-level-browsing-context>
    pub fn new_top_browsing_context(store: impl AsContextMut) -> (BrowsingContextID, Document) {
        // 1. Let group and document be the result of creating a new browsing context group and document.
        let (context, document) =
            BrowsingContextGroup::new_browsing_context_group_and_document(store);
        let id = context.id();
        BROWSING_CONTEXT_SET.lock().unwrap().insert(id, context);

        (id, document)
    }

    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-browsing-context>
    /// TODO: implement embedder
    pub fn new_browsing_context(
        _creator: Option<Document>,
        embedder: Option<bool>,
        group: &mut BrowsingContextGroup,
        mut store: impl AsContextMut,
    ) -> (Self, Document) {
        // 1. Let browsingContext be a new browsing context.
        let context = BrowsingContext {
            id: BrowsingContextID::default(),
            group: None,
            popup_flag: SandboxingFlag::empty(),
        };
        // 2. Let unsafeContextCreationTime be the unsafe shared current time.
        let _time = Instant::now();
        // 3. Let creatorOrigin be null.
        let creator_origin: Option<ImmutableOrigin> = None;
        // 4. Let creatorBaseURL be null.
        let creator_url: Option<DOMUrl> = None;
        // 5. TODO: If creator is non-null, then:

        // 6. Let sandboxFlags be the result of determining the creation sandboxing flags given browsingContext and
        // embedder.
        let flags = context.determine_creation_sandbox_flags(&embedder);
        // 7. Let origin be the result of determining the origin given about:blank, sandboxFlags, and creatorOrigin.
        let origin = determin_origin(
            Some(&DOMUrl::parse("about:blank").unwrap()),
            flags,
            creator_origin,
        );
        // 8. TODO: Let permissionsPolicy be the result of creating a permissions policy given embedder and origin.
        let policy = false;
        // 9. Let agent be the result of obtaining a similar-origin window agent given origin, group, and false.
        let agent = group.window_agent(&origin, false);
        // 10. Let realm execution context be the result of creating a new realm given agent and the following customizations:
        let realm = Realm::create(
            agent,
            Some(Window::new(&mut store).expect("Failed to create window")),
            Some(WindowProxy {}),
        );
        // 11. Let topLevelCreationURL be about:blank if embedder is null; TODO: otherwise embedder's relevant settings
        // object's top-level creation URL.
        let top_url = DOMUrl::parse("about:blank").unwrap();
        // 12. Let topLevelOrigin be origin if embedder is null; TODO: otherwise embedder's relevant settings object's top-level origin.
        let top_origin = origin.clone();
        // 13. Set up a window environment settings object with about:blank, realm execution context, null,
        // topLevelCreationURL, and topLevelOrigin.
        realm.set_window_settings_object(
            DOMUrl::parse("about:blank").unwrap(),
            top_url,
            top_origin,
            None,
        );

        // 14. Let loadTimingInfo be a new document load timing info with its navigation start time set to the result
        // of calling coarsen time with unsafeContextCreationTime and the new environment settings object's
        // cross-origin isolated capability.
        // TODO: implement load time info
        let load_time_info = false;
        // 15. Let document be a new Document
        let document = Document::new(
            true,
            ContentType::html(),
            DocumentMode::Quirks,
            origin,
            context.id(),
            policy,
            flags,
            load_time_info,
            true,
            creator_url,
            true,
            // TODO: Define CustomElementRegistry
            &mut store,
        )
        .expect("Failed to create document");
        // 16. TODO: If creator is non-null, then:
        // 18. Mark document as ready for post-load tasks.
        // XXX: Unimplemented because this is only used for printing.

        // 19. Populate with html/head/body given document.
        document
            .populate_hhb(&mut store)
            .expect("Failed to create Elements");
        // TODO: 20~21
        // 22. Return browsingContext and document.
        (context, document)
    }

    /// <https://html.spec.whatwg.org/multipage/browsers.html#determining-the-creation-sandboxing-flags>
    pub fn determine_creation_sandbox_flags(&self, embedder: &Option<bool>) -> SandboxingFlag {
        match embedder {
            // If embedder is null, then: the flags set on browsing context's popup sandboxing flag set.
            None => self.popup_flag,
            // TODO: If embedder is an element, then: the flags set on embedder's iframe sandboxing flag set.
            // If embedder is an element, then: the flags set on embedder's node document's active sandboxing flag set.
            Some(_) => SandboxingFlag::empty(),
        }
    }

    /// Get the ID of the BrowsingContext.
    pub fn id(&self) -> BrowsingContextID {
        self.id
    }
}

/// <https://html.spec.whatwg.org/multipage/#browsing-context-group-set>
static BROWSING_CONTEXT_GROUP_SET: LazyLock<
    Arc<Mutex<HashMap<BrowsingContextGroupID, BrowsingContextGroup>>>,
> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#browsing-context-group>
#[derive(Debug, Default)]
pub struct BrowsingContextGroup {
    id: BrowsingContextGroupID,
    browsing_context: HashSet<BrowsingContextID>,
    agent_cluster: HashMap<ImmutableOrigin, AgentCluster>,
    historical_agent_cluster: HashMap<ImmutableOrigin, ImmutableOrigin>,
    isolation_mode: IsolationMode,
}

impl BrowsingContextGroup {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-browsing-context-group-and-document>
    pub fn new_browsing_context_group_and_document(
        store: impl AsContextMut,
    ) -> (BrowsingContext, Document) {
        // 1. Let group be a new browsing context group.
        let mut group = BrowsingContextGroup::default();
        // 3. Let browsingContext and document be the result of creating a new browsing context and document with null,
        // null, and group.
        let (mut context, document) =
            BrowsingContext::new_browsing_context(None, None, &mut group, store);
        // 4. Append browsingContext to group.
        group.browsing_context.insert(context.id());
        context.group = Some(group.id());
        // 2. Append group to the user agent's browsing context group set.
        let id = group.id();
        BROWSING_CONTEXT_GROUP_SET.lock().unwrap().insert(id, group);
        // 5. Return group and document.
        (context, document)
    }

    /// <https://html.spec.whatwg.org/multipage/#obtain-similar-origin-window-agent>
    pub fn window_agent(&mut self, origin: &ImmutableOrigin, oac: bool) -> AgentID {
        // 3. If group's cross-origin isolation mode is not "none", then set key to origin.
        let key = if self.isolation_mode == IsolationMode::None {
            origin
            // 4. Otherwise, if group's historical agent cluster key map[origin] exists,
            // then set key to group's historical agent cluster key map[origin].
        } else if let Some(k) = self.historical_agent_cluster.get(origin) {
            k
        } else {
            // 5.1 If requestsOAC is true, then set key to origin.
            let k = if oac {
                origin.clone()
            } else {
                // 1. Let site be the result of obtaining a site with origin.
                // 2. Let key be site.
                obtain_site(origin)
            };
            // 5.2 Set group's historical agent cluster key map[origin] to key.
            self.historical_agent_cluster.insert(origin.clone(), k);
            self.historical_agent_cluster.get(origin).unwrap()
        };

        // 6. If group's agent cluster map[key] does not exist, then:
        if !self.agent_cluster.contains_key(key) {
            // 6.1. Let agentCluster be a new agent cluster.
            let agent_cluster = AgentCluster {
                // 6.2. Set agentCluster's cross-origin isolation mode to group's cross-origin isolation mode.
                isolation_mode: self.isolation_mode,
                // 6.3. If key is an origin: Set agentCluster's is origin-keyed to true.
                origin_keyed: key == origin,
                // 6.4. Add the result of creating an agent, given false, to agentCluster.
                agent: Agent::create(false),
            };
            // 6.5. Set group's agent cluster map[key] to agentCluster.
            self.agent_cluster.insert(key.clone(), agent_cluster);
        }
        // 7. Return the single similar-origin window agent contained in group's agent cluster map[key].
        self.agent_cluster.get(key).unwrap().agent
    }

    /// Get the ID of the `BrowsingContextGroup`.
    pub fn id(&self) -> BrowsingContextGroupID {
        self.id
    }
}

/// ID of `BrowsingContext`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BrowsingContextID(pub usize);

impl Default for BrowsingContextID {
    fn default() -> Self {
        static COUNT: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
        let id = Self(COUNT.load(Ordering::Relaxed));
        COUNT.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl Deref for BrowsingContextID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// ID of `BrowsingContext`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BrowsingContextGroupID(pub usize);

impl Default for BrowsingContextGroupID {
    fn default() -> Self {
        static COUNT: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
        let id = Self(COUNT.load(Ordering::Relaxed));
        COUNT.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl Deref for BrowsingContextGroupID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#cross-origin-isolation-mode>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IsolationMode {
    /// none
    #[default]
    None,
    /// logical
    Logical,
    /// concrete
    Concrete,
}

bitflags! {
    /// <https://html.spec.whatwg.org/multipage/#sandboxing-flag-set>
    #[derive(Clone, Copy, Debug)]
    pub struct SandboxingFlag: u32 {
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-navigation-browsing-context-flag>
        const NAVIGATION_BROWSING_CONTEXT = 1;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-auxiliary-navigation-browsing-context-flag>
        const AUXILIARY_NAVIGATION_BROWSING_CONTEXT = 1 << 1;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-top-level-navigation-without-user-activation-browsing-context-flag>
        const TOP_LEVEL_NAVIGATION_WITHOUT_USER_ACTIVATION_BROWSING_CONTEXT = 1 << 2;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-top-level-navigation-with-user-activation-browsing-context-flag>
        const TOP_LEVEL_NAVIGATION_WITH_USER_ACTIVATION_BROWSING_CONTEXT = 1 << 3;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-origin-browsing-context-flag>
        const ORIGIN_BROWSING_CONTEXT = 1 << 4;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-forms-browsing-context-flag>
        const FORMS_BROWSING_CONTEXT = 1 << 5;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-pointer-lock-browsing-context-flag>
        const POINTER_LOCK_BROWSING_CONTEXT = 1 << 6;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-scripts-browsing-context-flag>
        const SCRIPTS_BROWSING_CONTEXT = 1 << 7;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-automatic-features-browsing-context-flag>
        const AUTOMATIC_FEATURES_BROWSING_CONTEXT = 1 << 8;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-document.domain-browsing-context-flag>
        const DOCUMENT_DOMAIN_BROWSING_CONTEXT = 1 << 9;
        /// <https://html.spec.whatwg.org/multipage/#sandbox-propagates-to-auxiliary-browsing-contexts-flag>
        const PROPAGATES_TO_AUXILIARY_BROWSING_CONTEXT = 1 << 10;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-modals-flag>
        const MODALS = 1 << 11;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-orientation-lock-browsing-context-flag>
        const ORIENTATION_LOCK_BROWSING_CONTEXT = 1 << 12;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-presentation-browsing-context-flag>
        const PRESENTATION_BROWSING_CONTEXT = 1 << 13;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-downloads-browsing-context-flag>
        const DOWNLOADS_BROWSING_CONTEXT = 1 << 14;
        /// <https://html.spec.whatwg.org/multipage/#sandboxed-custom-protocols-navigation-browsing-context-flag>
        const CUSTOM_PROTOCOLS_NAVIGATION_BROWSING_CONTEXT = 1 << 15;

    }
}

/// <https://html.spec.whatwg.org/multipage/#determining-the-origin>
pub fn determin_origin(
    url: Option<&DOMUrl>,
    flags: SandboxingFlag,
    origin: Option<ImmutableOrigin>,
) -> ImmutableOrigin {
    // 1. If sandboxFlags has its sandboxed origin browsing context flag set, then return a new opaque origin.
    if flags.contains(SandboxingFlag::ORIGIN_BROWSING_CONTEXT) {
        return ImmutableOrigin::new_opaque();
    }
    match (url, origin) {
        // 2. If url is null, then return a new opaque origin.
        (None, _) => ImmutableOrigin::new_opaque(),
        // 3. If url is about:srcdoc, then:
        // 4. If url matches about:blank and sourceOrigin is non-null, then return sourceOrigin.
        (Some(u), Some(o)) => {
            // TODO: Implement matches URL
            if u.as_str() == "about:srcdoc" || u.as_str() == "about:blank" {
                o
            } else {
                // 5. Return url's origin.
                u.origin()
            }
        }
        // 5. Return url's origin.
        (Some(u), None) => u.origin(),
    }
}

/// <https://html.spec.whatwg.org/multipage/#obtain-a-site>
pub fn obtain_site(origin: &ImmutableOrigin) -> ImmutableOrigin {
    // 1. If origin is an opaque origin, then return origin.
    match origin {
        ImmutableOrigin::Opaque(_) => origin.clone(),
        ImmutableOrigin::Tuple(scheme, host, _) => {
            // 2. If origin's host's registrable domain is null, then return (origin's scheme, origin's host).
            // 3. Return (origin's scheme, origin's host's registrable domain).
            // TODO: implement registrable_domain (This requires a list of public domain)
            ImmutableOrigin::Tuple(scheme.clone(), host.clone(), u16::MAX)
        }
    }
}
