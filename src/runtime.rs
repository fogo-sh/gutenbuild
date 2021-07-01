use anyhow::Result;
use std::path::Path;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

use crate::Stage;

pub fn run_module(module: &Stage) -> Result<()> {
    if !Path::new(module.path).exists() {
        panic!("{} does not exist.", module.path)
    }
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, module.path)?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;
    Ok(())
}
