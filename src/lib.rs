use git2::{Config, Repository, RepositoryInitOptions};
use heck::KebabCase;
use include_dir::{include_dir, include_dir_impl, Dir};
use std::env;
use std::path::PathBuf;
use tera::{Context, Tera};

mod error;
mod template;

pub use self::error::Error;
use self::template::{Template, TemplateKind};

static VIZ_EXT_DIR: Dir = include_dir!("./templates/visualisation");

pub fn gen_viz(project_name: &str, git: bool) -> Result<(), error::Error> {
    let project_dir_name = project_name.to_kebab_case();
    let project_directory = create_project_directory(&project_dir_name)?;

    init_git_repo(&project_directory)?;

    let template = Template::new(VIZ_EXT_DIR);
    let context = create_context(project_name, &project_dir_name);

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

fn create_project_directory(project_name: &str) -> Result<PathBuf, Error> {
    let project_dir_name = project_name.to_kebab_case();

    let project_directory = env::current_dir()
        .unwrap_or_else(|_e| ".".into())
        .join(&project_dir_name);

    if project_directory.exists() {
        Err(Error::new(format!(
            "Directory {} already exists",
            project_dir_name
        )))?
    }

    Ok(project_directory)
}

fn create_context(project_name: &str, project_dir_name: &str) -> Context {
    let mut context = Context::new();

    context.insert("project_name", project_name);
    context.insert("project_dir_name", &project_dir_name);

    if let Some((author, email)) = get_git_author() {
        context.insert("authors", &author);
    } else {
        context.insert("authors", "Author");
    };

    context
}

fn get_git_author() -> Option<(String, String)> {
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
