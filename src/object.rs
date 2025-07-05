use std::{any::Any, marker::PhantomData, ops::Deref};

use wasmtime::{
    AsContextMut, Error, ExternRef, GcHeapOutOfMemory, Result, Rooted, StoreContext,
    StoreContextMut,
};

/// This is a GC traced object represented as a DOM object.
///
/// An `Object` is basically `Rooted<ExternRef>` with the type annotation preserved in
/// `PhantomData<T>`. This helps users understand what's the actual implementation of the object.
/// It can also dereference to `Rooted<ExternRef>`.
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
    /// Create an GC traced `Object` from provided value.
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

    /// Get a shared borrow of the underlying data for this `Object`.
    pub fn data<'a, U>(&self, store: impl Into<StoreContext<'a, U>>) -> &'a T
    where
        U: 'static,
    {
        self.try_data(store)
            .expect("externref was not requested type")
    }

    fn try_data<'a, U>(&self, store: impl Into<StoreContext<'a, U>>) -> Result<&'a T>
    where
        U: 'static,
    {
        self.object
            .data(store)?
            .ok_or_else(|| Error::msg("externref has no host data"))?
            .downcast_ref::<T>()
            .ok_or_else(|| Error::msg("externref was not requested type"))
    }

    ///  Get an exclusive borrow of the underlying data for this `Object`.
    pub fn data_mut<'a, U>(&self, store: impl Into<StoreContextMut<'a, U>>) -> &'a mut T
    where
        U: 'static,
    {
        self.try_data_mut(store)
            .expect("externref was not requested type")
    }

    fn try_data_mut<'a, U>(&self, store: impl Into<StoreContextMut<'a, U>>) -> Result<&'a mut T>
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
