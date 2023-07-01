use anyhow::{self, Result};
use wasmer::{Store, Module, Instance, Value, imports};

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
