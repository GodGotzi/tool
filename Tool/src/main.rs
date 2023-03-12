use std::env;

mod command;
mod commands;
mod utils;

use crate::command::*;
use crate::commands::*;
use crate::fdesc_cmd::create_fdesc_cmd;

fn register_all_commands(command_handler: &mut CommandHandler, home: &String) {

    let fdesc = create_fdesc_cmd(home);
    command_handler.register(fdesc);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let home_dir = args.get(0).unwrap().clone().replace("\\Tool.exe", "");

    let mut command_handler = CommandHandler::new();
    register_all_commands(&mut command_handler, &home_dir);

    command_handler.run_command(&mut args, &home_dir);
}