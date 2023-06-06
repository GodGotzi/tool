
use std::fs::{File, self};
use std::io::{self, Write};
use std::path::{PathBuf, Path};

use chrono::{DateTime, Local};

use crate::utils::create_folder;
use crate::command::*;

fn start(file_path: &str, content: &str, format: &str, now: DateTime<Local>) {
    let to_write = format!("$Start: {}", now.format(format));

    match fs::write(file_path, to_write) {
        Ok(_) => println!("Protocol for {} started", content),
        Err(err) => panic!("Couldn't write to file {}", err),
    }
}

fn end(file_path: &Path, file_buf: &str, content: &str, format: &str, now: DateTime<Local>) {

    let mut lines: Vec<&str> = file_buf.lines().collect();
    
    let now_str = format!("$End: {}", now.format(format));

    let start_str = lines.remove(0).replace("$Start: ", "");
    let start = match DateTime::parse_from_str(&start_str, format) {
        Ok(datetime) => datetime.with_timezone(&now.timezone()),
        Err(_) => panic!("Couldn't parse datetime!"),
    };

    let result = start - now;
    let buf_to_write = lines.join("\n");

    let to_write = format!("$Start: {}\n{}\n§{} hours {} minutes\n{}", start_str, now_str, result.num_hours(), -(result.num_minutes() % 60), buf_to_write);

    match fs::write(file_path, to_write) {
        Ok(_) => println!("Protcol for {} ended!", content),
        Err(err) => panic!("Couldn't write to file {}", err),
    }

}

fn time(_file_path: &Path, file_buf: &str, _content: &str, format: &str, now: DateTime<Local>) {

    let lines: Vec<&str> = file_buf.lines().collect();

    let start_str = lines[0].replace("$Start: ", "");

    let start = match DateTime::parse_from_str(&start_str, format) {
        Ok(datetime) => datetime.with_timezone(&now.timezone()),
        Err(_) => panic!("Couldn't parse datetime!"),
    };

    let result = start - now;

    println!("Protocol runs since {} for {} hours and {} minutes", start_str, result.num_hours(), -(result.num_minutes() % 60));
}

fn info(file_path: &Path, file_buf: &str, content: &str, format: &str, now: DateTime<Local>) {

    print!("Protcol Text Input: ");

    if let Err(err) = io::stdout().flush() {
        panic!("{}", err);
    }

    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();

    let now_str = format!("End: {}", now.format(format));

    let to_write = format!("{}\nInfo [{}]: {}", file_buf, now_str, input_line);

    match fs::write(file_path, to_write) {
        Ok(_) => println!("Info for Protocol {} added!", content),
        Err(err) => panic!("Couldn't write to file {}", err),
    }

}

fn run_cmd(command_context: CommandContext) {
    if command_context.args.len() != 3 {
        println!("CommandSyntaxError: protocol [content_folder] [protocol_name] start/end/time/info");
        return;
    }

    let fmt_ymd = String::from("%Y-%m-%d");

    let content = command_context.args.get(0).unwrap();

    let folder_name = format!("protocol\\{}", content);
    create_folder(folder_name.as_str());

    let now = chrono::offset::Local::now();

    if let Some(protocol_name) = command_context.args.get(1) {
        if let Some(mode) = command_context.args.get(2) {
            create_folder(format!("{}\\{}_{}", folder_name, protocol_name, now.format(&fmt_ymd)).as_str());

            match_mode(mode, protocol_name, content, now);
        }
    }
}

fn protocol_is_completed(buf: &str) -> bool {
    let lines: Vec<&str> = buf.lines().collect();
    if lines.len() > 1 {
        lines[1].contains("$End:")
    } else {
        false
    }
}

fn match_mode(arg_mode: &str, arg_protocol: &str, content: &str, now: DateTime<Local>) {
    let format = String::from("%Y-%m-%d %H:%M:%S %z");
    let fmt = String::from("%Y-%m-%d_%H-%M");
    let fmt_ymd = String::from("%Y-%m-%d");
    let folder_path = format!("protocol\\{}\\{}_{}", content, arg_protocol, now.format(&fmt_ymd));

    match arg_mode {
        "start" => {
            let protocol_path = format!("{}\\{}_{}.prtcl", folder_path, arg_protocol, now.format(&fmt));

            if let Ok(file_content) = fs::read_to_string(&protocol_path) {
                if protocol_is_completed(&file_content) {
                    println!("this Protocol is already finished, start a new protocol!");
                    return;
                }
            }

            match File::open(&protocol_path) {
                Ok(_) => {
                    println!("Protocol on {} already started!", content);
                },
                Err(_) => {
                    File::create(&protocol_path).unwrap();
                }
            };

            start(&protocol_path, content, &format, now)
        },
        "end" => {
            let protocol_path = match most_recent_file(&folder_path) {
                Some(path) => path,
                None => {
                    println!("Protocol on {}->{} not started! (You need to start the protocol first)", content, arg_protocol);
                    return;
                }
            };

            if let Ok(file_content) = fs::read_to_string(&protocol_path) {
                if protocol_is_completed(&file_content) {
                    println!("This Protocol is already finished, start a new protocol!");
                    return;
                }

                if File::open(&protocol_path).is_err() {
                    println!("Protocol on {} not started! (You need to start the protocol first)", content);
                    return;
                };
    
                end(&protocol_path, &file_content, content, &format, now)
            } else {
                println!("Protocol on {} not started! (You need to start the protocol first)", content);
            }
        
        },
        "time" => {
            let protocol_path = match most_recent_file(&folder_path) {
                Some(path) => path,
                None => {
                    println!("Protocol on {}->{} not started! (You need to start the protocol first)", content, arg_protocol);
                    return;
                }
            };

            if let Ok(file_content) = fs::read_to_string(&protocol_path) {
                if protocol_is_completed(&file_content) {
                    println!("this Protocol is already finished, start a new protocol!");
                    return;
                }

                if File::open(&protocol_path).is_err() {
                    println!("Protocol on {} not started! (You need to start the protocol first)", content);
                    return;
                };
    
                time(&protocol_path, &file_content, content, &format, now)
            } else {
                println!("Protocol on {} not started! (You need to start the protocol first)", content);
            }
        },
        "info" => {
            let protocol_path = match most_recent_file(&folder_path) {
                Some(path) => path,
                None => {
                    println!("Protocol on {}->{} not started! (You need to start the protocol first)", content, arg_protocol);
                    return;
                }
            };

            if let Ok(file_content) = fs::read_to_string(&protocol_path) {
                if protocol_is_completed(&file_content) {
                    println!("this Protocol is already finished, start a new protocol!");
                    return;
                }

                if File::open(&protocol_path).is_err() {
                    println!("Protocol on {} not started! (You need to start the protocol first)", content);
                    return;
                };
    
                info(&protocol_path, &file_content, content, &format, now)
            } else {
                println!("Protocol on {} not started! (You need to start the protocol first)", content);
            }
        },
        _ => println!("CommandSyntaxError: protocol [content_folder] [protocol_name] start/end/time/info")
    }
}

fn most_recent_file(folder_path: &str) -> Option<PathBuf> {
    let mut entries: Vec<fs::DirEntry> = fs::read_dir(folder_path)
    .expect("Couldn't access local directory")
    .flatten() // Remove failed
    .collect();
    entries.sort_by_cached_key(|f| f.metadata().unwrap().modified().unwrap());

    if !entries.is_empty() {
        Some(entries[0].path())
    } else {
        None
    }

}

pub fn create_protocol_cmd() -> Command {
    create_folder("protocol");

    let action = |command_context: CommandContext| {
        run_cmd(command_context);
    };

    let label = String::from("protocol");
    let desc = String::from("Protocol your work");

    Command {
        action,
        label,
        desc,
    }

}