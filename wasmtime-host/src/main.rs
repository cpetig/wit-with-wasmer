use anyhow::{self, Result};
use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

wasmtime::component::bindgen!({ path: "../wit/test.wit", world: "testw" });

// As guest, when you build the output is a wasm module, not a component in terms of wasmtime
// this allows us to do it in rust however as i'll note below it's preferred to do it through the wasm-tools cli
// and just import with Component::from_file
use wit_component::ComponentEncoder;

fn main() -> Result<()> {
    let mut config = Config::new();

    Config::wasm_component_model(&mut config, true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, 0);
    let linker = Linker::new(&engine);

    /* [0] This should be done manually by the guest
    by
    ``wasm-tools component new guest.wasm -o guest.wasmc``
    as there's no need to introduce this overhead

    SECTION:BEGIN  */

    // let wasm_module_path = format!(
    //     "{}/{}",
    //     std::env::var("OUT_DIR")?,
    //     "target/wasm32-unknown-unknown/debug/plugin.wasm"
    // );
    let wasm_module_path = "target/wasm32-unknown-unknown/debug/plugin.wasm";

    let module =
        std::fs::read(wasm_module_path).expect("WASM Module missing, did you build guest crate?");

    let component = ComponentEncoder::default()
        .module(module.as_slice())?
        .validate(true)
        .encode()?;

    // [0] SECTION:END

    // Unnecessary when using rust host, because we don't need to use the component anywhere else
    // std::fs::write("./target/component.wasm", &component)?;

    let component = Component::from_binary(&engine, &component)?;

    // NOTE: Usage is through instantation, the Markdown struct (our-world) was generated
    // with a instantiate method when calling Component::bindgen

    // after getting the component, we can instantiate a markdown instance.
    let (myworld, _) = Testw::instantiate(&mut store, &component, &linker)?;

    // Now let's use  the exported render function
    let res = myworld.call_testfun(&mut store, "hello")?;

    assert_eq!(res, "hello world");
    println!("Out: {}", res);

    Ok(())
}
