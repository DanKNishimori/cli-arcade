use std::fmt::Display;
use std::io::{Stdout, stdout};

use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType::FromCursorDown};

use super::input;
use super::messages::*;
use crate::tictactoe::BadCoodinateError::{OutRangeError, WrongSizeError};

const ROW_OFFSET: usize = 3;
const WINNING_MASKS: [u16; 8] = [
    0b000_000_111,
    0b000_111_000,
    0b111_000_000,
    0b001_001_001,
    0b010_010_010,
    0b100_100_100,
    0b100_010_001,
    0b001_010_100,
];

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
    stdout: Stdout,
}
impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [TileMark::None; 9],
            next_mark: TileMark::O,
            stdout: stdout(),
        }
    }

    pub fn start(&mut self) {
        let mut warning = "";

        loop {
            self.render_board();
            println!("{warning}");
            if let Some(result) = self.check_winner() {
                println!("the {result} has won!");
                break;
            }

            let raw_position = input("next mark: ");

            self.clear_screen();

            let Some(position) = raw_position else {
                continue;
            };

            let new_mark_position = match parse_coordinates(&position) {
                Ok(o) => o,
                Err(e) => {
                    warning = e.render_text();
                    continue;
                }
            };

            match self.update_board(new_mark_position) {
                Ok(_) => {
                    warning = "";
                    self.flip_mark();
                }
                Err(TileOverrideError) => warning = TILE_OVERRIDE,
            }
        }
    }

    fn check_winner(&self) -> Option<TileMark> {
        if self.has_won(TileMark::X) {
            Some(TileMark::X)
        } else if self.has_won(TileMark::O) {
            Some(TileMark::O)
        } else if !self.board.contains(&TileMark::None) {
            Some(TileMark::None)
        } else {
            None
        }
    }

    fn has_won(&self, player: TileMark) -> bool {
        let player_board = self.get_player_bitboard(player);
        for mask in WINNING_MASKS {
            if (player_board & mask) == mask {
                return true;
            }
        }
        false
    }

    fn get_player_bitboard(&self, player: TileMark) -> u16 {
        let mut bitboard: u16 = 0;

        for (i, cell) in self.board.iter().enumerate() {
            if *cell == player {
                bitboard |= 1 << i;
            }
        }

        bitboard
    }

    fn clear_screen(&mut self) {
        execute!(&mut self.stdout, cursor::MoveUp(10), Clear(FromCursorDown)).unwrap();
    }

    fn flip_mark(&mut self) {
        self.next_mark = match self.next_mark {
            TileMark::X => TileMark::O,
            TileMark::O => TileMark::X,
            TileMark::None => todo!(),
        }
    }

    fn update_board(&mut self, position: usize) -> Result<(), TileOverrideError> {
        if self.is_taken_tile(position) {
            return Err(TileOverrideError);
        }
        self.board[position] = self.next_mark;
        Ok(())
    }

    fn is_taken_tile(&self, position: usize) -> bool {
        self.board[position] != TileMark::None
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
        return Err(WrongSizeError);
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
    WrongSizeError,
    OutRangeError,
}
impl BadCoodinateError {
    fn render_text(self) -> &'static str {
        match self {
            WrongSizeError => WRONG_SIZE,
            OutRangeError => OUT_RANGE,
        }
    }
}

#[derive(Debug)]
struct TileOverrideError;
