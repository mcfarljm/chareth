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
    en_pas: i32,
    fifty_move: i32,

    ply: i32,
    hist_ply: i32,

    castle_perm: i32,
    position_key: u64,

    hash_keys: HashKeys,
}

impl Board {
    pub fn new() -> Board {
        let hash_keys = HashKeys::new();
        
        let mut board = Board{
            pieces: [Position::Offboard as i32; BOARD_SQ_NUM],

            pawns: [0; 3],

            piece_count: [0; 13],

            num_big_piece: [0; 3],
            num_major_piece: [0; 3],
            num_minor_piece: [0; 3],

            king_sq: [Position::None as i32; 2],

            side: Color::Both as i32,
            en_pas: Position::None as i32,
            fifty_move: 0,

            ply: 0,
            hist_ply: 0,

            castle_perm: 0,
            position_key: 0,

            hash_keys: hash_keys,
        };

        for i in 0..64 {
            board.pieces[SQUARE_64_TO_120[i]] = Pieces::Empty as i32;
        }

        board
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
