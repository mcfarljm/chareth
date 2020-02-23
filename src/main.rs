mod board;
mod bitboard;

use board::{Board,START_FEN};

fn main() {
    let board = Board::from_fen(START_FEN);
    let s = board.to_string();

    println!("{}", s);
}
