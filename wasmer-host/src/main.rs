use anyhow::{self, Result};
use wasmer::{Store, Module, Instance, Value, imports};
use adapter::wasmtime;

pub struct Testw {
  testfun:wasmtime::component::Func,
}
const _:() = {
  use wasmtime::component::__internal::anyhow;
  impl Testw {
    #[doc = " Instantiates the provided `module` using the specified"]
    #[doc = " parameters, wrapping up the result in a structure that"]
    #[doc = " translates between wasm and the host."]
    pub fn instantiate<T>(mut store:impl wasmtime::AsContextMut<Data = T>,component: &wasmtime::component::Component,linker: &wasmtime::component::Linker<T>,) -> wasmtime::Result<(Self,wasmtime::component::Instance)>{
      let instance = linker.instantiate(&mut store,component)?;
      Ok((Self::new(store, &instance)?,instance))
    }
    #[doc = " Instantiates a pre-instantiated module using the specified"]
    #[doc = " parameters, wrapping up the result in a structure that"]
    #[doc = " translates between wasm and the host."]
    pub fn instantiate_pre<T>(mut store:impl wasmtime::AsContextMut<Data = T>,instance_pre: &wasmtime::component::InstancePre<T>,) -> wasmtime::Result<(Self,wasmtime::component::Instance)>{
      let instance = instance_pre.instantiate(&mut store)?;
      Ok((Self::new(store, &instance)?,instance))
    }
    #[doc = " Low-level creation wrapper for wrapping up the exports"]
    #[doc = " of the `instance` provided in this structure of wasm"]
    #[doc = " exports."]
    #[doc = ""]
    #[doc = " This function will extract exports from the `instance`"]
    #[doc = " defined within `store` and wrap them all up in the"]
    #[doc = " returned structure which can be used to interact with"]
    #[doc = " the wasm module."]
    pub fn new(mut store:impl wasmtime::AsContextMut,instance: &wasmtime::component::Instance,) -> wasmtime::Result<Self>{
      let mut store = store.as_context_mut();
      let mut exports = instance.exports(&mut store);
      let mut __exports = exports.root();
      let testfun =  *__exports.typed_func::<(&str,),(String,)>("testfun")?.func();
      Ok(Testw {
        testfun,
      })
    }
    pub fn call_testfun<S:wasmtime::AsContextMut>(&self,mut store:S,arg0: &str,) -> wasmtime::Result<String>{
      let callee = unsafe {
        wasmtime::component::TypedFunc::<(&str,),(String,)>::new_unchecked(self.testfun)
      };
      let(ret0,) = callee.call(store.as_context_mut(),(arg0,))?;
      callee.post_return(store.as_context_mut())?;
      Ok(ret0)
    }
  
    }

  };
const _: &str = include_str!(r#"/home/christof/github/wasmer-wit/wasmer-host/../wit/test.wit"#);

fn main() -> Result<()> {
    let mut store = Store::default();
    let module = Module::from_file(&store, "target/wasm32-unknown-unknown/debug/plugin.wasm")?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("testfun")?;
    let result = add_one.call(&mut store, &[Value::I32(0), Value::I32(0)])?;
    println!("Result {result:?}");
    Ok(())
}
