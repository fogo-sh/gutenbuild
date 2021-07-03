use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::PathBuf;

struct ReadFile {
    path: PathBuf,
    contents: String,
}

impl ReadFile {
    fn write(&self) -> Result<()> {
        fs::write(&self.path, &self.contents)?;
        Ok(())
    }
}

fn read_all(current_dir: &str) -> Result<Vec<ReadFile>> {
    fs::read_dir(current_dir)?
        .filter_map(|entry| {
            // TODO: handle ok as return error result
            let entry = entry.ok()?;
            let path = entry.path();
            if let Some(extension) = &path.extension() {
                if extension.to_str().unwrap() == "md" {
                    let contents = fs::read_to_string(&path).ok()?;
                    return Some(Ok(ReadFile { path, contents }));
                }
            }
            None
        })
        .collect()
}

fn to_html(files: &mut Vec<ReadFile>) -> Result<()> {
    for mut file in files {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&file.contents, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        file.path.set_extension("html");
        file.contents = html_output;
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut read_files = read_all("./")?;
    to_html(&mut read_files)?;
    for file in read_files {
        file.write()?;
    }
    Ok(())
}
