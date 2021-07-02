use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

fn process(current_dir: &str) -> Result<(), String> {
    for entry in fs::read_dir(current_dir).map_err(|err| format!("{}", err))? {
        let entry = entry.map_err(|err| format!("{}", err))?;
        let path = entry.path();
        if let Some(extension) = &path.extension() {
            let contents =
                fs::read_to_string(&path).expect("Something went wrong reading the file");
            if extension.to_str().unwrap() == "md" {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                let parser = Parser::new_ext(&contents, options);
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                fs::write(
                    Path::new(&format!(
                        "{}.html",
                        &path
                            .file_stem()
                            .expect("Should have a file name")
                            .to_str()
                            .unwrap()
                    )),
                    html_output,
                )
                .expect("Couldn't write file");
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    process("./")
}
