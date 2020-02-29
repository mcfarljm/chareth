use crate::board::*;
use crate::moves;

use std::time::{Duration, Instant};

struct SearchInfo {
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
}

impl Board {
    pub fn search(&mut self, info: &mut SearchInfo) {
        let mut best_move: moves::Move;
        let mut best_score = std::i32::MIN;
        let pv_num = 0;

        self.clear_for_search(info);

        // Iterative deepening
        for current_depth in 1..=info.depth {
            best_score = self.alpha_beta(std::i32::MIN, std::i32::MAX, current_depth, info, true);
            self.get_pv_line(current_depth);
            best_move = self.pv_array[0];
            print!("Depth:{} score:{} move:{} nodes{} ", current_depth, best_score, best_move.to_string(), info.nodes);
            print!("pv");
            for mv in &self.pv_array {
                print!(" {}", mv.to_string());
            }
            println!("");
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
    }

    pub fn alpha_beta(&mut self, alpha: i32, beta: i32, depth: u32, info: &SearchInfo, do_null: bool) -> i32 {
        0
    }

    pub fn quiescence(&mut self, alpha: i32, beta: i32, info: &SearchInfo) -> i32 {
        0
    }

    fn is_repetition(&self) -> bool {
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
