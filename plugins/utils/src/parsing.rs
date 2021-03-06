
// https://github.com/fogo-sh/fogo.sh/blob/master/src/parser.rs
// Something I wrote a while ago to parse the `---` delimited frontmatter

use serde::{Deserialize, Serialize};
use comrak::{markdown_to_html, ComrakOptions};
use toml;

pub type Value = toml::Value;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Markdown<T> {
    pub meta: T,
    pub content_markdown: String,
    pub content_html: String,
}

const FRONTMATTER_DELIMITER: &'static str = "---";

fn parse_frontmatter<'de, T>(post: &'de str) -> Option<Result<T, toml::de::Error>>
where
    T: serde::Deserialize<'de>,
{
    let first_line = post.lines().next();
    let mut sections = post.split(FRONTMATTER_DELIMITER);
    if let Some(line) = first_line {
        if FRONTMATTER_DELIMITER == line {
            if let Some(toml) = sections.nth(1) {
                return Some(toml::from_str(toml));
            }
        }
    }
    None
}

pub fn parse<'de, T>(source: &'de str) -> Result<Markdown<T>, toml::de::Error>
where
    T: serde::Deserialize<'de>,
{
    let mut options = ComrakOptions::default();
    options.extension.tasklist = true;
    options.extension.front_matter_delimiter = Some(FRONTMATTER_DELIMITER.to_owned());

    let html = markdown_to_html(&source, &options);
    let meta = parse_frontmatter(&source).expect("No frontmatter for source");

    Ok(Markdown {
        meta: meta?,
        content_html: html,
        content_markdown: String::from(source),
    })
}