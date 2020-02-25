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

    let ROOKS = "6k1/8/5r2/8/1nR5/5N2/8/6K1 b - - 0 1";
    let QUEENS = "6k1/8/4nq2/8/1nQ5/5N2/1N6/6K1 b - - 0 1 ";
    let BISHOPS = "6k1/1b6/4n3/8/1n4B1/1B3N2/1N6/2b3K1 b - - 0 1 ";

    let CASTLE1 = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
    let CASTLE2 = "3rk2r/8/8/8/8/8/6p1/R3K2R b KQk - 0 1";

    let board = Board::from_fen(CASTLE2);
    println!("{}", board.to_string());
    board.check();

    let mut ml = movegen::MoveList::new();
    ml.generate_all_moves(&board);
    // ml.print();

}
