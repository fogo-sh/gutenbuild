use anyhow::Result;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::time::Instant;
use wasmtime::*;
use wasmtime_wasi::sync::{Dir, WasiCtxBuilder};

use crate::Pipeline;

pub fn run_modules(pipeline: Pipeline) -> Result<()> {
    log::info!("Beginning runtime.");
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    for stage in pipeline.stages.stage.iter() {
        let mut builder = WasiCtxBuilder::new();
        let module = pipeline.modules.get(stage.module).expect(&format!(
            "{} isn't a module. Please define it in the [modules] section.",
            stage.module
        ));
        log::info!(" - Found pipeline stage: {}", stage.module);

        if !Path::new(module.path).exists() {
            panic!("{} does not exist.", module.path)
        }

        log::debug!(" ... mounting volumes.");
        for volume in stage.volumes.iter() {
            log::debug!("     from {} to {}", volume.host, volume.guest);
            if !Path::new(volume.host).exists() {
                create_dir_all(volume.host)?;
            }
            let preopen_dir = unsafe { Dir::from_std_file(File::open(volume.host)?) };
            builder = builder.preopened_dir(preopen_dir, Path::new(volume.guest))?;
        }

        log::debug!(" ... wasmtime.");

        let wasi = builder
            .inherit_stderr()
            .inherit_stdio()
            .inherit_args()?
            .build();
        let mut store = Store::new(&engine, wasi);

        let mut now = Instant::now();
        let module = Module::from_file(&engine, module.path)?;
        log::debug!(
            "     from_file {} took {}ms.",
            stage.module,
            now.elapsed().as_millis()
        );
        now = Instant::now();
        linker.module(&mut store, "", &module)?;
        log::debug!(
            "     linker.module {} took {}ms.",
            stage.module,
            now.elapsed().as_millis()
        );
        now = Instant::now();
        linker
            .get_default(&mut store, "")?
            .typed::<(), (), _>(&store)?
            .call(&mut store, ())?;
        log::debug!(
            "     linker.call {} took {}ms.",
            stage.module,
            now.elapsed().as_millis()
        );
    }

    Ok(())
}
