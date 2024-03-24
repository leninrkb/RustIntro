use std::io;

fn main() {
    println!("Welcome to the guessing GAME !!!!");
    println!("Guess the number!");
    let mut guess = String::new();
    let name = "lenin"; //inmutable
    let mut last_name = "acosta"; //mutable
    io::stdin()
    .read_line(& mut guess)
    .expect("failes to read line...");
    println!("you guessed the number: {guess}");
    placeholders();
}

fn placeholders(){
    let x = 1;
    let y = 10;
    println!("x = {x} and x + y = {}",y + x);
}
