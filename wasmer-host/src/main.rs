use adapter::wasmtime::{
    self,
    component::{Component, Linker},
};
use anyhow::{self, Result};
use wasmer::{imports, Instance, Module, Store, Value};

wasmtime::component::bindgen!({ path: "../wit/test.wit", world: "testw" });

fn main() -> Result<()> {
    let mut store = Store::default();
    let engine = store.engine();
    let linker = Linker::new(engine);

    // load module in wasmer
    let module = Module::from_file(&store, "target/wasm32-unknown-unknown/debug/plugin.wasm")?;

    // create an adapter
    let component = Component::from_binary(engine, &module)?;

    // instantiate the world using both
    let (myworld, _) = Testw::instantiate(&mut store, &component, &linker)?;

    // call the exported function
    let res = myworld.call_testfun(&mut store, "hello")?;

    assert_eq!(res, "hello world");
    println!("Out: {}", res);

    Ok(())
}
