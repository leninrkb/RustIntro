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
    dbg!(vector);
}

pub fn get_vector_element() {
    let mut vector: Vec<u32> = Vec::new();
    vector.push(1234);
    vector.push(1);
    vector.push(1);

    let value: Option<&u32> = vector.get(0);
    dbg!(value);
}

pub fn out_of_range() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let i = v.get(3);
    dbg!(i);
}

pub fn iterating_over_vector() {
    let mut v = vec![4,5,6,5,4,3,4,5,2,4,3,2,1];
    for i in &mut v {
        *i += *i;
    }
    dbg!(v);
}

struct Persona {
    name: String,
    age: u8
}

enum Human {
    Ecuadorian,
    Japanese,
    Russian,
}

pub fn different_types() {
    let mut people = vec![];
    people.push(Persona{ name: String::from("alex"), age: 24 });
    people.push(Persona{ name: String::from("karen"), age: 24 });

    let mut humans = vec![];
    humans.push(Human::Ecuadorian);
    humans.push(Human::Japanese);
    humans.push(Human::Japanese);
    humans.push(Human::Russian);
}