use wasmtime::{AsContextMut, Result};

use super::Object;

/// <https://html.spec.whatwg.org/multipage/#window>
#[derive(Clone, Debug)]
pub struct Window(Object<WindowImpl>);

impl Window {
    /// Create a `Window` object.
    pub fn new(store: impl AsContextMut) -> Result<Self> {
        Ok(Window(Object::new(store, WindowImpl {})?))
    }
}

/// Implementation of acutal `Window` object.
#[derive(Debug)]
struct WindowImpl {}

/// <https://html.spec.whatwg.org/multipage/#windowproxy>
#[derive(Clone, Debug)]
pub struct WindowProxy {}
