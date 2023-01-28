pub mod world;

use std::{collections::HashMap, io::Write, path};

use itertools::Itertools;

use crate::world::{DirEntry, Directory, File, Machine, Path};

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut world = Machine {
        cwd: Path::root(),
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
        print!("{} > ", world.cwd.to_string());
        std::io::stdout().flush()?;
        let mut query = String::new();
        stdin.read_line(&mut query)?;
        let Some(query) = Query::new(&query) else { continue; };

        match (query.executable.as_ref(), query.params.as_slice()) {
            ("ls", _) => {
                let cwd = match world.traverse(world.cwd.clone()).unwrap() {
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
                println!("{}", world.cwd.to_string());
            }
            ("cd", [path]) => {
                let path = world.cwd.clone() + Path::parse(path);
                let is_valid = matches!(world.traverse(path.clone()), Some(DirEntry::Directory(_)));

                if is_valid {
                    world.cwd = path;
                } else {
                    println!("No such path: \"{}\"", path.to_string());
                }
            }
            ("cd", [..]) => {
                println!("Invalid number of parameters.\nExpected usage: cd <dir>");
            }
            // TODO: cat
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
