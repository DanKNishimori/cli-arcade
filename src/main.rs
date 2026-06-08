use std::io::{self, Write};

fn main() {
    println!("<< Welcome to the CLI Arcade! >>\n");

    loop {
        print_input(">>> ");
        let command = match get_input() {
            Some(c) => c,
            None => continue,
        };

        if command.contains("!quit") {
            break;
        }
    }
}

fn print_input(message: &str) {
    print!("{message}");
    io::stdout().flush().unwrap();
}

fn get_input() -> Option<String> {
    let mut content = String::new();
    io::stdin()
        .read_line(&mut content)
        .expect("input error on read");

    if content.trim().is_empty() {
        return None;
    }
    Some(content.trim().to_owned())
}
