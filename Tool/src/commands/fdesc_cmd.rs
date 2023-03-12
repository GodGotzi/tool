use std::fs;
use std::fs::FileType;
use std::path::{Path, PathBuf};
use crate::command::*;
use crate::utils::{create_file, create_folder};

enum FilePosition {
    TOP,
    BOTTOM
}

fn run_cmd(command_context: CommandContext) {

    if command_context.args.len() != 3 {
        println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]");
        return;
    }

    match command_context.args.get(0) {
        Some(arg) => {
            match arg.as_str() {
                "top" => {
                    iter_project_files(&command_context, arg, FilePosition::TOP);
                },
                "bottom" => {
                    iter_project_files(&command_context, arg, FilePosition::BOTTOM);
                },
                _ => println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]")
            }
        },
        None => println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]")
    }
}

fn iter_project_files(command_context: &CommandContext, file_position: &String, position: FilePosition) {
    let content_folder = command_context.args.get(1).unwrap();

    let p = format!("{}\\fdesc-pattern\\{}\\{}.txt",
                    command_context.home, content_folder, file_position);

    let path = Path::new( p.as_str());

    if path.exists() {
        loop_files(&(command_context.from), command_context, &position);
    } else {
        println!("Content File doesn't exits in {content_folder}")
    }
}

fn loop_files(path: &PathBuf, command_context: &CommandContext, position: &FilePosition) {

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if path.file_name().unwrap().to_str().unwrap().ends_with(command_context.args.get(2).unwrap()) {
                        modify_file(entry.path(), position);
                    }
                } else {
                    loop_files(&path, command_context, position);
                }
            }
        }
    } else {
        println!("Error reading directory {:?}", command_context.from);
    }
}

fn modify_file(path: PathBuf, position: &FilePosition) {
    println!("Visited File {:?}", path);






}

fn create_folder_structure(home: &String) {

    let folder_name = format!("{}\\fdesc-pattern", home);
    create_folder(folder_name.as_str());

    let default_folder = format!("{}\\fdesc-pattern\\default", home);
    create_folder(default_folder.as_str());

    let top_file = format!("{}\\fdesc-pattern\\default\\top.txt", home);
    create_file(top_file.as_str());
    let bottom_file = format!("{}\\fdesc-pattern\\default\\bottom.txt", home);
    create_file(bottom_file.as_str());
}

pub fn create_fdesc_cmd(home: &String) -> Command {
    create_folder_structure(home);

    let action = |command_action: CommandContext| run_cmd(command_action);
    let label = String::from("fdesc");
    let desc = String::from("Filters all source files and adds descriptions (author, project and more)!");

    let cmd = Command {
        action,
        label,
        desc,
    };

    return cmd;
}