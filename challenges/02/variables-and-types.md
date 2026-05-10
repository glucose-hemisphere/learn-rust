# Challenge 02: `variables-and-types`

- [Challenge 02: `variables-and-types`](#challenge-02-variables-and-types)
  - [Theory](#theory)
    - [Variables with `let`](#variables-with-let)
    - [Primitive scalar types](#primitive-scalar-types)
    - [Type inference and annotations](#type-inference-and-annotations)
    - [Numeric literals](#numeric-literals)
    - [Mutability with `mut`](#mutability-with-mut)
    - [Shadowing](#shadowing)
    - [Constants with `const`](#constants-with-const)
    - [Casting with `as`](#casting-with-as)
    - [Tuples](#tuples)
    - [Fixed-size arrays](#fixed-size-arrays)
  - [Task](#task)
    - [1 — Set up the package](#1--set-up-the-package)
    - [2 — Print a header and two typed bindings](#2--print-a-header-and-two-typed-bindings)
    - [3 — Add a `bool` and a `char`](#3--add-a-bool-and-a-char)
    - [4 — Use `mut` to update a reading](#4--use-mut-to-update-a-reading)
    - [5 — Use shadowing to refine a value](#5--use-shadowing-to-refine-a-value)
    - [6 — Use a `const`](#6--use-a-const)
    - [7 — Numeric literal forms](#7--numeric-literal-forms)
    - [8 — Cast types with `as`](#8--cast-types-with-as)
    - [9 — A tuple for coordinates](#9--a-tuple-for-coordinates)
    - [10 — A fixed-size array of readings](#10--a-fixed-size-array-of-readings)
  - [References](#references)

## Theory

### Variables with `let`

In Rust, you create a variable using the `let` keyword:

```rust
let count = 5;
```

By default, every variable is **immutable**: once a value is bound to it, that value cannot change. The following program is rejected by the compiler before it even runs:

```rust
let count = 5;
count = 6; // error: cannot assign twice to immutable variable `count`
```

Immutability by default is a deliberate design choice — when you read a variable later in the program, you can be confident its value hasn't been silently changed somewhere else.

Rust supports the usual arithmetic operators on numbers — `+`, `-`, `*`, `/`, and `%` (remainder) — so you can write expressions like `let total = count + 1;`.

### Primitive scalar types

Rust has a small fixed set of built-in **primitive** **scalar** types:

| Category          | Types                                      | Notes                                                                                    |
| ----------------- | ------------------------------------------ | ---------------------------------------------------------------------------------------- |
| Signed integers   | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | The number is how many bits are used to represent the integer; `isize` is pointer-sized. |
| Unsigned integers | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | Same idea, but holds non-negative values only.                                           |
| Floating-point    | `f32`, `f64`                               | Single and double precision floats.                                                      |
| Boolean           | `bool`                                     | Exactly `true` or `false`.                                                               |
| Character         | `char`                                     | A single Unicode scalar value (4 bytes wide).                                            |

A `bool` is exactly `true` or `false`:

```rust
let ready = true;
```

A `char` is **not** a single byte — it is a 4-byte Unicode scalar value, written with single quotes (double quotes would make a string instead):

```rust
let letter = 'A';
let symbol = '€';
let glyph  = '中';
```

### Type inference and annotations

Rust is statically typed: every variable has a single, fixed type that is known at compile time. But you don't always have to write the type yourself — the compiler **infers** it from how the value is used:

```rust
let answer = 42;     // inferred as i32 (the default integer type)
let pi     = 3.14;   // inferred as f64 (the default float type)
let yes    = true;   // inferred as bool
```

When inference isn't enough, or when you want to be explicit about the type, you can add an **annotation** with `: Type` after the name:

```rust
let small:          u8  = 200;
let big_positive:   i64 = 2_000_000_000;
let big_negative:   i64 = -2_000_000_000;
let exact:          f32 = 1.5;
```

Note that the "default" types (`i32`, `f64`) only apply when nothing else fixes the type. For example, in `let small: u8 = 200;` the literal `200` is taken as a `u8` because the annotation requires it — there is no inference fallback to `i32`.

### Numeric literals

Numeric literals can be written in several forms:

```rust
let dec = 1_000_000;     // decimal, with underscores for readability
let hex = 0xFF;          // hexadecimal (decimal value: 255)
let oct = 0o77;          // octal       (decimal value: 63)
let bin = 0b1111_0000;   // binary      (decimal value: 240)
```

Underscores can appear anywhere inside the digits and are ignored by the compiler — they exist purely to make long numbers readable. Similarly, the literal forms only affect how the source code reads, not the runtime value:

```rust
println!("{dec}"); // Output: 1000000
println!("{hex}"); // Output: 255
println!("{oct}"); // Output: 63
println!("{bin}"); // Output: 240
```

You can also append a **type suffix** directly to a literal to fix its type at the source:

```rust
let a = 5u8;      // a is u8
let b = 1.5_f32;  // b is f32
```

### Mutability with `mut`

If you need a variable whose value can change, declare it with `mut`:

```rust
let mut score = 0;
score = 10;
score = 20;
```

`mut` means *this binding may be reassigned*. The type is still fixed — `score` here is `i32` and must always hold an `i32`.

### Shadowing

Rust lets you re-declare a variable with the same name using another `let`. This is called **shadowing**: the new binding hides the old one for the rest of the current scope.

```rust
let value = 5;
let value = value + 1;  // a brand-new binding; the old one is no longer reachable
let value = value * 2;
println!("{value}");    // prints: 12
```

Shadowing differs from `mut` in two important ways:

- With `mut`, the variable is the *same* binding and the type cannot change. With shadowing, each `let` creates a *new* binding, so the type is allowed to change.
- A shadowed variable is still immutable, because each `let` produces a fresh immutable binding.

Because shadowing creates a new binding, the type can differ between the old and the new:

```rust
let label = 42;            // i32
let label = "forty-two";   // shadowed; now a textual value of a different type
```

### Constants with `const`

A `const` is a compile-time constant: a fixed value baked into the program. By convention, constants use `SCREAMING_SNAKE_CASE`, and unlike `let`, a `const` **must** have a type annotation.

```rust
const SECONDS_PER_MINUTE: u32 = 60;
const PI: f64 = 3.141592653589793;
```

A `const` differs from a `let` binding in three ways:

- It must be annotated with its type.
- Its value must be computable at compile time (no runtime computation).
- It can be declared at any scope, including at the top level of a file (outside any function), and is visible everywhere inside that scope.

### Casting with `as`

Rust does **not** automatically convert between numeric types — even when the conversion would be safe. You have to convert explicitly using the `as` keyword:

```rust
let big:    i32 = 1000;
let narrow: u8  = big as u8;    // truncates — high bits are discarded
let wide:   f64 = big as f64;   // safe widening conversion
```

A cast with `as` always succeeds at compile time, but it can lose information at runtime. For example, `259_i32 as u8` yields `3`, because only the low 8 bits are kept (`259 - 256 = 3`). And a floating-point value cast to an integer is **truncated towards zero**, so `3.9_f64 as i32` is `3`, and `-3.9_f64 as i32` is `-3`.

### Tuples

A **tuple** groups a fixed number of values, possibly of different types, into one compound value:

```rust
let triple: (i32, i32, i32) = (3, 4, 5);
let mixed = (42, 3.14, true, 'x');  // type is inferred as (i32, f64, bool, char)
```

Once a tuple is created, its length and the type at each position are fixed.

You access individual fields by position with `.0`, `.1`, `.2`, and so on:

```rust
println!("first:  {}", mixed.0); // 42
println!("second: {}", mixed.1); // 3.14
println!("third:  {}", mixed.2); // true
println!("last:   {}", mixed.3); // 'x'
```

You can also **destructure** a tuple by writing a matching pattern on the left of `let`:

```rust
let (a, b, c) = triple;
println!("{a} {b} {c}");
// Output: 3 4 5
```

### Fixed-size arrays

An **array** holds a fixed number of values of the **same** type. Its type is written `[T; N]`, where `T` is the element type and `N` is the length:

```rust
let scores: [i32; 4] = [85, 92, 78, 96];
let zeros:  [u8; 8]  = [0; 8];   // shorthand for [0, 0, 0, 0, 0, 0, 0, 0]
```

You access elements by index with square brackets, starting at 0:

```rust
println!("first: {}", scores[0]);
```

Indexing past the end of the array is a compile error if the index is a constant the compiler can see, and a runtime panic otherwise (the program aborts with an error message).

Every array has a `len()` method that returns the number of elements it holds:

```rust
println!("count: {}", scores.len());  // prints: 4
```

## Task

You will build a program that prints a formatted **daily weather summary** for an imaginary station. Each subtask adds a small piece of code that exercises one of the concepts above.

After every subtask, run `cargo run -p variables-and-types` and confirm the printed output matches what is specified exactly.

This task is composed of **10** subtasks.

### 1 — Set up the package

The workspace was created in Challenge 1, so you just need to add a new package to it.

From the workspace root (`rust-by-challenge/`), make the directory for this challenge:

```sh
mkdir -p challenges/02
```

Then create the package itself:

```sh
cargo new challenges/02/variables-and-types
```

This will automatically add the new package to the `members` list of the workspace's `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = [
    "challenges/01/hello-cargo",
    "challenges/02/variables-and-types",
]
```

The directory structure should now look like:

```sh
rust-by-challenge/
├── Cargo.toml
└── challenges/
    ├── 01/
    │   └── hello-cargo/
    │       ├── Cargo.toml
    │       └── src/
    │           └── main.rs
    └── 02/
        └── variables-and-types/
            ├── Cargo.toml
            └── src/
                └── main.rs
```

Run:

```sh
cargo run -p variables-and-types
```

You should see exactly:

```sh
Hello, world!
```

### 2 — Print a header and two typed bindings

Replace the body of `main` so the program declares two *annotated*, *immutable* variables:

- `temperature_c` of type `f64` with value `18.5`
- `humidity` of type `u8` with value `67`

Print a header line followed by these two values. The output must be exactly:

```sh
=== Daily Weather Summary ===
Temperature: 18.5°C
Humidity: 67%
```

### 3 — Add a `bool` and a `char`

Below the previous bindings, define the *unannotated*, *immutable* variables:

- `is_raining` of type `bool` with value `true`
- `wind_direction` of type `char` with value `N`

Print them on two new lines:

```sh
Raining: true
Wind direction: N
```

### 4 — Use `mut` to update a reading

Below the previous bindings, initialise a *mutable* wind speed variable:

- `wind_speed_kmh` with value `8`

Print a line showing the morning reading. Then **reassign** `wind_speed_kmh` to `15` (no second `let` — that would be shadowing, not mutation), and print a second line showing the evening reading:

```sh
Wind speed (morning): 8 km/h
Wind speed (evening): 15 km/h
```

### 5 — Use shadowing to refine a value

Below the previous code, add an uncalibrated pressure reading in hectopascals (hPa) by defining the *immutable* variable:

- `pressure` with value `1013`

Now apply a calibration offset of `-5` (i.e. subtract `5`) by **shadowing** `pressure` using the old value to compute the new one.

Print the calibrated reading:

```sh
Pressure (calibrated): 1008 hPa
```

### 6 — Use a `const`

At the **top of the file**, before `fn main()`, define a *constant* for the maximum possible humidity:

- `MAX_HUMIDITY` of type `u8` with value `100`

Inside `main`, after the previous code, compute the headroom as `humidity` subtracted from `MAX_HUMIDITY` and print it (the value must be computed, not typed as a literal). The new line must be exactly:

```sh
Humidity headroom: 33%
```

### 7 — Numeric literal forms

Below the previous code, add three bindings, each using a **different literal form**:

- `sensor_id` of type `u32` with value `0xFF_AA_01`
- `alert_mask` of type `u8` with value `0b1010_1010`
- `pressure_pa` of type `u32` with value `101_325`

Print them so that the new lines are exactly:

```sh
Sensor ID:    16755201
Alert mask:   170
Pressure raw: 101325 Pa
```

### 8 — Cast types with `as`

Below the previous code, use `as` casting twice:

1. Convert `temperature_c` (an `f64`) into an `i32`, naming the new variable `temperature_int`.
2. Convert `humidity` (a `u8`) into an `f64`, naming the new variable `humidity_f`.

Print them so that the new lines are exactly:

```sh
Rounded temperature: 18°C
Humidity as float: 67.0
```

For the float, use the appropriate precision specifier to force one decimal place.

### 9 — A tuple for coordinates

Below the previous code, define an annotated tuple of two `f64` values:

- `coordinates` with values `51.501` and `-0.142`

Use **tuple indexing** (not destructuring) to print the latitude and longitude on separate lines. The new lines must be exactly:

```sh
Latitude:  51.501
Longitude: -0.142
```

(Note the two spaces after `Latitude:` so the values line up under each other.)

### 10 — A fixed-size array of readings

Below the previous code, define an annotated array of five recent temperature readings called `recent_temperatures` of type `f64` with the values `16.2`, `17.1`, `18.5`, `19.0`, and `18.4`.

Print three new lines:

1. The total number of readings based on the length of the array.
2. The first reading.
3. The last reading.

The new lines must be exactly:

```sh
Recorded 5 readings.
First reading: 16.2°C
Last reading:  18.4°C
```

(Note the two spaces after `Last reading:` so the temperatures line up under each other.)

After this final subtask, the complete output of the program should be:

```sh
=== Daily Weather Summary ===
Temperature: 18.5°C
Humidity: 67%
Raining: true
Wind direction: N
Wind speed (morning): 8 km/h
Wind speed (evening): 15 km/h
Pressure (calibrated): 1008 hPa
Humidity headroom: 33%
Sensor ID:    16755201
Alert mask:   170
Pressure raw: 101325 Pa
Rounded temperature: 18°C
Humidity as float: 67.0
Latitude:  51.501
Longitude: -0.142
Recorded 5 readings.
First reading: 16.2°C
Last reading:  18.4°C
```

## References

- The Rust Programming Language Book — Chapter 3.1, *Variables and Mutability*: <https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html>
- The Rust Programming Language Book — Chapter 3.2, *Data Types*: <https://doc.rust-lang.org/book/ch03-02-data-types.html>
- Rust by Example — *Variable Bindings*: <https://doc.rust-lang.org/rust-by-example/variable_bindings.html>
- Rust by Example — *Primitives*: <https://doc.rust-lang.org/rust-by-example/primitives.html>
- Rust by Example — *Literals and operators*: <https://doc.rust-lang.org/rust-by-example/primitives/literals.html>
- Rust by Example — *Casting*: <https://doc.rust-lang.org/rust-by-example/types/cast.html>
- Rust by Example — *Tuples*: <https://doc.rust-lang.org/rust-by-example/primitives/tuples.html>
- Rust by Example — *Arrays and Slices*: <https://doc.rust-lang.org/rust-by-example/primitives/array.html>
- The Rust Reference — *Number literals*: <https://doc.rust-lang.org/reference/tokens.html#number-literals>
