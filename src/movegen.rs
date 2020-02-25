use crate::moves;
use crate::board;
use crate::pieces;
use crate::pieces::Piece;

pub struct MoveList {
    pub moves: Vec<moves::Move>,
}

impl MoveList {
    pub fn new() -> MoveList {
        // let l = Vec::<moves::Move>::new();
        MoveList{moves: Vec::<moves::Move>::new()}
    }

    fn add_quiet_move(&mut self, b: &board::Board, mv: moves::Move) {
        self.moves.push(mv);
    }

    fn add_capture_move(&mut self, b: &board::Board, mv: moves::Move) {
        self.moves.push(mv);
    }

    fn add_en_passant_move(&mut self, b: &board::Board, mv: moves::Move) {
        self.moves.push(mv);
    }

    fn add_white_pawn_move(&mut self, b: &board::Board, from: usize, to: usize, capture: usize) {
        // VICE uses two separate functions for capture/no-capture,
        // but by using a closure, we can handle both cases here.
        let mut adder: Box<dyn FnMut(usize)>;
        if capture != Piece::EMPTY {
            adder = Box::new(|promote: usize| self.add_capture_move(b, moves::Move::new(from, to, capture, promote, false, false)));
        }
        else {
            adder = Box::new(|promote: usize| self.add_quiet_move(b, moves::Move::new(from, to, capture, promote, false, false)));
        }
        
        if board::RANKS[from] == board::RANK_7 {
            // Add a version of the move with each possible promotion
            for promote in Piece::WN..=Piece::WQ {
                adder(promote);
            }
        }
        else {
            adder(Piece::EMPTY);
        }
    }

    // Todo: this could be consolidated with above function.  Only
    // difference is RANK_2 and BN,BQ
    fn add_black_pawn_move(&mut self, b: &board::Board, from: usize, to: usize, capture: usize) {
        // VICE uses two separate functions for capture/no-capture,
        // but by using a closure, we can handle both cases here.
        let mut adder: Box<dyn FnMut(usize)>;
        if capture != Piece::EMPTY {
            adder = Box::new(|promote: usize| self.add_capture_move(b, moves::Move::new(from, to, capture, promote, false, false)));
        }
        else {
            adder = Box::new(|promote: usize| self.add_quiet_move(b, moves::Move::new(from, to, capture, promote, false, false)));
        }
        
        if board::RANKS[from] == board::RANK_2 {
            // Add a version of the move with each possible promotion
            for promote in Piece::BN..=Piece::BQ {
                adder(promote);
            }
        }
        else {
            adder(Piece::EMPTY);
        }
    }


    pub fn generate_all_moves(&mut self, b: &board::Board) {
        debug_assert!(b.check());

        let mut t_sq: usize;

        if b.side == pieces::WHITE {
            for sq in &b.piece_lists[Piece::WP] {

                if b.pieces[sq+10] == Piece::EMPTY {
                    // pawn forward one square
                    self.add_white_pawn_move(b, *sq, sq+10, Piece::EMPTY);
                    if board::RANKS[*sq] == board::RANK_2 && b.pieces[sq+20] == Piece::EMPTY {
                        // pawn forward two squares
                        self.add_quiet_move(b, moves::Move::new(*sq, sq+20, Piece::EMPTY, Piece::EMPTY, false, true));
                    }
                }

                // Check pawn captures in both directions
                let dirs: [usize; 2] = [9, 11];
                for dir in &dirs {
                    t_sq = sq + dir;
                    if board::square_on_board(t_sq) && pieces::PIECE_COLOR[b.pieces[t_sq]] == pieces::BLACK {
                        self.add_white_pawn_move(b, *sq, t_sq, b.pieces[t_sq]);
                    }
                }

                // Check en passant captures
                if sq + 9 == b.en_pas {
                    self.add_capture_move(b, moves::Move::new(*sq, sq+9, Piece::EMPTY, Piece::EMPTY, true, false));
                }
                if sq + 11 == b.en_pas {
                    self.add_capture_move(b, moves::Move::new(*sq, sq+11, Piece::EMPTY, Piece::EMPTY, true, false));
                }
            }
        }
        else {
            for sq in &b.piece_lists[Piece::BP] {

                if b.pieces[sq-10] == Piece::EMPTY {
                    // pawn forward one square
                    self.add_black_pawn_move(b, *sq, sq-10, Piece::EMPTY);
                    if board::RANKS[*sq] == board::RANK_7 && b.pieces[sq-20] == Piece::EMPTY {
                        // pawn forward two squares
                        self.add_quiet_move(b, moves::Move::new(*sq, sq-20, Piece::EMPTY, Piece::EMPTY, false, true));
                    }
                }

                // Check pawn captures in both directions
                let dirs: [usize; 2] = [9, 11];
                for dir in &dirs {
                    t_sq = sq - dir;
                    if board::square_on_board(t_sq) && pieces::PIECE_COLOR[b.pieces[t_sq]] == pieces::WHITE {
                        self.add_black_pawn_move(b, *sq, t_sq, b.pieces[t_sq]);
                    }
                }

                // Check en passant captures
                if sq - 9 == b.en_pas {
                    self.add_capture_move(b, moves::Move::new(*sq, sq-9, Piece::EMPTY, Piece::EMPTY, true, false));
                }
                if sq - 11 == b.en_pas {
                    self.add_capture_move(b, moves::Move::new(*sq, sq-11, Piece::EMPTY, Piece::EMPTY, true, false));
                }
            }
        }
    }

    pub fn print(&self) {
        println!("Move list: {}", self.moves.len());
        for (i, mv) in self.moves.iter().enumerate() {
            println!("Move: {} > {} (score: {})", i+1, mv.to_string(), mv.score());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::movegen::*;

    // Utility function to check move count for a given FEN string
    fn check_move_count(fen: &str, num_moves: usize) {
        let board = board::Board::from_fen(fen);

        let mut ml = MoveList::new();
        ml.generate_all_moves(&board);
        assert_eq!(ml.moves.len(), num_moves);
    }
    
    #[test]
    fn white_pawn_start() {
        let pawn_moves_w = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
        // Todo: move count will need to be updated once moves for
        // other pieces are generated
        check_move_count(pawn_moves_w, 26);
    }

    #[test]
    fn black_pawn_start() {
        let pawn_moves_b = "rnbqkbnr/p1p1p3/3p3p/1p1p4/2P1Pp2/8/PP1P1PpP/RNBQKB1R b KQkq e3 0 1";
        check_move_count(pawn_moves_b, 26);
    }
}
