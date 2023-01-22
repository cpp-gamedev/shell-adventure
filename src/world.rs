use std::collections::HashMap;

use crate::Query;

pub struct File {
    pub data: String,
    pub is_executable: bool,
}

pub struct Directory {
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
    pub is_writable: bool,
}

pub struct Machine {
    pub cwd: String,
    pub dir_tree: Directory,
}
