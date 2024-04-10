fn main() {
    let mut lenin: User = User {
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
    let mut google_account = Account {
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
    let pixel: Color = Color(123, 234, 255);
    let pixel2: Color = Color(pixel.0, pixel.1, 255);
    let mut just_unit: UnitLike = UnitLike;
    let rectangle: Rectangle = Rectangle {
        width: 12,
        height: 34,
    };
    println!("{}", rectangle.area());
    println!("{:#?}", rectangle);
    dbg!(&rectangle);
    dbg!(&rectangle);
    if rectangle.width() {
        println!("width is greatter than 0: {}", rectangle.width);
    }
    let rec1: Rectangle = Rectangle {
        width: 13,
        height: 20
    };
    let rec2: Rectangle = Rectangle {
        width: 12,
        height: 14
    };
    println!("rec1 can hold rect2: {}", rec1.can_hold(&rec2)); 
    let mut rec1 = Rectangle::new();
    rec1.width = 10;
    rec1.height = 23;
    let rec2 = Rectangle::from(23, 78);
    let rec2 = rec1.copy();
    rec1.width = 90;
    println!("{:#?}", rec1);
    println!("{:#?}", rec2);
}

#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize
}

impl Rectangle {
    fn area(self: &Self) -> usize {
        self.width * self.height
    }

    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn new() -> Self {
        Self{
            width: 0,
            height: 0
        }
    }

    fn from(width: usize, height: usize) -> Self {
        Self {
            width,
            height
        }
    }

    fn copy(&self) -> Self {
        Self {
            width: self.width,
            height: self.height
        }
    }
}

struct UnitLike;

struct Color(u8, u8, u8);

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



