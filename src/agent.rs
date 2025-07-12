//! User-Agent related types

use crate::browsing_context::IsolationMode;

/// <https://tc39.es/ecma262/#sec-agent-clusters>
#[derive(Debug, Default)]
pub struct AgentCluster {
    /// <https://html.spec.whatwg.org/multipage/#agent-cluster-cross-origin-isolation>
    pub isolation_mode: IsolationMode,
    /// <https://html.spec.whatwg.org/multipage/#is-origin-keyed>
    pub origin_keyed: bool,
    pub agent: Agent,
}

/// <https://tc39.es/ecma262/#sec-agents>
#[derive(Debug, Default)]
pub struct Agent {}

/// <https://tc39.es/ecma262/#sec-code-realms>
#[derive(Debug, Default)]
pub struct Realm {}
