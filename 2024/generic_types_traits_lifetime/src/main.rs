fn main() {
    let numbers = vec![1,2,3];
    let mut largest = &numbers[0];
    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    dbg!(largest);
    dbg!(numbers);
    let numbers = vec![1,2,3,5,6,7,8];
    let largest = largest_number(&numbers);

}

fn largest_number(numbers: &[i32]) -> &i32 {
    let largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    return &largest;
}
