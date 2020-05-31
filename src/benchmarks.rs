use crate::board;

use std::time::Instant;

pub fn benchmark_move_gen(iterations: u64) {
    let board = board::Board::from_fen(board::START_FEN);

    let start_time = Instant::now();
    
    for _i in 0..iterations {
        board.generate_all_moves();
    }

    println!("move gen: {} iterations in {}ms ({:.4}us per iter)", iterations, start_time.elapsed().as_millis(), (start_time.elapsed().as_secs_f32() / (iterations as f32)) * 1e6);
}

pub fn benchmark_eval(iterations: u64) {
    let board = board::Board::from_fen(board::START_FEN);

    let start_time = Instant::now();
    
    for _i in 0..iterations {
        board.evaluate();
    }

    println!("eval: {} iterations in {}ms ({:.4}us per iter)", iterations, start_time.elapsed().as_millis(), (start_time.elapsed().as_secs_f32() / (iterations as f32)) * 1e6);
}
