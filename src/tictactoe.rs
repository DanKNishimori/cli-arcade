use std::fmt::Display;

use super::input;

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

    pub fn test(&self) {
        self.render_board();
    }

    #[allow(unused)]
    pub fn start(&mut self) {
        todo!()
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
