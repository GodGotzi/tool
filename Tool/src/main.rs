use std::env;

mod command;
mod commands;

use crate::command::*;
use crate::commands::*;
use crate::fdesc_cmd::create_copyright_cmd;

fn register_all_commands(command_handler: &mut CommandHandler) {
    //copyright cmd
    let copyright = create_copyright_cmd();
    command_handler.register(copyright);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    //println!("{:?}", args);

    let mut command_handler = CommandHandler::new();
    register_all_commands(&mut command_handler);

    command_handler.run_command(&mut args);
}
