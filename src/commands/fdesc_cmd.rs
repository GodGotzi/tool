/*
	Author: Elias (Gotzi) Gottsbacher
	Copyright (c) 2023 Elias Gottsbacher
*/

use std::fs;
use std::path::Path;
use crate::command::*;
use crate::utils::{create_file, create_folder};

enum FilePosition {
    Top,
    Bottom
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
                    iter_project_files(&command_context, modifier, arg, FilePosition::Top);
                },
                "bottom" => {
                    iter_project_files(&command_context, modifier, arg, FilePosition::Bottom);
                },
                _ => println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]")
            }
        },
        None => println!("CommandSyntaxError: fdesc top/bottom [content_folder] [file_ending]")
    }
}

fn iter_project_files(command_context: &CommandContext, modifier: &FileModifier, file_position: &str, position: FilePosition) {
    let content_folder = command_context.args.get(1).unwrap();

    let content_path_str = format!("fdesc-pattern\\{}\\{}.txt",
                    content_folder, file_position);

    let content_path = Path::new( content_path_str.as_str());

    if content_path.exists() {
        loop_files(command_context.from.as_path(), content_path, modifier, command_context.args.get(2).unwrap(), &position);
    } else {
        println!("Content File doesn't exits in {}", content_folder);
    }
}

fn loop_files(dir_path: &Path, content_path: &Path, modifier: &FileModifier, file_ending: &str, position: &FilePosition) {

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
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
    } else {
        println!("Error reading directory {:?}", content_path);
    }
}

fn modify_file_add(path: &Path, content_path: &Path, position: &FilePosition) {
    println!("Visited File {:?}", path);

    let source = fs::read_to_string(path).unwrap();
    let content = fs::read_to_string(content_path).unwrap();

    let stuff_to_write = match position {
        FilePosition::Top => format!("{}\n\n{}", content, source),
        FilePosition::Bottom => format!("{}\n\n{}", source, content)
    };

    while fs::write(path, &stuff_to_write).is_err() {
        println!("Couldn't write content retry again!");
    };
}

fn modify_file_remove(path: &Path, content_path: &Path, _position: &FilePosition) {
    println!("Visited File {:?}", path);

    let source = fs::read_to_string(path).unwrap();
    let content = fs::read_to_string(content_path).unwrap();

    let pattern = format!("{}\n\n", content);
    let stuff_to_write = source.replacen(pattern.as_str(), "", 1);

    while fs::write(path, &stuff_to_write).is_err() {
        println!("Couldn't write content retry again!");
    };
}

fn create_folder_structure() {
    create_folder("fdesc-pattern");
    create_folder("fdesc-pattern\\default");
    create_file("fdesc-pattern\\default\\top.txt");
    create_file("fdesc-pattern\\default\\bottom.txt");

}

pub fn create_fdesc_cmd() -> Command {
    create_folder_structure();

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

    Command {
        action,
        label,
        desc,
    }

}

pub fn create_nfdesc_cmd() -> Command {
    create_folder_structure();

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

    Command {
        action,
        label,
        desc,
    }

}