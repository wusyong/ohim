use ohim::dom::event::{Host, HostEvent};
use wasmtime::{
    Result, Store,
    component::{Resource, ResourceTable, bindgen},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};

mod dom_object;

bindgen!({
    path: "./wit/event.wit",
    with: {
        "ohim:dom/event/event": DOMObject,
    }
});

pub type DOMObject = dom_object::DOMObject<Event>;

pub struct Event {
    type_: String,
}

pub struct DOM {
    pub table: ResourceTable,
    pub ctx: WasiCtx,
    pub store: Store<()>,
}

impl HostEvent for DOM {
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

impl Host for DOM {}

impl IoView for DOM {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for DOM {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
