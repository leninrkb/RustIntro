fn main() {
    println!("Hello, world!");
    another();
    say_hi("Hayase", "I hope you're fine");

    // expresions
    let i = {
        let j = 3;
        let x = j + 2;
        x + 1
    };
    println!("the value of i is: {i}");

    let s: i8 = times2(20);
    println!("s = {s}");
}

fn times2(number: i8) -> i8 {
    number * 2
}

fn say_hi(name: &str, greeting: &str){
    println!("Hi {name}, how's going? {greeting}.");
}

fn another(){
    println!("another function");
}
