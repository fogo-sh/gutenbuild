use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use anyhow::Result;
use std::path::{Path, PathBuf, StripPrefixError};
use tera::Context;
use tera::Tera;
use toml;

use std::fs::read_to_string;

use utils::fs::read_all;
use utils::parsing::{parse, Value};

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

    let site_path = Path::new("site.toml");
    let meta = read_to_string(site_path)?;
    let site_metadata = toml::from_str::<Site>(&meta)?;

    let mut read_files = read_all(&PathBuf::from("./posts/"))?;
    let mut all_markdown_parsed = vec![];
    let mut read_to_markdown = read_files
        .iter_mut()
        .map(|file| {
            let markdown = parse::<HashMap<String, Value>>(&file.contents)?;
            file.path.set_extension("html");
            file.path = replace_prefix(&file.path, "./posts", "./.html_output/")?;

            all_markdown_parsed.push(markdown.clone());
            println!("Wrote: {:?}", file.path);
            Ok((file, markdown))
        })
        .collect::<Result<Vec<_>>>()?;

    for (file, markdown) in &mut read_to_markdown {
        let mut context = Context::new();

        context.insert("meta", &markdown.meta);
        context.insert("blog", &site_metadata.blog);
        context.insert("documents", &all_markdown_parsed);
        context.insert("content_html", &markdown.content_html);

        let post_template_value = markdown
            .meta
            .get("template")
            .expect("Template is missing from post.");
        let post_template = match post_template_value {
            toml::Value::String(template) => template,
            _ => panic!("Invalid template value: {}", post_template_value),
        };

        let templated = tera.render(&post_template, &context)?;
        file.contents = templated;

        file.write()?;

        println!("Wrote: {:?}", file.path);
    }

    Ok(())
}
