//! User-Agent related types

use crate::browsing_context::IsolationMode;

/// <https://tc39.es/ecma262/#sec-agent-clusters>
#[derive(Debug, Default)]
pub struct AgentCluster {
    pub isolation_mode: IsolationMode,
    pub origin_keyed: bool,
    pub agent: Agent,
}

/// <https://tc39.es/ecma262/#sec-agents>
#[derive(Debug, Default)]
pub struct Agent {}
