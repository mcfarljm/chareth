mod pieces;
mod board;
mod bitboard;
mod moves;
mod validate;

use board::{Board,START_FEN};

fn main() {
    let board = Board::from_fen(START_FEN);
    board.check();

    println!("{}", board.to_string());

    println!("{}", board.pawns[0].to_string());
    println!("{}", board.pawns[1].to_string());
    println!("{}", board.pawns[2].to_string());

    let mv = moves::Move::new(23, 33, 0, pieces::Piece::WR, false, false, false);
    let s = mv.to_string();
    println!("{}", s);
}
