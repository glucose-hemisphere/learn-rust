fn main() {
    let name: &str = "Alice";
    let age: u8 = 30;
    let mut count: u32 = 0;
    count += 1;
    println!("Hi, I'm {name}. I am {age} years old. This program has run {count} time(s).");
}

// Output:
//
// Hi, I'm Alice. I am 30 years old. This program has run 1 time(s).
