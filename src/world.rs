use std::collections::HashMap;

use itertools::Itertools;

pub struct File {
    pub data: String,
    pub is_executable: bool,
}

pub struct Directory {
    pub files: HashMap<String, File>,
    pub dirs: HashMap<String, Directory>,
    pub is_writable: bool,
}

pub enum DirEntry<'f> {
    File(&'f File),
    Directory(&'f Directory),
}

pub struct Machine {
    pub cwd: PathBuf,
    pub root_dir: Directory,
}

// TODO: Special components (.., .)
#[derive(Clone)]
pub struct PathBuf {
    components: Vec<String>,
    is_absolute: bool,
}

#[derive(Clone)]
pub struct Path<'a> {
    components: &'a [String],
    is_absolute: bool,
}

impl PathBuf {
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

    pub fn as_view(&self) -> Path {
        Path {
            components: self.components.as_slice(),
            is_absolute: self.is_absolute,
        }
    }
}

impl Path<'_> {
    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    pub fn is_relative(&self) -> bool {
        !self.is_absolute
    }

    pub fn to_path(&self) -> PathBuf {
        PathBuf {
            components: self.components.iter().cloned().collect_vec(),
            is_absolute: self.is_absolute,
        }
    }
}

// TODO: Same semantics as String add (Owned + Borrowed)
impl std::ops::Add<Path<'_>> for PathBuf {
    type Output = PathBuf;

    fn add(mut self, rhs: Path) -> Self::Output {
        // cd foo/bar + cd /foo = /foo resulting dir
        if rhs.is_absolute() {
            rhs.to_path()
        } else {
            self.components.extend(rhs.components.iter().cloned());
            self
        }
    }
}

impl std::ops::AddAssign<Path<'_>> for PathBuf {
    fn add_assign(&mut self, rhs: Path<'_>) {
        if rhs.is_absolute() {
            *self = rhs.to_path();
        } else {
            self.components.extend(rhs.components.iter().cloned());
        }
    }
}

// TODO: impl std::fmt::Display for Path
impl ToString for PathBuf {
    fn to_string(&self) -> String {
        let relative_result = self
            .components
            .iter()
            .map(String::as_ref)
            .intersperse("/")
            .collect::<String>();
        if self.is_absolute {
            "/".to_owned() + relative_result.as_ref()
        } else {
            relative_result
        }
    }
}

impl Machine {
    // TODO: Return a Result instead of an Option. Use thiserror for the error type
    pub fn traverse(&self, path: Path) -> Option<DirEntry> {
        let path = if path.is_relative() {
            self.cwd.clone() + path
        } else {
            path.to_path()
        };

        let mut current_dir = &self.root_dir;
        for (is_last, component) in path
            .components
            .iter()
            .enumerate()
            .map(|(idx, x)| (idx == path.components.len() - 1, x))
        {
            if is_last {
                if current_dir.files.contains_key(component) {
                    return Some(DirEntry::File(&current_dir.files[component]));
                }
            }
            if current_dir.dirs.contains_key(component) {
                current_dir = &current_dir.dirs[component];
            } else {
                return None;
            }
        }

        Some(DirEntry::Directory(current_dir))
    }
}
