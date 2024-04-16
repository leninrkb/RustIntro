pub fn strings_main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    let mut ss: Option<&mut  str> = s.get_mut(..3);
    dbg!(ss);
    let s1 = String::from("value");
    let s2 = String::from("value2");
    let s3 = s1 + &s2;
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let news = format!("{tic}-{tac}-{toe}");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    dbg!(s);
    for i in hello.as_bytes() {
        println!("{i}");
    }
}