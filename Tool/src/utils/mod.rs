/*
	Author: Elias (Gotzi) Gottsbacher
	Copyright (c) 2023 Elias Gottsbacher
*/

use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

pub fn create_folder(folder_name: &str) {
    match fs::create_dir(folder_name) {
        Ok(_) => {},
        Err(err) => {
            if err.kind() != ErrorKind::AlreadyExists {
                println!("Couldn't create directory {}", err);
            }
        }
    }
}

pub fn create_file(file_name: &str) {
    let path = Path::new(file_name);

    if !path.exists() {
        match File::create(file_name) {
            Ok(_) => {},
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    panic!("Couldn't create file {}", err);
                }
            }
        }
    }
}