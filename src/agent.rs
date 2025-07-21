//! User-Agent related types

use std::{
    collections::HashMap,
    ops::Deref,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use crate::{
    Window, WindowProxy,
    browsing_context::{self, BrowsingContextID, IsolationMode},
    url::{DOMUrl, ImmutableOrigin},
};

/// <https://tc39.es/ecma262/#sec-agent-clusters>
#[derive(Debug, Default)]
pub struct AgentCluster {
    /// <https://html.spec.whatwg.org/multipage/#agent-cluster-cross-origin-isolation>
    pub isolation_mode: IsolationMode,
    /// <https://html.spec.whatwg.org/multipage/#is-origin-keyed>
    pub origin_keyed: bool,
    pub agent: AgentID,
}

/// <https://tc39.es/ecma262/#sec-agents>
#[derive(Debug, Default)]
pub struct Agent {
    id: AgentID,
    block: bool,
}

impl Agent {
    /// <https://html.spec.whatwg.org/multipage/#create-an-agent>
    pub fn create(block: bool) -> AgentID {
        let id = AgentID::default();
        let agent = Self { id, block };
        RELEVANT_AGENT.lock().unwrap().insert(id, agent);
        id
    }

    /// Get the ID of `Agent`
    pub fn id(&self) -> AgentID {
        self.id
    }
}

/// <https://html.spec.whatwg.org/multipage/#relevant-agent>
static RELEVANT_AGENT: LazyLock<Arc<Mutex<HashMap<AgentID, Agent>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/// ID of `Agent`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentID(pub usize);

impl Default for AgentID {
    fn default() -> Self {
        static COUNT: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
        let id = Self(COUNT.load(Ordering::Relaxed));
        COUNT.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl Deref for AgentID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// <https://tc39.es/ecma262/#sec-code-realms>
#[derive(Debug, Default)]
pub struct Realm {
    id: RealmID,
    agent: AgentID,
    global_object: Option<Window>,
    global_this: Option<WindowProxy>,
    settings_object: Option<Environment>,
}

impl Realm {
    /// <https://html.spec.whatwg.org/multipage/#create-an-agent>
    ///
    /// # Note
    /// This returns `Realm` because there are more steps outside of this method to complete. Please
    /// make sure to run those steps and insert the `Realm` into `RELEVANT_REALM`
    pub fn create(
        agent: AgentID,
        global_object: Option<Window>,
        global_this: Option<WindowProxy>,
    ) -> Realm {
        let id = RealmID::default();
        Self {
            id,
            agent,
            global_object,
            global_this,
            settings_object: None,
        }
    }

    /// Get the ID of `Agent`
    pub fn id(&self) -> RealmID {
        self.id
    }

    /// <https://html.spec.whatwg.org/multipage/#set-up-a-window-environment-settings-object>
    pub fn set_window_settings_object(
        mut self,
        creation_url: DOMUrl,
        top_url: DOMUrl,
        top_origin: ImmutableOrigin,
        environment: Option<Environment>,
    ) {
        let (id, browsing_context) = match environment {
            // 4. If reservedEnvironment is non-null, then:
            Some(e) => (e.id, e.browsing_context),
            // 5. Otherwise, set settings object's id to a new unique opaque string, settings object's target
            // browsing context to null, and settings object's active service worker to null.
            None => (EnvironmentID::default(), None),
        };
        // 6. Set settings object's creation URL to creationURL, settings object's top-level creation URL to
        // topLevelCreationURL, and settings object's top-level origin to topLevelOrigin.
        let settings_object = Environment {
            id,
            creation_url,
            top_url: Some(top_url),
            top_origin: Some(top_origin),
            browsing_context,
            ready: false,
        };
        // 7. Set realm's [[HostDefined]] field to settings object.
        self.settings_object = Some(settings_object);
        let id = self.id;
        RELEVANT_REALM.lock().unwrap().insert(id, self);
    }
}

/// <https://html.spec.whatwg.org/multipage/#concept-relevant-realm>
static RELEVANT_REALM: LazyLock<Arc<Mutex<HashMap<RealmID, Realm>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/// ID of `Realm`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RealmID(pub usize);

impl Default for RealmID {
    fn default() -> Self {
        static COUNT: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
        let id = Self(COUNT.load(Ordering::Relaxed));
        COUNT.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl Deref for RealmID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// <https://html.spec.whatwg.org/multipage/#environment>
#[derive(Debug)]
pub struct Environment {
    id: EnvironmentID,
    creation_url: DOMUrl,
    top_url: Option<DOMUrl>,
    top_origin: Option<ImmutableOrigin>,
    browsing_context: Option<BrowsingContextID>,
    ready: bool,
    // TODO: An active service worker
}

/// ID of `Environment`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvironmentID(pub usize);

impl Default for EnvironmentID {
    fn default() -> Self {
        static COUNT: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));
        let id = Self(COUNT.load(Ordering::Relaxed));
        COUNT.fetch_add(1, Ordering::Relaxed);
        id
    }
}

impl Deref for EnvironmentID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// <https://infra.spec.whatwg.org/#namespaces>
#[derive(Clone, Copy, Debug)]
pub enum NameSpace {
    /// <https://infra.spec.whatwg.org/#html-namespace>
    HTML,
    /// None
    None,
}
