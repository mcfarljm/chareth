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

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;
pub const BOTH: usize = 2;

pub struct Piece;

// Placing the values inside a struct serves as a namespace (these are
// termed "associated constants")
impl Piece {
    pub const Empty: usize = 0;
    pub const WP: usize = 1;
    pub const WN: usize = 2;
    pub const WB: usize = 3;
    pub const WR: usize = 4;
    pub const WQ: usize = 5;
    pub const WK: usize = 6;
    pub const BP: usize = 7;
    pub const BN: usize = 8;
    pub const BB: usize = 9;
    pub const BR: usize = 10;
    pub const BQ: usize = 11;
    pub const BK: usize = 12;
    pub const Offboard: usize = 13;
}
