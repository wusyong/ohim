use std::{any::Any, marker::PhantomData, ops::Deref};

use wasmtime::{
    AsContextMut, Error, ExternRef, GcHeapOutOfMemory, Result, Rooted, StoreContext,
    StoreContextMut,
};

/// A DOM Object is a GC traced data object.
#[derive(Copy, Debug)]
pub struct Object<T: 'static + Any + Send + Sync> {
    object: Rooted<ExternRef>,
    _phantom: PhantomData<T>,
}

impl<T: 'static + Any + Send + Sync> Clone for Object<T> {
    fn clone(&self) -> Self {
        Self {
            object: self.object,
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static + Any + Send + Sync> Object<T> {
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

    pub fn data_mut<'a, U>(&self, store: impl Into<StoreContextMut<'a, U>>) -> &'a mut T
    where
        U: 'static,
    {
        self.try_data_mut(store)
            .expect("externref was not requested type")
    }

    pub fn try_data_mut<'a, U>(&self, store: impl Into<StoreContextMut<'a, U>>) -> Result<&'a mut T>
    where
        U: 'static,
    {
        self.object
            .data_mut(store)?
            .ok_or_else(|| Error::msg("externref has no host data"))?
            .downcast_mut::<T>()
            .ok_or_else(|| Error::msg("externref was not requested type"))
    }
}

impl<T: 'static + Any + Send + Sync> Deref for Object<T> {
    type Target = Rooted<ExternRef>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
