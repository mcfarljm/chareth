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

    let pawn_moves_w = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
    let pawn_moves_b = "rnbqkbnr/p1p1p3/3p3p/1p1p4/2P1Pp2/8/PP1P1PpP/RNBQKB1R b KQkq e3 0 1";
    let knights_kings = "5k2/1n6/4n3/6N1/8/3N4/8/5K2 w - - 0 1";
    let board = Board::from_fen(knights_kings);
    println!("{}", board.to_string());
    board.check();

    let mut ml = movegen::MoveList::new();
    ml.generate_all_moves(&board);
    // ml.print();

}
