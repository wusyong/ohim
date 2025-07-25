use crate::IsEventTarget;

/// <https://dom.spec.whatwg.org/#event>
#[derive(Clone, Debug)]
pub struct Event {
    _type_: String,
    _target: Option<IsEventTarget>,
}
