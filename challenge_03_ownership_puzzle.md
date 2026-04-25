# Challenge 3: The Ownership Puzzle

## The Theory

This is Rust's big idea. No garbage collector, no manual `malloc`/`free` — instead, a set of rules the compiler enforces at compile time.

**The core problem Rust is solving:**
Memory has to be *allocated* when you create data, and *freed* when you're done with it. Every other language either makes you do this manually (C/C++, prone to bugs) or runs a garbage collector to clean up (Python, Java — safe but adds overhead). Rust does neither. It uses **ownership rules** instead.

**The three rules:**

1. Every value has exactly one owner
2. When the owner goes out of scope, the value is dropped (memory freed)
3. There can only ever be one owner at a time

---

### 1. Move semantics — when ownership transfers

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED into s2 — s1 no longer exists
println!("{}", s1);  // ❌ compiler error: s1 was moved
```

### 2. Borrowing — lending without transferring

```rust
let s1 = String::from("hello");
let s2 = &s1;  // s2 borrows s1 — s1 still owns the data
println!("{} {}", s1, s2);  // ✅ both work
```

Think of it like a library book. You (the owner) can lend it to a friend (`&s1`), but you still own it. You can't give it away twice though.

### 3. Mutable borrowing

```rust
let mut s = String::from("hello");
let s2 = &mut s;  // mutable borrow
s2.push_str(", world");
```

The key rule: **you can have many immutable borrows, OR exactly one mutable borrow — never both at once.** This is how Rust prevents data races at compile time.

---

## Your Task

Write a program that demonstrates you understand ownership and borrowing. It must do all of the following:

1. Create a `String` called `original`
2. Create a function `print_length` that takes an **immutable borrow** of a `String` and prints its length — `original` must still be usable after calling it
3. Create a function `make_loud` that takes a **mutable borrow** of a `String` and converts it to uppercase in place (hint: `s.make_ascii_uppercase()`)
4. After calling both functions, print `original` to prove it still lives

---

## Acceptance Criteria

- ✅ `print_length` takes `&String`, not `String` (no ownership transfer)
- ✅ `make_loud` takes `&mut String`
- ✅ `original` is still usable and printed at the end — proving the functions only borrowed it
- ✅ Compiles with no warnings

---

## Solution

```rust
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
```

## Notes

- `&str` is more idiomatic than `&String` in function signatures — it accepts *any* string type, not just heap-allocated `String`s.
- The compiler allows the mutable borrow in `make_loud` because the immutable borrow is no longer in use by that point. If both were active simultaneously, the compiler would refuse.
- Lifetime elision means `first_word(s: &str) -> &str` doesn't need explicit annotations — the compiler infers the output borrows from the input. `longest` needs explicit annotations because it takes two inputs and the compiler can't guess which one the return value points at.

---

## Bonus Challenge: Lifetimes

### Additional Concepts

Returning a reference from a function raises a question for the compiler: *how long does the thing you're pointing at live?*

Lifetime annotations make relationships explicit:

```rust
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
```

Read `'a` as: *"some lifetime I'm calling 'a"*. This says: *"given two string references that both live at least as long as `'a`, I'll return a reference that also lives at least as long as `'a`."*

### Bonus Task

Write a program with two functions:

1. `longest<'a>(s1: &'a str, s2: &'a str) -> &'a str` — returns the longer of two string slices with no cloning
2. `first_word(s: &str) -> &str` — returns a slice containing only the first word

### Bonus Solution

```rust
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
```
