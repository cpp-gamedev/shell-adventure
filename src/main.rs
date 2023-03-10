pub mod commands;
pub mod props;
pub mod world;

use std::{collections::HashMap, io::Write};

use crate::{
    commands::{Command, ListCommand, LookCommand},
    props::{Prop, Table},
    world::World,
};

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
    commands.insert("list".to_owned(), Box::new(ListCommand));
    commands.insert("help".to_owned(), Box::new(ListCommand));
    commands.insert("look".to_owned(), Box::new(LookCommand));

    let mut props: HashMap<String, Box<dyn Prop>> = HashMap::new();
    props.insert("table".to_owned(), Box::new(Table));
    let mut world = World { props, commands };

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut query = String::new();
        stdin.read_line(&mut query)?;
        let query = Query::new(&query);
        println!(
            "{}\n",
            match world.parse_command(&query) {
                Some(mut executable) => executable.execute(&mut world),
                None => "Unrecognized command".to_owned(),
            }
        );
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
