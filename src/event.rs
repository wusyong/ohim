use wasmtime::{Result, component::Resource};

use crate::{WindowStates, object::Object, ohim::dom::event::HostEvent};

pub struct Event {
    type_: String,
}

impl HostEvent for WindowStates {
    fn new(&mut self, ty: String) -> Resource<Object<Event>> {
        let data = Object::new(&mut self.store, Event { type_: ty }).unwrap();
        self.table.push(data).unwrap()
    }

    fn drop(&mut self, rep: Resource<Object<Event>>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn get_type(&mut self, self_: Resource<Object<Event>>) -> String {
        let event = self.table.get(&self_).unwrap();
        event.data(&self.store).unwrap().type_.to_string()
    }
}
