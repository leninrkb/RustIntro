fn main() {
    let mut lenin = User {
        age: 23,
        height: 176,
        weight: 53,
        gender: String::from("male")
    };

    lenin.weight = 53;
    let mut karen = build_user(23);
    let lenin = Person {
        gender: String::from("male"),
        age: 23
    };
    // lenin.age = 24; not possible
    let mut   karen = Person {
        gender: String::from("female"),
        age: 23
    };
    karen.age = 24;
    let mut alex = build_person();
    alex.age = 25;
    let alexandra = build_person_params(String::from("female"), 25);
    let mut account = Account{
        signed_count: 0,
        account: String::from("leninrkb"),
        password: 123 
    };
    let mut google_account = Account{
        signed_count: account.signed_count,
        account: String::from("google"),
        password: 123
    };
    let another_google = Account {
        signed_count: 2,
        ..google_account
    };
    google_account.account = String::from("asd");
    let account_name = google_account.account;
}

struct Account {
    signed_count: u32,
    account: String,
    password: u32
}

struct User {
    age: u32,
    height: u32,
    weight: u32,
    gender: String
}

fn build_user(age: u32) -> User {
    return User {
        age,
        height: 150,
        weight: 50,
        gender: String::from("female")
    }
}

struct Person {
    gender: String,
    age: u8,
}

fn build_person() -> Person {
    Person {
        gender: String::from("male"),
        age: 20
    }
}

fn build_person_params(gender: String, age: u8) -> Person {
    Person{
        gender,
        age 
    }
}



