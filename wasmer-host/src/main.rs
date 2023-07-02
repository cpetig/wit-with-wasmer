use adapter::wasmtime::{
    self,
    component::{Component, Linker},
};
use anyhow::{self, Result};
use wasmer::{imports, Instance, Module, Store, Value};

wasmtime::component::bindgen!({ path: "../wit/test.wit", world: "testw" });

fn main() -> Result<()> {
    //let engine = wasmer::Engine::default();
    let mut store = Store::default();
    let engine = store.engine();
    let linker = Linker::new(engine);

    let module = Module::from_file(&store, "target/wasm32-unknown-unknown/debug/plugin.wasm")?;
    // The module doesn't import anything, so we create an empty import object.
    // let import_object = imports! {};
    // let instance = Instance::new(&mut store, &module, &import_object)?;

    let component = Component::from_binary(engine, &module)?;

    let (myworld, _) = Testw::instantiate(&mut store, &component, &linker)?;

    // Now let's use  the exported render function
    let res = myworld.call_testfun(&mut store, "hello")?;

    assert_eq!(res, "hello world");
    println!("Out: {}", res);

    // let add_one = instance.exports.get_function("testfun")?;
    // let result = add_one.call(&mut store, &[Value::I32(0), Value::I32(0)])?;
    // println!("Result {result:?}");
    Ok(())
}
