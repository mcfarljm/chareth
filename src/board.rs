use rand::Rng;

const BOARD_SQ_NUM: usize = 120;

pub const FILE_A: i32 = 0;
pub const FILE_B: i32 = 1;
pub const FILE_C: i32 = 2;
pub const FILE_D: i32 = 3;
pub const FILE_E: i32 = 4;
pub const FILE_F: i32 = 5;
pub const FILE_G: i32 = 6;
pub const FILE_H: i32 = 7;

pub const RANK_1: i32 = 0;
pub const RANK_2: i32 = 1;
pub const RANK_3: i32 = 2;
pub const RANK_4: i32 = 3;
pub const RANK_5: i32 = 4;
pub const RANK_6: i32 = 5;
pub const RANK_7: i32 = 6;
pub const RANK_8: i32 = 7;

pub const START_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn fr_to_sq(file: i32, rank: i32) -> usize {
    (21 + file + rank * 10) as usize
}

pub fn ranks() -> std::ops::Range<i32> {
    (RANK_1..RANK_8 + 1)
}

pub fn files() -> std::ops::Range<i32> {
    (FILE_A..FILE_H + 1)
}

pub struct Board {
    pieces: [i32; BOARD_SQ_NUM],

    pawns: [u64; 3],

    piece_count: [i32; 13],

    num_big_piece: [i32; 3],
    num_major_piece: [i32; 3],
    num_minor_piece: [i32; 3],

    king_sq: [i32; 2],

    side: i32,
    en_pas: usize,
    fifty_move: i32,

    ply: i32,
    hist_ply: i32,

    castle_perm: u8,
    position_hash: u64,

    hash_keys: HashKeys,
}

impl Board {
    pub fn new() -> Board {
        let hash_keys = HashKeys::new();
        
        let mut board = Board{
            pieces: [Pieces::Empty as i32; BOARD_SQ_NUM],

            pawns: [0; 3],

            piece_count: [0; 13],

            num_big_piece: [0; 3],
            num_major_piece: [0; 3],
            num_minor_piece: [0; 3],

            king_sq: [Position::None as i32; 2],

            side: Color::Both as i32,
            en_pas: Position::None as usize,
            fifty_move: 0,

            ply: 0,
            hist_ply: 0,

            castle_perm: 0,
            position_hash: 0,

            hash_keys: hash_keys,
        };

        for i in 0..64 {
            board.pieces[SQUARE_64_TO_120[i]] = Pieces::Empty as i32;
        }

        board
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();

        let mut rank = RANK_8;
        let mut file = FILE_A;
        let mut piece = Pieces::Empty as i32;
        let mut count = 0;
        let mut i = 0;
        let mut sq120: usize;

        let mut fen_iter = fen.chars();
        let mut c;

        while rank >= RANK_1 {
            c = fen_iter.next().unwrap();
            count = 1;
            match c {
                'p' => piece = Pieces::BP as i32,
                'r' => piece = Pieces::BR as i32,
                'n' => piece = Pieces::BN as i32,
                'b' => piece = Pieces::BB as i32,
                'k' => piece = Pieces::BK as i32,
                'q' => piece = Pieces::BQ as i32,

                'P' => piece = Pieces::WP as i32,
                'R' => piece = Pieces::WR as i32,
                'N' => piece = Pieces::WN as i32,
                'B' => piece = Pieces::WB as i32,
                'K' => piece = Pieces::WK as i32,
                'Q' => piece = Pieces::WQ as i32,

                '1'..='8' => {
                    piece = Pieces::Empty as i32;
                    count = c.to_digit(10).unwrap();
                }, 

                '/' | ' ' => {
                    rank -= 1;
                    file = FILE_A;
                    continue;
                },
                
                _ => panic!("FEN error"),
            }

            for i in 0..count {
                if piece != Pieces::Empty as i32 {
                    sq120 = fr_to_sq(file, rank);
                    board.pieces[sq120] = piece;
                    file += 1;
                }
            }
        }

        c = fen_iter.next().unwrap();
        board.side = match c {
            'w' => Color::White as i32,
            'b' => Color::Black as i32,
            _ => panic!("unexpected FEN side color character"),
        };

        // Castling permissions:
        fen_iter.next();
        c = fen_iter.next().unwrap();
        for i in 0..4 {
            match c {
                'K' => board.castle_perm |= Castling::WK as u8,
                'Q' => board.castle_perm |= Castling::WQ as u8,
                'k' => board.castle_perm |= Castling::BK as u8,
                'q' => board.castle_perm |= Castling::BQ as u8,
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

        board
    }

    pub fn get_position_hash(&self) -> u64 {
        let mut hash: u64 = 0;

        let mut piece;
        let mut sq = 0;
        for sq in 0..BOARD_SQ_NUM {
            piece = self.pieces[sq];
            if piece != Pieces::Empty as i32 {
                hash ^= self.hash_keys.piece_keys[piece as usize][sq];
            }
        }

        if self.side == Color::White as i32 {
            hash ^= self.hash_keys.side_key;
        }

        if self.en_pas != Position::None as usize {
            hash ^= self.hash_keys.piece_keys[Pieces::Empty as usize][self.en_pas];
        }

        hash ^= self.hash_keys.castle_keys[self.castle_perm as usize];
        
        hash
    }

    pub fn to_string(&self) -> String {
        let piece_chars = ".PNBRQKpnbrqk";
        let side_chars = "wb-";
        let rank_chars = "12345678";
        let file_chars = "abcdefgh";

        let mut s = String::new();

        let mut sq;
        let mut piece;
        for rank in ranks().rev() {
            s.push_str(&format!("{}     ", rank+1));
            for file in files() {
                sq = fr_to_sq(file, rank);
                piece = self.pieces[sq];
                s.push_str(&format!("{:3}", piece_chars.chars().nth(piece as usize).unwrap()))
            }
            s.push('\n');
        }
        // s.push('\n');
        s.push_str(&"\n      ");

        for file in files() {
            s.push_str(&format!("{:3}", file_chars.chars().nth(file as usize).unwrap()));
        }

        s.push('\n');
        s.push_str(&format!("side: {}\n", side_chars.chars().nth(self.side as usize).unwrap()));
        s.push_str(&format!("enPas: {}\n", self.en_pas));

        s.push_str(&format!("castle: {}{}{}{}\n",
                            if self.castle_perm & Castling::WK as u8 != 0 {'K'} else {'-'},
                            if self.castle_perm & Castling::WQ as u8 != 0 {'Q'} else {'-'},
                            if self.castle_perm & Castling::BK as u8 != 0 {'k'} else {'-'},
                            if self.castle_perm & Castling::BQ as u8 != 0 {'q'} else {'-'}));

        s
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
        let mut i = 0;
        let mut j = 0;
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

pub enum Position {
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8,
    None, Offboard
}

pub enum Pieces {
    Empty,
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK
}

pub enum Color {
    White, Black,
    Both
}

pub enum Castling {
    WK = 1, WQ = 2, BK = 4, BQ = 8
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


#[cfg(test)]
mod tests {
    use crate::board::*;
    
    #[test]
    fn init_board_string() {
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
    }
}
