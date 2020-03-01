use crate::board::*;

use std::io::{self, Write};

impl Board {
    pub fn parse_go(&self, line: &str, info: &SearchInfo) {
    }

    // position startpos
    // position fen <string>
    // ... moves e2e4 e7e5 etc
    pub fn parse_pos(self, line: &str) -> Board {
        let mut slice = &line[9..];
        let mut board = self;

        if slice.starts_with("startpos") {
            board = board.update_from_fen(START_FEN);
        } else if slice.starts_with("fen") {
            slice = &slice[4..];
            board = board.update_from_fen(slice);
        } else {
            // Unexpected input, but just assume startpos
            board = board.update_from_fen(START_FEN);
        }

        println!("Checking: {}", slice);

        if let Some(i) = slice.find("moves") {
            println!("Found moves keyword");
            println!("Splitting up: {}", &slice[i..]);
            for word in slice[i+6..].split(' ') {
                match board.parse_move(word) {
                    Some(mv) => {
                        board.make_move(&mv);
                        board.ply = 0;
                    }
                    _ => { break; }
                }
            }
        }

        board.print();
        
        board
    }
}

// May make more sense for this function to be outside of the board module...

fn uci_ok() {
    println!("id name crust");
    println!("id author John McFarland");
    println!("uciok");
}

pub fn uci_loop() {

    let mut board = Board::new();
    let mut info = SearchInfo::new(5);

    uci_ok();

    loop {
        io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // println!("Got input: {}", input);
        // io::stdout().flush();

        if input.len() == 1 && input.chars().nth(0).unwrap() == '\n' {
            continue;
        }

        if input.starts_with('\n') {
            continue;
        } else if input.starts_with("isready") {
            println!("readyok");
        } else if input.starts_with("position") {
            board = board.parse_pos(&input);
        } else if input.starts_with("ucinewgame") {
            board = board.parse_pos("position startpos\n");
        } else if input.starts_with("go") {
            board.parse_go(&input, &info);
        } else if input.starts_with("uci") {
            uci_ok();
        } else if input.starts_with("quit") {
            break;
        }
        
    }

}
