use crate::board::*;
use crate::version::PROGRAM_NAME;
use crate::pieces::{WHITE,BLACK,BOTH};

use std::thread;
use std::sync::mpsc::{self,Receiver};
use std::time::{Duration, Instant};

use std::io::{self, Write};

pub fn xboard_loop() {

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

    let mut depth = MAX_DEPTH; // Default max depth
    let mut moves_to_go = 30;  // Default value if not provided
    let mut move_time: Option<u64> = None;
    let mut time: Option<u64> = None;
    let mut inc: Option<u64> = None;
    
    let mut engine_side = BOTH;
    // let mut time_life;
    // let mut moves_per_session;

    loop {
        io::stdout().flush();

        if board.side == engine_side {
            // think

            io::stdout().flush();
        }

        // if info.quit {
        //     break;
        // }

        match rx.recv() {
            Ok(input) => {
                let mut words = input.split_whitespace();

                match words.next() {
                    Some("quit") => {
                        break;
                    }
                    Some("force") => {
                        engine_side = BOTH;
                    }
                    Some("protover") => {
                        println!("feature ping=1 setboard=1 colors=0 usermove=1");
                        println!("feature done=1");
                    }
                    Some("sd") => {
                        if let Some(w) = words.next() {
                            depth = w.parse().unwrap();
                        }
                    }
                    Some("st") => {
                        if let Some(w) = words.next() {
                            move_time = Some(w.parse().unwrap());
                        }
                    }
                    Some("ping") => {
                        println!("pong {}", words.next().unwrap());
                    }
                    Some("new") => {
                        engine_side = BLACK;
                        board = board.update_from_fen(START_FEN);
                        depth = MAX_DEPTH;
                        continue;
                    }
                    Some("setboard") => {
                        if let Some(fen) = words.next() {
                            engine_side = BOTH;
                            board = board.update_from_fen(fen);
                        }
                        continue;
                    }
                    Some("go") => {
                        engine_side = board.side;
                    }
                    Some("usermove") => {
                        if let Some(move_str) = words.next() {
                            if let Some(mv) = board.parse_move(move_str) {
                                board.make_move(&mv);
                                board.reset_ply();
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => {
            panic!("stdin channel closed");
            }
        }
    }
}
