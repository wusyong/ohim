use anyhow::Context;
use ohim::{All, WindowStates, ohim::dom::event};
use std::{fs, path::Path};

use wasmtime::{
    Config, Engine, Result, Store,
    component::{Component, HasSelf, Linker},
};

/// This function is only needed until rust can natively output a component.
///
/// Generally embeddings should not be expected to do this programmatically, but instead
/// language specific tooling should be used, for example in Rust `cargo component`
/// is a good way of doing that: https://github.com/bytecodealliance/cargo-component
///
/// In this example we convert the code here to simplify the testing process and build system.
fn convert_to_component(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    fs::read(&path).context("failed to read input file")
}

fn main() -> Result<()> {
    // Create an engine with the component model enabled (disabled by default).
    let engine = Engine::new(Config::new().wasm_component_model(true))?;

    let component = convert_to_component("target/wasm32-wasip1/debug/test.wasm")?;

    // Create our component and call our generated host function.
    let component = Component::new(&engine, &component)?;
    let mut store = Store::new(&engine, WindowStates::create());
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    event::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
    let convert = All::instantiate(&mut store, &component, &linker)?;
    let result = convert.call_test(&mut store)?;
    println!("Converted to: {result:?}");
    Ok(())
}
