use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;

pub fn body() {
    let file = File::open("../maifn.rs");
    error_handler(&file);
    error_handler_fully(&file);
}

fn error_handler(file: &Result<File, Error>) {
    match file {
        Result::Ok(ok) => println!("tudu ben"),
        Result::Err(error) => println!("error bro"),
    }
}

fn error_handler_fully(file: &Result<File, Error>) {
    match file {
        Result::Ok(_file) => (),
        Result::Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("main_test.rs") {
                Result::Ok(f) => { 
                    println!("tudu bien mao mio");
                },
                Result::Err(error) => {
                    println!("[ERROR]: no pude crear el archivo bro :( disculpame");
                },
            },
            other_error => {
                panic!("[ERROR]: juesumadre llamen a dios!")
            }
        },
    }
}
