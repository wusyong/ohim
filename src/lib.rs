use ohim::dom::event;
use wasmtime::{
    AsContextMut, Result, RootScope,
    component::{Resource, ResourceTable, bindgen},
};

mod dom_object;

bindgen!({
    path: "./wit/event.wit",
    with: {
        "ohim:dom/event/event": DOMObject,
    }
});

pub type DOMObject = dom_object::DOMObject<EventImpl>;

pub struct EventImpl {
    type_: String,
}

pub struct Event<C: AsContextMut> {
    table: ResourceTable,
    scope: RootScope<C>,
}

impl<C: AsContextMut> event::HostEvent for Event<C> {
    fn new(&mut self, ty: String) -> Resource<DOMObject> {
        let data = DOMObject::new(&mut self.scope, EventImpl { type_: ty }).unwrap();
        self.table.push(data).unwrap()
    }

    fn drop(&mut self, rep: Resource<DOMObject>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn get_type(&mut self, self_: Resource<DOMObject>) -> String {
        let event = self.table.get(&self_).unwrap();
        event.data(&self.scope).unwrap().type_.to_string()
    }
}
