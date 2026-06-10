use rand::random_range;
use std::cmp::Ordering;

use super::input;

pub struct GuessingGame {
    secret: i16,
    tries: i16,
}
impl GuessingGame {
    pub fn new(hardness: Hardness) -> GuessingGame {
        GuessingGame {
            secret: random_range(01..99),
            tries: give_tries(hardness),
        }
    }

    pub fn start(&mut self) {
        println!("<[ Guessing Game ]>");
        println!("Hey, I have a secret number,\n");

        loop {
            if self.tries <= 0 {
                println!("you loose!");
                break;
            }

            let Some(guess) = input("what's your guess? ") else {
                continue;
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
                Ordering::Less => println!("{guess} is too high!"),
                Ordering::Greater => println!("{guess} is too low!"),
                Ordering::Equal => {
                    println!("RIGHT!\nYou win!\n");
                    break;
                }
            }
        }
    }
}

fn give_tries(hardness: Hardness) -> i16 {
    match hardness {
        Hardness::Easy => 20,
        Hardness::Medium => 15,
        Hardness::Hard => 7,
    }
}

pub enum Hardness {
    Easy,
    Medium,
    Hard,
}
