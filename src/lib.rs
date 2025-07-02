use event::Event;
use object::Object;
use ohim::dom::event::Host;
use wasmtime::{
    Store,
    component::{ResourceTable, bindgen},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};

mod event;
mod object;

bindgen!({
    path: "wit",
    with: {
        "ohim:dom/event/event": EventObject,
    }
});

pub type EventObject = Object<Event>;

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
