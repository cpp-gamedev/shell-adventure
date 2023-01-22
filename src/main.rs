pub mod world;

use std::{collections::HashMap, io::Write};

use crate::world::Machine;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut world = Machine {
        cwd: "/".to_string(),
        dir_tree: world::Directory {
            files: vec![],
            dirs: vec![],
            is_writable: false,
        },
    };

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut query = String::new();
        stdin.read_line(&mut query)?;
        let query = Query::new(&query);
    }
}

pub struct Query {
    contents: String,
}

impl Query {
    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.trim().to_string(),
        }
    }
}
