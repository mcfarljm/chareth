use crate::board::{Board,START_FEN,SearchInfo};

use std::io::{self, Write};

// A simple io loop for text based moves and searches
pub fn simple_loop() {

    let WAC1 = "r1b1k2r/ppppnppp/2n2q2/2b5/3NP3/2P1B3/PP3PPP/RN1QKB1R w KQkq - 0 1";

    let mut board = Board::from_fen(WAC1);
    // let mut board = Board::from_fen(START_FEN);

    loop {
        board.print();
        
        print!("Enter a move > ");
        io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.chars().next().unwrap() {
            'q' => { break; }
            't' => { board.undo_move(); }
            's' => {
               let mut info = SearchInfo::new(6); 
                board.search(&mut info);
            }
            _ => {
                match board.parse_move(&input) {
                    Some(mv) => {
                        board.store_pv_move(mv);
                        board.make_move(&mv);
                    }
                    _ => { println!("Move not parsed"); }
                }
            }
        }
        println!("");
    }
        

}
