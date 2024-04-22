use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;

pub fn body() {
    let file = File::open("../maifn.rs");
    // error_handler(&file);
    error_handler_fully(&file);
}

fn error_handler(file: &Result<File, Error>) {
    match file {
        Result::Ok(ok) => println!("everything's fine..."),
        Result::Err(error) => println!("holy crab..."),
    }
}

fn error_handler_fully(result: &Result<File, Error>) {
    match result {
        Result::Ok(file) => {
            println!("file is ok...");
        },
        Result::Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    let created = File::create("delete_this_file.txt");
                    match created {
                        Ok(ok) => {
                            println!("file created succesfully");
                        },
                        Err(err) => {
                            println!("file cannot be created...");
                        },
                    }
                },
                other => println!("there;s an error..."),
            }
        },
    } 
}
