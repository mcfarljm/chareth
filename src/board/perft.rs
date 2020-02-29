use crate::board::*;
use crate::moves::Move;

impl Board {
    pub fn perft(&mut self, depth: u32, verbose: bool) -> u64 {
        debug_assert!(self.check());

        if depth == 0 {
            return 1;
        }

        let move_list = self.generate_all_moves();

        let mut count: u64 = 0;
        let mut new: u64 = 0;
        for mv in move_list.moves.iter() {
            if ! self.make_move(mv) {
                continue;
            }
            new = self.perft(depth - 1, false);
            if verbose {
                println!("move {} : {}", mv.to_string(), new);
            }
            count += new;
            self.undo_move();
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::board::*;

    const PERFT_FEN: &'static str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    
    #[test]
    fn perft_init_3() {
        let mut board = Board::from_fen(START_FEN);
        let count = board.perft(3, false);
        assert_eq!(count, 8902);
    }

    #[test]
    fn perft_fen2_2() {
        let mut board = Board::from_fen(PERFT_FEN);
        let count = board.perft(2, false);
        assert_eq!(count, 2039);
    }
}
