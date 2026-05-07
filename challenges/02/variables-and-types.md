# Challenge 02: `variables-and-types`

- [Challenge 02: `variables-and-types`](#challenge-02-variables-and-types)
  - [Theory](#theory)
    - [Variables and immutability](#variables-and-immutability)
    - [Mutable bindings: `mut`](#mutable-bindings-mut)
    - [Shadowing](#shadowing)
    - [Primitive scalar types](#primitive-scalar-types)
    - [Type inference and annotations](#type-inference-and-annotations)
    - [Numeric literals](#numeric-literals)
    - [Constants: `const`](#constants-const)
    - [Casting with `as`](#casting-with-as)
    - [Tuples](#tuples)
    - [Fixed-size arrays](#fixed-size-arrays)
    - [Debug formatting](#debug-formatting)
  - [Task](#task)
    - [1 — Set up the package](#1--set-up-the-package)
    - [2 — Print a header](#2--print-a-header)
    - [3 — Declare and print the basic readings](#3--declare-and-print-the-basic-readings)
    - [4 — Add typed values and a constant](#4--add-typed-values-and-a-constant)
    - [5 — Update a value with `mut`](#5--update-a-value-with-mut)
    - [6 — Reuse a name with shadowing](#6--reuse-a-name-with-shadowing)
    - [7 — Convert between numeric types with `as`](#7--convert-between-numeric-types-with-as)
    - [8 — Group values in a tuple](#8--group-values-in-a-tuple)
    - [9 — Store values in a fixed-size array](#9--store-values-in-a-fixed-size-array)
  - [References](#references)

## Theory

### Variables and immutability

In Rust, you introduce a variable with `let`. By default, variables are **immutable** — once a value is bound to a name, you cannot reassign that name:

```rust
let x = 5;
println!("{x}");        // 5
// x = 6;               // error: cannot assign twice to immutable variable
```

This is the opposite default from many languages: you have to *opt in* to mutability. The tradeoff is that immutability is a strong guarantee — once you've read `x`, you know it cannot change behind your back.

### Mutable bindings: `mut`

Adding the `mut` keyword to a `let` makes the binding mutable, so it can be reassigned:

```rust
let mut x = 5;
println!("{x}");        // 5
x = 6;
println!("{x}");        // 6
```

The *type* is fixed by the first assignment, though. Once Rust knows `x` is `i32`, you can only assign other `i32` values to it. `x = "six"` would not compile.

### Shadowing

Rust also lets you re-declare a variable with the same name. The new declaration **shadows** the old one inside the same scope:

```rust
let n = 5;          // i32
let n = n + 1;      // still i32, value is 6
let n = "six";      // shadowed again — type changed from i32 to a string
println!("{n}");    // six
```

Shadowing is **not** the same as `mut`:

- With `mut`, you reassign the same binding; the type cannot change.
- With shadowing, you create a brand-new binding that happens to share the name; the type *can* change.

Shadowing is often used to transform a value through a chain of types while keeping a single, descriptive name.

### Primitive scalar types

Rust's primitive scalar types are:

- **Integers** — fixed-size whole numbers, signed or unsigned:
  - signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
  - unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

  The number is the bit width. `isize` and `usize` are pointer-sized for the target platform (typically 64 bits on modern desktops); they're the types Rust uses for indexing into collections. When inference can't decide, an integer literal defaults to `i32`.

- **Floating-point numbers** — `f32` (32-bit) and `f64` (64-bit), following IEEE 754. The default is `f64`.

- **Boolean** — `bool`, with values `true` and `false`. One byte in size.

- **Character** — `char`, a single Unicode scalar value (4 bytes). Written with single quotes: `'a'`, `'7'`, `'🌧'`. Note: a `char` is *not* a byte and is *not* the same as a one-character string (which would be written with double quotes).

### Type inference and annotations

Rust has a strong type inference engine, so most of the time you don't need to spell out a type:

```rust
let n = 10;         // inferred as i32
let pi = 3.14;      // inferred as f64
let ok = true;      // inferred as bool
let letter = 'Q';   // inferred as char
```

When you want a non-default type, or when you want to be explicit for clarity, write a type annotation after the variable name:

```rust
let n: u8 = 10;
let pi: f32 = 3.14;
```

### Numeric literals

Integer literals are decimal by default. You can also write them in hexadecimal (`0x`), octal (`0o`), or binary (`0b`):

```rust
let dec = 255;
let hex = 0xff;            // 255
let oct = 0o377;           // 255
let bin = 0b1111_1111;     // 255
```

Underscores can appear anywhere inside a numeric literal as visual separators — they are ignored by the compiler:

```rust
let big = 1_000_000;
```

You can also tag a literal with its type as a *suffix*:

```rust
let small = 7u8;            // u8
let amount = 1_000_u64;     // u64, with a separator
let fraction = 2.5f32;      // f32
```

### Constants: `const`

A `const` declares a value that is fixed at compile time. The rules and conventions differ from `let`:

```rust
const SECONDS_IN_HOUR: u32 = 60 * 60;
println!("{SECONDS_IN_HOUR}");      // 3600
```

- A `const` **must** have a type annotation.
- The value must be computable at compile time (e.g. arithmetic on literals — no calling functions that read files, etc.).
- The convention is `SCREAMING_SNAKE_CASE`.
- Constants can live at module/file level — they don't have to be inside `main`.

### Casting with `as`

Rust does not perform implicit numeric conversions. To convert one numeric type to another, use the `as` keyword:

```rust
let n: i64 = 1_000;
let m: i32 = n as i32;     // narrowing: would lose data if n didn't fit, but 1000 fits
let r: f64 = n as f64;     // widening from integer to float
```

Be aware that `as` can silently lose information. For example, `300i32 as u8` yields `44`, because only the low 8 bits are kept. For floats-to-integers, the result is *saturating* — values beyond the target's range clamp to its minimum or maximum. `as` is the right tool when you understand the conversion is safe (or you genuinely want truncation); the standard library has alternative methods for fallible conversion that you'll meet later.

### Tuples

A **tuple** groups a fixed number of values, possibly of different types. Its type is the parenthesised list of its element types:

```rust
let pair: (i32, f64) = (10, 3.14);
println!("{} {}", pair.0, pair.1);    // 10 3.14
```

You access fields by position with `.0`, `.1`, and so on. You can also **destructure** a tuple into separate names with a `let` pattern:

```rust
let (a, b) = pair;
println!("{a} and {b}");              // 10 and 3.14
```

### Fixed-size arrays

A **fixed-size array** holds a known number of values, all of the same type. Its type is `[T; N]` where `T` is the element type and `N` is the length, fixed at compile time:

```rust
let nums: [i32; 3] = [10, 20, 30];
println!("{}", nums[0]);              // 10
println!("{}", nums.len());           // 3
```

Index with square brackets. Indexing past the end panics at runtime — Rust does not allow out-of-bounds reads.

You can also write `[value; N]` to create an array of `N` copies of the same value:

```rust
let zeros: [u8; 4] = [0; 4];          // [0, 0, 0, 0]
```

Arrays live entirely on the stack and have a fixed size, so they're not the right tool for growing or unknown-length sequences — that's what `Vec` is for, in a later challenge.

### Debug formatting

Some types implement the `Display` trait (used by `{}` in format strings), and some don't. Tuples and arrays don't implement `Display`, but they do implement `Debug`. Use `{:?}` to print a value with its `Debug` representation:

```rust
let arr = [1, 2, 3];
println!("{arr:?}");                  // [1, 2, 3]

let pair = (10, 'q');
println!("{pair:?}");                 // (10, 'q')
```

`{:#?}` adds line breaks and indentation for a "pretty" multi-line debug print. Debug format is meant for programmers (in logs, panics, etc.), while Display is meant for end-user-facing output.

## Task

You will build a small program that prints a **weather-station snapshot** — a few readings from an imaginary outdoor sensor. Each subtask adds one more piece of output to the program. After each subtask, run `cargo run -p variables-and-types` and confirm the printed output exactly matches what is specified.

This task is composed of **9** subtasks.

### 1 — Set up the package

From the workspace root you created in Challenge 1, edit `Cargo.toml` and add `"variables-and-types"` to the workspace's `members` list, so it reads (the order doesn't matter):

```toml
[workspace]
resolver = "3"
members = ["hello-cargo", "variables-and-types"]
```

From the workspace root, run:

```sh
cargo new variables-and-types
```

Then run:

```sh
cargo run -p variables-and-types
```

You should see Cargo compile the package, then print exactly:

```sh
Hello, world!
```

### 2 — Print a header

Replace the body of `main()` so that running the program prints exactly:

```sh
=== Weather Station Snapshot ===
```

### 3 — Declare and print the basic readings

Below the header line, declare these six variables and let Rust **infer** the type for each one (no annotations):

```rust
let station = "WX-7";
let channel = 'A';
let temperature = -3;
let pressure = 1013;
let wind_speed = 14.5;
let sensor_online = true;
```

Note the deliberate mix: a string value, a `char` (single quotes), two integer literals (one negative), a float literal, and a boolean.

Print each on its own line. The output lines must be exactly:

```sh
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
```

After this subtask the full output should be:

```sh
=== Weather Station Snapshot ===
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
```

### 4 — Add typed values and a constant

Below the readings, add three more values:

- `battery` — declare with an explicit `u8` annotation and the value `87`. (The point is to *override* the `i32` default with a deliberately chosen smaller type.)
- `samples` — declare with an explicit `i64` annotation and the value `1234567`. Write the literal in source as `1_234_567` so the digits are easy to read. (The underscores are visual only — they do not appear in the output.)
- `MAX_PRESSURE_HPA` — declare a `const` of type `u16` with the value `1100`, **at file scope** (above `main`, not inside it). Use the `SCREAMING_SNAKE_CASE` convention.

Print them so the new lines are exactly:

```sh
Battery: 87
Samples: 1234567
Max safe pressure: 1100
```

After this subtask the full output should be:

```sh
=== Weather Station Snapshot ===
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
Battery: 87
Samples: 1234567
Max safe pressure: 1100
```

### 5 — Update a value with `mut`

Below the previous lines, declare a mutable counter:

```rust
let mut alerts = 0;
```

Print it, then assign the new value `3`, then print it again. The two new output lines must be exactly:

```sh
Alerts: 0
Alerts: 3
```

After this subtask the full output should be:

```sh
=== Weather Station Snapshot ===
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
Battery: 87
Samples: 1234567
Max safe pressure: 1100
Alerts: 0
Alerts: 3
```

### 6 — Reuse a name with shadowing

Below the alerts lines, declare a status code as an integer, print it, then **shadow** the name with a string description and print that:

```rust
let status = 1;
// print here so the line is "Status: 1"
let status = "nominal";
// print here so the line is "Status: nominal"
```

The two `status` bindings have *different types* (an integer, then a string) — this is what shadowing buys you over `mut`. The two new output lines must be exactly:

```sh
Status: 1
Status: nominal
```

After this subtask the full output should be:

```sh
=== Weather Station Snapshot ===
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
Battery: 87
Samples: 1234567
Max safe pressure: 1100
Alerts: 0
Alerts: 3
Status: 1
Status: nominal
```

### 7 — Convert between numeric types with `as`

Below the status lines, define a distance in metres:

```rust
let distance_m: u32 = 12_500;
```

You will print the distance in **kilometres** with two decimal places. To do the division, the integer `distance_m` first has to become a float — Rust will *not* convert it automatically. Use `as` to cast it to `f64`, then divide by `1000.0`, and store the result in a new variable. Then format that variable with two decimal places (the precision specifier you used in Challenge 1).

The output line must be exactly:

```sh
Distance from base: 12.50 km
```

(The point of this subtask is to use `as` to cross between integer and float types — using a float literal for `distance_m` to avoid the cast doesn't count.)

After this subtask the full output should be:

```sh
=== Weather Station Snapshot ===
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
Battery: 87
Samples: 1234567
Max safe pressure: 1100
Alerts: 0
Alerts: 3
Status: 1
Status: nominal
Distance from base: 12.50 km
```

### 8 — Group values in a tuple

Below the distance line, define a coordinate as a tuple of two `f64` values, with an explicit type annotation:

```rust
let coords: (f64, f64) = (51.4769, -0.0005);
```

**Destructure** the tuple into two new variables `lat` and `lon` using a `let` pattern, then print them on a single line. The output line must be exactly:

```sh
Coordinates: 51.4769, -0.0005
```

(The point of this subtask is to use destructuring — accessing `coords.0` and `coords.1` directly doesn't count.)

After this subtask the full output should be:

```sh
=== Weather Station Snapshot ===
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
Battery: 87
Samples: 1234567
Max safe pressure: 1100
Alerts: 0
Alerts: 3
Status: 1
Status: nominal
Distance from base: 12.50 km
Coordinates: 51.4769, -0.0005
```

### 9 — Store values in a fixed-size array

Below the coordinates line, define a fixed-size array of four `f64` values, with an explicit type annotation:

```rust
let recent_wind: [f64; 4] = [12.4, 13.1, 13.7, 14.5];
```

Print the whole array using **debug formatting** (`{:?}`), since arrays don't implement `Display`. Then, on a separate line, print just the last element by indexing into the array. The two new output lines must be exactly:

```sh
Recent wind readings: [12.4, 13.1, 13.7, 14.5]
Latest reading: 14.5
```

After this subtask the full output should be:

```sh
=== Weather Station Snapshot ===
Station: WX-7
Channel: A
Temperature: -3
Pressure: 1013
Wind speed: 14.5
Sensor online: true
Battery: 87
Samples: 1234567
Max safe pressure: 1100
Alerts: 0
Alerts: 3
Status: 1
Status: nominal
Distance from base: 12.50 km
Coordinates: 51.4769, -0.0005
Recent wind readings: [12.4, 13.1, 13.7, 14.5]
Latest reading: 14.5
```

## References

- The Rust Programming Language Book — Chapter 3, *Common Programming Concepts*: <https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html>
- Rust by Example — *Variable Bindings*: <https://doc.rust-lang.org/rust-by-example/variable_bindings.html>
- Rust by Example — *Primitives*: <https://doc.rust-lang.org/rust-by-example/primitives.html>
- Rust by Example — *Casting*: <https://doc.rust-lang.org/rust-by-example/types/cast.html>
- Rust by Example — *Constants*: <https://doc.rust-lang.org/rust-by-example/custom_types/constants.html>
- The `std::fmt` module documentation (Display vs Debug): <https://doc.rust-lang.org/std/fmt/>
- The Rust Reference — *Number literals*: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
