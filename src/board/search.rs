use crate::board::*;
use crate::moves;
use self::movegen::MoveList;

use std::time::{Duration, Instant};

const MATE: i32 = 29000;

// Avoid overflow when negating
const I32_SAFE_MIN: i32 = std::i32::MIN + 1;

pub struct SearchInfo {
    start_time: Instant,
    have_time_limit: bool,
    time_limit: Duration,
    
    depth: u32,
    // depth_set: bool,

    moves_to_go: u32,
    // infinite: bool,

    // Count of all positioned visited
    nodes: u64,

    quit: bool,
    pub stopped: bool,

    fail_high: u32,
    fail_high_first: u32,
}

impl SearchInfo {
    pub fn new(depth: u32) -> SearchInfo {
        SearchInfo{
            start_time: Instant::now(),
            time_limit: Duration::new(0,0),
            have_time_limit: false,
            
            depth: depth,
            // depth_set: true,

            moves_to_go: 0,
            // infinite: false,

            nodes: 0,
            
            quit: false,
            stopped: false,

            fail_high: 0,
            fail_high_first: 0
        }
    }

    // Set the time limit and start counting
    pub fn set_time_limit(&mut self, duration: Duration) {
        self.start_time = Instant::now();
        self.time_limit = duration;
        self.have_time_limit = true;
    }

    pub fn unset_time_limit(&mut self) {
        self.have_time_limit = false;
    }

    pub fn set_depth(&mut self, depth: u32) {
        self.depth = depth;
        // self.depth_set = true;
    }
    
    pub fn checkup(&mut self) {
        if self.have_time_limit && self.start_time.elapsed() > self.time_limit {
            self.stopped = true;
        }
    }

    pub fn maybe_checkup(&mut self) {
        if self.nodes > 0 && self.nodes % 2000 == 0 {
            self.checkup();
        }
    }
}

// Modify movelist in place by switching selected move into position
// move_num
fn pick_next_move(move_num: usize, move_list: &mut MoveList) {
    let mut best_score = 0;
    let mut best_num = move_num;
    for i in move_num..move_list.moves.len() {
        if move_list.moves[i].score > best_score {
            best_score = move_list.moves[i].score;
            best_num = i;
        }

    }
    move_list.moves.swap(move_num, best_num);
}

impl Board {
    pub fn search(&mut self, info: &mut SearchInfo) {
        let mut best_move: Option<moves::Move> = None;
        let mut best_score;

        self.clear_for_search(info);

        // Iterative deepening
        for current_depth in 1..=info.depth {
            best_score = self.alpha_beta(I32_SAFE_MIN, std::i32::MAX, current_depth, info, true);

            if info.stopped {
                break;
            }
            
            self.get_pv_line(current_depth);
            best_move = Some(self.pv_array[0]);

            print!("info score cp score {} depth {} nodes {} time {} ",
                   best_score, current_depth, info.nodes, info.start_time.elapsed().as_millis());

            print!("pv");
            for mv in &self.pv_array {
                print!(" {}", mv.to_string());
            }
            println!("");
            println!("Ordering: {:.2}", info.fail_high_first as f32 /info.fail_high as f32);
        }

        if let Some(mv) = best_move {
            println!("bestmove {}", mv.to_string());
        }
    }

    pub fn clear_for_search(&mut self, info: &mut SearchInfo) {
        self.search_history = [[0; BOARD_SQ_NUM]; 13];
        self.search_killers.clear();

        self.pv_table.clear();
        self.pv_array.clear();

        self.ply = 0;

        info.stopped = false;
        info.nodes = 0;

        info.fail_high = 0;
        info.fail_high_first = 0;
    }

    pub fn alpha_beta(&mut self, alpha_in: i32, beta: i32, depth: u32, info: &mut SearchInfo, _do_null: bool) -> i32 {
        debug_assert!(self.check());

        if depth == 0 {
            return self.quiescence(alpha_in, beta, info);
        }

        info.maybe_checkup();

        info.nodes += 1;

        if self.is_repetition() || self.fifty_move >= 100 {
            return 0;
        }

        let mut move_list = self.generate_all_moves();

        // Score PV move first if found
        match self.pv_table.get(&self.hash) {
            Some(pv_mv) => {
                for smv in move_list.moves.iter_mut() {
                    if smv.mv == *pv_mv {
                        // Prioritize above all other moves
                        smv.score = 2_000_000;
                        break;
                    }
                }
            }
            _ => (),
        }

        let mut legal = 0;
        let mut alpha = alpha_in;
        let mut score;
        // Use option to workaround uninitalized values
        let mut best_move: Option<moves::Move> = None;

        // Loop is not done with iter, because pick_next_move may swap
        // elements in the move list
        for imove in 0..move_list.moves.len() {
            pick_next_move(imove, &mut move_list);
            let smv = &move_list.moves[imove];
            
            if ! self.make_move(&smv.mv) {
                continue;
            }
            legal += 1;
            score = - self.alpha_beta(-beta, -alpha, depth-1, info, true);
            self.undo_move();

            if info.stopped {
                return 0;
            }

            if score > alpha {
                if score >= beta {
                    if legal == 1 {
                        info.fail_high_first += 1;
                    }
                    info.fail_high += 1;

                    if ! smv.mv.is_capture() {
                        // So-called "killer" move (non-capture
                        // causing beta cutoff)
                        let killers = self.search_killers.entry(self.ply).or_insert([None, None]);
                        killers.swap(0,1); // Really just need k[1]=k[0];
                        killers[0] = Some(smv.mv);
                    }
                    
                    return beta;
                }
                alpha = score;
                best_move = Some(smv.mv);
                if ! smv.mv.is_capture() {
                    // VICE video 64: mentions prioritizing moves
                    // "nearest to ply", but this seems to be the
                    // opposite of adding depth?
                    self.search_history[self.pieces[smv.mv.from() as usize] as usize][smv.mv.to() as usize] += depth;
                }
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

    // Mitigate the horizon effect by making sure evaluation is only
    // done at quiet positions
    pub fn quiescence(&mut self, alpha_in: i32, beta: i32, info: &mut SearchInfo) -> i32 {
        debug_assert!(self.check());

        info.maybe_checkup();

        info.nodes += 1;

        if self.is_repetition() || self.fifty_move >= 100 {
            return 0;
        }

        let mut alpha = alpha_in;

        let mut score = self.evaluate();
        if score >= beta {
            return beta;
        }
        else if score > alpha {
            // Standing pat
            alpha = score;
        }

        let mut legal = 0;
        // Use option to workaround uninitalized values
        let mut best_move: Option<moves::Move> = None;

        let old_alpha = alpha;

        let mut move_list = self.generate_all_captures();
        // Loop is not done with iter, because pick_next_move may swap
        // elements in the move list
        for imove in 0..move_list.moves.len() {
            pick_next_move(imove, &mut move_list);
            let smv = &move_list.moves[imove];
            
            if ! self.make_move(&smv.mv) {
                continue;
            }
            legal += 1;
            score = - self.quiescence(-beta, -alpha, info);
            self.undo_move();

            if info.stopped {
                return 0;
            }

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

        if alpha != old_alpha {
            self.store_pv_move(best_move.unwrap());
        }

        alpha
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

#[cfg(test)]
mod tests {
    use crate::board::*;
    
    #[test]
    fn search_start_depth3() {
        let mut board = Board::from_fen(START_FEN);
        let mut info = SearchInfo::new(3); 
        board.search(&mut info);
        assert_eq!(board.pv_array[0].to_string(), "d2d4");
        assert_eq!(info.nodes, 637);
    }

    #[test]
    fn search_wac1_depth3() {
        let wa_c1 = "r1b1k2r/ppppnppp/2n2q2/2b5/3NP3/2P1B3/PP3PPP/RN1QKB1R w KQkq - 0 1";
        let mut board = Board::from_fen(wa_c1);
        let mut info = SearchInfo::new(3); 
        board.search(&mut info);
        assert_eq!(board.pv_array[0].to_string(), "f1c4");
        assert_eq!(info.nodes, 5965);
    }
}
