use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::{Path, PathBuf, StripPrefixError};

fn replace_prefix(
    p: impl AsRef<Path>,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<PathBuf, StripPrefixError> {
    p.as_ref().strip_prefix(from).map(|p| to.as_ref().join(p))
}

struct ReadFile {
    path: PathBuf,
    contents: String,
}

impl ReadFile {
    fn write(&self) -> Result<()> {
        if self.path.is_dir() {
            let parent = &self.path.parent().unwrap();
            if !Path::new(parent).exists() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&self.path, &self.contents)?;
        }
        Ok(())
    }
}

fn read_all(current_dir: &PathBuf) -> Result<Vec<ReadFile>> {
    let dir_entries = fs::read_dir(current_dir)?;
    let mut read_files = vec![];
    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            read_files.append(&mut read_all(&path)?);
        }
        if let Some(extension) = &path.extension() {
            if extension.to_str().unwrap() == "md" {
                let contents = fs::read_to_string(&path)?;
                read_files.push(ReadFile { path, contents });
            }
        }
    }
    Ok(read_files)
}

fn to_html(files: &mut Vec<ReadFile>) -> Result<()> {
    for mut file in files {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&file.contents, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        file.path.set_extension("html");
        file.path = replace_prefix(&file.path, "./", "./.html_output/")?;
        file.contents = html_output;
        println!("Wrote: {:?}", file.path);
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut read_files = read_all(&PathBuf::from("./"))?;
    to_html(&mut read_files)?;
    for file in read_files {
        file.write()?;
    }
    Ok(())
}
