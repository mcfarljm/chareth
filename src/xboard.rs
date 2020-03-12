use crate::board::*;
use crate::pieces::{BLACK,BOTH};
use crate::version::PROGRAM_NAME;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

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
    board.side = BLACK;
    let mut info = SearchInfo::new(5, GameMode::Xboard);
    // Store a receiver in the search info so that it can catch "stop"
    // messages
    info.set_receiver(&rx);

    let mut depth = MAX_DEPTH; // Default max depth
    let mut moves_to_go: [u32; 2] = [30, 30];
    let mut move_time: Option<u64> = None;
    let mut time: Option<u64> = None;
    let mut inc: Option<u64> = None;
    
    let mut engine_side = BOTH;
    let mut time_left: u64;
    // xboard uses 0 to indicate that whole game is played in one period
    let mut moves_per_session = 0;

    loop {
        io::stdout().flush();

        if board.side == engine_side && ! board.check_game_result() {
            info.set_depth(depth);
            // Todo: is there a case with only depth and no time limit
            // that needs to be handled?
            info.set_search_time(time, move_time, moves_to_go[board.side], inc);

            println!("time:{:?} depth:{} mvoestogo:{:?} mps:{}", time, depth, moves_to_go, moves_per_session);
            // Unlike vice, we have search return the move and make it
            // here, for clarity
            let best_move = board.search(&mut info);
            if let Some(mv) = best_move {
                board.make_move(&mv);
            }

            if moves_per_session != 0 {
                moves_to_go[board.side^1] -= 1;
                if moves_to_go[board.side^1] < 1 {
                    moves_to_go[board.side^1] = moves_per_session;
                }
            }

            io::stdout().flush();

            if info.quit {
                break;
            }
        }

        match rx.recv() {
            Ok(input) => {
                let mut words = input.split_whitespace();

                match words.next() {
                    Some("quit") => {
                        info.quit = true;
                        break;
                    }
                    Some("force") => {
                        engine_side = BOTH;
                    }
                    Some("protover") => {
                        println!("feature ping=1 setboard=1 colors=0 usermove=1");
                        // sigint=0 needed on Linux
                        println!("feature sigint=0");
                        println!("feature myname=\"{}\"", PROGRAM_NAME);
                        println!("feature done=1");
                    }
                    Some("sd") => {
                        if let Some(w) = words.next() {
                            depth = w.parse().unwrap();
                        }
                    }
                    Some("st") => {
                        if let Some(w) = words.next() {
                            move_time = Some(w.parse::<u64>().unwrap() * 1000);
                        }
                    }
                    Some("time") => {
                        if let Some(t) = words.next() {
                            // Centiseconds -> milliseconds
                            let cs: u64 = t.parse().unwrap();
                            time = Some(cs * 10); // centiseconds -> milliseconds
                        }
                    }
                    Some("level") => {
                        move_time = None;
                        moves_per_session = words.next().unwrap().parse().unwrap();
                        let time_spec = words.next().unwrap();
                        inc = Some(words.next().unwrap().parse::<u64>().unwrap() * 1000); // Seconds -> ms
                        let mut time_args = time_spec.split(':');
                        time_left = time_args.next().unwrap().parse().unwrap();
                        time_left *= 60000; // Min -> sec
                        if let Some(sec) = time_args.next() {
                            let seconds: u64 = sec.parse().unwrap();
                            time_left += seconds * 1000;
                        }
                        time = None;
                        moves_to_go[0] = 30;
                        moves_to_go[1] = 30;
                        if moves_per_session != 0 {
                            moves_to_go[0] = moves_per_session;
                            moves_to_go[1] = moves_per_session;
                        }
                        println!("level debug: time_left:{} moves_to_go:{} inc:{:?} moves_per_session:{}", time_left, moves_to_go[0], inc, moves_per_session);
                            
                    }
                    Some("ping") => {
                        println!("pong {}", words.next().unwrap());
                    }
                    Some("new") => {
                        engine_side = BLACK;
                        board = board.update_from_fen(START_FEN);
                        depth = MAX_DEPTH;
                        time = None;
                        move_time = None;
                        inc = None;
                        moves_per_session = 0;
                    }
                    Some("setboard") => {
                        if let Some(fen) = words.next() {
                            engine_side = BOTH;
                            board = board.update_from_fen(fen);
                        }
                    }
                    Some("go") => {
                        engine_side = board.side;
                    }
                    Some("usermove") => {
                        moves_to_go[board.side] -= 1;
                        if let Some(move_str) = words.next() {
                            if let Some(mv) = board.parse_move(move_str) {
                                board.make_move(&mv);
                                board.reset_ply();
                            } else {
                                println!("invalid user move detected: {}", move_str);
                            }
                        }
                    }
                    Some("remove") => {
                        // User asked to undo move (happens during
                        // user turn).  Undo last 2 moves and continue
                        // playing the same color.
                        board.undo_move();
                        board.undo_move();
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
