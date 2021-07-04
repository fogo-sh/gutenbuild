use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

mod logging;
mod runtime;

use logging::env_logger_init;
use runtime::run_modules;

#[derive(Debug, Serialize, Deserialize)]
pub struct StageModule<'a> {
    path: &'a str,
}

type ModuleDirectory<'a> = HashMap<&'a str, StageModule<'a>>;

#[derive(Debug, Serialize, Deserialize)]
struct Volume<'a> {
    host: &'a str,
    guest: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage<'a> {
    module: &'a str,
    volumes: Vec<Volume<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StageVec<'a> {
    #[serde(borrow)]
    stage: Vec<Stage<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline<'a> {
    #[serde(borrow)]
    modules: ModuleDirectory<'a>,
    stages: StageVec<'a>,
}

fn main() -> Result<()> {
    env_logger_init();

    log::info!("Loading ./pipeline.toml...");
    let pipeline = fs::read_to_string("./pipeline.toml")?;
    log::info!("Parsing ./pipeline.toml...");
    let pipeline: Pipeline = toml::from_str(&pipeline)?;
    run_modules(pipeline)?;
    Ok(())
}
