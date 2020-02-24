mod pieces;
mod board;
mod bitboard;
mod moves;
mod validate;
mod movegen;

use board::{Board,START_FEN};

fn main() {
    // let board = Board::from_fen(START_FEN);
    // board.check();

    // println!("{}", board.to_string());

    // println!("{}", board.pawns[0].to_string());
    // println!("{}", board.pawns[1].to_string());
    // println!("{}", board.pawns[2].to_string());

    // let mv = moves::Move::new(23, 33, 0, pieces::Piece::WR, false, false);
    // let s = mv.to_string();
    // println!("{}", s);

    let pawn_moves = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
    let board = Board::from_fen(pawn_moves);
    println!("{}", board.to_string());
    board.check();

    let mut ml = movegen::MoveList::new();
    ml.generate_all_moves(&board);
    ml.print();

}
