/*
	Author: Elias (Gotzi) Gottsbacher
	Copyright (c) 2023 Elias Gottsbacher
*/

use std::fs;
use std::path::Path;
use crate::command::*;
use crate::utils::{create_file, create_folder};

enum FilePosition {
    TOP,
    BOTTOM
}

struct FileModifier {
    modify_file: fn(path: &Path, content_path: &Path, position: &FilePosition)
}

fn run_cmd(command_context: CommandContext, modifier: &FileModifier) {

    if command_context.args.len() != 3 {
        println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]");
        return;
    }

    match command_context.args.get(0) {
        Some(arg) => {
            match arg.as_str() {
                "top" => {
                    iter_project_files(&command_context, modifier, arg, FilePosition::TOP);
                },
                "bottom" => {
                    iter_project_files(&command_context, modifier, arg, FilePosition::BOTTOM);
                },
                _ => println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]")
            }
        },
        None => println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]")
    }
}

fn iter_project_files(command_context: &CommandContext, modifier: &FileModifier, file_position: &str, position: FilePosition) {
    let content_folder = command_context.args.get(1).unwrap();

    let content_path_str = format!("{}\\fdesc-pattern\\{}\\{}.txt",
                    command_context.home, content_folder, file_position);

    let content_path = Path::new( content_path_str.as_str());

    if content_path.exists() {
        loop_files(command_context.from.as_path(), content_path, modifier, command_context.args.get(2).unwrap(), &position);
    } else {
        println!("Content File doesn't exits in {}", content_folder);
    }
}

fn loop_files(dir_path: &Path, content_path: &Path, modifier: &FileModifier, file_ending: &str, position: &FilePosition) {

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if path.file_name().unwrap().to_str().unwrap().ends_with(file_ending) {
                        (modifier.modify_file)(path.as_path(), content_path, position);
                        //modify_file(path.as_path(), content_path, modifier, position);
                    }
                } else {
                    loop_files(path.as_path(), content_path, modifier, file_ending, position);
                }
            }
        }
    } else {
        println!("Error reading directory {:?}", content_path);
    }
}

fn modify_file_add(path: &Path, content_path: &Path, position: &FilePosition) {
    println!("Visited File {:?}", path);

    let source = fs::read_to_string(path).unwrap();
    let content = fs::read_to_string(content_path).unwrap();

    let stuff_to_write = match position {
        FilePosition::TOP => format!("{}\n\n{}", content, source),
        FilePosition::BOTTOM => format!("{}\n\n{}", source, content)
    };

    while match fs::write(path, &stuff_to_write) {
        Ok(_) => false,
        Err(_) => true
    } {
        println!("Couldn't write content retry again!");
    };
}

fn modify_file_remove(path: &Path, content_path: &Path, _position: &FilePosition) {
    println!("Visited File {:?}", path);

    let source = fs::read_to_string(path).unwrap();
    let content = fs::read_to_string(content_path).unwrap();

    let pattern = format!("{}\n\n", content);
    let stuff_to_write = source.replacen(pattern.as_str(), "", 1);

    while match fs::write(path, &stuff_to_write) {
        Ok(_) => false,
        Err(_) => true
    } {
        println!("Couldn't write content retry again!");
    };
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

    let action = |command_action: CommandContext| {
        let modifier = | path: &Path, content_path: &Path, position: &FilePosition | {
            modify_file_add(path, content_path, position);
        };

        let file_modifier = FileModifier {
            modify_file: modifier
        };

        run_cmd(command_action, &file_modifier);
    };

    let label = String::from("fdesc");
    let desc = String::from("Filters all source files and adds descriptions (author, project and more)!");

    let cmd = Command {
        action,
        label,
        desc,
    };

    return cmd;
}

pub fn create_nfdesc_cmd(home: &String) -> Command {
    create_folder_structure(home);

    let action = |command_action: CommandContext| {
        let modifier = | path: &Path, content_path: &Path, position: &FilePosition | {
            modify_file_remove(path, content_path, position);
        };

        let file_modifier = FileModifier {
          modify_file: modifier
        };

        run_cmd(command_action, &file_modifier);
    };

    let label = String::from("nfdesc");
    let desc = String::from("Filters all source files and adds descriptions (author, project and more)!");

    let cmd = Command {
        action,
        label,
        desc,
    };

    return cmd;
}