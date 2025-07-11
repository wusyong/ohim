//! A browsing context is a programmatic representation of a series of documents, multiple of which can live within a single navigable.

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::{
        LazyLock,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
    u16,
};

use headers::ContentType;
use url::Url;
use wasmtime::AsContextMut;

use crate::{
    Document, DocumentMode,
    url::{DOMUrl, ImmutableOrigin},
    user_agent::{Agent, AgentCluster},
};

/// <https://html.spec.whatwg.org/multipage/document-sequences.html#browsing-context>
#[derive(Debug)]
pub struct BrowsingContext {
    id: BrowsingContextID,
    group: Option<BrowsingContextGroupID>,
}

impl BrowsingContext {
    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-top-level-browsing-context>
    pub fn new_top_browsing_context(
        store: impl AsContextMut,
    ) -> (BrowsingContextGroup, Self, Document) {
        // 1. Let group and document be the result of creating a new browsing context group and document.
        BrowsingContextGroup::new_browsing_context_group_and_document(store)
    }

    /// <https://html.spec.whatwg.org/multipage/document-sequences.html#creating-a-new-browsing-context>
    /// TODO: implement embedder
    pub fn new_browsing_context(
        creator: Option<Document>,
        embedder: Option<bool>,
        group: &mut BrowsingContextGroup,
        store: impl AsContextMut,
    ) -> (Self, Document) {
        // 1. Let browsingContext be a new browsing context.
        let context = BrowsingContext {
            id: BrowsingContextID::default(),
            group: None,
        };
        // 2. Let unsafeContextCreationTime be the unsafe shared current time.
        let time = Instant::now();
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
        // TODO: step 10 ~13

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
            store,
        )
        .expect("Failed to create document");
        // 16. TODO: If creator is non-null, then:
        // 17 TODO: Assert: document's URL and document's relevant settings object's creation URL are about:blank.
        // 18. TODO: Mark document as ready for post-load tasks.
        // TODO: 19~21
        (context, document)
    }

    /// <https://html.spec.whatwg.org/multipage/browsers.html#determining-the-creation-sandboxing-flags>
    /// TODO: Implement sandbox flags
    pub fn determine_creation_sandbox_flags(&self, embedder: &Option<bool>) -> bool {
        false
    }

    /// Get the ID of the BrowsingContext.
    pub fn id(&self) -> BrowsingContextID {
        self.id
    }
}

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
    ) -> (Self, BrowsingContext, Document) {
        // 1. Let group be a new browsing context group.
        let mut group = BrowsingContextGroup::default();
        // 2. Append group to the user agent's browsing context group set.
        // This is done by returning Self.
        // 3. Let browsingContext and document be the result of creating a new browsing context and document with null,
        // null, and group.
        let (mut context, document) =
            BrowsingContext::new_browsing_context(None, None, &mut group, store);
        // 4. Append browsingContext to group.
        group.browsing_context.insert(context.id());
        context.group = Some(group.id());
        // 5. Return group and document.
        (group, context, document)
    }

    /// <https://html.spec.whatwg.org/multipage/#obtain-similar-origin-window-agent>
    pub fn window_agent(&mut self, origin: &ImmutableOrigin, oac: bool) -> &Agent {
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
                ..Default::default()
            };
            // 6.4. TODO: Add the result of creating an agent, given false, to agentCluster.
            // 6.5. Set group's agent cluster map[key] to agentCluster.
            self.agent_cluster.insert(key.clone(), agent_cluster);
        }
        // 7. Return the single similar-origin window agent contained in group's agent cluster map[key].
        &self.agent_cluster.get(key).unwrap().agent
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

/// <https://html.spec.whatwg.org/multipage/#determining-the-origin>
pub fn determin_origin(
    url: Option<&DOMUrl>,
    flags: bool,
    origin: Option<ImmutableOrigin>,
) -> ImmutableOrigin {
    // 1. If sandboxFlags has its sandboxed origin browsing context flag set, then return a new opaque origin.
    if flags {
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
