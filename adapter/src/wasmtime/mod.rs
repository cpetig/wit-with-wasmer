pub mod component;

pub struct Store<T>(PhantomData<T>);
pub struct StoreContext<'a, T>(&'a Store<T>);
pub struct StoreContextMut<'a, T>(&'a mut Store<T>);

pub trait AsContext {
    type Data;

    fn as_context(&self) -> StoreContext<'_, Self::Data>;
}

pub trait AsContextMut: AsContext {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::Data>;
}

impl<'a, T> AsContextMut for StoreContextMut<'a, T> {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::Data> {
        todo!()
    }
}
impl<'a, T> AsContext for StoreContextMut<'a, T> {
    type Data = T;

    fn as_context(&self) -> StoreContext<'_, Self::Data> {
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
impl AsContextMut for &mut wasmer::Store {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, DefaultDataType> {
        todo!()
    }
}
impl AsContext for &mut wasmer::Store {
    type Data = DefaultDataType;

    fn as_context(&self) -> StoreContext<'_, DefaultDataType> {
        todo!()
    }
}

use std::marker::PhantomData;

pub use anyhow::Result;

//struct SomeContext<T> ;

//impl<T> AsContextMut<Data = T> for Store<T> {}
