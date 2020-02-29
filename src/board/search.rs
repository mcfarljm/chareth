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
    pub fn search(&mut self, info: &SearchInfo) {
    }

    pub fn clear_for_search(&mut self, info: &SearchInfo) {
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
