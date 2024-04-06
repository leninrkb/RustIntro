fn main() {
    let mut name = String::from("Lala");
    let name_length: usize = calculate_length(&name);
    println!("{name_length}");
    add_string(&mut name);
    println!("{name}");
    let mut s1 = String::from("a string");
    let s2 = &s1;
    let s3 = &s1;
    println!("{s2} {s3}");
    let s4 = &mut s1;
    // let d = dangle();
    let d2 = no_dangle();
}

fn calculate_length(val: &String) -> usize {
    val.len()
}

fn add_string(val: &mut String) {
    val.push_str(", hi");
}

fn dangle() -> &String {
    let s = String::from("asdasd");
    &s
}

fn no_dangle() -> String {
    let s = String::from("asd");
    s
}
