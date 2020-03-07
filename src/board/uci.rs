use crate::board::*;
use crate::version::PROGRAM_NAME;

use std::thread;
use std::sync::mpsc::{self,Receiver};
use std::time::{Duration, Instant};

use std::io::{self, Write};

impl Board {
    // go depth <> wtime <> btime <> binc <> winc <> movetime <> movestogo <>
    pub fn parse_go(&mut self, line: &str, info: &mut SearchInfo) {
        let mut depth = MAX_DEPTH; // Default max depth
        let mut moves_to_go = 30;  // Default value if not provided
        let mut move_time: Option<u64> = None;
        let mut time: Option<u64> = None;
        let mut inc: Option<u64> = None;

        let words: Vec<&str> = line.split(' ').collect();
        for (i,word) in words.iter().enumerate() {
            match *word {
                "winc" if self.side == WHITE => {
                    inc = Some(words[i+1].trim().parse().unwrap());
                }
                "binc" if self.side == BLACK => {
                    inc = Some(words[i+1].trim().parse().unwrap());
                }
                "wtime" if self.side == WHITE => {
                    time = Some(words[i+1].trim().parse().unwrap());
                }
                "btime" if self.side == BLACK => {
                    time = Some(words[i+1].trim().parse().unwrap());
                }
                "movestogo" => {
                    moves_to_go = words[i+1].trim().parse().unwrap();
                }
                "movetime" => {
                    move_time = Some(words[i+1].trim().parse().unwrap());
                }
                "depth" => {
                    depth = words[i+1].trim().parse().unwrap();
                }
                _ => (),
            }
        }
        
        if let Some(mt) = move_time {
            time = Some(mt);
            moves_to_go = 1;
        }

        info.set_depth(depth);

        if let Some(t) = time {
            let mut time_val = t;
            time_val /= moves_to_go;
            time_val -= 50; // To be safe
            if let Some(i) = inc {
                time_val += i;
            }
            info.set_time_limit(Duration::from_millis(time_val));
            println!("time:{:?} depth:{:?}", time_val, depth);
        } else {
            info.unset_time_limit();
            println!("depth:{:?}", depth);
        }


        self.search(info);
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

        if let Some(i) = slice.find("moves") {
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
    println!("id name {}", PROGRAM_NAME);
    println!("id author John McFarland");
    println!("uciok");
}

pub fn uci_loop() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            tx.send(buffer).unwrap();
        }
    });

    let mut board = Board::new();
    let mut info = SearchInfo::new(5);
    // Store a receiver in the search info so that it can catch "stop"
    // messages
    info.set_receiver(&rx);

    uci_ok();

    loop {
        io::stdout().flush();

        match rx.recv() {
            Ok(input) => {
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
                    board.parse_go(&input, &mut info);
                } else if input.starts_with("uci") {
                    uci_ok();
                } else if input.starts_with("quit") {
                    break;
                }
                if info.quit {
                    break;
                }
            }
            _ => {
                panic!("stdin channel closed");
            }
        }
        
    }

}
