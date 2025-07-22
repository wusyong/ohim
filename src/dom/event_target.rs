use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::{Event, Node};

/// <https://dom.spec.whatwg.org/#eventtarget>
#[derive(Clone, Debug, Default)]
pub struct EventTarget {
    _callbacks: Option<HashMap<String, EventListener>>,
}

impl EventTarget {
    /// Create an `EventTarget` with empty `EventListener`.
    pub fn new() -> Self {
        Self::default()
    }
}

// impl EventTarget {
//     fn add_event_listener(
//         &mut self,
//         ty: String,
//         callback: EventListener,
//         store: impl AsContextMut,
//     ) {
//     }
//     fn remove_event_listener(
//         &mut self,
//         ty: String,
//         callback: EventListener,
//         store: impl AsContextMut,
//     ) {
//     }
//     fn dispatch_event(&self, event: Event) {}
// }

/// <https://dom.spec.whatwg.org/#callbackdef-eventlistener>
#[derive(Clone)]
pub struct EventListener(Arc<dyn FnMut(Event) + Send + Sync>);

impl Debug for EventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("...")
    }
}

/// Types that inherent `EventTarget` and can be added to `Event`'s target fields.
///
/// See <https://dontcallmedom.github.io/webidlpedia/names/EventTarget.html> for full list.
#[derive(Clone, Debug)]
pub enum IsEventTarget {
    /// `EventTarget`
    EventTarget(EventTarget),
    /// `Node`
    Node(Node),
}

// impl EventListener {
//     fn call(&mut self, event: Event) {
//         self.0(event)
//     }
// }

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
