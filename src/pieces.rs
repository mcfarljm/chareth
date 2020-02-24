pub const PIECE_IS_BIG: [bool; 13] = [ false, false, true, true, true, true, true, false, true, true, true, true, true ];
pub const PIECE_IS_MAJ: [bool; 13] = [ false, false, false, false, true, true, true, false, false, false, true, true, true ];
pub const PIECE_IS_MIN: [bool; 13] = [ false, false, true, true, false, false, false, false, true, true, false, false, false ];
pub const PIECE_VAL: [i32; 13]= [ 0, 100, 325, 325, 550, 1000, 50000, 100, 325, 325, 550, 1000, 50000  ];
pub const PIECE_COLOR: [usize; 13] = [ 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1 ];

pub const PIECE_IS_KNIGHT: [bool; 13] = [ false, false, true, false, false, false, false, false, true, false, false, false, false ];
pub const PIECE_IS_KING: [bool; 13] = [ false, false, false, false, false, false, true, false, false, false, false, false, true ];
pub const PIECE_IS_ROOK_OR_QUEEN: [bool; 13] = [ false, false, false, false, true, true, false, false, false, false, true, true, false ];
pub const PIECE_IS_BISHOP_OR_QUEEN: [bool; 13] = [ false, false, false, true, false, true, false, false, false, true, false, true, false ];

pub const KNIGHT_DIR: [i32; 8] = [-8, -19, -21, -12, 8, 19, 21, 12];
pub const ROOK_DIR: [i32; 4] = [-1, -10, 1, 10];
pub const BISHOP_DIR: [i32; 4] = [-9, -11, 11, 9];
pub const KING_DIR: [i32; 8] = [-1, -10, 1, 10, -9, -11, 11, 9];

pub enum Pieces {
    Empty,
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
    Offboard,
}

pub enum Color {
    White, Black,
    Both
}
