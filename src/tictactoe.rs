use std::fmt::Display;

use crate::tictactoe::BadCoodinateError::{OutRangeError, WorngSizeError};

#[allow(unused)]
use super::input;

const ROW_OFFSET: usize = 3;

#[allow(unused)]
#[derive(Clone, Copy)]
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
}
impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [TileMark::None; 9],
        }
    }

    pub fn start(&mut self) {
        let positions = vec![
            parse_coordinates("a1").unwrap(),
            parse_coordinates("b2").unwrap(),
            parse_coordinates("c3").unwrap(),
        ];
        for i in positions {
            self.board[i] = TileMark::X;
        }

        self.render_board();
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
