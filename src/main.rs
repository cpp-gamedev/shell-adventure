pub mod world;

use std::{collections::HashMap, io::Write, path};

use itertools::Itertools;

use crate::world::{DirEntry, Directory, File, Machine, PathBuf};

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut machine = Machine {
        cwd: PathBuf::root(),
        root_dir: world::Directory {
            files: HashMap::from_iter(
                [(
                    "README.txt".to_owned(),
                    File {
                        data: "Hello World!".to_owned(),
                        is_executable: false,
                    },
                )]
                .into_iter(),
            ),
            dirs: HashMap::from_iter(
                [(
                    "test".to_owned(),
                    Directory {
                        dirs: Default::default(),
                        files: Default::default(),
                        is_writable: false,
                    },
                )]
                .into_iter(),
            ),
            is_writable: false,
        },
    };

    loop {
        print!("{} > ", machine.cwd.to_string());
        std::io::stdout().flush()?;
        let mut query = String::new();
        stdin.read_line(&mut query)?;
        let Some(query) = Query::new(&query) else { continue; };

        match (query.executable.as_ref(), query.params.as_slice()) {
            ("ls", _) => {
                let cwd = match machine.traverse(machine.cwd.as_view()).unwrap() {
                    world::DirEntry::Directory(dir) => dir,
                    world::DirEntry::File(_) => unreachable!(),
                };
                let result = cwd
                    .dirs
                    .keys()
                    .map(String::as_ref)
                    .chain(cwd.files.keys().map(String::as_ref))
                    .intersperse("\t")
                    .collect::<String>();
                println!("{}", result);
            }
            ("cwd", _) => {
                println!("{}", machine.cwd.to_string());
            }
            ("cd", [path]) => {
                let path = machine.cwd.clone() + PathBuf::parse(path).as_view();
                let is_valid = matches!(
                    machine.traverse(path.as_view()),
                    Some(DirEntry::Directory(_))
                );

                if is_valid {
                    machine.cwd = path;
                } else {
                    println!("No such path: \"{}\"", path.to_string());
                }
            }
            ("cd", [..]) => {
                println!("Invalid number of parameters.\nExpected usage: cd <dir>");
            }
            ("cat", [path]) => match machine.traverse(PathBuf::parse(path).as_view()) {
                Some(DirEntry::File(file)) => println!("{}", file.data),
                _ => println!("TODO: better error messsage (invalid cat target)"),
            },
            ("quit" | "exit", [..]) => {
                return Ok(());
            }
            _ => {
                println!("Unrecognized executable or command");
            }
        }
    }
}

pub struct Query {
    executable: String,
    params: Vec<String>,
}

impl Query {
    pub fn new(contents: &str) -> Option<Self> {
        let mut parts = contents.split_whitespace();
        let executable = parts.next()?.to_owned();
        let params = parts.map(ToOwned::to_owned).collect();
        Some(Self { executable, params })
    }
}
