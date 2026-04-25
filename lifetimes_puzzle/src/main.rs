fn main() {
    let string_1 = String::from("foo");
    let string_2 = String::from("barbaz");
    println!("{}", longest(&string_1, &string_2));
    println!("{string_1}");
    println!("{string_2}");

    let sentence = String::from("Hello, World!");
    println!("{}", first_word(&sentence));
    println!("{sentence}");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
