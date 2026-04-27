# Challenge 4: Model a Thing

## The Theory

**Structs** are Rust's way of grouping related data together into a named type. If you've used Python classes, think of a struct as a class that *only* holds data — no methods yet (those come in a later challenge).

```rust
struct Point {
    x: f64,
    y: f64,
}
```

You create an instance like this:

```rust
let p = Point { x: 3.0, y: 4.0 };
println!("{}", p.x); // access fields with dot notation
```

### Adding methods with `impl`

To give a struct behaviour, you write an `impl` block:

```rust
impl Point {
    // "associated function" — called with Point::new(), like a constructor
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }  // shorthand when variable name matches field name
    }

    // method — first parameter is always &self (immutable) or &mut self (mutable)
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

### Deriving Debug

By default you can't print a struct. Add this line above the struct definition and you can:

```rust
#[derive(Debug)]
struct Point { ... }

println!("{:?}", p);   // compact:  Point { x: 3.0, y: 4.0 }
println!("{:#?}", p);  // pretty:   Point {
                       //               x: 3.0,
                       //               y: 4.0,
                       //           }
```

---

## Your Task

Model a **rectangle**. Create a `Rectangle` struct and implement the following:

1. An associated function `new(width: f64, height: f64) -> Rectangle`
2. A method `area(&self) -> f64`
3. A method `perimeter(&self) -> f64`
4. A method `is_square(&self) -> bool`
5. A method `scale(&mut self, factor: f64)` that multiplies both dimensions by `factor` in place

In `main`:

- Create a rectangle using `new`
- Print it with `{:#?}`
- Print its area, perimeter, and whether it's a square
- Scale it by a factor of your choice, then print it again and re-check if it's a square

---

## Acceptance Criteria

- ✅ `Rectangle` is defined as a struct with two `f64` fields
- ✅ All five functions/methods are implemented in an `impl` block
- ✅ `scale` takes `&mut self` and modifies the struct in place
- ✅ `#[derive(Debug)]` is used so the struct can be printed
- ✅ Compiles with no warnings
