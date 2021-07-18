use std::collections::HashMap;

use anyhow::Result;
use std::path::{Path, PathBuf, StripPrefixError};
use tera::Context;
use tera::Tera;

use utils::fs::read_all;
use utils::parsing::{parse, Markdown, Value};

fn replace_prefix(
    p: impl AsRef<Path>,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<PathBuf, StripPrefixError> {
    p.as_ref().strip_prefix(from).map(|p| to.as_ref().join(p))
}

fn main() -> Result<()> {
    let tera = match Tera::new("templates/**/*") {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            return Ok(());
        }
    };
    let read_files = match read_all(&PathBuf::from("./posts/")) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            return Ok(());
        }
    };
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
        let templated = match tera.render("blog/base.html", &context) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("{}", err);
                return Ok(());
            }
        };
        file.path.set_extension("html");
        file.path = replace_prefix(&file.path, "./", "./.html_output/")?;
        file.contents = templated;
        file.write()?;
        println!("Wrote: {:?}", file.path);
    }

    Ok(())
}
