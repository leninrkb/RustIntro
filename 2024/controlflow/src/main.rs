fn main() {
    let number: i32 = 10;
    if number % 2 == 0 {
        print!("{number} is even");
    }else{
        println!("{number} is odd");
    }

    if number != 0 {
        println!("{number} is something else than zero");
    }

    if number == 1{
        println!("{number} is one");
    }else if number == 2 {
        println!("{number} is two");
    }

    let odd_even: i32 = if number % 2 == 0 {2} else {1};
    println!("variable is {odd_even}");

    // let anerror = if true {123} else {"string"}; incorrect, both fi arms
    // must retur the same type

    // loops
    // loop{
    //     println!("press ctrl + C to stop");
    // }

    let mut counter: u16 = 0;
    let number1: u16 = loop{
        counter += 1;
        if counter >= 100 {
            break counter;
        }
    };

    println!("number2 = {number1}");

    let mut counter = 0;
    'first_loop: loop{
        println!("counter = {counter}");
        let mut remaining = 10;
        'second_loop: loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break 'second_loop;
            }
            if counter == 2 {
                break 'first_loop;
            }
            remaining -= 1;
        };
        counter += 1;
    };
    println!("counter of first loop= {counter}");
    let mut i = 0;
    'mywhile: while i < 10 {
        if i % 5 == 0 && i > 1 {
            break 'mywhile;
        }
        println!("{i}");
        i += 1;
    }

    let names: [&str; 3] = ["lenin","hayase","karen"];
    for name in names {
        println!("current name is: {name}");
    }

    for number in (0..10).rev() {
        println!("current is: {number}");
    }
}
