#[macro_use]
extern crate include_dir;
use git2::{Repository, RepositoryInitOptions};
use include_dir::Dir;
use std::env;
use std::path::PathBuf;
use tera::{Context, Tera};

pub mod error;

static VIZ_EXT_DIR: Dir = include_dir!("./templates/visualisation");

pub fn gen_viz(project_name: &str, path: &Option<PathBuf>, git: bool) -> Result<(), error::Error> {
    for file in VIZ_EXT_DIR.find("*").unwrap() {
        println!("{}", file.path().display());
    }

    let mut context = Context::new();

    context.insert("project_name", project_name);
    context.insert("authors", "");

    Ok(())
}

pub fn init_git_repo(path: &Option<PathBuf>) -> Result<Repository, error::Error> {
    let path = env::current_dir()?;

    let repo = Repository::init_opts(path, RepositoryInitOptions::new().bare(false))?;

    Ok(repo)
}
