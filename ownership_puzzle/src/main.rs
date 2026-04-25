fn main() {
    let mut original: String = String::from("Hello, World!");

    let immutable_borrow: &String = &original;
    print_length(immutable_borrow);

    let mutable_borrow: &mut String = &mut original;
    make_loud(mutable_borrow);

    println!("{original}");
}

fn print_length(s: &str) {
    let length: usize = s.len();
    println!("{length}");
}

fn make_loud(s: &mut str) {
    s.make_ascii_uppercase();
    println!("{s}");
}
