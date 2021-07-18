use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use anyhow::Result;
use std::path::{Path, PathBuf, StripPrefixError};
use tera::Context;
use tera::Tera;
use toml;

use std::fs::read_to_string;

use utils::fs::read_all;
use utils::parsing::{parse, Markdown, Value};

#[derive(Serialize, Deserialize)]
struct Blog<'a> {
    title: &'a str,
    templates: &'a str,
    posts: &'a str,
}

#[derive(Serialize, Deserialize)]
struct Site<'a> {
    #[serde(borrow)]
    blog: Blog<'a>,
}

fn replace_prefix(
    p: impl AsRef<Path>,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<PathBuf, StripPrefixError> {
    p.as_ref().strip_prefix(from).map(|p| to.as_ref().join(p))
}

fn main() -> Result<()> {
    let tera = Tera::new("templates/**/*")?;
    let read_files = read_all(&PathBuf::from("./posts/"))?;
    let site_path = Path::new("site.toml");
    let meta = read_to_string(site_path)?;
    let site_metadata = toml::from_str::<Site>(&meta)?;
    for mut file in read_files {
        let markdown: Markdown<HashMap<String, Value>> = match parse(&file.contents) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("{}", err);
                return Ok(());
            }
        };
        let mut context = Context::new();
        context.insert("meta", &markdown.meta);
        context.insert("content_html", &markdown.content_html);
        let post_template_value = markdown.meta.get("template").expect("Template is missing from post.");
        let post_template= match post_template_value {
            toml::Value::String(template) => template,
            _ => panic!("Invalid template value: {}", post_template_value)
        };
        let templated = tera.render(post_template, &context)?;
        file.path.set_extension("html");
        file.path = replace_prefix(&file.path, "./", "./.html_output/")?;
        file.contents = templated;
        file.write()?;
        println!("Wrote: {:?}", file.path);
    }

    Ok(())
}
