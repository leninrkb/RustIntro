pub fn vector_main() {
    let mut specific_type_vector: Vec<i32> = Vec::new();
    let mut same_shit = vec![1,2,3];
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);

    let mut a = vector[0];
    println!("{}",a);
    println!("{}",vector[0]);
    vector[0] = 5;
    println!("{}", a);
}

fn get_vector_element() {
    let mut vector: Vec<u32> = Vec::new();
    vector.push(1);
    vector.push(1);
    vector.push(1);

    let value: Option<&u32> = vector.get(0);
}