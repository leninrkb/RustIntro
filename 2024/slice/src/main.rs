fn main() {
    let love = String::from("Miko is cute");
    let word = &love[0..4];
    println!("{word}");
}

fn first_word(s: &mut String) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate(){
        if byte == b' ' {
            return i;
        }
    }
    return s.len();
}
