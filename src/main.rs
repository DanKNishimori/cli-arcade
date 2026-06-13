use crossterm::{cursor, execute, terminal};
use std::io::{self, Write, stdout};

mod guessing;
mod messages;

use guessing::{GuessingGame, Hardness};
use messages::*;

fn main() {
    println!("{OPEN}");

    loop {
        let command = match input(">>> ") {
            Some(c) => c.to_lowercase(),
            None => continue,
        };

        if command.starts_with("guessing") {
            println!("{WHICH_HARDNESS}");
            let hardness = match input("? ").as_deref() {
                Some("1") => Hardness::Easy,
                Some("3") => Hardness::Hard,
                _ => Hardness::Medium,
            };

            execute!(
                stdout(),
                cursor::MoveUp(4),
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )
            .expect("some error");
            GuessingGame::new(hardness).start().unwrap(); // living dangerously
        } else if command.starts_with("help") {
            println!("{HELP}");
        }

        if command.contains("&quit") {
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

    let trimmed_content = content.trim();

    if trimmed_content.is_empty() {
        return None;
    }
    Some(trimmed_content.to_string())
}
