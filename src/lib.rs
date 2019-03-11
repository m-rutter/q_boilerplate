use std::path::PathBuf;
use tera::{Context, Tera};

pub mod error;

pub fn gen_viz(project_name: &str, path: &Option<PathBuf>, git: bool) -> Result<(), error::Error> {
    let mut context = Context::new();

    context.insert("project-name", project_name);
    context.insert("authors", "");

    let tera = Tera::new("templates/visualisation/**/*")?;

    dbg!(tera.render("package.json", &context));

    println!("test");

    Ok(())
}
