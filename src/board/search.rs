use crate::board::*;
use crate::moves;

use std::time::{Duration, Instant};

const MATE: i32 = 29000;

// Avoid overflow when negating
const I32_SAFE_MIN: i32 = std::i32::MIN + 1;

pub struct SearchInfo {
    start_time: Instant,
    depth: u32,

    depth_set: u32,
    time_set: Duration,

    moves_to_go: u32,
    infinite: bool,

    // Count of all positioned visited
    nodes: u64,

    quit: bool,
    stopped: bool,

    fail_high: u32,
    fail_high_first: u32,
}

impl SearchInfo {
    pub fn new(depth: u32) -> SearchInfo {
        SearchInfo{
            start_time: Instant::now(),
            depth: depth,

            depth_set: 0,
            time_set: Duration::new(0,0),

            moves_to_go: 0,
            infinite: false,

            nodes: 0,
            
            quit: false,
            stopped: false,

            fail_high: 0,
            fail_high_first: 0
        }
    }
}

impl Board {
    pub fn search(&mut self, info: &mut SearchInfo) {
        let mut best_move: moves::Move;
        let mut best_score = I32_SAFE_MIN;
        let pv_num = 0;

        self.clear_for_search(info);

        // Iterative deepening
        for current_depth in 1..=info.depth {
            best_score = self.alpha_beta(I32_SAFE_MIN, std::i32::MAX, current_depth, info, true);
            self.get_pv_line(current_depth);
            best_move = self.pv_array[0];
            print!("Depth:{} score:{} move:{} nodes:{} ", current_depth, best_score, best_move.to_string(), info.nodes);
            print!("pv");
            for mv in &self.pv_array {
                print!(" {}", mv.to_string());
            }
            println!("");
            println!("Ordering: {:.2}", info.fail_high_first as f32 /info.fail_high as f32);
        }
    }

    pub fn clear_for_search(&mut self, info: &mut SearchInfo) {
        self.search_history = [[0; BOARD_SQ_NUM]; 13];
        self.search_killers.clear();

        self.pv_table.clear();
        self.pv_array.clear();

        self.ply = 0;

        info.start_time = Instant::now();
        info.stopped = false;
        info.nodes = 0;

        info.fail_high = 0;
        info.fail_high_first = 0;
    }

    pub fn alpha_beta(&mut self, alpha_in: i32, beta: i32, depth: u32, info: &mut SearchInfo, do_null: bool) -> i32 {
        debug_assert!(self.check());

        if depth == 0 {
            info.nodes += 1;
            return self.evaluate();
        }

        info.nodes += 1;

        if self.is_repetition() || self.fifty_move >= 100 {
            return 0;
        }

        let mut legal = 0;
        let mut alpha = alpha_in;
        let mut score = I32_SAFE_MIN;
        // Use option to workaround uninitalized values
        let mut best_move: Option<moves::Move> = None;

        let move_list = self.generate_all_moves();
        for smv in move_list.moves.iter() {
            if ! self.make_move(&smv.mv) {
                continue;
            }
            legal += 1;
            score = - self.alpha_beta(-beta, -alpha, depth-1, info, true);
            self.undo_move();

            if score > alpha {
                if score >= beta {
                    if legal == 1 {
                        info.fail_high_first += 1;
                    }
                    info.fail_high += 1;
                    return beta;
                }
                alpha = score;
                best_move = Some(smv.mv);
            }
        }

        if legal == 0 {
            if self.square_attacked(self.king_sq[self.side], self.side^1) {
                return -MATE + self.ply as i32;
            } else {
                return 0;
            }
        }

        if alpha != alpha_in {
            self.store_pv_move(best_move.unwrap());
        }

        alpha
    }

    pub fn quiescence(&mut self, alpha: i32, beta: i32, info: &SearchInfo) -> i32 {
        0
    }

    fn is_repetition(&self) -> bool {
        if self.hist_ply <= 0 {
            return false;
        }
        for i in self.hist_ply-self.fifty_move..self.hist_ply-1 {
            if self.history[i as usize].hash == self.hash {
                return true;
            }
        }
        false
    }

    pub fn store_pv_move(&mut self, mv: moves::Move) {
        self.pv_table.insert(self.hash, mv);
    }

    pub fn get_pv_line(&mut self, depth: u32) {
        let mut count: u32 = 0;
        self.pv_array.clear();
        loop {
            match self.pv_table.get(&self.hash) {
                Some(&mv) if count < depth => {
                    self.make_move(&mv);
                    self.pv_array.push(mv);
                    count += 1;
                }
                _ => { break; }
            }
        }

        while self.ply > 0 {
            self.undo_move();
        }
    }
}
