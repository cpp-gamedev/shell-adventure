use crate::world::World;

pub trait Command {
    fn build(&mut self, name: &str) -> Box<dyn Executable>;
}

pub trait Executable {
    fn execute(&mut self, world: &mut World) -> String;
}

impl<T: ToString> Executable for T {
    fn execute(&mut self, _world: &mut World) -> String {
        self.to_string()
    }
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
    pub contents: String,
}

impl Command for PrintCommand {
    fn build(&mut self, _: &str) -> Box<dyn Executable> {
        Box::new(self.contents.clone())
    }
}
