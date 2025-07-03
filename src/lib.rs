pub use event::Event;
pub use event_target::EventTarget;
pub use node::Node;
use ohim::dom::event::Host;
use wasmtime::{
    Store,
    component::{ResourceTable, bindgen},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};

mod event;
mod event_target;
mod node;
mod object;

bindgen!({
    path: "wit",
    with: {
        "ohim:dom/event/event": Event,
        "ohim:dom/event-target/event-target": EventTarget,
        "ohim:dom/node/node": Node,
    }
});

/// `Store` states to use when `[Exposed=Window]`
pub struct WindowStates {
    table: ResourceTable,
    ctx: WasiCtx,
    store: Store<()>,
}

impl WindowStates {
    pub fn create() -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtx::builder().build(),
            store: Store::<()>::default(),
        }
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
