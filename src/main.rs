use crossterm::{cursor, execute, terminal};
use std::io::{self, Write, stdout};

mod guessing;

use guessing::{GuessingGame, Hardness};

const HELP_MSG: &str = "
┌────────────────────────────────────────────────────────────────────────┐
│                          CLI ARCADE - HELP MENU                        │
└────────────────────────────────────────────────────────────────────────┘

COMMAND STRUCTURE:
  > [game name] [options...]

GAMES AVAILABLE:
  • tictactoe
  • guessing <easy|medium|hard>
  • hangman <theme_name>
    Available themes match the filenames in the words folder.
     - <list>

EXIT:
  • Type or add '&quit' to exit.
  • If its placed after a game tittle, it will close the Arcade after your game concludes.
  
Examples:
  > tictactoe
  > guessing hard &quit
";

fn main() {
    println!("<< Welcome to the CLI Arcade! >>\n");
    println!("Type \"help\" for more information.");

    loop {
        let command = match input(">>> ") {
            Some(c) => c.to_lowercase(),
            None => continue,
        };

        if command.starts_with("guessing") {
            println!("Before start, in what difficulty you like to play?");
            println!("1) Easy  2) Medium  3) Hard");
            let hardness = match input("? ").as_deref() {
                Some("1") => Hardness::Easy,
                Some("3") => Hardness::Hard,
                _ => Hardness::Medium,
            };

            execute!(
                stdout(),
                cursor::MoveUp(3),
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )
            .expect("some error");
            GuessingGame::new(hardness).start().unwrap(); // living dangerously
        } else if command.starts_with("help") {
            println!("{HELP_MSG}");
        }

        if command.contains("!quit") {
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
