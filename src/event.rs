use wasmtime::{Result, component::Resource};

// use crate::{EventTarget, WindowStates, ohim::dom::event::HostEvent};
use crate::EventTarget;

pub struct Event {
    type_: String,
    // TODO: Should be a enum type
    target: Option<EventTarget>,
}

// impl HostEvent for WindowStates {
//     fn new(&mut self, ty: String) -> Resource<Event> {
//         self.table
//             .push(Event {
//                 type_: ty,
//                 target: None,
//             })
//             .unwrap()
//     }
//
//     fn drop(&mut self, rep: Resource<Event>) -> Result<()> {
//         self.table.delete(rep)?;
//         Ok(())
//     }
//
//     fn get_type(&mut self, self_: Resource<Event>) -> String {
//         let event = self.table.get(&self_).unwrap();
//         event.type_.to_string()
//     }
// }
