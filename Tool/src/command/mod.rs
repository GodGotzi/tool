/*
	Author: Elias (Gotzi) Gottsbacher
	Copyright (c) 2023 Elias Gottsbacher
*/

use std::collections::HashMap;
use std::env;
use std::path::{PathBuf};

pub struct CommandContext {
    pub args: Vec<String>,
    pub label: String,
    pub from: PathBuf,
    pub home: String
}

pub struct Command {
    pub(crate) action: fn(CommandContext),
    pub(crate) label: String,
    pub(crate) desc: String,
}

pub struct CommandHandler {
    commands: HashMap<String, Command>
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        let commands = HashMap::new();

        Self {
            commands
        }
    }

    pub fn register(&mut self, command: Command) {
        self.commands.insert(command.label.clone(), command);
    }

    /*
    pub fn deregister_with_label(&mut self, label: &String) {
        if !self.commands.contains_key(label) {
            return;
        }

        self.commands.remove(label);
    }
     */

    pub fn run_command(&mut self, raw_args: &mut [String], home_dir: &String) {
        let cmd = match raw_args.get(1) {
            Some(str) => str.to_string(),
            None => {
                self.no_command_used();
                return;
            }
        };

        let from = env::current_dir().unwrap();

        let command = match self.commands.get(&cmd) {
            Some(command) => command,
            None => {
                self.command_not_exists();
                return;
            }
        };

        let command_action = CommandContext {
            args: raw_args.iter().skip(2).map(|s| s.to_owned()).collect(),
            label: cmd.to_string(),
            from,
            home: home_dir.to_string()
        };

        (command.action)(command_action);
    }

    fn no_command_used(&self) {
        println!("You need to specify a command you want to use!");
        println!("~~for example~~");

        for entry in self.commands.iter() {
            println!("Command: {:?} Description: {:?}", entry.0, entry.1.desc);
        }

        println!("~~~~~~~~~~~~~~~");
    }

    fn command_not_exists(&self) {
        println!("You need to specify a command that exists!");
        println!("~~for example~~");

        for entry in self.commands.iter() {
            println!("Command: {:?} Description: {:?}", entry.0, entry.1.desc);
        }

        println!("~~~~~~~~~~~~~~~");
    }
}