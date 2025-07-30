//! Ohim is Webassembly based web script engine.

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

use std::fmt::Debug;

pub use bindings::{Imports, ohim};

pub use dom::*;
use ohim::dom::node::Host;
use wasmtime::{Store, component::ResourceTable};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};

pub mod agent;
pub mod browsing_context;
pub mod dom;
pub mod navigible;
pub mod string;
pub mod url;

#[allow(missing_debug_implementations, missing_docs, unreachable_pub)]
mod bindings {
    pub use super::*;
    wasmtime::component::bindgen!({
        path: "wit",
        world: "ohim:dom/imports",
        with: {
            "ohim:dom/node/node": Node,
            "ohim:dom/node/document": Document,
            "ohim:dom/node/element": Element,
        },
        trappable_imports: true,
    });
}

/// `Store` states to use when `[Exposed=Window]`
pub struct WindowStates {
    table: ResourceTable,
    ctx: WasiCtx,
    store: Store<()>,
}

impl WindowStates {
    /// Create `WindowStates` data for initializing a new `Store`.
    pub fn create() -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtx::builder().inherit_stdout().build(),
            store: Store::<()>::default(),
        }
    }
}

impl Debug for WindowStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowStates")
            .field("table", &self.table)
            .field("store", &self.store)
            .finish()
    }
}

impl Host for WindowStates {}

impl IoView for WindowStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for WindowStates {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
