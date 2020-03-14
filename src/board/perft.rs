use crate::board::*;

impl Board {
    pub fn perft(&mut self, depth: u32, verbose: bool) -> u64 {
        debug_assert!(self.check());

        if depth == 0 {
            return 1;
        }

        let move_list = self.generate_all_moves();

        let mut count: u64 = 0;
        let mut new: u64;
        for smv in move_list.moves.iter() {
            if ! self.make_move(&smv.mv) {
                continue;
            }
            new = self.perft(depth - 1, false);
            if verbose {
                println!("move {} : {}", smv.mv.to_string(), new);
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

    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;

    const PERFT_FEN: &'static str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    // A couple fast-running examples:
    
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

    // Full test suite:

    #[test]
    #[ignore]
    fn perft_test_all() {
        // Largest depth in suite is 6
        const MAX_DEPTH: u32 = 6;
        let f = File::open("perftsuite.txt").expect("error opening perftsuite.txt");
        let f = BufReader::new(f);        
        for line in f.lines() {
            perft_test_line(line.unwrap().as_str(), MAX_DEPTH);
        }
    }

    // Run perft test on an individual line from the test suite
    fn perft_test_line(line: &str, max_depth: u32) {
        println!("Testing: {}", line);
        let mut items = line.split(';');
        let fen = items.next().unwrap();
        let mut board = Board::from_fen(fen);

        for depth_entry in items {
            let vals: Vec<&str> = depth_entry.split_whitespace().collect();
            let depth: u32 = vals[0][1..].parse().unwrap();
            let expected: u64 = vals[1].parse().unwrap();
            if depth > max_depth {
                break;
            }
            println!("Depth: {}, {}", depth, expected);

            let got = board.perft(depth, false);
            assert_eq!(expected, got);
        }
    }

}
