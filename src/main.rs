use std::io::{self, Write};

mod guessing;

use guessing::{GuessingGame, Hardness};

fn main() {
    println!("<< Welcome to the CLI Arcade! >>\n");

    loop {
        let command = match input(">>> ") {
            Some(c) => c.to_lowercase(),
            None => continue,
        };

        if command.trim().starts_with("guessing") {
            println!("Before start, in what difficulty you like to play?");
            println!("1) Easy  2) Medium  3) Hard");
            let hardness = match input("? ") {
                Some(s) => match s.trim() {
                    "1" => Hardness::Easy,
                    "3" => Hardness::Hard,
                    _ => Hardness::Medium,
                },
                None => Hardness::Medium,
            };
            println!();
            GuessingGame::new(hardness).start();
        }

        if command.trim() == "!quit" {
            break;
        }
    }
}

fn input(message: &str) -> Option<String> {
    print!("{message}");
    io::stdout().flush().unwrap();

    let mut content = String::new();
    io::stdin()
        .read_line(&mut content)
        .expect("input error on read");

    if content.trim().is_empty() {
        return None;
    }
    Some(content.to_owned())
}
