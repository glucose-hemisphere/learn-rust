# Challenge 01: `hello-cargo`

- [Challenge 01: `hello-cargo`](#challenge-01-hello-cargo)
  - [Theory](#theory)
    - [What is Cargo?](#what-is-cargo)
    - [Creating and running a package](#creating-and-running-a-package)
    - [Cargo workspaces](#cargo-workspaces)
    - [The printing macros](#the-printing-macros)
    - [Format strings](#format-strings)
    - [Format specifiers](#format-specifiers)
  - [Task](#task)
    - [1 — Set up the workspace and the package](#1--set-up-the-workspace-and-the-package)
    - [2 — Print a header](#2--print-a-header)
    - [3 — Print the venue using positional arguments](#3--print-the-venue-using-positional-arguments)
    - [4 — Print the artist using a named argument](#4--print-the-artist-using-a-named-argument)
    - [5 — Print the price with two decimal places](#5--print-the-price-with-two-decimal-places)
    - [6 — Print a right-aligned seat label](#6--print-a-right-aligned-seat-label)
    - [7 — Build a footer with `format!`](#7--build-a-footer-with-format)
    - [8 — Write a diagnostic to stderr](#8--write-a-diagnostic-to-stderr)
  - [References](#references)

## Theory

### What is Cargo?

Cargo is Rust's build tool and package manager. It compiles your code, manages dependencies, runs tests, and generates documentation. Almost every Rust project is built with Cargo.

A Cargo **package** contains a `Cargo.toml` manifest file (which describes the package — its name, version, dependencies) and a `src/` directory containing source code. A package that produces an executable has a `src/main.rs` whose `main` function is the program's entry point.

### Creating and running a package

To create a new executable package:

```sh
cargo new my-app
```

This produces:

```sh
my-app/
├── Cargo.toml
└── src/
    └── main.rs
```

The generated `src/main.rs` contains a small "Hello, world!" program. To compile and run it:

```sh
cargo run
```

To compile without running:

```sh
cargo build
```

Both produce a `target/` directory containing build artefacts. By default, builds are unoptimised (fast to compile, slow to run — ideal during development). Pass `--release` for an optimised build.

### Cargo workspaces

A **workspace** is a collection of related packages that share a single `Cargo.lock` file and a single `target/` directory. The workspace root contains a `Cargo.toml` that lists its member packages:

```toml
[workspace]
resolver = "3"
members = [
  "my-app",
  "my-lib",
]
```

You will create a workspace once at the start of this challenge, then add a new package for each subsequent challenge.

### The printing macros

Rust provides several macros for printing text. A macro is invoked with a `!` after the name:

| Macro       | Stream | Newline appended? |
| ----------- | ------ | ----------------- |
| `print!`    | stdout | No                |
| `println!`  | stdout | Yes               |
| `eprint!`   | stderr | No                |
| `eprintln!` | stderr | Yes               |

`stdout` is the program's normal output stream; `stderr` is a separate stream typically used for diagnostic and error messages. They can be redirected independently by the shell, so logs and "real" output don't get mixed up.

There is also `format!`, which behaves like `println!` but instead of printing, *returns* the formatted text as a `String` value that you can store in a variable.

### Format strings

The first argument to these macros is a **format string** containing literal text and `{}` placeholders. Subsequent arguments fill in the placeholders in order:

```rust
println!("{} is {} years old", "Ada", 36);
// Output: Ada is 36 years old
```

You can also use **positional** arguments by putting an index inside the braces. This lets you reuse or reorder arguments:

```rust
println!("{0}, {1}, and {0} again", "foo", "bar");
// Output: foo, bar, and foo again
```

Or **named** arguments. When a name appears inside the braces and a variable with that name is already in scope, Rust uses it directly — there's no need to pass it as a separate argument:

```rust
let title = "Encyclopedia";
let pages = 1000;
println!("{title} has {pages} pages");
// Output: Encyclopedia has 1000 pages
```

### Format specifiers

Inside the braces, after a colon, you can request specific formatting. A few common specifiers:

- `{:.N}` — print a floating-point number with `N` digits after the decimal point.

  ```rust
  println!("{:.3}", 3.14159);   // Output: 3.142
  ```

- `{:>W}` — right-align within a field of width `W`. Use `<` for left-align, `^` for centre-align.

  ```rust
  println!("[{:>8}]", "hi");    // Output: [      hi]
  ```

- `{:F>W}` — pad with character `F` instead of spaces (the fill character goes *before* the alignment).

  ```rust
  println!("{:0>4}", 42);       // Output: 0042
  ```

The full grammar is documented in the `std::fmt` module — see References.

## Task

You will build a small program that prints a formatted **concert ticket** for an imaginary event. Each subtask adds one more piece of output to the program. After each subtask, run `cargo run -p hello-cargo` and confirm the printed output exactly matches what is specified.

This task is composed of **8** subtasks.

### 1 — Set up the workspace and the package

To create a nested directory structure, run:

```sh
mkdir -p rust-by-challenge/challenges/01
```

To change into the `rust-by-challenge/` directory, which will be the root of your workspace, run:

```sh
cd rust-by-challenge
```

Create a file in the workspace root named `Cargo.toml` with these exact contents:

   ```toml
   [workspace]
   resolver = "3"
   members = ["challenges/01/hello-cargo"]
   ```

From the workspace root, run:

   ```sh
   cargo new challenges/01/hello-cargo
   ```

This creates the `hello-cargo/` directory containing a fresh package with its own `Cargo.toml` and `src/main.rs`.

The directory structure should now be:

```sh
rust-by-challenge/
├── Cargo.toml
└── challenges/
    └── 01/
        └── hello-cargo/
            ├── Cargo.toml
            └── src/
                └── main.rs
```

Run:

   ```sh
   cargo run -p hello-cargo
   ```

   You should see Cargo compile the package, then print exactly:

   ```sh
   Hello, world!
   ```

   The `-p hello-cargo` flag selects the package to run. From now on, every challenge will use the same pattern: add the package to the workspace's `members` list, run `cargo new`, then `cargo run -p <name>`.

### 2 — Print a header

Replace the body of `main()` so that running the program prints exactly:

```sh
=== Concert Ticket ===
```

### 3 — Print the venue using positional arguments

Below the header line, add code that defines two variables:

- `city` with value `"London"`
- `venue` with value `"Barbican Centre"`

Use `println!` with a format string containing the positional placeholders `{1}` and `{0}`, passing `city` first and `venue` second, so that the output line is exactly:

```sh
Venue: Barbican Centre, London
```

(The point of this subtask is to use positional indices — the obvious "just pass them in the right order" solution doesn't count.)

### 4 — Print the artist using a named argument

Below the venue line, define the variable:

- `artist` with value `"London Symphony Orchestra"`

Then use `println!` with the placeholder `{artist}` (no extra positional argument) to print:

```sh
Artist: London Symphony Orchestra
```

### 5 — Print the price with two decimal places

Below the artist line, define the variable:

- `price` with value `34.5`

Print it so the output line is exactly:

```sh
Price: £34.50
```

The number must be formatted using a precision specifier so that it always shows exactly two digits after the decimal point — typing `34.50` as a literal in the format string does not count.

### 6 — Print a right-aligned seat label

Below the price line, define the variable:

- `seat` with value `"A12"`

Print it using a width-and-alignment specifier so the seat value is right-aligned within a field of width 10. The output line must be exactly:

```sh
Seat:        A12
```

(That is: the literal `Seat:`, one space, then exactly 7 spaces of padding, then `A12`.)

After this subtask the full output should be:

### 7 — Build a footer with `format!`

Use the `format!` macro — not `println!`, not a string literal — to build a `String` named `footer`. Reference the variables `artist`, `venue`, and `city` directly inside the format string (do not retype the strings). The resulting `String` must equal exactly:

```sh
Tonight: London Symphony Orchestra at Barbican Centre, London
```

Then print `footer` on its own line. The line should be exactly `67` characters wide, left-aligned with `!` as the fill character. After this subtask the full stdout output should be:

```sh
=== Concert Ticket ===
Venue: Barbican Centre, London
Artist: London Symphony Orchestra
Price: £34.50
Seat:        A12
Tonight: London Symphony Orchestra at Barbican Centre, London!!!!!!
```

### 8 — Write a diagnostic to stderr

After all of the above, add one final line that writes to **standard error** (not standard out) the exact text:

```sh
ticket printed successfully
```

To verify it really went to stderr and not stdout, run:

```sh
cargo run -p hello-cargo 2> /dev/null
```

`2> /dev/null` discards stderr on Unix-like shells. You should see the ticket lines but **not** the diagnostic line. Conversely:

```sh
cargo run -p hello-cargo > /dev/null
```

discards stdout, so you should see Cargo's own build messages and the diagnostic line, but **none** of the ticket lines.

(On Windows PowerShell, the equivalents are `2>$null` and `>$null`.)

## References

- The Rust Programming Language Book — Chapter 1, *Getting Started*: <https://doc.rust-lang.org/book/ch01-00-getting-started.html>
- Rust by Example — *Hello World*: <https://doc.rust-lang.org/rust-by-example/hello.html>
- Rust by Example — *Formatted print*: <https://doc.rust-lang.org/rust-by-example/hello/print.html>
- The `std::fmt` module documentation (full format-string grammar): <https://doc.rust-lang.org/std/fmt/>
- The `println!` macro documentation: <https://doc.rust-lang.org/std/macro.println.html>
- The Cargo Book — *Workspaces*: <https://doc.rust-lang.org/cargo/reference/workspaces.html>
