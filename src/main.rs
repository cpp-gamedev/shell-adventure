use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut world = World {
        props: vec![
            Box::new(Table),
            Box::new(Table),
            Box::new(Table),
            Box::new(Table),
            Box::new(Table),
        ],
    };

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut query = String::new();
        stdin.read_line(&mut query)?;
        let query = Query::new(&query);
        println!("{}\n", world.process_command(&query).to_string());
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

pub type Command = String;

pub trait Prop {
    fn name(&self) -> String;
    fn commands(&self) -> Vec<Command>;
}

pub struct Table;

impl Prop for Table {
    fn name(&self) -> String {
        "Table".to_owned()
    }

    fn commands(&self) -> Vec<Command> {
        vec!["look".to_owned()]
    }
}

pub struct World {
    props: Vec<Box<dyn Prop>>,
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
    pub fn process_command(&mut self, prompt: &Query) -> Box<dyn ToString> {
        match prompt.contents.as_ref() {
            "look" => Box::new(self.to_string()),
            _ => Box::new("Unrecognized command."),
        }
    }
}
