use std::{any::Any, marker::PhantomData};

use wasmtime::{AsContextMut, Error, ExternRef, GcHeapOutOfMemory, Result, Rooted, StoreContext};

/// A DOM Object is a GC traced data object.
pub struct DOMObject<T: 'static + Any + Send + Sync> {
    object: Rooted<ExternRef>,
    _phantom: PhantomData<T>,
}

impl<T: 'static + Any + Send + Sync> DOMObject<T> {
    pub fn new(mut context: impl AsContextMut, value: T) -> Result<Self> {
        let object = match ExternRef::new(&mut context, value) {
            Ok(x) => x,
            Err(e) => match e.downcast::<GcHeapOutOfMemory<&str>>() {
                Ok(oom) => {
                    let (inner, oom) = oom.take_inner();
                    context.as_context_mut().gc(Some(&oom));
                    ExternRef::new(&mut context, inner)?
                }
                Err(e) => return Err(e),
            },
        };

        Ok(Self {
            object,
            _phantom: PhantomData,
        })
    }

    pub fn data<'a, U>(&self, store: impl Into<StoreContext<'a, U>>) -> Result<&'a T>
    where
        U: 'static,
    {
        self.object
            .data(store)?
            .ok_or_else(|| Error::msg("externref has no host data"))?
            .downcast_ref::<T>()
            .ok_or_else(|| Error::msg("externref was not requested type"))
    }
}
