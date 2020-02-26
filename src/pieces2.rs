#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Piece {
    Empty,
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
    Offboard,
}

impl Piece {

    pub fn exists(&self) -> bool {
        match *self {
            Piece::Empty | Piece::Offboard => false,
            _ => true,
        }
    }

    pub fn not_offboard(&self) -> bool {
        match *self {
            Piece::Offboard => false,
            _ => true,
        }
    }
    
    pub fn color(&self) -> usize {
        match *self {
            Piece::WP | Piece::WN | Piece::WB | Piece::WR | Piece::WQ | Piece::WK => WHITE,
            Piece::BP | Piece::BN | Piece::BB | Piece::BR | Piece::BQ | Piece::BK => BLACK,
            _ => BOTH,
        }
    }
            
    pub fn slides(&self) -> bool {
        match *self {
            Piece::WB | Piece::WR | Piece::WQ => true,
            _ => false
        }
    }

    pub fn is_big(&self) -> bool {
        match *self {
            Piece::Empty | Piece::WP | Piece::BP | Piece::Offboard => false,
            _ => true,
        }
    }

    pub fn is_major(&self) -> bool {
        match *self {
            Piece::WR | Piece::BR | Piece::WQ | Piece::BQ | Piece::WK | Piece::BK => true,
            _ => false,
        }
    }

    pub fn is_minor(&self) -> bool {
        match *self {
            Piece::WN | Piece::WB | Piece::BN | Piece::BB => true,
            _ => false,
        }
    }

    pub fn is_knight(&self) -> bool {
        match *self {
            Piece::WN | Piece::BN => true,
            _ => false,
        }
    }

    pub fn is_king(&self) -> bool {
        match *self {
            Piece::WK | Piece::BK => true,
            _ => false,
        }
    }

    pub fn is_rook_or_queen(&self) -> bool {
        match *self {
            Piece::WR | Piece::BR | Piece::WQ | Piece::BQ => true,
            _ => false,
        }
    }

    pub fn is_bishop_or_queen(&self) -> bool {
        match *self {
            Piece::WB | Piece::BB | Piece::WQ | Piece::BQ => true,
            _ => false,
        }
    }

    pub fn value(&self) -> i32 {
        match *self {
            Piece::WP | Piece::BP => 100,
            Piece::WN | Piece::BN => 325,
            Piece::WB | Piece::BB => 325,
            Piece::WR | Piece::BR => 550,
            Piece::WQ | Piece::BQ => 1000,
            Piece::WK | Piece::BK => 50000,
            Piece::Empty | Piece::Offboard => 0,
        }
    }
}

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;
pub const BOTH: usize = 2;

pub const KNIGHT_DIR: [i32; 8] = [-8, -19, -21, -12, 8, 19, 21, 12];
pub const ROOK_DIR: [i32; 4] = [-1, -10, 1, 10];
pub const BISHOP_DIR: [i32; 4] = [-9, -11, 11, 9];
pub const KING_DIR: [i32; 8] = [-1, -10, 1, 10, -9, -11, 11, 9];

// SLIDERS[color] produces an array that can be iterated through
pub const SLIDERS: [[Piece; 3]; 2] = [[Piece::WB, Piece::WR, Piece::WQ], [Piece::BB, Piece::BR, Piece::BQ]];
pub const NON_SLIDERS: [[Piece; 2]; 2] = [[Piece::WN, Piece::WK], [Piece::BN, Piece::BK]];

// Todo: replace DIRECTIONS array with methods

// PIECE_DIRS[piece] will give an array of move directions for that
// piece.  A zero value is used to indicate the end, since the counts
// are not the same.  Pawns are not included.  Storing with vectors
// would make more sense but can't be statically allocated.
pub const DIRECTIONS: [[i32; 9]; 13] =
    [ [0; 9],
       [0; 9],
       [ -8, -19, -21, -12, 8, 19, 21, 12, 0 ],
       [ -9, -11, 11, 9, 0, 0, 0, 0, 0 ],
       [ -1, -10, 1, 10, 0, 0, 0, 0, 0 ],
       [ -1, -10, 1, 10, -9, -11, 11, 9, 0 ],
       [ -1, -10, 1, 10, -9, -11, 11, 9, 0 ],
       [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
       [ -8, -19, -21, -12, 8, 19, 21, 12, 0 ],
       [ -9, -11, 11, 9, 0, 0, 0, 0, 0 ],
       [ -1, -10, 1, 10, 0, 0, 0, 0, 0 ],
       [ -1, -10, 1, 10, -9, -11, 11, 9, 0 ],
       [ -1, -10, 1, 10, -9, -11, 11, 9, 0 ]
    ];
