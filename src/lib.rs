use ohim::dom::event;
use wasmtime::{
    AsContextMut, ExternRef, Result, RootScope, Rooted,
    component::{Resource, ResourceTable, bindgen},
};

bindgen!({
    path: "./wit/event.wit",
    with: {
        "ohim:dom/event/event": RootedRef,
    }
});

pub type RootedRef = Rooted<ExternRef>;

pub struct Event {
    ty: String,
}

pub struct EventHost<C: AsContextMut> {
    table: ResourceTable,
    scope: RootScope<C>,
}

impl<C: AsContextMut> event::HostEvent for EventHost<C> {
    fn new(&mut self, ty: String) -> Resource<RootedRef> {
        let data = ExternRef::new(&mut self.scope, Event { ty }).unwrap();
        self.table.push(data).unwrap()
    }

    fn drop(&mut self, rep: Resource<RootedRef>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn ty(&mut self, self_: Resource<RootedRef>) -> String {
        let event = self.table.get(&self_).unwrap();
        event
            .data(&self.scope)
            .unwrap()
            .unwrap()
            .downcast_ref::<&Event>()
            .unwrap()
            .ty
            .to_string()
    }
}
