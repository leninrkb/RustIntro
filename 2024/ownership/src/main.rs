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

    // clone
    let mut clone1 = s2.clone();
    clone1.push_str(" this is different string");
    println!("original: {}, copy: {}",s2, clone1);
    
    // ownership and functions
    let string1 = String::from("tohka");
    let number1: i32 = -123;
    takes_ownership(string1);
    // println!("{string1}"); string1 no longer avialable
    make_copy(number1);
    println!("{number1}");

    // return values and scope
    // Returning values can also transfer ownership
    let takes = give_ownership();
    println!("{takes}");
    let newstring = takes_gives_back(takes);
    println!("{newstring}");
    // println!("{takes}"); takes ownership is now at newstring
    
}

fn takes_ownership(value: String){
    println!("value of {} moved into de fn", value);
}

fn make_copy(value: i32){
    println!("a copy of {value}");
}

fn give_ownership() -> String {
    let temp = String::from("it's yours");
    temp
}

fn takes_gives_back(val: String) -> String {
    val
}
