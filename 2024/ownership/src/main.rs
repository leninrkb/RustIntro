fn main() {
    // the string type 
    let mut name = String::from("hayase");
    name.push_str(", i love you");
    println!("{}", name);     

    let x = 5;
    let y = x;
    // 5 is pushed twice on to the stack

    let s1 = String::from("hayase");
    let s2 = s1;
    // both variables point to the same location
    // after s2 = s1 rust rust consider s1 as no longer valid
    println!("{}", s2);
}
