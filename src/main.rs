mod pieces;
mod board;
mod bitboard;
mod moves;
mod validate;

use board::{Board,START_FEN};

use std::io::{self, Write};

fn main() {

    let mut board = board::Board::from_fen(START_FEN);

    loop {
        board.print();
        
        print!("Enter a move > ");
        io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.chars().next().unwrap() {
            'q' => { break; }
            't' => { board.undo_move(); }
            _ => {
                match board.parse_move(&input) {
                    Some(mv) => { board.make_move(&mv); }
                    _ => { println!("Move not parsed"); }
                }
            }
        }
        println!("");
    }
        

}
