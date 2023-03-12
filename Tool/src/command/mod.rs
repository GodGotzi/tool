
pub struct Command {
    action:
}

pub struct CommandHandler {
    commands: Vec<Command>
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        let commands =  Vec::new();

        Self {
            commands
        }
    }

    pub fn register(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn deregister(&mut self, command: &Command) {
        if !self.commands.contains(command) {
            return;
        }

        if let Some(pos) = self.commands.iter().position(|x| *x == elem) {
            self.commands.remove(pos);
        }
    }
}