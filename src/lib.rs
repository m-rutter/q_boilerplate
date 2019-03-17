use git2::{Config, Repository, RepositoryInitOptions};
use heck::KebabCase;
use include_dir::{include_dir, include_dir_impl, Dir};
use std::env;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

mod error;
mod template;

pub use self::error::Error;
use self::template::{Template, TemplateKind};

static VIZ_EXT_DIR: Dir = include_dir!("./templates/visualisation");

pub fn gen_viz(project_name: &str, git: bool) -> Result<(), error::Error> {
    let project_dir_name = project_name.to_kebab_case();
    let project_directory = create_project_path(&project_dir_name)?;

    if git {
        init_git_repo(&project_directory)?;
    } else {
        std::fs::create_dir(&project_directory)?;
    }

    let template = Template::new(VIZ_EXT_DIR);
    let context = create_context(&project_dir_name);

    for thing in template.iter() {
        match thing {
            TemplateKind::Dir(dir) => {
                let dir_name = dir.path();

                let path = Tera::one_off(dir_name.to_str().unwrap(), &context, true)?;

                let dir_name = Path::new(&path);

                std::fs::create_dir(project_directory.join(&dir_name))?;
            }
            TemplateKind::File(file) => {
                let file_name = file.path();

                let path = Tera::one_off(file_name.to_str().unwrap(), &context, true)?;
                let contents = Tera::one_off(
                    std::str::from_utf8(file.contents()).unwrap(),
                    &context,
                    true,
                )
                .unwrap();

                let file_name = Path::new(&path);

                std::fs::write(project_directory.join(&file_name), contents)?;
            }
        };
    }

    Ok(())
}

fn create_project_path(project_name: &str) -> Result<PathBuf, Error> {
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

fn create_context(project_dir_name: &str) -> Context {
    let mut context = Context::new();

    context.insert("project_name", &project_dir_name);

    if let Some(user) = get_git_author() {
        context.insert("author", &user.user_name);

        if let Some(email) = user.email {
            context.insert("email", &email);
        } else {
            context.insert("email", "author@example.com");
        }
    } else {
        context.insert("author", "author");
        context.insert("email", "author@example.com");
    };

    context
}

struct GitUser {
    user_name: String,
    email: Option<String>,
}

fn get_git_author() -> Option<GitUser> {
    let config = Config::open_default().ok()?;

    let user = config.get_string("user.name").ok()?;

    if let Ok(email) = config.get_string("user.email") {
        Some(GitUser {
            user_name: user,
            email: Some(email),
        })
    } else {
        Some(GitUser {
            user_name: user,
            email: None,
        })
    }
}

pub fn init_git_repo(path: &PathBuf) -> Result<Repository, error::Error> {
    let repo = Repository::init_opts(path, RepositoryInitOptions::new().bare(false))?;

    Ok(repo)
}
