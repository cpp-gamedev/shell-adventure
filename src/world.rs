use std::collections::HashMap;

use crate::{
    commands::{Command, Executable},
    props::Prop,
    Query,
};

pub struct World {
    pub props: HashMap<String, Box<dyn Prop>>,
    pub commands: Vec<Box<dyn Command>>,
}

impl World {
    pub fn parse_command(&mut self, prompt: &Query) -> Option<Box<dyn Executable>> {
        let mut matches = self
            .commands
            .iter_mut()
            .filter_map(|cmd| cmd.regex().find(&prompt.contents).map(|m| (m, cmd)))
            .collect::<Vec<_>>();

        matches.first_mut().map(|(m, cmd)| cmd.build(m))
    }
}
