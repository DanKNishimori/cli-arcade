macro_rules! make_message {
    ($name:ident : $msg:expr) => {
        pub const $name: &str = $msg;
    };
}

macro_rules! make_formatted_msg {
    ($name:ident ($($param:ident : $ty:ty),*) : $msg:expr) => {
        pub fn $name($($param: $ty),*) {
            println!($msg);
        }
    };
}

// ———————————————————————————————————————————————————————————————————————————
// — Main system messages ————————————————————————————————————————————————————
// ———————————————————————————————————————————————————————————————————————————

make_message!(OPEN : "
┌──────────────────────────────────┐
│  < Welcome to the CLI Arcade! >  │
└──────────────────────────────────┘
< Type \"help\" for more information.
");

make_message!(HELP : "
┌──────────────────────────────────┐
│          < Help Menu >           │
└──────────────────────────────────┘
COMMAND STRUCTURE:
  > [game name] [options...]

GAMES AVAILABLE:
  • guessing

EXIT:
  • Type or add '&quit' to exit.
  • If it's placed after a game tittle, it will close the Arcade after your game concludes.

");

make_message!(WHICH_HARDNESS: "
Before start, in what difficulty you like to play?
1) Easy  2) Medium  3) Hard");

// ———————————————————————————————————————————————————————————————————————————
// — Guessing Game messages ——————————————————————————————————————————————————
// ———————————————————————————————————————————————————————————————————————————

make_message!(START_GUESSING_GAME: "
<[ Guessing Game ]>
Hey, I have a secret number...
");

make_message!(GET_GUESS: "what's your guess? ");

make_message!(BAD_INPUT: "Bad guess! Try some number like 1, 5, 42 or 100.");

make_message!(WIN: "
RIGHT!
You win!
");

make_formatted_msg!(
    print_loose_with(secret: i16) : "
Wrong! My number was {secret}
You lose!
");

make_formatted_msg!(
    it_is_higher_hint(guess: i16) : "\n{guess} is too high!"
);

make_formatted_msg!(
    it_is_lower_hint(guess: i16) : "\n{guess} is too low!"
);

// ———————————————————————————————————————————————————————————————————————————
// — Tic-Tac-Toe messages ————————————————————————————————————————————————————
// ———————————————————————————————————————————————————————————————————————————

make_message!(X_WIN: "The X has won!");
make_message!(O_WIN: "The O has won!");
make_message!(DRAW: "Finished, it's a DRAW!");

make_message!(WRONG_SIZE: "Please, type a valid input! (e.g.: A1)");

make_message!(OUT_RANGE: "Pay attention to the size of the board!");

make_message!(TILE_OVERRIDE: "You can't take the others tile!");
