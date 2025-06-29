use std::any::Any;

use wasmtime::{AsContextMut, Error, ExternRef, GcHeapOutOfMemory, Result, Rooted, StoreContext};

pub struct DOMObject(pub Rooted<ExternRef>);

impl DOMObject {
    pub fn new<T>(mut context: impl AsContextMut, value: T) -> Result<Self>
    where
        T: 'static + Any + Send + Sync,
    {
        Ok(DOMObject(match ExternRef::new(&mut context, value) {
            Ok(x) => x,
            Err(e) => match e.downcast::<GcHeapOutOfMemory<&str>>() {
                Ok(oom) => {
                    let (inner, oom) = oom.take_inner();
                    context.as_context_mut().gc(Some(&oom));
                    ExternRef::new(&mut context, inner)?
                }
                Err(e) => return Err(e),
            },
        }))
    }

    pub fn data<'a, T, U>(&self, store: impl Into<StoreContext<'a, U>>) -> Result<&'a T>
    where
        T: 'static,
        U: 'static,
    {
        self.0
            .data(store)?
            .ok_or_else(|| Error::msg("externref has no host data"))?
            .downcast_ref::<T>()
            .ok_or_else(|| Error::msg("externref was not requested type"))
    }
}
