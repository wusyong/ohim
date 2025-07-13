//! User-Agent related types

use std::{
    collections::HashMap,
    ops::Deref,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use crate::{Window, WindowProxy, browsing_context::IsolationMode};

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
        }
    }

    /// Get the ID of `Agent`
    pub fn id(&self) -> RealmID {
        self.id
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
