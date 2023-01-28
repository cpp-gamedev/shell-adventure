use std::{collections::HashMap, fmt::Write};

use thiserror::Error;

pub struct File {
    path: Path,
    pub data: String,
    pub is_executable: bool,
}

impl File {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub struct Directory {
    path: Path,
    pub files: HashMap<String, File>,
    pub dirs: HashMap<String, Directory>,
    pub is_writable: bool,
}

#[derive(Debug, Error)]
#[error("tried to create directory or file, but object with that name already existed")]
pub struct AlreadyExistsError;

impl Directory {
    pub fn new_root() -> Self {
        Self {
            path: Path::root(),
            files: Default::default(),
            dirs: Default::default(),
            is_writable: Default::default(),
        }
    }

    pub fn create_dir(&mut self, name: String) -> Result<&mut Directory, AlreadyExistsError> {
        match self.dirs.entry(name) {
            std::collections::hash_map::Entry::Occupied(_) => Err(AlreadyExistsError),
            std::collections::hash_map::Entry::Vacant(v) => {
                let name = v.key().clone();
                Ok(v.insert(Directory {
                    path: self.path.clone().with_component(name),
                    dirs: Default::default(),
                    files: Default::default(),
                    is_writable: false,
                }))
            }
        }
    }

    pub fn create_file(&mut self, name: String) -> Result<&mut File, AlreadyExistsError> {
        match self.files.entry(name) {
            std::collections::hash_map::Entry::Occupied(_) => Err(AlreadyExistsError),
            std::collections::hash_map::Entry::Vacant(v) => {
                let name = v.key().clone();
                Ok(v.insert(File {
                    path: self.path.clone().with_component(name),
                    data: Default::default(),
                    is_executable: false,
                }))
            }
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub enum DirEntry<'f> {
    File(&'f File),
    Directory(&'f Directory),
}

pub struct Machine {
    pub cwd: Path,
    pub root_dir: Directory,
}

#[derive(Clone)]
pub struct Path {
    components: Vec<String>,
    is_absolute: bool,
}

impl Path {
    pub fn root() -> Self {
        Self {
            components: vec![],
            is_absolute: true,
        }
    }

    pub fn parse(str: &str) -> Self {
        let (str, is_absolute) = if str.starts_with('/') {
            (&str[1..], true)
        } else {
            (str, false)
        };
        let mut components: Vec<String> = str.split('/').map(ToOwned::to_owned).collect();
        // trailing slash hack
        if components.last().map_or(false, String::is_empty) {
            components.pop();
        }

        Self {
            components,
            is_absolute,
        }
    }

    pub fn with_component(mut self, component: String) -> Self {
        self.components.push(component);
        self
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    pub fn is_relative(&self) -> bool {
        !self.is_absolute
    }
}

impl std::ops::Add<Path> for Path {
    type Output = Path;

    fn add(mut self, rhs: Path) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Path> for Path {
    fn add_assign(&mut self, mut rhs: Path) {
        // cd foo/bar + cd /foo = /foo resulting dir
        if rhs.is_absolute() {
            *self = rhs;
        } else {
            self.components.extend(rhs.components.drain(..));
        }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_absolute {
            f.write_char('/')?;
        }

        for part in itertools::intersperse(self.components.iter().map(String::as_ref), "/") {
            f.write_str(part)?
        }

        Ok(())
    }
}

impl Machine {
    // TODO: Return a Result instead of an Option. Use thiserror for the error type
    pub fn traverse(&self, path: &Path) -> Option<DirEntry> {
        let cwd_plus_path;
        let path = if path.is_relative() {
            cwd_plus_path = self.cwd.clone() + path.clone();
            &cwd_plus_path
        } else {
            path
        };

        let mut traversal_stack = Vec::with_capacity(path.components.len());
        let mut current_dir = &self.root_dir;
        for (is_last, component) in path
            .components
            .iter()
            .enumerate()
            .map(|(idx, x)| (idx == path.components.len() - 1, x))
        {
            match component.as_str() {
                ".." => {
                    current_dir = traversal_stack.pop().unwrap_or(&self.root_dir);
                }
                "." => (),
                component => {
                    if is_last {
                        if current_dir.files.contains_key(component) {
                            return Some(DirEntry::File(&current_dir.files[component]));
                        }
                    }
                    traversal_stack.push(current_dir);
                    if current_dir.dirs.contains_key(component) {
                        current_dir = &current_dir.dirs[component];
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(DirEntry::Directory(current_dir))
    }
}
