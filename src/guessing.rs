use rand::random_range;
use std::cmp::Ordering;

use super::input;

pub struct GuessingGame {
    secret: i16,
    tries: i16,
}
impl GuessingGame {
    pub fn new() -> GuessingGame {
        GuessingGame {
            secret: random_range(01..99),
            tries: 15,
        }
    }

    pub fn start(&mut self) {
        println!("Ok, try to guess my number.\n");

        loop {
            if self.tries <= 0 {
                println!("you loose!");
                break;
            }

            let guess = match input("what's your guess? ") {
                Some(g) => g,
                None => continue,
            };

            let guess: i16 = match guess.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Bad guess! Try some number like 1, 5, 42 or 100.");
                    continue;
                }
            };

            self.tries -= 1;
            match self.secret.cmp(&guess) {
                Ordering::Less => println!("Too high!"),
                Ordering::Greater => println!("Too low!"),
                Ordering::Equal => {
                    println!("RIGHT!\nYou win!\n");
                    break;
                }
            }
        }
    }
}
