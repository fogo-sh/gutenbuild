use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

mod runtime;

use runtime::run_module;

#[derive(Serialize, Deserialize)]
pub struct Stage<'a> {
    path: &'a str,
}

type ModuleDirectory<'a> = HashMap<&'a str, Stage<'a>>;

type Order<'a> = Vec<&'a str>;

#[derive(Serialize, Deserialize)]
struct BuildDirectory<'a> {
    #[serde(borrow)]
    order: Order<'a>,
}

#[derive(Serialize, Deserialize)]
struct Pipeline<'a> {
    #[serde(borrow)]
    modules: ModuleDirectory<'a>,
    build: BuildDirectory<'a>,
}

fn main() -> Result<()> {
    let pipeline = fs::read_to_string("./pipeline.toml")?;
    let module_directory: Pipeline = toml::from_str(&pipeline)?;
    for name in module_directory.build.order {
        let stage = module_directory.modules.get(name).expect(&format!(
            "{} isn't a module. Please define it in the [modules] section.",
            name
        ));
        run_module(stage)?;
    }
    Ok(())
}
