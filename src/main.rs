pub mod world;

use std::io::Write;

use itertools::Itertools;

use crate::world::{DirEntry, Machine, Path};

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut machine = Machine {
        cwd: Path::root(),
        root_dir: {
            let mut root = world::Directory::new_root();
            root.create_dir("test".to_owned())
                .unwrap()
                .create_file("inside_test".to_owned())
                .unwrap();
            root.create_file("README.txt".to_owned()).unwrap();
            root
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
                let cwd = match machine.traverse(&machine.cwd).unwrap() {
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
                let path = machine.cwd.clone() + Path::parse(path);
                let Some(DirEntry::Directory(traversal_dir)) = machine.traverse(&path) else { println!("No such path: \"{}\"", path.to_string()); continue;};

                machine.cwd = traversal_dir.path().clone();
            }
            ("cd", [..]) => {
                println!("Invalid number of parameters.\nExpected usage: cd <dir>");
            }
            ("cat", [path]) => match machine.traverse(&Path::parse(path)) {
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
