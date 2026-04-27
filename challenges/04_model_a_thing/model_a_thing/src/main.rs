    use approx::relative_eq;

    fn main() {
        let width = 0.1 + 0.2;
        let height = 0.3;
        // let factor = 45.; // Correct but not as readable
        let factor = 45.0;

        let mut rectangle = Rectangle::new(width, height);
        println!("{:#?}", rectangle);
        println!("{}", rectangle.area());
        println!("{}", rectangle.perimeter());
        println!("{}", rectangle.is_square());

        rectangle.scale(factor);
        println!("{:#?}", rectangle);
        println!("{}", rectangle.is_square());
    }

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn new(width: f64, height: f64) -> Self {
            // Rectangle { width: width, height: height} // Correct but verbose
            Rectangle { width, height }
        }

        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }

        fn is_square(&self) -> bool {
            // self.width == self.height // May be imprecise due to floating point arithmetic
            // (self.width - self.height).abs() < f64::EPSILON // Better, but still not ideal
            relative_eq!(self.width, self.height) // The best approach, using `relative_eq` from the `approx` crate
        }

        fn scale(&mut self, factor: f64) {
            self.width *= factor;
            self.height *= factor;
        }
    }

    // Output:
    //
    // Rectangle {
    //     width: 0.30000000000000004,
    //     height: 0.3,
    // }
    // 0.09000000000000001
    // 1.2000000000000002
    // true
    // Rectangle {
    //     width: 13.500000000000002,
    //     height: 13.5,
    // }
    // true
