#[macro_use]
extern crate lazy_static;

use serde::{Serialize, Deserialize};
use anyhow::Result;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("examples/basic/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

#[derive(Serialize, Deserialize)]
struct Product<'a> {
    name: &'a str
}

fn main() -> Result<()> {
    use tera::Context;

    let mut context = Context::new();
    context.insert("product", &Product{
        name: "hello",
    });
    context.insert("vat_rate", &0.20);
    tera.render("products/product.html", &context)?;

    // or a struct
    tera.render("products/product.html", &Context::from_serialize(&product)?)?;

    Ok(())
}
