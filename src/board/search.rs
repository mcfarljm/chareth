use crate::board::*;
use crate::moves;

impl Board {
    pub fn is_repetition(&self) -> bool {
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
