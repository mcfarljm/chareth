use crate::moves;
use crate::board;
use crate::board::Castling;
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
            adder = Box::new(|promote: usize| self.add_capture_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
        }
        else {
            adder = Box::new(|promote: usize| self.add_quiet_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
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
            adder = Box::new(|promote: usize| self.add_capture_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
        }
        else {
            adder = Box::new(|promote: usize| self.add_quiet_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
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
                        self.add_quiet_move(b, moves::Move::new(*sq, sq+20, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::pawn_start));
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
                    self.add_capture_move(b, moves::Move::new(*sq, sq+9, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::en_pas));
                }
                if sq + 11 == b.en_pas {
                    self.add_capture_move(b, moves::Move::new(*sq, sq+11, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::en_pas));
                }
            }

            // Castling
            if b.castle_perm & Castling::WK != 0 {
                if b.pieces[board::Position::F1 as usize] == Piece::EMPTY && b.pieces[board::Position::G1 as usize] == Piece::EMPTY {
                    if (! b.square_attacked(board::Position::E1 as usize, pieces::BLACK)) && (!b.square_attacked(board::Position::F1 as usize, pieces::BLACK)) {
                        self.add_quiet_move(b, moves::Move::new(board::Position::E1 as usize, board::Position::G1 as usize, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::castle));
                    }
                }
            }

            if b.castle_perm & Castling::WQ != 0 {
                if b.pieces[board::Position::D1 as usize] == Piece::EMPTY && b.pieces[board::Position::C1 as usize] == Piece::EMPTY && b.pieces[board::Position::B1 as usize] == Piece::EMPTY {
                    if (! b.square_attacked(board::Position::E1 as usize, pieces::BLACK)) && (!b.square_attacked(board::Position::D1 as usize, pieces::BLACK)) {
                        self.add_quiet_move(b, moves::Move::new(board::Position::E1 as usize, board::Position::C1 as usize, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::castle));
                    }
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
                        self.add_quiet_move(b, moves::Move::new(*sq, sq-20, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::pawn_start));
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
                    self.add_capture_move(b, moves::Move::new(*sq, sq-9, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::en_pas));
                }
                if sq - 11 == b.en_pas {
                    self.add_capture_move(b, moves::Move::new(*sq, sq-11, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::en_pas));
                }
            }

            // Castling
            if b.castle_perm & Castling::BK != 0 {
                if b.pieces[board::Position::F8 as usize] == Piece::EMPTY && b.pieces[board::Position::G8 as usize] == Piece::EMPTY {
                    if (! b.square_attacked(board::Position::E8 as usize, pieces::WHITE)) && (!b.square_attacked(board::Position::F8 as usize, pieces::WHITE)) {
                        self.add_quiet_move(b, moves::Move::new(board::Position::E8 as usize, board::Position::G8 as usize, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::castle));
                    }
                }
            }

            if b.castle_perm & Castling::BQ != 0 {
                if b.pieces[board::Position::D8 as usize] == Piece::EMPTY && b.pieces[board::Position::C8 as usize] == Piece::EMPTY && b.pieces[board::Position::B8 as usize] == Piece::EMPTY {
                    if (! b.square_attacked(board::Position::E8 as usize, pieces::WHITE)) && (!b.square_attacked(board::Position::D8 as usize, pieces::WHITE)) {
                        self.add_quiet_move(b, moves::Move::new(board::Position::E8 as usize, board::Position::C8 as usize, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::castle));
                    }
                }
            }
        }

        let mut t_sq: usize;

        // Sliders
        for piece in pieces::SLIDERS[b.side].iter() {
            for sq in &b.piece_lists[*piece] {

                for dir in &pieces::DIRECTIONS[*piece] {
                    t_sq = (*sq as i32 + dir) as usize;

                    while board::square_on_board(t_sq) {
                        // BLACK ^ 1 == WHITE;  WHITE ^ 1 == BLACK
                        if b.pieces[t_sq] != Piece::EMPTY {
                            if pieces::PIECE_COLOR[b.pieces[t_sq]] == b.side ^ 1 {
                                self.add_capture_move(b, moves::Move::new(*sq, t_sq, b.pieces[t_sq], Piece::EMPTY, moves::MoveFlag::None));
                            }
                            break;
                        }
                        self.add_quiet_move(b, moves::Move::new(*sq, t_sq, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::None));
                        t_sq = (t_sq as i32 + dir) as usize;
                    }
                }
            }
        }

        // Non-sliders
        for piece in &pieces::NON_SLIDERS[b.side] {
            for sq in &b.piece_lists[*piece] {

                for dir in &pieces::DIRECTIONS[*piece] {
                    t_sq = (*sq as i32 + dir) as usize;
                    if ! board::square_on_board(t_sq) {
                        continue;
                    }

                    // BLACK ^ 1 == WHITE;  WHITE ^ 1 == BLACK
                    if b.pieces[t_sq] != Piece::EMPTY {
                        if pieces::PIECE_COLOR[b.pieces[t_sq]] == b.side ^ 1 {
                            self.add_capture_move(b, moves::Move::new(*sq, t_sq, b.pieces[t_sq], Piece::EMPTY, moves::MoveFlag::None));
                        }
                        continue;
                    }
                    self.add_quiet_move(b, moves::Move::new(*sq, t_sq, Piece::EMPTY, Piece::EMPTY, moves::MoveFlag::None));
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
        // These were originally used in VICE to check pawn moves, of
        // which there are 26.
        let pawn_moves_w = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
        check_move_count(pawn_moves_w, 42);
    }

    #[test]
    fn black_pawn_start() {
        let pawn_moves_b = "rnbqkbnr/p1p1p3/3p3p/1p1p4/2P1Pp2/8/PP1P1PpP/RNBQKB1R b KQkq e3 0 1";
        check_move_count(pawn_moves_b, 42);
    }

    #[test]
    fn castling() {
        // A fairly complicated setup used in VICE video 36
        let castle_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        check_move_count(castle_fen, 48);
    }
}
