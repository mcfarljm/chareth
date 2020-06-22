use std::fmt;

pub const PAWN_VAL: i32 = 100;
pub const KNIGHT_VAL: i32 = 325;
pub const BISHOP_VAL: i32 = 325;
pub const ROOK_VAL: i32 = 550;
pub const QUEEN_VAL: i32 = 1000;
pub const KING_VAL: i32 = 50000;

pub const NUM_PIECE_TYPES_BOTH: usize = 12;
pub const PIECE_TYPES: [Piece; NUM_PIECE_TYPES_BOTH] = [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK];

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Piece {
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
    Empty,
    Offboard,
}

impl Piece {

    pub fn exists(&self) -> bool {
        match *self {
            Piece::Empty | Piece::Offboard => false,
            _ => true,
        }
    }

    #[allow(dead_code)]
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
            
    #[allow(dead_code)]
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

    pub fn is_pawn(&self) -> bool {
        match *self {
            Piece::WP | Piece::BP => true,
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
            Piece::WP | Piece::BP => PAWN_VAL,
            Piece::WN | Piece::BN => KNIGHT_VAL,
            Piece::WB | Piece::BB => BISHOP_VAL,
            Piece::WR | Piece::BR => ROOK_VAL,
            Piece::WQ | Piece::BQ => QUEEN_VAL,
            Piece::WK | Piece::BK => KING_VAL,
            Piece::Empty | Piece::Offboard => 0,
        }
    }

    pub fn swap(&self) -> Piece {
        match *self {
            Piece::WP => Piece::BP,
            Piece::WN => Piece::BN,
            Piece::WB => Piece::BB,
            Piece::WR => Piece::BR,
            Piece::WQ => Piece::BQ,
            Piece::WK => Piece::BK,

            Piece::BP => Piece::WP,
            Piece::BN => Piece::WN,
            Piece::BB => Piece::WB,
            Piece::BR => Piece::WR,
            Piece::BQ => Piece::WQ,
            Piece::BK => Piece::WK,

            Piece::Empty => Piece::Empty,
            Piece::Offboard => Piece::Offboard,
        }
    }

    // Move directions, not including pawns
    pub fn directions(&self) -> &'static [i8] {
        match * self {
            Piece::WN | Piece::BN => &[-8, -19, -21, -12, 8, 19, 21, 12],
            Piece::WB | Piece::BB => &[-9, -11, 11, 9],
            Piece::WR | Piece::BR => &[-1, -10, 1, 10],
            Piece::WQ | Piece::BQ => &[-1, -10, 1, 10, -9, -11, 11, 9],
            Piece::WK | Piece::BK => &[-1, -10, 1, 10, -9, -11, 11, 9],
            _ => &[],
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Piece::WP => { write!(f, "P") }
            Piece::WN => { write!(f, "N") }
            Piece::WB => { write!(f, "B") }
            Piece::WR => { write!(f, "R") }
            Piece::WQ => { write!(f, "Q") }
            Piece::WK => { write!(f, "K") }
            Piece::BP => { write!(f, "p") }
            Piece::BN => { write!(f, "n") }
            Piece::BB => { write!(f, "b") }
            Piece::BR => { write!(f, "r") }
            Piece::BQ => { write!(f, "q") }
            Piece::BK => { write!(f, "k") }
            Piece::Empty => { write!(f, ".") }
            _ => { Err(fmt::Error) }
        }
    }
}

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;
pub const BOTH: usize = 2;

pub const KNIGHT_DIR: [i8; 8] = [-8, -19, -21, -12, 8, 19, 21, 12];
pub const ROOK_DIR: [i8; 4] = [-1, -10, 1, 10];
pub const BISHOP_DIR: [i8; 4] = [-9, -11, 11, 9];
pub const KING_DIR: [i8; 8] = [-1, -10, 1, 10, -9, -11, 11, 9];

// SLIDERS[color] produces an array that can be iterated through
pub const SLIDERS: [[Piece; 3]; 2] = [[Piece::WB, Piece::WR, Piece::WQ], [Piece::BB, Piece::BR, Piece::BQ]];
pub const NON_SLIDERS: [[Piece; 2]; 2] = [[Piece::WN, Piece::WK], [Piece::BN, Piece::BK]];
