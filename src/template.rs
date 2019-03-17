use include_dir::{Dir, File};

pub struct Template<'a> {
    root: Dir<'a>,
}

impl<'a> Template<'a> {
    pub fn new(root: Dir<'a>) -> Self {
        Self { root: root }
    }

    /// Creates iterator for Template, which does a breadth first iteration of
    /// the directory tree
    pub fn iter(&self) -> TemplateIter<'a> {
        let files = self.root.files().iter();
        let dirs = self.root.dirs().iter();

        TemplateIter {
            dirs: dirs,
            files: files,
        }
    }
}

/// Breadth first iterator of the directory tree of the template
pub struct TemplateIter<'a> {
    dirs: std::slice::Iter<'a, Dir<'a>>,
    files: std::slice::Iter<'a, File<'a>>,
}

impl<'a> Iterator for TemplateIter<'a> {
    type Item = TemplateKind<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self.files.next() {
            Some(TemplateKind::File(*file))
        } else if let Some(dir) = self.dirs.next() {
            self.files = dir.files().iter();
            Some(TemplateKind::Dir(*dir))
        } else {
            None
        }
    }
}

pub enum TemplateKind<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
}

impl<'a> IntoIterator for Template<'a> {
    type Item = TemplateKind<'a>;
    type IntoIter = TemplateIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let files = self.root.files().iter();
        let dirs = self.root.dirs().iter();

        Self::IntoIter {
            dirs: dirs,
            files: files,
        }
    }
}
