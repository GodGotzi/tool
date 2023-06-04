
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use chrono::DateTime;

use crate::utils::create_folder;
use crate::command::*;

fn create_folder_structure(home: &String) {
    let folder_name = format!("{}\\protocol", home);
    create_folder(folder_name.as_str());

}

fn start(info_file: &mut (File, bool), content: &str, format: &str) {
    if !info_file.1 {
        println!("protocol on {} already started!", content);
        return;
    }

    let date_time = chrono::offset::Local::now();
    let to_write = format!("Start: {:?}", date_time.format(format));

    match info_file.0.write_all(to_write.as_bytes()) {
        Ok(_) => println!("Protocol for {} started", content),
        Err(_) => panic!("Couldn't write to file")
    }

    if let Err(err) = info_file.0.flush() {
        panic!("Couldn't write to file {}", err);
    }

}

fn end(info_file: &mut (File, bool), content: &str, format: &str) {
    if info_file.1 {
        println!("protocol on {} not started! (You need to start the protocol first)", content);
        return;
    }

    let mut buf = String::new();
    info_file.0.read_to_string(&mut buf).unwrap();
    let mut lines: Vec<&str> = buf.lines().collect();
    
    let end = chrono::offset::Local::now();
    let end_str = format!("End: {:?}", end.format(format));

    let start_str = lines.remove(0);
    let start = match DateTime::parse_from_str(start_str, format) {
        Ok(datetime) => datetime.with_timezone(&end.timezone()),
        Err(_) => panic!("Couldn't parse datetime!"),
    };

    let result = start - end;
    let buf_to_write = lines.join("\n");

    let to_write = format!("{}\n{}\n{}\n{}", start_str, end_str, result.num_hours(), buf_to_write);

    match info_file.0.write_all(to_write.as_bytes()) {
        Ok(_) => println!("Protcol for {} ended!", content),
        Err(_) => panic!("Couldn't write to file"),
    }

    if let Err(err) = info_file.0.flush() {
        panic!("Couldn't write to file {}", err);
    }
}

fn time(info_file: &mut (File, bool), content: &str, format: &str) {
    if info_file.1 {
        println!("protocol on {} not started! (You need to start the protocol first)", content);
        return;
    }

    let mut buf = String::new();
    info_file.0.read_to_string(&mut buf).unwrap();
    let lines: Vec<&str> = buf.lines().collect();

    let start_str = lines[0];

    let end = chrono::offset::Local::now();
    let end_str = format!("End: {:?}", end.format(format));

    let start = match DateTime::parse_from_str(start_str, format) {
        Ok(datetime) => datetime.with_timezone(&end.timezone()),
        Err(_) => panic!("Couldn't parse datetime!"),
    };

    let result = start - end;

    println!("Protcol runs since {} for {}", end_str, result.num_hours());
}

fn info(info_file: &mut (File, bool), content: &str, _format: &str) {
    if info_file.1 {
        println!("protocol on {} not started! (You need to start the protocol first)", content);
        return;
    }

    let mut buf = String::new();
    info_file.0.read_to_string(&mut buf).unwrap();

    print!("Protcol text input and include other files (must be in content folder zb File{{test.png}}: ");

    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();

    let to_write = format!("{}\n{}" ,buf ,input_line);

    match info_file.0.write_all(to_write.as_bytes()) {
        Ok(_) => println!("Text for Protocol {} added!", content),
        Err(_) => panic!("Couldn't write to file"),
    }

    if let Err(err) = info_file.0.flush() {
        panic!("Couldn't write to file {}", err);
    }
}

fn run_cmd(command_context: CommandContext) {
    let format = String::from("%Y-%m-%d %H:%M:%S %z");

    if command_context.args.len() != 2 {
        println!("CommandSyntaxError: protocol [content_folder] start/end/time/info");
        return;
    }

    let content = command_context.args.get(0).unwrap();

    let folder_name = format!("{}\\protocol\\{}", command_context.home, content);
    create_folder(folder_name.as_str());

    let info_file_name = format!("{}\\protocol\\{}", command_context.home, content);
    let info_file_path = Path::new(info_file_name.as_str());

    let mut info_file: (File, bool) =  match File::open(info_file_path) {
        Ok(file) => (file, false),
        Err(_) => (File::create(info_file_name.as_str()).unwrap(), true)
    };

    match command_context.args.get(0) {
        Some(arg) => {
            match arg.as_str() {
                "start" => start(&mut info_file, content, &format),
                "end" => end(&mut info_file, content, &format),
                "time" => time(&mut info_file, content, &format),
                "info" => info(&mut info_file, content, &format),
                _ => println!("CommandSyntaxError: protocol [content_folder] start/end/time/info")
            }
        },
        None => println!("CommandSyntaxError: protocol [content_folder] start/end/time/info")
    };

}

pub fn create_protocol_cmd(home: &String) -> Command {
    create_folder_structure(home);

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