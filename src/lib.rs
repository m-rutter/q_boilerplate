#[macro_use]
extern crate include_dir;
use git2::{Config, Repository, RepositoryInitOptions};
use include_dir::{Dir, File};
use std::path::PathBuf;
use tera::{Context, Tera};

pub mod error;
pub mod template;

use template::{Template, TemplateKind};

static VIZ_EXT_DIR: Dir = include_dir!("./templates/visualisation");

pub fn gen_viz(project_name: &str, path: &Option<PathBuf>, git: bool) -> Result<(), error::Error> {
    let mut context = Context::new();

    context.insert("project_name", project_name);

    if let Some(author) = get_git_author() {
        context.insert("authors", &author);
    } else {
        context.insert("authors", "");
    }

    let template = Template::new(VIZ_EXT_DIR);

    for thing in template.iter() {
        match thing {
            TemplateKind::Dir(dir) => {
                println!("{:?}", dir);
            }
            TemplateKind::File(file) => {
                println!("{:?}", file);
            }
        };
    }

    Ok(())
}

pub fn get_git_author() -> Option<String> {
    if let Ok(config) = Config::open_default() {
        if let Ok(user) = config.get_string("user.name") {
            Some(user)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn init_git_repo(path: &PathBuf) -> Result<Repository, error::Error> {
    let repo = Repository::init_opts(path, RepositoryInitOptions::new().bare(false))?;

    Ok(repo)
}
