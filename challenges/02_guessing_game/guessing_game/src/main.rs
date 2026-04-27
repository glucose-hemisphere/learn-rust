use rand::Rng;
use std::io;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    let mut count: u8 = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let input: u32 = input.trim().parse().expect("Please enter a number");
        count += 1;

        let result: &str = check_guess(input, secret);
        if result == "Correct!" {
            println!("Correct! You got it in {count} guess(es).");
            break;
        } else {
            println!("{result}");
        }
    }
}

fn check_guess(guess: u32, secret: u32) -> &'static str {
    if guess > secret {
        "Too high!"
    } else if guess < secret {
        "Too low!"
    } else {
        "Correct!" // Better to use an enum instead
    }
}

// Example Output:
//
// 50
// Too low!
// 75
// Too high!
// 57
// Too low!
// 61
// Too low!
// 68
// Too high!
// 64
// Too high!
// 63
// Too high!
// 62
// Correct! You got it in 8 guess(es).
