use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ReadFile {
    pub path: PathBuf,
    pub contents: String,
}

impl ReadFile {
    pub fn write(&self) -> Result<()> {
        if !self.path.is_dir() {
            let parent = &self.path.parent().unwrap();
            if !Path::new(parent).exists() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&self.path, &self.contents)?;
        }
        Ok(())
    }
}

pub fn read_all(current_dir: &PathBuf) -> Result<Vec<ReadFile>> {
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
