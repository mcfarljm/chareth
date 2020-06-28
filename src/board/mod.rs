mod movegen;
mod makemove;
mod perft;
mod io;
mod search;
mod evaluate;
mod uci;

use rand::Rng;
use std::collections::HashMap;
use std::fmt;

use crate::pieces::*;
use crate::bitboard::{self,Bitboard};
use crate::validate;
use crate::moves;
use crate::version::PROGRAM_NAME;

use evaluate::MIRROR64;
pub use search::{SearchInfo,GameMode,benchmark_search};
pub use uci::uci_loop;
pub use movegen::init_mvv_lva;

// usize is used to avoid need for conversion with array access
pub type Square = u8;
type FileRank = Square;

const BOARD_SQ_NUM: usize = 64;
pub const MAX_DEPTH: u32 = 64;

pub const FILE_A: FileRank = 0;
// pub const FILE_B: FileRank = 1;
// pub const FILE_C: FileRank = 2;
// pub const FILE_D: FileRank = 3;
// pub const FILE_E: FileRank = 4;
// pub const FILE_F: FileRank = 5;
// pub const FILE_G: FileRank = 6;
pub const FILE_H: FileRank = 7;

pub const RANK_1: FileRank = 0;
pub const RANK_2: FileRank = 1;
pub const RANK_3: FileRank = 2;
// pub const RANK_4: FileRank = 3;
// pub const RANK_5: FileRank = 4;
pub const RANK_6: FileRank = 5;
pub const RANK_7: FileRank = 6;
pub const RANK_8: FileRank = 7;

pub struct Castling;

impl Castling {
    pub const WK: u8 = 1;
    pub const WQ: u8 = 2;
    pub const BK: u8 = 4;
    pub const BQ: u8 = 8;
}

pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn fr_to_sq(file: FileRank, rank: FileRank) -> Square {
    file + rank * 8
}

pub const RANKS_ITER: std::ops::Range<FileRank> = RANK_1..RANK_8+1;
pub const FILES_ITER: std::ops::Range<FileRank> = FILE_A..FILE_H+1;

pub fn square_on_board(sq: Square) -> bool {
    // Type limits enforce sq>=0
    sq <= 63
}

pub struct Undo {
    mv: moves::Move,
    castle_perm: u8,
    en_pas: Square,
    fifty_move: u32,
    hash: u64,
}

pub struct Board {
    pub pieces: [Piece; BOARD_SQ_NUM],

    pub bitboards: [Bitboard; NUM_PIECE_TYPES_BOTH],
    // White, black, and both-side bitboards for all pieces
    pub bb_sides: [Bitboard; 3],

    num_big_piece: [i32; 2],
    num_major_piece: [i32; 2],
    num_minor_piece: [i32; 2],
    material: [i32; 2],

    king_sq: [Square; 2],

    pub side: usize,
    pub en_pas: Square,
    fifty_move: u32,

    ply: u32,
    hist_ply: u32,

    pub history: Vec<Undo>,

    pub castle_perm: u8,
    hash: u64,

    hash_keys: HashKeys,

    pub pv_table: HashMap<u64, moves::Move>,
    // Todo: better as a member or a return value?
    pub pv_array: Vec<moves::Move>,

    // Incremented for piece type and its to square when move beats alpha
    search_history : [[u32; BOARD_SQ_NUM]; NUM_PIECE_TYPES_BOTH],
    // Two most recent moves that recently caused a beta cutoff that
    // aren't captures; vector length is by depth.

    // Store two most recent moves that caused a beta cutoff but
    // aren't captures.  
    search_killers: [[Option<moves::Move>; 2]; MAX_DEPTH as usize],
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board{
            pieces: [Piece::Offboard; BOARD_SQ_NUM],

            bitboards : Default::default(),
            bb_sides: Default::default(),

            num_big_piece: [0; 2],
            num_major_piece: [0; 2],
            num_minor_piece: [0; 2],
            material: [0; 2],

            king_sq: [Position::NONE as Square; 2],

            side: BOTH,
            en_pas: Position::NONE as Square,
            fifty_move: 0,

            ply: 0,
            hist_ply: 0,

            history: Vec::new(),

            castle_perm: 0,
            hash: 0,

            hash_keys: HashKeys::new(),

            // Todo: VICE uses a reset board function, and the
            // pv_table map is not re-initialized when the board is
            // re-initialized.  Do we need a reset_board separate from
            // just creating a new board object, so that the table can
            // be retained during reset?  See video 62.
            pv_table: HashMap::new(),
            pv_array: Vec::new(),

            search_history: [[0; BOARD_SQ_NUM]; NUM_PIECE_TYPES_BOTH],
            search_killers: [[None, None]; MAX_DEPTH as usize],
        };

        for i in 0..64 {
            board.pieces[i] = Piece::Empty;
        }

        board
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();

        let mut rank = RANK_8;
        let mut file = FILE_A;
        let mut piece;
        let mut count;
        let mut sq: Square;

        let mut fen_iter = fen.chars();
        let mut c;

        // This is essentially just a while(true) loop that must be
        // broken out off
        while rank >= RANK_1 {
            c = fen_iter.next().unwrap();
            count = 1;
            match c {
                'p' => piece = Piece::BP,
                'r' => piece = Piece::BR,
                'n' => piece = Piece::BN,
                'b' => piece = Piece::BB,
                'k' => piece = Piece::BK,
                'q' => piece = Piece::BQ,

                'P' => piece = Piece::WP,
                'R' => piece = Piece::WR,
                'N' => piece = Piece::WN,
                'B' => piece = Piece::WB,
                'K' => piece = Piece::WK,
                'Q' => piece = Piece::WQ,

                '1'..='8' => {
                    piece = Piece::Empty;
                    count = c.to_digit(10).unwrap();
                }, 

                '/' | ' ' => {
                    file = FILE_A;
                    if rank <= RANK_1 {
                        break;
                    }
                    rank -= 1;
                    continue;
                },
                
                _ => panic!("FEN error"),
            }

            for _i in 0..count {
                if piece.exists() {
                    sq = fr_to_sq(file, rank);
                    board.pieces[sq as usize] = piece;
                }
                file += 1;
            }
        }

        c = fen_iter.next().unwrap();
        board.side = match c {
            'w' => WHITE,
            'b' => BLACK,
            _ => panic!("unexpected FEN side color character"),
        };

        // Castling permissions:
        fen_iter.next();
        c = fen_iter.next().unwrap();
        for _i in 0..4 {
            match c {
                'K' => board.castle_perm |= Castling::WK,
                'Q' => board.castle_perm |= Castling::WQ,
                'k' => board.castle_perm |= Castling::BK,
                'q' => board.castle_perm |= Castling::BQ,
                '-' => (),
                ' ' => break,
                _ => panic!("unexpected FEN castling permission character"),
            }
            c = fen_iter.next().unwrap();
        }

        // En passant
        c = fen_iter.next().unwrap();
        if c != '-' {
            file = c as FileRank - 'a' as FileRank;
            c = fen_iter.next().unwrap();
            rank = c as FileRank - '1' as FileRank;
            assert!(file >= FILE_A && file <= FILE_H);
            assert!(rank >= RANK_1 && rank <= RANK_8);
            board.en_pas = fr_to_sq(file, rank);
        }

        board.hash = board.get_position_hash();

        board.update_lists_and_material();

        board
    }

    // Moves the current board into a new board with the given FEN string
    //
    // The only information retained is the pv_table
    //
    // An alternative would be to separate out parse_fen into a member
    // function and implement a reset function, but then there is some
    // duplication of initialization code between the reset function
    // and new()
    pub fn update_from_fen(self, fen: &str) -> Board {
        let mut board = Board::from_fen(fen);
        board.pv_table = self.pv_table;
        board
    }

    pub fn get_position_hash(&self) -> u64 {
        let mut hash: u64 = 0;

        let mut piece;
        for sq in 0..BOARD_SQ_NUM {
            piece = self.pieces[sq as usize];
            if piece.exists() {
                hash ^= self.hash_keys.piece_keys[piece as usize][sq as usize];
            }
        }

        if self.side == WHITE {
            hash ^= self.hash_keys.side_key;
        }

        if self.en_pas != Position::NONE as Square {
            hash ^= self.hash_keys.piece_keys[Piece::Empty as usize][self.en_pas as usize];
        }

        hash ^= self.hash_keys.castle_keys[self.castle_perm as usize];
        
        hash
    }

    // Print to stdout.  Unlike fmt, this includes the position key
    pub fn print(&self) {
        println!("{}Hash: {:x}", self, self.hash);
    }

    fn update_lists_and_material(&mut self) {
        let mut color;
        let mut piece;
        for sq in 0..64 {
            piece = self.pieces[sq as usize];
            if piece.exists() {
                color = piece.color();
                if piece.is_big() {
                    self.num_big_piece[color] += 1;
                }
                if piece.is_minor() {
                    self.num_minor_piece[color] += 1;
                }
                if piece.is_major() {
                    self.num_major_piece[color] += 1;
                }
                self.material[color] += piece.value();
                if piece.is_king() {
                    self.king_sq[color] = sq;
                }
                self.bitboards[piece as usize].set_bit(sq);
                self.bb_sides[color].set_bit(sq);
                self.bb_sides[BOTH].set_bit(sq);
            }
        }
    }

    pub fn check(&self) -> bool {
        let mut piece_count: [i32; NUM_PIECE_TYPES_BOTH] = [0; NUM_PIECE_TYPES_BOTH];
        let mut num_big_piece = [0; 2];
        let mut num_major_piece = [0; 2];
        let mut num_minor_piece = [0; 2];
        let mut material = [0; 2];

        // Check counts
        let mut piece;
        let mut color;
        for sq in 0..64 {
            piece = self.pieces[sq as usize];
            if piece.exists() {
                piece_count[piece as usize] += 1;
                color = piece.color();
                if piece.is_big() {
                    num_big_piece[color] += 1;
                }
                if piece.is_minor() {
                    num_minor_piece[color] += 1;
                }
                if piece.is_major() {
                    num_major_piece[color] += 1;
                }
                material[color] += piece.value();
            }
        }

        for piece in 0..NUM_PIECE_TYPES_BOTH {
            assert_eq!(piece_count[piece as usize], self.bitboards[piece as usize].count());
        }

        // Check pawn bitboards:
        assert_eq!(piece_count[Piece::WP as usize], self.bitboards[Piece::WP as usize].count());
        assert_eq!(piece_count[Piece::BP as usize], self.bitboards[Piece::BP as usize].count());

        // Check pawn bitboard squares:
        let bitboards = self.bitboards.clone();
        for sq in bitboards[Piece::WP as usize].into_iter() {
            assert_eq!(self.pieces[sq as usize], Piece::WP);
        }
        for sq in bitboards[Piece::BP as usize].into_iter() {
            assert_eq!(self.pieces[sq as usize], Piece::BP);
        }
        
        fn checker(a1: [i32; 2], a2: [i32; 2]) {
            assert_eq!(a1[0], a2[0]);
            assert_eq!(a1[1], a2[1]);
        }
        checker(material, self.material);
        checker(num_big_piece, self.num_big_piece);
        checker(num_major_piece, self.num_major_piece);
        checker(num_minor_piece, self.num_minor_piece);

        assert!(self.side == WHITE || self.side == BLACK);
        assert_eq!(self.hash, self.get_position_hash());

        assert!(self.en_pas == Position::NONE as Square ||
                (self.en_pas/8 == RANK_6 && self.side == WHITE) ||
                (self.en_pas/8 == RANK_3 && self.side == BLACK));

        assert_eq!(self.pieces[self.king_sq[WHITE] as usize], Piece::WK);
        assert_eq!(self.pieces[self.king_sq[BLACK] as usize], Piece::BK);
        assert_eq!(self.king_sq[WHITE], self.bitboards[Piece::WK as usize].clone().pop_bit());
        assert_eq!(self.king_sq[BLACK], self.bitboards[Piece::BK as usize].clone().pop_bit());

        // Check side piece bitboards:
        assert_eq!(self.bb_sides[WHITE].count(), PIECE_TYPES.iter().filter(|p| p.color()==WHITE).map(|&p| self.bitboards[p as usize].count()).sum());
        assert_eq!(self.bb_sides[BLACK].count(), PIECE_TYPES.iter().filter(|p| p.color()==BLACK).map(|&p| self.bitboards[p as usize].count()).sum());
        
        true
    }

    pub fn square_attacked(&self, sq: Square, side: usize) -> bool {
        debug_assert!(square_on_board(sq));
        debug_assert!(validate::side_valid(side));
        debug_assert!(self.check());
        
        let mut piece;

        // pawns
        if side == WHITE {
            if self.bitboards[Piece::WP as usize].0 & BLACK_PAWN_MOVES[sq as usize].0 != 0 {
                return true;
            }
        }
        else {
            if self.bitboards[Piece::BP as usize].0 & WHITE_PAWN_MOVES[sq as usize].0 != 0 {
                return true;
            }
        }

        // knights
        piece = match side {
            WHITE => Piece::WN,
            BLACK => Piece::BN,
            _ => unreachable!(),
        };
        if KNIGHT_MOVES[sq as usize].0 & self.bitboards[piece as usize].0 != 0 {
            return true;
        }

        let mut sq_bb = Bitboard::new();
        sq_bb.set_bit(sq);
        let occ = self.bb_sides[BOTH].0;

        // bishops or queens
        let bishop_queens = match side {
            WHITE => self.bitboards[Piece::WB as usize].0 | self.bitboards[Piece::WQ as usize].0,
            BLACK => self.bitboards[Piece::BB as usize].0 | self.bitboards[Piece::BQ as usize].0,
            _ => unreachable!(),
        };
        if bitboard::get_bishop_attacks(sq, occ) & bishop_queens != 0 {
            return true;
        }

        // rooks or queens
        let rooks_queens = match side {
            WHITE => self.bitboards[Piece::WR as usize].0 | self.bitboards[Piece::WQ as usize].0,
            BLACK => self.bitboards[Piece::BR as usize].0 | self.bitboards[Piece::BQ as usize].0,
            _ => unreachable!(),
        };
        if bitboard::get_rook_attacks(sq, occ) & rooks_queens != 0 {
            return true;
        }


        // kings
        piece = match side {
            WHITE => Piece::WK,
            BLACK => Piece::BK,
            _ => unreachable!(),
        };
        if KING_MOVES[sq as usize].0 & self.bitboards[piece as usize].0 != 0 {
            return true;
        }

        false
    }

    fn repetition_count(&self) -> u32 {
        let mut repetitions = 0;
        for item in &self.history {
            if item.hash == self.hash {
                repetitions += 1;
            }
        }
        repetitions
    }

    // Checks whether the position is a draw because neither side can
    // give mate
    fn is_draw_by_material(&self) -> bool {
        if self.bitboards[Piece::WP as usize].count() > 0 || self.bitboards[Piece::BP as usize].count() > 0 { return false; }
        if self.bitboards[Piece::WQ as usize].count() > 0 || self.bitboards[Piece::BQ as usize].count() > 0 || self.bitboards[Piece::WR as usize].count() > 0 || self.bitboards[Piece::BR as usize].count() > 0 { return false; }
        if self.bitboards[Piece::WB as usize].count() > 1 || self.bitboards[Piece::BB as usize].count() > 1 { return false; }
        if self.bitboards[Piece::WN as usize].count() > 1 || self.bitboards[Piece::BN as usize].count() > 1 { return false; }
        if self.bitboards[Piece::WN as usize].count() > 0 && self.bitboards[Piece::WB as usize].count() > 0 { return false; }
        if self.bitboards[Piece::BN as usize].count() > 0 && self.bitboards[Piece::BB as usize].count() > 0 { return false; }
        // Otherwise, it must be a draw:
        true
    }

    // Returns true if game is over
    pub fn check_game_result(&mut self) -> bool {
        // This move count may not be exact (?)
        if self.fifty_move > 100 {
            println!("1/2-1/2 (fifty move rule (claimed by {}))", PROGRAM_NAME);
            return true;
        }

        if self.repetition_count() >= 2 {
            println!("1/2-1/2 (3-fold repetition (claimed by {}))", PROGRAM_NAME);
            return true;
        }

        if self.is_draw_by_material() {
            println!("1/2-1/2 (insufficient material (claimed by {}))", PROGRAM_NAME);
            return true;
        }

        // Check for legal move:
        let mut found = false;
        let move_list = self.generate_all_moves();
        for smv in move_list.moves.into_iter() {
            if ! self.make_move(&smv.mv) {
                continue;
            }
            found = true;
            self.undo_move();
            break;
        }
        if found { return false; }

        let in_check = self.square_attacked(self.king_sq[self.side], self.side^1);
        if in_check {
            if self.side == WHITE {
                println!("0-1 (black mates (claimed by {}))", PROGRAM_NAME);
            } else {
                println!("1-0 (white mates (claimed by {}))", PROGRAM_NAME);
            }
        } else {
            println!("1/2-1/2 (stalemate (claimed by {}))", PROGRAM_NAME);
        }
        true
    }

    pub fn reset_ply(&mut self) {
        self.ply = 0;
    }

    // Mirror the board, for verifying that the evaluation function is
    // symmetrical
    pub fn mirror(&mut self) -> Board {
        
        let mut board = Board::new();

        if self.castle_perm & Castling::WK != 0 {
            board.castle_perm |= Castling::BK;
        }
        if self.castle_perm & Castling::WQ != 0 {
            board.castle_perm |= Castling::BQ;
        }

        if self.castle_perm & Castling::BK != 0 {
            board.castle_perm |= Castling::WK;
        }
        if self.castle_perm & Castling::BQ != 0 {
            board.castle_perm |= Castling::WQ;
        }

        if self.en_pas != Position::NONE as Square {
            board.en_pas = MIRROR64[self.en_pas as usize] as Square;
        }

        for sq in 0..64 {
            board.pieces[sq as usize] = self.pieces[MIRROR64[sq as usize]].swap();
        }

        board.side = self.side^1;
        board.hash = board.get_position_hash();
        board.update_lists_and_material();

        debug_assert!(board.check());
        
        board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Todo: change to list of chars to simplify indexing
        let side_chars = "wb-";
        let file_chars = "abcdefgh";

        let mut sq;
        let mut piece;
        for rank in RANKS_ITER.rev() {
            write!(f, "{}     ", rank+1)?;
            for file in FILES_ITER {
                sq = fr_to_sq(file, rank);
                piece = self.pieces[sq as usize];
                write!(f, "{:3}", piece.to_string())?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n      ")?;

        for file in FILES_ITER {
            write!(f, "{:3}", file_chars.chars().nth(file as usize).unwrap())?;
        }

        write!(f, "\n")?;
        write!(f, "side: {}\n", side_chars.chars().nth(self.side).unwrap())?;
        write!(f, "enPas: {}\n", self.en_pas)?;

        write!(f, "castle: {}{}{}{}\n",
                            if self.castle_perm & Castling::WK != 0 {'K'} else {'-'},
                            if self.castle_perm & Castling::WQ != 0 {'Q'} else {'-'},
                            if self.castle_perm & Castling::BK != 0 {'k'} else {'-'},
                            if self.castle_perm & Castling::BQ != 0 {'q'} else {'-'})?;

        Ok(())
    }
}

struct HashKeys {
    // Hashing also includes EMPTY pieces
    piece_keys: [[u64; BOARD_SQ_NUM]; NUM_PIECE_TYPES_BOTH+1],
    side_key: u64,
    castle_keys: [u64; 16],
}

impl HashKeys {
    fn new() -> HashKeys {
        let mut hasher = HashKeys {
            piece_keys: [[0; BOARD_SQ_NUM]; NUM_PIECE_TYPES_BOTH+1],
            side_key: 0,
            castle_keys: [0; 16],
        };

        hasher.side_key = rand::thread_rng().gen::<u64>();
        for i in 0..NUM_PIECE_TYPES_BOTH+1 {
            for j in 0..64 {
                hasher.piece_keys[i][j] = rand::thread_rng().gen::<u64>();
            }
        }
        for i in 0..16 {
            hasher.castle_keys[i] = rand::thread_rng().gen::<u64>();
        }

        hasher
    }
}

#[allow(dead_code)]
pub enum Position {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2 = 8, B2, C2, D2, E2, F2, G2, H2,
    A3 = 16, B3, C3, D3, E3, F3, G3, H3,
    A4 = 24, B4, C4, D4, E4, F4, G4, H4,
    A5 = 32, B5, C5, D5, E5, F5, G5, H5,
    A6 = 40, B6, C6, D6, E6, F6, G6, H6,
    A7 = 48, B7, C7, D7, E7, F7, G7, H7,
    A8 = 56, B8, C8, D8, E8, F8, G8, H8,
    NONE, OFFBOARD
}


#[cfg(test)]
mod tests {
    use crate::board::*;
    
    #[test]
    fn init_board() {
        let board = Board::from_fen(START_FEN);
        let s = "8     r  n  b  q  k  b  n  r  \n\
                 7     p  p  p  p  p  p  p  p  \n\
                 6     .  .  .  .  .  .  .  .  \n\
                 5     .  .  .  .  .  .  .  .  \n\
                 4     .  .  .  .  .  .  .  .  \n\
                 3     .  .  .  .  .  .  .  .  \n\
                 2     P  P  P  P  P  P  P  P  \n\
                 1     R  N  B  Q  K  B  N  R  \n\
                 \n      \
                       a  b  c  d  e  f  g  h  \n\
                 side: w\n\
                 enPas: 64\n\
                 castle: KQkq\n";
        println!("Board:\n{}", board);
        assert_eq!(board.to_string(), s);
        assert!(board.check());
    }
}
