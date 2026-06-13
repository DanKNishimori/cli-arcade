use std::{fmt::Display, io::stdout};

use crossterm::{
    cursor::{self, position},
    execute,
    terminal::{Clear, ClearType::FromCursorDown},
};

use crate::tictactoe::BadCoodinateError::{OutRangeError, WorngSizeError};

#[allow(unused)]
use super::input;

const ROW_OFFSET: usize = 3;

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum TileMark {
    X,
    O,
    None,
}
impl Display for TileMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileMark::O => write!(f, "O"),
            TileMark::X => write!(f, "X"),
            TileMark::None => write!(f, " "),
        }
    }
}

pub struct TicTacToe {
    board: [TileMark; 9],
    next_mark: TileMark,
}
impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [TileMark::None; 9],
            next_mark: TileMark::O,
        }
    }

    pub fn start(&mut self) {
        loop {
            self.render_board();
            println!();
            let Some(pos) = input("next mark: ") else {
                continue;
            };
            let new_mark_position = match parse_coordinates(&pos) {
                Ok(o) => o,
                Err(WorngSizeError) => todo!(),
                Err(OutRangeError) => todo!(),
            };
            if self.board[new_mark_position] != TileMark::None {
                continue;
            }
            self.board[new_mark_position] = self.next_mark;
            self.flip_mark();

            execute!(stdout(), cursor::MoveUp(10), Clear(FromCursorDown)).unwrap()
        }
    }

    fn flip_mark(&mut self) {
        self.next_mark = match self.next_mark {
            TileMark::X => TileMark::O,
            TileMark::O => TileMark::X,
            TileMark::None => todo!(),
        }
    }

    fn render_board(&self) {
        println!("  ╷ 1 ╷ 2 ╷ 3 ╷");
        let labels = ['a', 'b', 'c'];
        for (i, row) in self.board.chunks(3).enumerate() {
            println!(" ─┼───┼───┼───┤");
            println!("{} │ {} │ {} │ {} │", labels[i], row[0], row[1], row[2]);
        }
        println!(" ─┴───┴───┴───┘");
    }
}

fn parse_coordinates(position: &str) -> Result<usize, BadCoodinateError> {
    if position.len() != 2 {
        return Err(WorngSizeError);
    }
    let (y, x) = position.split_at(1);
    let x: usize = x.parse().unwrap();

    if x > 3 || x < 1 {
        return Err(OutRangeError);
    }

    match y {
        "a" | "A" => Ok(0 * ROW_OFFSET + x - 1),
        "b" | "B" => Ok(1 * ROW_OFFSET + x - 1),
        "c" | "C" => Ok(2 * ROW_OFFSET + x - 1),
        _other => Err(OutRangeError),
    }
}

#[derive(Debug)]
enum BadCoodinateError {
    WorngSizeError,
    OutRangeError,
}
