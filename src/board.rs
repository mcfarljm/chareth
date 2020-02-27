use rand::Rng;

use crate::pieces::*;
use crate::bitboard;
use crate::validate;

const BOARD_SQ_NUM: usize = 120;

pub const FILE_A: i32 = 0;
// pub const FILE_B: i32 = 1;
// pub const FILE_C: i32 = 2;
// pub const FILE_D: i32 = 3;
// pub const FILE_E: i32 = 4;
// pub const FILE_F: i32 = 5;
// pub const FILE_G: i32 = 6;
pub const FILE_H: i32 = 7;

pub const RANK_1: i32 = 0;
pub const RANK_2: i32 = 1;
pub const RANK_3: i32 = 2;
// pub const RANK_4: i32 = 3;
// pub const RANK_5: i32 = 4;
pub const RANK_6: i32 = 5;
pub const RANK_7: i32 = 6;
pub const RANK_8: i32 = 7;

pub struct Castling;

impl Castling {
    pub const WK: u8 = 1;
    pub const WQ: u8 = 2;
    pub const BK: u8 = 4;
    pub const BQ: u8 = 8;
}

pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn fr_to_sq(file: i32, rank: i32) -> usize {
    (21 + file + rank * 10) as usize
}

pub const RANKS_ITER: std::ops::Range<i32> = (RANK_1..RANK_8+1);
pub const FILES_ITER: std::ops::Range<i32> = (FILE_A..FILE_H+1);

pub fn square_on_board(sq: usize) -> bool {
    SQUARE_120_TO_64[sq] <= 63
}

pub struct Board {
    pub pieces: [Piece; BOARD_SQ_NUM],

    pub pawns: Vec<bitboard::Bitboard>,

    num_big_piece: [i32; 2],
    num_major_piece: [i32; 2],
    num_minor_piece: [i32; 2],
    material: [i32; 2],

    // piece_lists[piece] produces a vector of squares for that piece
    pub piece_lists: Vec<Vec<usize>>,

    // Redundant with piece_lists
    king_sq: [usize; 2],

    pub side: usize,
    pub en_pas: usize,
    fifty_move: i32,

    ply: i32,
    hist_ply: i32,

    pub castle_perm: u8,
    position_hash: u64,

    hash_keys: HashKeys,
}

impl Board {
    pub fn new() -> Board {
        let hash_keys = HashKeys::new();

        let mut piece_lists: Vec<Vec<usize>> = Vec::new();
        for _i in 0..13 {
            piece_lists.push(Vec::new());
        }
        
        let mut board = Board{
            pieces: [Piece::Offboard; BOARD_SQ_NUM],

            pawns: vec![bitboard::Bitboard::new(); 3],

            num_big_piece: [0; 2],
            num_major_piece: [0; 2],
            num_minor_piece: [0; 2],
            material: [0; 2],

            piece_lists: piece_lists,

            king_sq: [Position::NONE as usize; 2],

            side: BOTH,
            en_pas: Position::NONE as usize,
            fifty_move: 0,

            ply: 0,
            hist_ply: 0,

            castle_perm: 0,
            position_hash: 0,

            hash_keys: hash_keys,
        };

        for i in 0..64 {
            board.pieces[SQUARE_64_TO_120[i]] = Piece::Empty;
        }

        board
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();

        let mut rank = RANK_8;
        let mut file = FILE_A;
        let mut piece;
        let mut count;
        let mut sq120: usize;

        let mut fen_iter = fen.chars();
        let mut c;

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
                    rank -= 1;
                    file = FILE_A;
                    continue;
                },
                
                _ => panic!("FEN error"),
            }

            for _i in 0..count {
                if piece.exists() {
                    sq120 = fr_to_sq(file, rank);
                    board.pieces[sq120] = piece;
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
            file = c as i32 - 'a' as i32;
            c = fen_iter.next().unwrap();
            rank = c as i32 - '1' as i32;
            assert!(file >= FILE_A && file <= FILE_H);
            assert!(rank >= RANK_1 && rank <= RANK_8);
            board.en_pas = fr_to_sq(file, rank);
        }

        board.position_hash = board.get_position_hash();

        board.update_lists_and_material();

        board
    }

    pub fn get_position_hash(&self) -> u64 {
        let mut hash: u64 = 0;

        let mut piece;
        for sq in 0..BOARD_SQ_NUM {
            piece = self.pieces[sq];
            if piece.exists() {
                hash ^= self.hash_keys.piece_keys[piece as usize][sq];
            }
        }

        if self.side == WHITE {
            hash ^= self.hash_keys.side_key;
        }

        if self.en_pas != Position::NONE as usize {
            hash ^= self.hash_keys.piece_keys[Piece::Empty as usize][self.en_pas];
        }

        hash ^= self.hash_keys.castle_keys[self.castle_perm as usize];
        
        hash
    }

    pub fn to_string(&self) -> String {
        // Todo: change to list of chars to simplify indexing
        let piece_chars = ".PNBRQKpnbrqk";
        let side_chars = "wb-";
        let file_chars = "abcdefgh";

        let mut s = String::new();

        let mut sq;
        let mut piece;
        for rank in RANKS_ITER.rev() {
            s.push_str(&format!("{}     ", rank+1));
            for file in FILES_ITER {
                sq = fr_to_sq(file, rank);
                piece = self.pieces[sq];
                s.push_str(&format!("{:3}", piece_chars.chars().nth(piece as usize).unwrap()))
            }
            s.push('\n');
        }
        // s.push('\n');
        s.push_str(&"\n      ");

        for file in FILES_ITER {
            s.push_str(&format!("{:3}", file_chars.chars().nth(file as usize).unwrap()));
        }

        s.push('\n');
        s.push_str(&format!("side: {}\n", side_chars.chars().nth(self.side as usize).unwrap()));
        s.push_str(&format!("enPas: {}\n", self.en_pas));

        s.push_str(&format!("castle: {}{}{}{}\n",
                            if self.castle_perm & Castling::WK != 0 {'K'} else {'-'},
                            if self.castle_perm & Castling::WQ != 0 {'Q'} else {'-'},
                            if self.castle_perm & Castling::BK != 0 {'k'} else {'-'},
                            if self.castle_perm & Castling::BQ != 0 {'q'} else {'-'}));

        s
    }

    fn update_lists_and_material(&mut self) {
        let mut sq120;
        let mut color;
        let mut piece;
        for sq in 0..64 {
           sq120 = SQUARE_64_TO_120[sq]; 
            piece = self.pieces[sq120];
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
                self.piece_lists[piece as usize].push(sq120);
                if piece.is_king() {
                    self.king_sq[color] = sq120;
                }
                let sq64;
                if let Piece::WP | Piece::BP = piece {
                    sq64 = SQUARE_120_TO_64[sq120];
                    self.pawns[color].set_bit(sq64);
                    self.pawns[BOTH].set_bit(sq64);
                }
            }
        }
    }

    pub fn check(&self) -> bool {
        let mut piece_count: [i32; 13] = [0; 13];
        let mut num_big_piece = [0; 2];
        let mut num_major_piece = [0; 2];
        let mut num_minor_piece = [0; 2];
        let mut material = [0; 2];

        // Check piece lists:
        for piece in 1..13 {
            for sq in &self.piece_lists[piece as usize] {
                assert_eq!(self.pieces[*sq as usize] as usize, piece);
            }
        }

        // Check counts
        let mut sq120;
        let mut piece;
        let mut color;
        for sq64 in 0..64 {
            sq120 = SQUARE_64_TO_120[sq64]; 
            piece = self.pieces[sq120];
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

        for piece in 1..13 {
            assert_eq!(piece_count[piece as usize] as usize, self.piece_lists[piece as usize].len());
        }

        // Check pawn bitboards:
        let mut pawns = self.pawns.clone();
        assert_eq!(piece_count[Piece::WP as usize], pawns[WHITE].count());
        assert_eq!(piece_count[Piece::BP as usize], pawns[BLACK].count());
        assert_eq!(piece_count[Piece::WP as usize] + piece_count[Piece::BP as usize], self.pawns[BOTH].count());

        // Check pawn bitboard squares:
        let mut sq64;
        while pawns[WHITE].nonzero() {
            sq64 = pawns[WHITE].pop_bit();
            assert_eq!(self.pieces[SQUARE_64_TO_120[sq64]], Piece::WP);
        }
        while pawns[BLACK].nonzero() {
            sq64 = pawns[BLACK].pop_bit();
            assert_eq!(self.pieces[SQUARE_64_TO_120[sq64]], Piece::BP);
        }
        while pawns[BOTH].nonzero() {
            sq64 = pawns[BOTH].pop_bit();
            assert!(self.pieces[SQUARE_64_TO_120[sq64]] == Piece::WP || self.pieces[SQUARE_64_TO_120[sq64]] == Piece::BP);
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
        assert_eq!(self.position_hash, self.get_position_hash());

        assert!(self.en_pas == Position::NONE as usize ||
                (RANKS[self.en_pas] == RANK_6 && self.side == WHITE) ||
                (RANKS[self.en_pas] == RANK_3 && self.side == BLACK));

        assert_eq!(self.pieces[self.king_sq[WHITE]], Piece::WK);
        assert_eq!(self.pieces[self.king_sq[BLACK]], Piece::BK);
        
        true
    }

    pub fn square_attacked(&self, sq: usize, side: usize) -> bool {
        debug_assert!(square_on_board(sq));
        debug_assert!(validate::side_valid(side));
        debug_assert!(self.check());
        
        let mut piece;

        // pawns
        if side == WHITE {
            if self.pieces[sq-11] == Piece::WP || self.pieces[sq-9] == Piece::WP { return true; }
        }
        else {
            if self.pieces[sq+11] == Piece::BP || self.pieces[sq+9] == Piece::BP { return true; }
        }

        let mut t_sq: usize;

        // knights
        for dir in &KNIGHT_DIR {
            t_sq = (sq as i32 + *dir) as usize;
            if ! square_on_board(t_sq) {
                continue;
            }
            piece = self.pieces[t_sq];
            if piece.is_knight() && piece.color() == side as usize {
                return true;
            }
        }

        // rooks, queens
        for dir in &ROOK_DIR {
            t_sq = (sq as i32 + *dir) as usize;
            piece = self.pieces[t_sq];
            while piece != Piece::Offboard {
                if piece.exists() {
                    if piece.is_rook_or_queen() && piece.color() == side as usize { return true; }
                    break;
                }
                t_sq = (t_sq as i32 + *dir) as usize;
                piece = self.pieces[t_sq];
            }
        }

        // rooks, queens
        for dir in &BISHOP_DIR {
            t_sq = (sq as i32 + *dir) as usize;
            piece = self.pieces[t_sq];
            while piece != Piece::Offboard {
                if piece.exists() {
                    if piece.is_bishop_or_queen() && piece.color() == side as usize { return true; }
                    break;
                }
                t_sq = (t_sq as i32 + *dir) as usize;
                piece = self.pieces[t_sq];
            }
        }

        // kings
        for dir in &KING_DIR {
            t_sq = (sq as i32 + *dir) as usize;
            if ! square_on_board(t_sq) {
                continue;
            }
            piece = self.pieces[t_sq];
            if piece.is_king() && piece.color() == side {
                return true;
            }
        }

        false
    }
}

struct HashKeys {
    piece_keys: [[u64; 120]; 13],
    side_key: u64,
    castle_keys: [u64; 16],
}

impl HashKeys {
    fn new() -> HashKeys {
        let mut hasher = HashKeys {
            piece_keys: [[0; 120]; 13],
            side_key: 0,
            castle_keys: [0; 16],
        };

        hasher.side_key = rand::thread_rng().gen::<u64>();
        for i in 0..13 {
            for j in 0..120 {
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
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8,
    NONE, OFFBOARD
}

pub const SQUARE_120_TO_64: [usize; BOARD_SQ_NUM] = [
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
    65,  0,  1,  2,  3,  4,  5,  6,  7, 65,
    65,  8,  9, 10, 11, 12, 13, 14, 15, 65,
    65, 16, 17, 18, 19, 20, 21, 22, 23, 65,
    65, 24, 25, 26, 27, 28, 29, 30, 31, 65,
    65, 32, 33, 34, 35, 36, 37, 38, 39, 65,
    65, 40, 41, 42, 43, 44, 45, 46, 47, 65,
    65, 48, 49, 50, 51, 52, 53, 54, 55, 65,
    65, 56, 57, 58, 59, 60, 61, 62, 63, 65,
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
    65, 65, 65, 65, 65, 65, 65, 65, 65, 65
];

pub const SQUARE_64_TO_120: [usize; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];


pub const FILES: [i32; BOARD_SQ_NUM] = [
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 0, 1, 2, 3, 4, 5, 6, 7, 100,
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100
];

pub const RANKS: [i32; BOARD_SQ_NUM] = [
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    100, 0, 0, 0, 0, 0, 0, 0, 0, 100,
    100, 1, 1, 1, 1, 1, 1, 1, 1, 100,
    100, 2, 2, 2, 2, 2, 2, 2, 2, 100,
    100, 3, 3, 3, 3, 3, 3, 3, 3, 100,
    100, 4, 4, 4, 4, 4, 4, 4, 4, 100,
    100, 5, 5, 5, 5, 5, 5, 5, 5, 100,
    100, 6, 6, 6, 6, 6, 6, 6, 6, 100,
    100, 7, 7, 7, 7, 7, 7, 7, 7, 100,
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    100, 100, 100, 100, 100, 100, 100, 100, 100, 100
];




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
                 enPas: 99\n\
                 castle: KQkq\n";
        assert_eq!(board.to_string(), s);
        assert!(board.check());
    }
}
