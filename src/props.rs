use crate::commands::{Command, PrintCommand};

pub trait Prop {
    fn name(&self) -> String;
}

pub struct Table;

impl Prop for Table {
    fn name(&self) -> String {
        "Table".to_owned()
    }
}
