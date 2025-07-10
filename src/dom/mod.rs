//! DOM standard implementation
/// The `Document` object
pub mod document;
/// The `Element` object
pub mod element;
/// The `Event` object
pub mod event;
/// The `EventTarget` object
pub mod event_target;
/// The `Node` object
pub mod node;
/// The general DOM object
pub mod object;

pub use document::*;
pub use element::*;
pub use event::*;
pub use event_target::*;
pub use node::*;
pub use object::*;
