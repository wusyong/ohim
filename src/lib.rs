use ohim::dom::event::{Host, HostEvent};
use wasmtime::{
    Result, Store,
    component::{Resource, ResourceTable, bindgen},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};

mod dom_object;

bindgen!({
    path: "wit",
    with: {
        "ohim:dom/event/event": DOMObject,
    }
});

pub type DOMObject = dom_object::DOMObject<Event>;

pub struct Event {
    type_: String,
}

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

impl HostEvent for WindowStates {
    fn new(&mut self, ty: String) -> Resource<DOMObject> {
        let data = DOMObject::new(&mut self.store, Event { type_: ty }).unwrap();
        self.table.push(data).unwrap()
    }

    fn drop(&mut self, rep: Resource<DOMObject>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn get_type(&mut self, self_: Resource<DOMObject>) -> String {
        let event = self.table.get(&self_).unwrap();
        event.data(&self.store).unwrap().type_.to_string()
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
