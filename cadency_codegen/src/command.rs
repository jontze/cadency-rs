use crate::argument::Argument;

pub(crate) struct Command {
    pub name: String,
    pub description: String,
    pub deferred: bool,
    pub arguments: Vec<Argument>,
}

impl Command {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            deferred: false,
            arguments: Vec::new(),
        }
    }

    pub fn is_deferred(&mut self) {
        self.deferred = true;
    }

    pub fn add_argument(&mut self, argument: Argument) {
        self.arguments.push(argument);
    }

    pub fn name(&mut self, name: String) {
        self.name = name;
    }

    pub fn description(&mut self, description: String) {
        self.description = description;
    }
}
