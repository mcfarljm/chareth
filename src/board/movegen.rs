use crate::moves;
use crate::board::{self,WHITE,BLACK,BOTH};
use crate::board::{Castling,Square};
use crate::pieces::{self,Piece,PIECE_TYPES,NUM_PIECE_TYPES_BOTH,KNIGHT_MOVES,KING_MOVES};
use crate::bitboard::{Bitboard,BB_RANK_4,BB_RANK_5,BB_FILE_A,BB_FILE_H,get_rook_attacks,get_bishop_attacks,get_queen_attacks};

// Could be a method of Piece, but nice to have it here for
// organizational purposes
fn victim_score(piece: &Piece) -> i32 {
    match piece {
        Piece::WP | Piece::BP => 100,
        Piece::WN | Piece::BN => 200,
        Piece::WB | Piece::BB => 300,
        Piece::WR | Piece::BR => 400,
        Piece::WQ | Piece::BQ => 500,
        Piece::WK | Piece::BK => 600,
        _ => 0,
    }
}

lazy_static! {
    static ref MVV_LVA_SCORES: [[i32; NUM_PIECE_TYPES_BOTH]; NUM_PIECE_TYPES_BOTH] = get_mvv_lva();
}

pub struct ScoredMove {
    pub mv: moves::Move,
    pub score: i32,
}

// Having a separate data structure for move items that go into the
// list is similar to what is done in VICE.  Alternatively, since we
// already have a Move structure (VICE just uses an int), we could
// include the score there.  With that approach, it would be good to
// then represent score as an Option, since it would be initially
// undefined, whereas the ScoredMove structure ensures that we only
// create entries that do have a score defined.
impl ScoredMove {
    pub fn new(mv: moves::Move, score: i32) -> ScoredMove {
        ScoredMove{
            mv: mv,
            score: score,
        }
    }
}

pub struct MoveList {
    pub moves: Vec<ScoredMove>,
}

impl MoveList {
    fn new() -> MoveList {
        // Speed up by avoiding reallocation
        MoveList{moves: Vec::<ScoredMove>::with_capacity(256)}
    }

    fn add_quiet_move(&mut self, b: &board::Board, mv: moves::Move) {
        debug_assert!(board::square_on_board(mv.from()));
        debug_assert!(board::square_on_board(mv.to()));

        let mut score = match b.search_killers[b.ply as usize][0] {
            Some(kmv) if kmv == mv => 900_000,
            _ => match b.search_killers[b.ply as usize][1] {
                Some(kmv) if kmv == mv => 800_000,
                _ => 0,
            }
        };
                
        if score == 0 {
            score = b.search_history[b.pieces[mv.from() as usize] as usize][mv.to() as usize] as i32;
        }
        
        self.moves.push(ScoredMove::new(mv, score));
    }

    fn add_capture_move(&mut self, b: &board::Board, mv: moves::Move) {
        debug_assert!(board::square_on_board(mv.from()));
        debug_assert!(board::square_on_board(mv.to()));
        debug_assert!(mv.capture.exists());
        let score = MVV_LVA_SCORES[mv.capture as usize][b.pieces[mv.from() as usize] as usize] + 1_000_000;
        self.moves.push(ScoredMove::new(mv, score));
    }

    fn add_en_passant_move(&mut self, _b: &board::Board, mv: moves::Move) {
        debug_assert!(board::square_on_board(mv.from()));
        debug_assert!(board::square_on_board(mv.to()));
        self.moves.push(ScoredMove::new(mv, 105 + 1_000_000));
    }

    fn add_white_pawn_move(&mut self, b: &board::Board, from: Square, to: Square, capture: Piece) {
        
        if from/8 == board::RANK_7 {
            // Add a version of the move with each possible promotion
            for promote in &[Piece::WN, Piece::WB, Piece::WR, Piece::WQ] {
                self.add_quiet_move(b, moves::Move::new(from, to, capture, *promote, moves::MoveFlag::None));
            }
        }
        else {
            self.add_quiet_move(b, moves::Move::new(from, to, capture, Piece::Empty, moves::MoveFlag::None));
        }
    }

    
    fn add_white_pawn_capture_move(&mut self, b: &board::Board, from: Square, to: Square, capture: Piece) {
        
        if from/8 == board::RANK_7 {
            // Add a version of the move with each possible promotion
            for promote in &[Piece::WN, Piece::WB, Piece::WR, Piece::WQ] {
                self.add_capture_move(b, moves::Move::new(from, to, capture, *promote, moves::MoveFlag::None));
            }
        }
        else {
            self.add_capture_move(b, moves::Move::new(from, to, capture, Piece::Empty, moves::MoveFlag::None));
        }
    }

    fn add_black_pawn_move(&mut self, b: &board::Board, from: Square, to: Square, capture: Piece) {
        
        if from/8 == board::RANK_2 {
            // Add a version of the move with each possible promotion
            for promote in &[Piece::BN, Piece::BB, Piece::BR, Piece::BQ] {
                self.add_quiet_move(b, moves::Move::new(from, to, capture, *promote, moves::MoveFlag::None));
            }
        }
        else {
            self.add_quiet_move(b, moves::Move::new(from, to, capture, Piece::Empty, moves::MoveFlag::None));
        }
    }

    
    fn add_black_pawn_capture_move(&mut self, b: &board::Board, from: Square, to: Square, capture: Piece) {
        
        if from/8 == board::RANK_2 {
            // Add a version of the move with each possible promotion
            for promote in &[Piece::BN, Piece::BB, Piece::BR, Piece::BQ] {
                self.add_capture_move(b, moves::Move::new(from, to, capture, *promote, moves::MoveFlag::None));
            }
        }
        else {
            self.add_capture_move(b, moves::Move::new(from, to, capture, Piece::Empty, moves::MoveFlag::None));
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("Move list: {}", self.moves.len());
        for (i, smv) in self.moves.iter().enumerate() {
            println!("Move: {} > {} (score: {})", i+1, smv.mv, smv.score);
        }
    }

}

impl board::Board {

    pub fn generate_all_moves(&self) -> MoveList {
        self.generate_moves(true)
    }

    pub fn generate_all_captures(&self) -> MoveList {
        self.generate_moves(false)
    }

    fn generate_moves(&self, non_captures: bool) -> MoveList {
        debug_assert!(self.check());

        let mut move_list = MoveList::new();

        if self.side == pieces::WHITE {

            // Pawn non-captures:
            if non_captures {

                let to_step1 = (self.bitboards[Piece::WP as usize].0 << 8) & (!self.bb_sides[BOTH].0);
                let to_step2 = (to_step1 << 8) & BB_RANK_4 & (!self.bb_sides[BOTH].0);

                for to64 in Bitboard(to_step1).into_iter() {
                    move_list.add_white_pawn_move(self, to64 - 8, to64, Piece::Empty); 
                }
                for to64 in Bitboard(to_step2).into_iter() {
                    move_list.add_quiet_move(self, moves::Move::new(to64 - 2*8, to64, Piece::Empty, Piece::Empty, moves::MoveFlag::PawnStart));
                }
            }

            // Pawn captures:
            let to_cap_left = ((self.bitboards[Piece::WP as usize].0 & !BB_FILE_A) << 7) & self.bb_sides[BLACK].0;
            let to_cap_right = ((self.bitboards[Piece::WP as usize].0 & !BB_FILE_H) << 9) & self.bb_sides[BLACK].0;

            for to64 in Bitboard(to_cap_left).into_iter() {
                move_list.add_white_pawn_capture_move(self, to64 - 7, to64, self.pieces[to64]); 
            }
            for to64 in Bitboard(to_cap_right).into_iter() {
                move_list.add_white_pawn_capture_move(self, to64 - 9, to64, self.pieces[to64]); 
            }

            // Check en passant captures
            if self.en_pas != board::Position::NONE as Square {
                let ep_bb = Bitboard(1 << self.en_pas);
                let ep_to_left = ((self.bitboards[Piece::WP as usize].0 & ! BB_FILE_A) << 7) & ep_bb.0;
                let ep_to_right = ((self.bitboards[Piece::WP as usize].0 & ! BB_FILE_H) << 9) & ep_bb.0;

                for to64 in Bitboard(ep_to_left).into_iter() {
                    move_list.add_en_passant_move(self, moves::Move::new(to64-7, to64, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
                for to64 in Bitboard(ep_to_right).into_iter() {
                    move_list.add_en_passant_move(self, moves::Move::new(to64-9, to64, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
            }


            // Castling
            if non_captures && self.castle_perm & Castling::WK != 0 {
                if self.pieces[board::Position::F1 as usize] == Piece::Empty && self.pieces[board::Position::G1 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E1 as Square, pieces::BLACK)) && (!self.square_attacked(board::Position::F1 as Square, pieces::BLACK)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E1 as Square, board::Position::G1 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }

            if non_captures && self.castle_perm & Castling::WQ != 0 {
                if self.pieces[board::Position::D1 as usize] == Piece::Empty && self.pieces[board::Position::C1 as usize] == Piece::Empty && self.pieces[board::Position::B1 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E1 as Square, pieces::BLACK)) && (!self.square_attacked(board::Position::D1 as Square, pieces::BLACK)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E1 as Square, board::Position::C1 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }
        }
        else {

            // Pawn non-captures:
            if non_captures {

                let to_step1 = (self.bitboards[Piece::BP as usize].0 >> 8) & (!self.bb_sides[BOTH].0);
                let to_step2 = (to_step1 >> 8) & BB_RANK_5 & (!self.bb_sides[BOTH].0);

                for to64 in Bitboard(to_step1).into_iter() {
                    move_list.add_black_pawn_move(self, to64 + 8, to64, Piece::Empty); 
                }
                for to64 in Bitboard(to_step2).into_iter() {
                    move_list.add_quiet_move(self, moves::Move::new(to64 + 2*8, to64, Piece::Empty, Piece::Empty, moves::MoveFlag::PawnStart));
                }
            }

            // Pawn captures:
            let to_cap_left = ((self.bitboards[Piece::BP as usize].0 & !BB_FILE_A) >> 9) & self.bb_sides[WHITE].0;
            let to_cap_right = ((self.bitboards[Piece::BP as usize].0 & !BB_FILE_H) >> 7) & self.bb_sides[WHITE].0;

            for to64 in Bitboard(to_cap_left).into_iter() {
                move_list.add_black_pawn_capture_move(self, to64 + 9, to64, self.pieces[to64]); 
            }
            for to64 in Bitboard(to_cap_right).into_iter() {
                move_list.add_black_pawn_capture_move(self, to64 + 7, to64, self.pieces[to64]); 
            }

            // Check en passant captures
            if self.en_pas != board::Position::NONE as Square {
                let ep_bb = Bitboard(1 << self.en_pas);
                let ep_to_left = ((self.bitboards[Piece::BP as usize].0 & ! BB_FILE_A) >> 9) & ep_bb.0;
                let ep_to_right = ((self.bitboards[Piece::BP as usize].0 & ! BB_FILE_H) >> 7) & ep_bb.0;

                for to64 in Bitboard(ep_to_left).into_iter() {
                    move_list.add_en_passant_move(self, moves::Move::new(to64+9, to64, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
                for to64 in Bitboard(ep_to_right).into_iter() {
                    move_list.add_en_passant_move(self, moves::Move::new(to64+7, to64, Piece::Empty, Piece::Empty, moves::MoveFlag::EnPas));
                }
            }

            // Castling
            if non_captures && self.castle_perm & Castling::BK != 0 {
                if self.pieces[board::Position::F8 as usize] == Piece::Empty && self.pieces[board::Position::G8 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E8 as Square, pieces::WHITE)) && (!self.square_attacked(board::Position::F8 as Square, pieces::WHITE)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E8 as Square, board::Position::G8 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }

            if non_captures && self.castle_perm & Castling::BQ != 0 {
                if self.pieces[board::Position::D8 as usize] == Piece::Empty && self.pieces[board::Position::C8 as usize] == Piece::Empty && self.pieces[board::Position::B8 as usize] == Piece::Empty {
                    if (! self.square_attacked(board::Position::E8 as Square, pieces::WHITE)) && (!self.square_attacked(board::Position::D8 as Square, pieces::WHITE)) {
                        move_list.add_quiet_move(self, moves::Move::new(board::Position::E8 as Square, board::Position::C8 as Square, Piece::Empty, Piece::Empty, moves::MoveFlag::Castle));
                    }
                }
            }
        }

        // Sliders
        for piece in &pieces::SLIDERS[self.side] {
            for sq64 in self.bitboards[*piece as usize] {
                let mut attacks = match piece {
                    Piece::WR | Piece::BR => get_rook_attacks(sq64, self.bb_sides[BOTH].0),
                    Piece::WB | Piece::BB => get_bishop_attacks(sq64, self.bb_sides[BOTH].0),
                    Piece::WQ | Piece::BQ => get_queen_attacks(sq64, self.bb_sides[BOTH].0),
                    _ => unreachable!(),
                };
                attacks &= !self.bb_sides[self.side].0;
                // Todo: once sq64 representation is adopted, would it be
                // faster to use bitwise operations to first find captures
                // and non_captures, and then iterate over those?
                for t_sq in Bitboard(attacks).into_iter() { 
                    let t_piece = self.pieces[t_sq];
                    if t_piece != Piece::Empty {
                        move_list.add_capture_move(self, moves::Move::new(sq64, t_sq, t_piece, Piece::Empty, moves::MoveFlag::None));
                    } else if non_captures {
                        move_list.add_quiet_move(self, moves::Move::new(sq64, t_sq, Piece::Empty, Piece::Empty, moves::MoveFlag::None));
                    }
                    
                }
            }
        }

        // Non-sliders
        for piece in &pieces::NON_SLIDERS[self.side] {
            for sq64 in self.bitboards[*piece as usize].into_iter() {
                let bb = match *piece {
                    Piece::WN | Piece::BN => KNIGHT_MOVES[sq64],
                    Piece::WK | Piece::BK => KING_MOVES[sq64],
                    _ => { unreachable!() },
                };
                // Take moves bitboard and filter out side's pieces
                let iterator = Bitboard(bb.0 & !self.bb_sides[self.side].0).into_iter();
                for t_sq in iterator {
                    let t_piece = self.pieces[t_sq];
                    if t_piece != Piece::Empty {
                        // Have already filtered out own side's pieces on target square
                        move_list.add_capture_move(self, moves::Move::new(sq64, t_sq, t_piece, Piece::Empty, moves::MoveFlag::None));
                    } else if non_captures {
                        move_list.add_quiet_move(self, moves::Move::new(sq64, t_sq, Piece::Empty, Piece::Empty, moves::MoveFlag::None));
                    }
                }
            }
        }
        move_list
    }
}

// Not necessary as the lazy static is automatically initialized, but
// provides a way to force initialization when the program starts
pub fn init_mvv_lva() {
    lazy_static::initialize(&MVV_LVA_SCORES);
}

// Initialize most valuable victim, least valuable attacker array
fn get_mvv_lva() -> [[i32; NUM_PIECE_TYPES_BOTH]; NUM_PIECE_TYPES_BOTH] {
    let mut mvv_lva_scores: [[i32; NUM_PIECE_TYPES_BOTH]; NUM_PIECE_TYPES_BOTH] = [[0; NUM_PIECE_TYPES_BOTH]; NUM_PIECE_TYPES_BOTH];
    for attacker in &PIECE_TYPES {
        for victim in &PIECE_TYPES {
            mvv_lva_scores[*victim as usize][*attacker as usize] = victim_score(victim) + 6 - victim_score(attacker)/100;
        }
    }
    mvv_lva_scores
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
