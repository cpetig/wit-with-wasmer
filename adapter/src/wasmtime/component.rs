use std::marker::PhantomData;
use anyhow::Result;
use wasmer::Store;

pub use wasmtime::component::bindgen;

pub mod __internal {
    pub use anyhow;
}

//pub struct Store;
pub struct Callable {}
impl Callable {
    pub fn call<A>(&self, store: &mut impl crate::wasmtime::AsContextMut<Data = ()>, args: A) -> Result<()> { todo!(); }
}

pub struct TypedFunc<A,R> {
    phantom: PhantomData<(A,R)>,
}
impl<A,R> TypedFunc<A,R> {
    pub fn new_unchecked(f: Func) -> Callable {
        Callable {  }
    }
}

pub struct Func;
pub struct Component;

pub struct Linker<T> { phantom: PhantomData<T> }
impl<T> Linker<T> {
    pub fn instantiate(&mut self, store: &mut impl crate::wasmtime::AsContextMut<Data = T>, component: &Component) -> Result<Instance> {
        todo!();
    }
}

pub struct Exports;
impl Exports {
    pub fn root(&self) -> Symboltable { todo!()}
}

pub struct Symboltable;
impl Symboltable {
    pub fn typed_func(&self, name: &str) -> Result<()> {todo!()}
}

pub struct Instance;
impl Instance {
    pub fn exports<T>(&self, store: &mut impl crate::wasmtime::AsContextMut<Data = T>) -> Exports {
        Exports{}
    }
}

pub struct InstancePre<T> { phantom: PhantomData<T> }
impl<T> InstancePre<T> {
    pub fn instantiate(&mut self, store: &mut impl crate::wasmtime::AsContextMut<Data = T>) -> Result<Instance>{
        todo!();
    }
}
