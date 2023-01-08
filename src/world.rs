use std::collections::HashMap;

use crate::{
    commands::{Command, Executable},
    props::Prop,
    Query,
};

pub struct World {
    pub props: HashMap<String, Box<dyn Prop>>,
    pub commands: HashMap<String, Box<dyn Command>>,
}

impl World {
    pub fn parse_command(&mut self, prompt: &Query) -> Option<Box<dyn Executable>> {
        self.commands
            .iter_mut()
            .find(|(name, _cmd)| name.as_str() == &prompt.contents)
            .map(|(name, cmd)| cmd.build(name))
    }
}
