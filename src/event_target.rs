use std::collections::HashMap;

use wasmtime::{AsContext, AsContextMut, Result, component::Resource};

use crate::{Event, WindowStates, ohim::dom::event_target::HostEventTarget};

pub struct EventTarget {
    callbacks: Option<HashMap<String, EventListener>>,
}

impl EventTarget {
    pub fn new() -> Self {
        Self { callbacks: None }
    }
}

impl EventTarget {
    fn add_listener(&mut self, ty: String, callback: EventListener, store: impl AsContextMut) {}
    fn remove_listener(&mut self, ty: String, callback: EventListener, store: impl AsContextMut) {}
    fn dispatch(&self, event: Event) {}
}

/// For Object type to implement
pub trait EventTargetMethods {
    fn add_event_listener(&mut self, ty: String, callback: EventListener, store: impl AsContextMut);
    fn remove_event_listener(
        &mut self,
        ty: String,
        callback: EventListener,
        store: impl AsContextMut,
    );
    fn dispatch_event(&self, store: impl AsContext);
}

/// <https://dom.spec.whatwg.org/#callbackdef-eventlistener>
pub struct EventListener(Box<dyn FnMut(Event) + Send + Sync>);

impl EventListener {
    fn call(&mut self, event: Event) {
        self.0(event)
    }
}

// impl EventTargetMethods
//     fn add_event_listener(&mut self, ty: String, callback: String) {
//         let callbacks = self.callbacks.get_or_insert_default();
//         callbacks
//             .entry(ty)
//             .and_modify(|v| v.push(callback.clone()))
//             .or_insert(vec![callback]);
//     }
// }
//
// impl HostEventTarget for WindowStates {
//     fn new(&mut self) -> Resource<EventTarget> {
//         let target = EventTarget::new();
//         self.table.push(target).unwrap()
//     }
//
//     fn add_event_listener(&mut self, self_: Resource<EventTarget>, ty: String, callback: String) {
//         let target = self.table.get_mut(&self_).unwrap();
//         target.add_event_listener(ty, callback);
//     }
//
//     fn drop(&mut self, rep: Resource<EventTarget>) -> Result<()> {
//         self.table.delete(rep)?;
//         Ok(())
//     }
// }
