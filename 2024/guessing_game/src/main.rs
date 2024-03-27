use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    // first_sample();
    // placeholders();
    // comparing_secret_number();
    comparing_secret_number_loop();
}

fn first_sample() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_number}");
    println!("Welcome to the guessing GAME !!!!");
    println!("Guess the number!");
    let mut guess = String::new();
    let _name = "lenin"; //inmutable
    let mut _last_name = "acosta"; //mutable
    io::stdin()
        .read_line(&mut guess)
        .expect("failes to read line...");
    println!("you guessed the number: {guess}");
}

fn placeholders() {
    let x = 1;
    let y = 10;
    println!("x = {x} and x + y = {}", y + x);
}

fn comparing_secret_number() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut user_input = String::new();
    println!("write a number:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    let user_input: u32 = user_input
        .trim()
        .parse()
        .expect("please write a NUMBER you stupid piece of shit :) ");
    match user_input.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win!"),
    }
}

fn comparing_secret_number_loop() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut user_input = String::new();
        println!("enter a NUMBER between 1 and 100:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("[error] failed to read line");
        println!("user input: {user_input}");
        let user_input: u32 = match user_input
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
        match user_input.cmp(&secret_number) {
            Ordering::Greater => println!("number too big"),
            Ordering::Less => println!("number too small"),
            Ordering::Equal => {
                println!("you win! stupid piece of shit !!!");
                break;
            }
        }
    }
}
