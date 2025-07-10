use crate::event_target::IsEventTarget;

/// <https://dom.spec.whatwg.org/#event>
#[derive(Clone, Debug)]
pub struct Event {
    type_: String,
    target: Option<IsEventTarget>,
}
