use crate::event_target::IsEventTarget;

/// <https://dom.spec.whatwg.org/#event>
#[derive(Clone, Debug)]
pub struct Event {
    type_: String,
    target: Option<IsEventTarget>,
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
