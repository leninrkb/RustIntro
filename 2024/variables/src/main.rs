fn main() {
    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");

    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;
    println!("one hour in seconds is: {ONE_HOUR_IN_SECONDS}s"); 

    let inmutable = "lalala";
    {
        let inmutable = "lololo";
        println!("inmutable var shadowed inside a new scope: {inmutable}");
    }
    println!("inmutable var out of new scope: {inmutable}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");

    
}
