
use crate::command::*;

fn run_cmd(command_action: CommandAction) {
    println!("Copyright Command has been executed!");
    println!("{:?}", command_action.args);
}

pub fn create_copyright_cmd() -> Command {
    let action = |command_action: CommandAction| run_cmd(command_action);
    let label = String::from("copyright");
    let desc = String::from("Filters all source files and adds Copyright information");

    let cmd = Command {
        action,
        label,
        desc,
    };

    return cmd;
}