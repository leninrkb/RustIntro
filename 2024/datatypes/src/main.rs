fn main() {
    // type subsets: scalar and compound
    let number = "123";
    let number:u32 = number.parse().expect("error");
    println!("{number}");

    // scalar types
    // represents a single value: integers, floating-point numbers, Booleans,
    // and characters

    // integers types
    // default is i32
    // unsigned variants store numbers from 0 to (2^n)-1 
    // signed variants store numbers from -(2^(n-1)) to (2^(n-1))-1 

    let integer_32bits_unsigned: u32 = 000; // n = 32, (2^32)-1
    println!("hi im u32 and i can store {integer_32bits_unsigned} bites!");

    // floating-point types
    // default type is f64
    // this types are all signed
    let floating_f64: f64 = 123.12; // it has double precision
    let floating_f32: f32 = 123.12; // it has single precision

    // numeric operations 
    // adition
    let sum = 2 + 3; // default i32

    // substraction
    let difference = 12.1 - 2.2;

    // multiplication
    let product = 23 * 23;

    // division
    let quotient = 23.2 / 123.1;
    let truncated = -5 / 3; // r = -1

    //remainder
    let remainder = 12 % 3;

    // Boolean type
    let isvalid: bool = false;
    let isvalid = true; 

    // character type
    let c = 'a';
    let mychar: char = 'a';

    // compound types
    // tuple type
    let tuple: (u8, i8, char, f64) = (255, -128, 'a', 111.111);
    let (age, cash, mychar, float) = tuple;
    let first = tuple.0;
    let third = tuple.2;
    let last = tuple.3;

    // array type
    let myarray = [1,2,3,4,5];
    let myarray2: [u8; 3] = [6,5,7];
    let initialize_array = [0; 10];
    let first_array_el = initialize_array[0];
    let last_array_el = initialize_array[initialize_array.len()-1];

}
