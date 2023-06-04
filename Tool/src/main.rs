/*
	Author: Elias (Gotzi) Gottsbacher
	Copyright (c) 2023 Elias Gottsbacher
*/

use std::env;

mod command;
mod commands;
mod utils;

use commands::protocol_cmd::create_protocol_cmd;

use crate::command::*;
use crate::commands::*;
use crate::fdesc_cmd::{create_fdesc_cmd, create_nfdesc_cmd};

fn register_all_commands(command_handler: &mut CommandHandler) {

    let fdesc = create_fdesc_cmd();
    command_handler.register(fdesc);

    let nfdesc = create_nfdesc_cmd();
    command_handler.register(nfdesc);

    let protocol = create_protocol_cmd();
    command_handler.register(protocol);
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut args: Vec<String> = env::args().collect();

    let mut command_handler = CommandHandler::new();
    register_all_commands(&mut command_handler);

    command_handler.run_command(&mut args);
}