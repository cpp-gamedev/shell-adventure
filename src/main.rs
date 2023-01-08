use std::{collections::HashMap, io::Write};

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
    commands.insert("list".to_owned(), Box::new(ListCommand));
    commands.insert("help".to_owned(), Box::new(ListCommand));
    let mut world = World {
        props: vec![
            Box::new(Table),
            Box::new(Table),
            Box::new(Table),
            Box::new(Table),
            Box::new(Table),
        ],
        commands,
    };

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

pub trait Executable {
    fn execute(&mut self, world: &mut World) -> String;
}

impl<T: ToString> Executable for T {
    fn execute(&mut self, _world: &mut World) -> String {
        self.to_string()
    }
}

pub trait Command {
    fn build(&mut self, name: &str) -> Box<dyn Executable>;
}

pub struct ListCommand;

impl Command for ListCommand {
    fn build(&mut self, _name: &str) -> Box<dyn Executable> {
        Box::new(ListCommandsExecutable)
    }
}

pub struct ListCommandsExecutable;

impl Executable for ListCommandsExecutable {
    fn execute(&mut self, world: &mut World) -> String {
        let mut str = "Commands available:\n".to_owned();
        for name in world.commands.keys() {
            str += &(name.clone() + "\n");
        }
        str.pop();
        str
    }
}

#[derive(Clone)]
pub struct PrintCommand {
    contents: String,
}

impl Command for PrintCommand {
    fn build(&mut self, _: &str) -> Box<dyn Executable> {
        Box::new(self.contents.clone())
    }
}

pub trait Prop {
    fn name(&self) -> String;
    fn commands(&self) -> Vec<Box<dyn Command>>;
}

pub struct Table;

impl Prop for Table {
    fn name(&self) -> String {
        "Table".to_owned()
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![Box::new(PrintCommand {
            contents: "A table.".to_owned(),
        })]
    }
}

pub struct World {
    props: Vec<Box<dyn Prop>>,
    commands: HashMap<String, Box<dyn Command>>,
}

impl ToString for World {
    fn to_string(&self) -> String {
        self.props
            .iter()
            .map(|prop| format!("{}", prop.name()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl World {
    pub fn parse_command(&mut self, prompt: &Query) -> Option<Box<dyn Executable>> {
        self.commands
            .iter_mut()
            .find(|(name, _cmd)| name.as_str() == &prompt.contents)
            .map(|(name, cmd)| cmd.build(name))
    }
}
