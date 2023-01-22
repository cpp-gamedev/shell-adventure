use regex::Regex;

use crate::world::World;

pub trait Command {
    fn usage(&self) -> &str;

    fn regex(&self) -> &Regex;

    fn build(&mut self, m: &regex::Match) -> Box<dyn Executable>;
}

pub trait Executable {
    fn execute(&mut self, world: &mut World) -> String;
}

impl<T: ToString> Executable for T {
    fn execute(&mut self, _world: &mut World) -> String {
        self.to_string()
    }
}

pub struct ListCommand {
    regex: Regex,
}

impl ListCommand {
    pub fn new() -> ListCommand {
        ListCommand {
            regex: Regex::new("(list)|(help)").unwrap(),
        }
    }
}

impl Command for ListCommand {
    fn usage(&self) -> &str {
        "list / help"
    }

    fn regex(&self) -> &Regex {
        &self.regex
    }

    fn build(&mut self, _m: &regex::Match) -> Box<dyn Executable> {
        Box::new(ListCommandsExecutable)
    }
}

pub struct ListCommandsExecutable;

impl Executable for ListCommandsExecutable {
    fn execute(&mut self, world: &mut World) -> String {
        let mut str = "Commands available:\n".to_owned();
        for cmd in world.commands.iter() {
            str += cmd.usage();
            str += "\n";
        }
        str.pop();
        str
    }
}

pub struct LookCommand {
    regex: Regex,
}

impl LookCommand {
    pub fn new() -> Self {
        Self {
            regex: Regex::new("look").unwrap(),
        }
    }
}

impl Command for LookCommand {
    fn usage(&self) -> &str {
        "look"
    }

    fn build(&mut self, _m: &regex::Match) -> Box<dyn Executable> {
        Box::new(LookCommandExecutable)
    }

    fn regex(&self) -> &Regex {
        &self.regex
    }
}

pub struct LookCommandExecutable;

impl Executable for LookCommandExecutable {
    fn execute(&mut self, world: &mut World) -> String {
        let mut str = "You see:\n".to_owned();
        for (identifier, prop) in world.props.iter() {
            str += format!("{} ({})\n", prop.name(), identifier).as_str();
        }
        str.pop();
        str
    }
}

#[derive(Clone)]
pub struct PrintCommand {
    pub contents: String,
    pub regex: Regex,
}

impl PrintCommand {
    pub fn new(contents: String, regex: Regex) -> Self {
        Self { contents, regex }
    }
}

impl Command for PrintCommand {
    fn usage(&self) -> &str {
        "print"
    }

    fn build(&mut self, m: &regex::Match) -> Box<dyn Executable> {
        Box::new(self.contents.clone())
    }

    fn regex(&self) -> &Regex {
        &self.regex
    }
}
