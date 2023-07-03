use anyhow::Result;
use std::marker::PhantomData;
// use wasmer::Store;

pub use wasmtime::component::bindgen;

pub mod __internal {
    pub use anyhow;
}

//pub struct Store;
pub struct Callable<A, R> {
    phantom: PhantomData<(A, R)>,
}
impl<A, R> Callable<A, R> {
    pub fn call(&self, store: impl crate::wasmtime::AsContextMut, args: A) -> Result<R> {
        todo!();
    }
    pub fn post_return(&self, store: impl crate::wasmtime::AsContextMut) -> Result<()> {
        todo!()
    }
}

pub struct TypedFunc<A, R> {
    phantom: PhantomData<(A, R)>,
}
impl<A, R> TypedFunc<A, R> {
    pub fn new_unchecked(f: Func) -> Callable<A, R> {
        Callable {
            phantom: PhantomData,
        }
    }
    pub fn func(&self) -> &Func {
        todo!();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Func;
pub struct Component<'a>(&'a wasmer::Module);
impl<'a> Component<'a> {
    pub fn from_binary(
        engine: &wasmer::Engine,
        module: &'a wasmer::Module,
    ) -> Result<Component<'a>> {
        Ok(Component(module))
    }
}

pub struct Linker<T> {
    phantom: PhantomData<T>,
}
impl<T> Linker<T> {
    pub fn new(engine: &wasmer::Engine) -> Self {
        // I don't think there is anything to do
        Linker {
            phantom: PhantomData,
        }
    }

    pub fn instantiate(
        &self,
        store: impl crate::wasmtime::AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        // register functions?
        //Ok(Instance(component))
        Ok(Instance(wasmer::Instance::new(
            &mut store,
            component.0,
            &wasmer::Imports::new(),
        )?))
    }
}

pub struct Exports;
impl Exports {
    pub fn root(&self) -> ExportInstance {
        todo!()
    }
}

pub struct ExportInstance;
impl ExportInstance {
    pub fn typed_func<A, R>(&self, name: &str) -> Result<TypedFunc<A, R>> {
        todo!()
    }
}

pub struct Instance(wasmer::Instance); //<'a>(&'a Component<'a>);
impl Instance {
    pub fn exports<'a, T: 'a>(
        &self,
        store: &mut impl Into<crate::wasmtime::StoreContextMut<'a, T>>,
    ) -> Exports {
        Exports {}
    }
}

pub struct InstancePre<T> {
    phantom: PhantomData<T>,
}
impl<T> InstancePre<T> {
    pub fn instantiate(
        &self,
        store: impl crate::wasmtime::AsContextMut<Data = T>,
    ) -> Result<Instance> {
        todo!();
    }
}
