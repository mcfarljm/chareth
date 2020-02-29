use crate::moves;
use crate::board;
use crate::board::{Castling,Square};
use crate::pieces;
use crate::pieces::Piece;

pub struct MoveList {
    pub moves: Vec<moves::Move>,
}

impl MoveList {
    fn new() -> MoveList {
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

    fn add_white_pawn_move(&mut self, b: &board::Board, from: Square, to: Square, capture: Piece) {
        // VICE uses two separate functions for capture/no-capture,
        // but by using a closure, we can handle both cases here.
        let mut adder: Box<dyn FnMut(Piece)>;
        if capture.exists() {
            adder = Box::new(|promote: Piece| self.add_capture_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
        }
        else {
            adder = Box::new(|promote: Piece| self.add_quiet_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
        }
        
        if board::RANKS[from as usize] == board::RANK_7 {
            // Add a version of the move with each possible promotion
            for promote in &[Piece::WN, Piece::WB, Piece::WR, Piece::WQ] {
                adder(*promote);
            }
        }
        else {
            adder(Piece::Empty);
        }
    }

    // Todo: this could be consolidated with above function.  Only
    // difference is RANK_2 and BN,BQ
    fn add_black_pawn_move(&mut self, b: &board::Board, from: Square, to: Square, capture: Piece) {
        // VICE uses two separate functions for capture/no-capture,
        // but by using a closure, we can handle both cases here.
        let mut adder: Box<dyn FnMut(Piece)>;
        if capture != Piece::Empty {
            adder = Box::new(|promote: Piece| self.add_capture_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
        }
        else {
            adder = Box::new(|promote: Piece| self.add_quiet_move(b, moves::Move::new(from, to, capture, promote, moves::MoveFlag::None)));
        }
        
        if board::RANKS[from as usize] == board::RANK_2 {
            // Add a version of the move with each possible promotion
            for promote in &[Piece::BN, Piece::BB, Piece::BR, Piece::BQ] {
                adder(*promote);
            }
        }
        else {
            adder(Piece::Empty);
        }
    }

    pub fn print(&self) {
        println!("Move list: {}", self.moves.len());
        for (i, mv) in self.moves.iter().enumerate() {
            println!("Move: {} > {} (score: {})", i+1, mv.to_string(), mv.score());
        }
    }

}

impl board::Board {

    pub fn generate_all_moves(&self) -> MoveList {
        debug_assert!(self.check());

        let mut move_list = MoveList::new();

        let mut t_sq: Square;

        if self.side == pieces::WHITE {
            for sq in &self.piece_lists[Piece::WP as usize] {

                if self.pieces[(sq+10) as usize] == Piece::Empty {
                    // pawn forward one square
                    move_list.add_white_pawn_move(self, *sq, sq+10, Piece::Empty);
                    if board::RANKS[*sq as usize] == board::RANK_2 && self.pieces[(sq+20) as usize] == Piece::Empty {
                        // pawn forward two squares
                        move_list.add_quiet_move(self, moves::Move::new(*sq, sq+20, Piece::Empty, Piece::Empty, moves::MoveFlag::PawnStart));
                    }
                }

                // Check pawn captures in both directions
                let dirs: [i8; 2] = [9, 11];
                for dir in &dirs {
                    t_sq = sq + dir;
                    if board::square_on_board(t_sq) && self.pieces[t_sq as usize].color() == pieces::BLACK {
                        move_list.add_white_pawn_move(self, *sq, t_sq, self.pieces[t_sq as usize]);
                    }
                }

                // Check en passant captures
                if sq + 9 == self.en_pas {
                    move_list.add_capture_move(self, moves::Move::new(*sq, sq+9, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
                if sq + 11 == self.en_pas {
                    move_list.add_capture_move(self, moves::Move::new(*sq, sq+11, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
            }

            // Castling
            if self.castle_perm & Castling::WK != 0 {
                if self.pieces[board::Position::F1 as usize] == Piece::Empty && self.pieces[board::Position::G1 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E1 as Square, pieces::BLACK)) && (!self.square_attacked(board::Position::F1 as Square, pieces::BLACK)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E1 as Square, board::Position::G1 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }

            if self.castle_perm & Castling::WQ != 0 {
                if self.pieces[board::Position::D1 as usize] == Piece::Empty && self.pieces[board::Position::C1 as usize] == Piece::Empty && self.pieces[board::Position::B1 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E1 as Square, pieces::BLACK)) && (!self.square_attacked(board::Position::D1 as Square, pieces::BLACK)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E1 as Square, board::Position::C1 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }
        }
        else {
            for sq in &self.piece_lists[Piece::BP as usize] {

                if self.pieces[(sq-10) as usize] == Piece::Empty {
                    // pawn forward one square
                    move_list.add_black_pawn_move(self, *sq, sq-10, Piece::Empty);
                    if board::RANKS[*sq as usize] == board::RANK_7 && self.pieces[(sq-20) as usize] == Piece::Empty {
                        // pawn forward two squares
                        move_list.add_quiet_move(self, moves::Move::new(*sq, sq-20, Piece::Empty, Piece::Empty, moves::MoveFlag::PawnStart));
                    }
                }

                // Check pawn captures in both directions
                let dirs: [i8; 2] = [9, 11];
                for dir in &dirs {
                    t_sq = sq - dir;
                    if board::square_on_board(t_sq) && self.pieces[t_sq as usize].color() == pieces::WHITE {
                        move_list.add_black_pawn_move(self, *sq, t_sq, self.pieces[t_sq as usize]);
                    }
                }

                // Check en passant captures
                if sq - 9 == self.en_pas {
                    move_list.add_capture_move(self, moves::Move::new(*sq, sq-9, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
                if sq - 11 == self.en_pas {
                    move_list.add_capture_move(self, moves::Move::new(*sq, sq-11, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
            }

            // Castling
            if self.castle_perm & Castling::BK != 0 {
                if self.pieces[board::Position::F8 as usize] == Piece::Empty && self.pieces[board::Position::G8 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E8 as Square, pieces::WHITE)) && (!self.square_attacked(board::Position::F8 as Square, pieces::WHITE)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E8 as Square, board::Position::G8 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }

            if self.castle_perm & Castling::BQ != 0 {
                if self.pieces[board::Position::D8 as usize] == Piece::Empty && self.pieces[board::Position::C8 as usize] == Piece::Empty && self.pieces[board::Position::B8 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E8 as Square, pieces::WHITE)) && (!self.square_attacked(board::Position::D8 as Square, pieces::WHITE)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E8 as Square, board::Position::C8 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }
        }

        let mut t_sq: Square;

        // Sliders
        for piece in pieces::SLIDERS[self.side].iter() {
            for sq in &self.piece_lists[*piece as usize] {

                for dir in &pieces::DIRECTIONS[*piece as usize] {
                    if *dir == 0 {
                        // dir==0 indicates end of list
                        break;
                    }
                    t_sq = *sq + dir;

                    while board::square_on_board(t_sq) {
                        // BLACK ^ 1 == WHITE;  WHITE ^ 1 == BLACK
                        if self.pieces[t_sq as usize] != Piece::Empty {
                            if self.pieces[t_sq as usize].color() == self.side ^ 1 {
                                move_list.add_capture_move(self, moves::Move::new(*sq, t_sq, self.pieces[t_sq as usize], Piece::Empty, moves::MoveFlag::None));
                            }
                            break;
                        }
                        move_list.add_quiet_move(self, moves::Move::new(*sq, t_sq, Piece::Empty, Piece::Empty, moves::MoveFlag::None));
                        t_sq += dir;
                    }
                }
            }
        }

        // Non-sliders
        for piece in &pieces::NON_SLIDERS[self.side] {
            for sq in &self.piece_lists[*piece as usize] {

                for dir in &pieces::DIRECTIONS[*piece as usize] {
                    if *dir == 0 {
                        // dir==0 indicates end of list
                        break;
                    }
                    t_sq = *sq + dir;
                    if ! board::square_on_board(t_sq) {
                        continue;
                    }

                    // BLACK ^ 1 == WHITE;  WHITE ^ 1 == BLACK
                    if self.pieces[t_sq as usize] != Piece::Empty {
                        if self.pieces[t_sq as usize].color() == self.side ^ 1 {
                            move_list.add_capture_move(self, moves::Move::new(*sq, t_sq, self.pieces[t_sq as usize], Piece::Empty, moves::MoveFlag::None));
                        }
                        continue;
                    }
                    move_list.add_quiet_move(self, moves::Move::new(*sq, t_sq, Piece::Empty, Piece::Empty, moves::MoveFlag::None));
                }
            }
        }
        move_list
    }

}

#[cfg(test)]
mod tests {

    use crate::board;

    // Utility function to check move count for a given FEN string
    fn check_move_count(fen: &str, num_moves: usize) {
        let board = board::Board::from_fen(fen);

        let ml = board.generate_all_moves();
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
