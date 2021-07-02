use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

mod runtime;

use runtime::run_module;

#[derive(Serialize, Deserialize)]
pub struct StageModule<'a> {
    path: &'a str,
}

type ModuleDirectory<'a> = HashMap<&'a str, StageModule<'a>>;
#[derive(Serialize, Deserialize)]

struct Volume<'a> {
    host: &'a str,
    guest: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct Stage<'a> {
    module: &'a str,
    volumes: Vec<Volume<'a>>,
}

#[derive(Serialize, Deserialize)]
struct StageVec<'a> {
    #[serde(borrow)]
    stage: Vec<Stage<'a>>,
}

#[derive(Serialize, Deserialize)]
struct Pipeline<'a> {
    #[serde(borrow)]
    modules: ModuleDirectory<'a>,
    stages: StageVec<'a>,
}

fn main() -> Result<()> {
    let pipeline = fs::read_to_string("./pipeline.toml")?;
    let module_directory: Pipeline = toml::from_str(&pipeline)?;
    for stage in module_directory.stages.stage.iter() {
        let module = module_directory.modules.get(stage.module).expect(&format!(
            "{} isn't a module. Please define it in the [modules] section.",
            stage.module
        ));
        run_module(&module, &stage)?;
    }
    Ok(())
}
