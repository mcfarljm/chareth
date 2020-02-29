use crate::board::*;

impl Board {
    pub fn is_repetition(&self) -> bool{
        for i in self.hist_ply-self.fifty_move..self.hist_ply-1 {
            if self.history[i as usize].hash == self.hash {
                return true;
            }
        }
        false
    }
}
