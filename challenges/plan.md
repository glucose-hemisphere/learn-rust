# Rust by Challenge — Plan

## The 18 challenges

### Phase 1 — Foundations

1. **`hello-cargo`** — Cargo basics, the printing macros, format strings (positional, named, precision, width/alignment/fill).
2. **`variables-and-types`** — `let`, `mut`, shadowing, primitive numerics/`bool`/`char`, type inference vs annotations, numeric literals, `as` casting, `const`, tuples, fixed-size arrays.
3. **`control-flow-and-functions`** — `if`/`else` as expression, `loop`/`while`/`for`, `break` returning a value, labelled loops, function definitions, expression-based returns, the unit type.

### Phase 2 — The ownership model

4. **`ownership`** — the three ownership rules, scope and `Drop`, move semantics, `Copy` by intuition, `.clone()`.
5. **`references-and-borrowing`** — `&T`, `&mut T`, borrowing rules, dangling-reference prevention, `&[T]` and `&str` slices.

### Phase 3 — Owned collections

6. **`strings`** — `String` vs `&str`, building strings, iterating over `chars`/`bytes`, conversions.
7. **`collections`** — `Vec` and `HashMap` from `std::collections`, common operations, the `entry` API.

### Phase 4 — Custom types

8. **`structs`** — named-field, tuple, and unit structs, `impl` blocks, methods, associated functions, field init shorthand, struct update syntax.
9. **`enums-and-matching`** — enums with and without data, exhaustive `match`, `if let`, `while let`.

### Phase 5 — Error handling *(introduces `anyhow`)*

10. **`error-handling`** — `Option<T>` and `Result<T, E>` combinators, the `?` operator, `Box<dyn Error>`, panic vs recoverable errors, then `anyhow` for ergonomics.

### Phase 6 — Abstraction

11. **`traits`** — defining and implementing traits, default methods, trait bounds (`impl Trait`, `where`), common `derive` macros.
12. **`generics`** — generic functions, generic types, trait bounds on generics, monomorphization explained briefly.
13. **`lifetimes`** — why lifetimes exist, elision rules, explicit `<'a>` annotations, references stored in structs, `'static` introduced lightly.
14. **`closures-and-iterators`** — closure syntax, capture-mode intuition (`Fn`/`FnMut`/`FnOnce` named), the practical `Iterator` surface (`map`, `filter`, `fold`, `collect`, `sum`, `enumerate`, `zip`).

### Phase 7 — Project structure

15. **`modules`** — `mod` blocks, file-based modules, `pub` visibility, `use`, `crate`/`super`/`self` paths.

### Phase 8 — Practical I/O *(introduces `rand`)*

16. **`cli-and-files`** — `std::env::args`, reading lines from `std::io::stdin` via `BufRead`, parsing strings to numbers, `std::fs::read_to_string` / `std::fs::write`, `File` + `BufReader::lines`, `BufWriter`. Adds `rand`.

### Phase 9 — Verification

17. **`testing`** — `#[cfg(test)] mod tests`, `assert!` / `assert_eq!` / `assert_ne!`, `#[should_panic]`, `Result`-returning tests, integration tests in the `tests/` directory.

### Phase 10 — Working with structured data *(introduces `serde` + `serde_json`)*

18. **`json-with-serde`** — deriving `Serialize` / `Deserialize`, `to_string_pretty`, `from_str`, round-tripping data through JSON.
