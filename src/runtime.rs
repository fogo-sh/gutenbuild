use anyhow::Result;
use std::fs::File;
use std::path::Path;
use wasmtime::*;
use wasmtime_wasi::sync::{Dir, WasiCtxBuilder};

use crate::{Stage, StageModule};

pub fn run_module(module: &StageModule, stage: &Stage) -> Result<()> {
    if !Path::new(module.path).exists() {
        panic!("{} does not exist.", module.path)
    }
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let mut builder = WasiCtxBuilder::new();
    for volume in stage.volumes.iter() {
        let preopen_dir = unsafe { Dir::from_std_file(File::open(volume.host)?) };
        builder = builder.preopened_dir(preopen_dir, Path::new(volume.guest))?;
    }
    let wasi = builder.inherit_stdio().inherit_args()?.build();
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
