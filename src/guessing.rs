use rand::random_range;
use std::cmp::Ordering;

use super::input;

const MINIMIUM_FOR_WIN: i16 = 7;

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
            if !self.has_tries() {
                println!("you loose!");
                break;
            }

            let Some(guess) = input("what's your guess? ") else {
                continue;
            };

            let guess: i16 = match guess.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Bad guess! Try some number like 1, 5, 42 or 100.");
                    continue;
                }
            };

            self.consume_try();
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

    fn consume_try(&mut self) {
        self.tries -= 1;
    }

    fn has_tries(&self) -> bool {
        self.tries >= 0
    }
}

fn give_tries(hardness: Hardness) -> i16 {
    match hardness {
        Hardness::Easy => MINIMIUM_FOR_WIN * 3,
        Hardness::Medium => MINIMIUM_FOR_WIN * 2,
        Hardness::Hard => MINIMIUM_FOR_WIN,
    }
}

pub enum Hardness {
    Easy,
    Medium,
    Hard,
}
