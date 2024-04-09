fn main() {
    let mut lenin = Person {
        age: 23,
        height: 176,
        weight: 53,
        gender: String::from("male")
    };

    lenin.weight = 53;
    let mut karen = build_person(23);
}

struct Person {
    age: u32,
    height: u32,
    weight: u32,
    gender: String
}

fn build_person(age: u32) -> Person {
    return Person {
        age,
        height: 150,
        weight: 50,
        gender: String::from("female")
    }
}
