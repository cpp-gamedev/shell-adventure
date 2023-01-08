use crate::commands::{Command, PrintCommand};

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
