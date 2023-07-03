pub mod component;

pub struct Store<T>(wasmer::Store, PhantomData<T>);
// impl<T> Default for Store<T> {
//     fn default() -> Self {
//         Self(Default::default(), Default::default())
//     }
// }
impl<T> Store<T> {
    pub fn engine(&self) -> &wasmer::Engine {
        self.0.engine()
    }
}
impl Store<DefaultDataType> {
    pub fn default() -> Store<DefaultDataType> {
        Store(Default::default(), Default::default())
    }
}
pub struct StoreContext<'a, T>(&'a wasmer::Store, PhantomData<T>);
pub struct StoreContextMut<'a, T>(&'a mut wasmer::Store, PhantomData<T>);

pub trait AsContext: wasmer::AsStoreRef {
    type Data;

    fn as_context(&self) -> StoreContext<'_, Self::Data>;
}

pub trait AsContextMut: AsContext + wasmer::AsStoreMut {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::Data>;
}

impl<'a, T> AsContextMut for StoreContextMut<'a, T> {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::Data> {
        todo!()
    }
}
impl<'a, T> DerefMut for StoreContextMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> DerefMut for Store<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<'a, T> AsContext for StoreContextMut<'a, T> {
    type Data = T;

    fn as_context(&self) -> StoreContext<'_, Self::Data> {
        todo!()
    }
}
impl<'a, T> Deref for StoreContextMut<'a, T> {
    type Target = wasmer::Store;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Deref for Store<T> {
    type Target = wasmer::Store;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> AsContextMut for Store<T> {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, T> {
        todo!()
    }
}
impl<T> AsContext for Store<T> {
    type Data = T;

    fn as_context(&self) -> StoreContext<'_, T> {
        todo!()
    }
}
impl<'a, U: AsContextMut> AsContextMut for &mut U {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::Data> {
        todo!()
    }
}
impl<'a, T: AsContext> AsContext for &mut T {
    type Data = T::Data;

    fn as_context(&self) -> StoreContext<'_, Self::Data> {
        todo!()
    }
}
type DefaultDataType = i32;
// impl AsContextMut for &mut wasmer::Store {
//     fn as_context_mut(&mut self) -> StoreContextMut<'_, DefaultDataType> {
//         StoreContextMut(self)
//     }
// }
// impl AsContext for &mut wasmer::Store {
//     type Data = DefaultDataType;

//     fn as_context(&self) -> StoreContext<'_, DefaultDataType> {
//         todo!()
//     }
// }
// impl<T> wasmer::AsStoreMut for &mut Store<T> {
//     fn as_store_mut(&mut self) -> wasmer::StoreMut<'_> {
//         self.as_store_mut()
//     }

//     fn objects_mut(&mut self) -> &mut wasmer_vm::StoreObjects {
//         self.objects_mut()
//     }
// }
// impl<T> Deref for Store<T> {
//     type Target = wasmer::Store;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl<T> DerefMut for Store<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use anyhow::Result;

//struct SomeContext<T> ;

//impl<T> AsContextMut<Data = T> for Store<T> {}
