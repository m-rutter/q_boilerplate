#[macro_use]
extern crate include_dir;
use git2::{Config, Repository, RepositoryInitOptions};
use include_dir::{Dir, File};
use std::env;
use std::path::PathBuf;
use tera::{Context, Tera};

pub mod error;

static VIZ_EXT_DIR: Dir = include_dir!("./templates/visualisation");

pub fn gen_viz(project_name: &str, path: &Option<PathBuf>, git: bool) -> Result<(), error::Error> {
    let mut context = Context::new();

    context.insert("project_name", project_name);

    if let Some(author) = get_git_author() {
        context.insert("authors", &author);
    } else {
        context.insert("authors", "");
    }

    for thing in Template(VIZ_EXT_DIR).into_iter() {
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

struct Template<'a>(Dir<'a>);

struct TemplateIntoIterator<'a> {
    dirs: Box<dyn Iterator<Item = &'a Dir<'a>> + 'a>,
    files: Box<dyn Iterator<Item = &'a File<'a>> + 'a>,
}

impl<'a> Iterator for TemplateIntoIterator<'a> {
    type Item = TemplateKind<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self.files.next() {
            Some(TemplateKind::File(*file))
        } else {
            if let Some(dir) = self.dirs.next() {
                self.files = Box::new(dir.files().iter());
                Some(TemplateKind::Dir(*dir))
            } else {
                None
            }
        }
    }
}

enum TemplateKind<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
}

impl<'a> IntoIterator for Template<'a> {
    type Item = TemplateKind<'a>;
    type IntoIter = TemplateIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let files = self.0.files().into_iter();
        let dirs = self.0.dirs().into_iter();

        Self::IntoIter {
            dirs: Box::new(dirs),
            files: Box::new(files),
        }
    }
}
