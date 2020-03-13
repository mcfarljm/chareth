use crate::board::*;
use crate::moves;
use self::movegen::MoveList;

use std::time::{Duration, Instant};
use std::sync::mpsc::Receiver;

const MATE: i32 = 29000;
pub const MAX_DEPTH: u32 = 64;

// Avoid overflow when negating
const I32_SAFE_MIN: i32 = std::i32::MIN + 1;

#[derive(PartialEq)]
pub enum GameMode {
    Uci,
    Xboard,
    Console,
    None,
}

pub struct SearchInfo<'a> {
    start_time: Instant,
    have_time_limit: bool,
    time_limit: Duration,
    
    depth: u32,
    // depth_set: bool,

    // moves_to_go: u32,
    // infinite: bool,

    // Count of all positioned visited
    nodes: u64,

    pub quit: bool,
    stopped: bool,

    fail_high: u32,
    fail_high_first: u32,

    message_channel: Option<&'a Receiver<String>>,

    game_mode: GameMode,
    show_thinking: bool,
}

impl<'a> SearchInfo<'a> {
    pub fn new(depth: u32, game_mode: GameMode) -> SearchInfo<'a> {
        SearchInfo{
            start_time: Instant::now(),
            time_limit: Duration::new(0,0),
            have_time_limit: false,
            
            depth: depth,
            // depth_set: true,

            // moves_to_go: 0,
            // infinite: false,

            nodes: 0,
            
            quit: false,
            stopped: false,

            fail_high: 0,
            fail_high_first: 0,

            message_channel: None,

            game_mode: game_mode,
            show_thinking: true,
        }
    }

    // Set the time limit and start counting
    fn set_time_limit(&mut self, duration: Duration) {
        self.start_time = Instant::now();
        self.time_limit = duration;
        self.have_time_limit = true;
    }

    // Set search time based on current clock conditions, and start counting
    //
    // If time_left and move_time are both None, then unset the clock.
    // move_time should be mutually exclusive with time_left and inc
    pub fn set_search_time(&mut self, time_left: Option<u64>, move_time: Option<u64>, moves_to_go: u32, increment: Option<u64>) {
        const BUFFER: Duration = Duration::from_millis(50);
        // Lower limit to make sure we have enough time to at least find a move
        const MIN_TIME: Duration = Duration::from_millis(50);

        let mut time_avail: Duration = MIN_TIME;

        if let Some(mt) = move_time {
            time_avail = Duration::from_millis(mt);
        } else if let Some(tl) = time_left {
            if let Some(inc) = increment {
                // This is different from VICE.  VICE adds the
                // increment to the current move, but don't believe
                // that is correct.  With a large increment relative
                // to time left, the time allocated to thinking could
                // exceed the time left.
                time_avail = Duration::from_millis(tl + (moves_to_go as u64 - 1) * inc);
            } else {
                time_avail = Duration::from_millis(tl);
            }
            time_avail /= moves_to_go;
        } else {
            // Don't have any time settings, so just make sure the
            // time limit is not set
            self.unset_time_limit();
            return;
        }
        if time_avail > BUFFER {
            time_avail -= BUFFER;
        }
        time_avail = std::cmp::max(MIN_TIME, time_avail);

        self.set_time_limit(time_avail);
        println!("Search time set: {:?}", time_avail);
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
        if let Some(rx) = self.message_channel {
            if let Ok(m) = rx.try_recv() {
                // println!("Message received: {}", m);
                if m.starts_with("quit") {
                    self.quit = true;
                    self.stopped = true;
                } else if m.starts_with("stop") {
                    // UCI
                    self.stopped = true;
                } else if m.starts_with("?") {
                    // Used by xboard
                    self.stopped = true;
                }
            }
        }
    }

    pub fn maybe_checkup(&mut self) {
        if self.nodes > 0 && self.nodes % 2000 == 0 {
            self.checkup();
        }
    }

    pub fn set_receiver(&mut self, rx: &'a Receiver<String>) {
        self.message_channel = Some(rx);
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
    pub fn search(&mut self, info: &mut SearchInfo) -> Option<moves::Move> {
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

            match info.game_mode {
                GameMode::Uci => {
                    print!("info score cp {} depth {} nodes {} time {} ",
                           best_score, current_depth, info.nodes, info.start_time.elapsed().as_millis());
                }
                GameMode::Xboard if info.show_thinking => {
                    print!("{} {} {} {} ",
                            current_depth, best_score, info.start_time.elapsed().as_millis() / 10, info.nodes);
                }
                GameMode::Console if info.show_thinking => {
                    print!("score {} depth {} nodes {} time {} ",
                           best_score, current_depth, info.nodes, info.start_time.elapsed().as_millis());
                }
                _ => (),
            }
            if info.game_mode == GameMode::Uci || info.show_thinking {
                if info.game_mode != GameMode::Xboard {
                    print!("pv");
                }
                for mv in &self.pv_array {
                    print!(" {}", mv.to_string());
                }
                println!("");
            }

            // println!("Ordering: {:.2}", info.fail_high_first as f32 /info.fail_high as f32);
        }

        match info.game_mode {
            GameMode::Uci => {
                if let Some(mv) = best_move {
                    println!("bestmove {}", mv.to_string());
                }
            }
            GameMode::Xboard => {
                if let Some(mv) = best_move {
                    println!("move {}", mv.to_string());
                }
            }
            GameMode::Console => {
                if let Some(mv) = best_move {
                    println!("{} makes move: {}", PROGRAM_NAME, mv.to_string());
                }
            }
            _ => (),
        }
        best_move
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

    pub fn alpha_beta(&mut self, alpha_in: i32, beta: i32, depth_in: u32, info: &mut SearchInfo, _do_null: bool) -> i32 {
        debug_assert!(self.check());

        let mut depth = depth_in;

        // Check extension.  See also VICE video 76.  I am somewhat
        // unclear on the different types of check extensions
        // discussed online ("extending check evasion" vs "extending
        // checking move").  Seems that there are three places this
        // could be done:  prior to quiescence, directly after
        // quiescence (VICE and tscp), or after make_move.  Don't like
        // the idea of doing it after quiescence because then we could
        // enter quiescence while in check.
        let in_check = self.square_attacked(self.king_sq[self.side], self.side^1);
        if in_check {
            depth += 1;
        }

        if depth == 0 {
            return self.quiescence(alpha_in, beta, info);
        }

        info.maybe_checkup();

        info.nodes += 1;

        if (self.is_repetition() || self.fifty_move >= 100) && self.ply > 0 {
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
            if in_check {
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
        let mut info = SearchInfo::new(3, GameMode::None); 
        board.search(&mut info);
        assert_eq!(board.pv_array[0].to_string(), "d2d4");
        assert_eq!(info.nodes, 664);
    }

    #[test]
    fn search_wac1_depth3() {
        let wa_c1 = "r1b1k2r/ppppnppp/2n2q2/2b5/3NP3/2P1B3/PP3PPP/RN1QKB1R w KQkq - 0 1";
        let mut board = Board::from_fen(wa_c1);
        let mut info = SearchInfo::new(3, GameMode::None); 
        board.search(&mut info);
        assert_eq!(board.pv_array[0].to_string(), "f1c4");
        assert_eq!(info.nodes, 6605);
    }
}
