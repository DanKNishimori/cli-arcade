use crossterm::terminal::ClearType;
use crossterm::terminal::ClearType::{CurrentLine, FromCursorDown};
use crossterm::{cursor, execute, terminal};
use rand::random_range;
use std::cmp::Ordering;
use std::io;
use std::io::Stdout;

use super::input;
use super::messages::*;

const MINIMIUM_FOR_WIN: i16 = 7;

pub struct GuessingGame {
    secret: i16,
    tries: i16,
    stdout: Stdout,
}
impl GuessingGame {
    pub fn new(hardness: Hardness) -> GuessingGame {
        GuessingGame {
            secret: random_range(01..99),
            tries: give_tries(hardness),
            stdout: io::stdout(),
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        println!("{START_GUESSING_GAME}\n\n");

        loop {
            if !self.has_tries() {
                self.clean(2, FromCursorDown)?;
                print_loose_with(self.secret);
                break;
            }

            self.clean(3, CurrentLine)?;

            let Some(guess) = input(GET_GUESS) else {
                continue;
            };
            let guess: i16 = match guess.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("{BAD_INPUT}\n");
                    continue;
                }
            };

            self.consume_try();
            match guess.cmp(&self.secret) {
                Ordering::Less => it_is_lower_hint(guess),
                Ordering::Greater => it_is_higher_hint(guess),
                Ordering::Equal => {
                    self.clean(0, FromCursorDown)?;
                    println!("{WIN}");
                    break;
                }
            }
        }
        Ok(())
    }

    fn consume_try(&mut self) {
        self.tries -= 1;
    }

    fn has_tries(&self) -> bool {
        self.tries >= 0
    }

    fn clean(&mut self, lines_up: u16, mode: ClearType) -> io::Result<()> {
        execute!(self.stdout, cursor::MoveUp(lines_up), terminal::Clear(mode))
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
