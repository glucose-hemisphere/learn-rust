fn main() {
    // let name= "Alice"; // This would also work, due to type inference
    let name = "Alice";
    let age: u8 = 30;
    let mut count: u32 = 0;
    count += 1;
    println!("Hi, I'm {name}. I am {age} years old. This program has run {count} time(s).");
}

// Output:
//
// Hi, I'm Alice. I am 30 years old. This program has run 1 time(s).
