use std::{
    fs::File,
    io::{ErrorKind, Read},
};

pub fn body() {
    let file = File::open("../maifn.rs");
    dbg!(&file);
    // match file {
    //     Result::Ok(file) => {
    //         dbg!(file);
    //         println!("todo bien");
    //     }
    //     Result::Err(error) => {
    //         dbg!(error);
    //         println!("[ERROR]: ayy! :( yo mejor me voy...");
    //         println!(" * termina el programa xd *");
    //     }
    // }

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
