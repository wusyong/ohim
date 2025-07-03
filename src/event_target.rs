use std::collections::HashMap;

use wasmtime::{Result, component::Resource};

use crate::{WindowStates, ohim::dom::event_target::HostEventTarget};

pub struct EventTarget {
    callbacks: HashMap<String, Vec<String>>,
}

impl EventTarget {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::default(),
        }
    }
}

pub trait EventTargetMethods {
    fn add_event_listener(&mut self, ty: String, callback: String);
}

impl EventTargetMethods for EventTarget {
    fn add_event_listener(&mut self, ty: String, callback: String) {
        self.callbacks
            .entry(ty)
            .and_modify(|v| v.push(callback.clone()))
            .or_insert(vec![callback]);
    }
}

impl HostEventTarget for WindowStates {
    fn new(&mut self) -> Resource<EventTarget> {
        let target = EventTarget::new();
        self.table.push(target).unwrap()
    }

    fn add_event_listener(&mut self, self_: Resource<EventTarget>, ty: String, callback: String) {
        let target = self.table.get_mut(&self_).unwrap();
        target.add_event_listener(ty, callback);
    }

    fn drop(&mut self, rep: Resource<EventTarget>) -> Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
