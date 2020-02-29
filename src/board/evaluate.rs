use crate::board::*;
use crate::pieces::Piece;

impl Board {
    // Evaluate position for side to move
    pub fn evaluate(&self) -> i32 {
        // Score is counted for white, and then return negative if
        // black is to move
        let mut score = self.material[WHITE] - self.material[BLACK];

        let mut piece;

        piece = Piece::WP;
        for sq in &self.piece_lists[piece as usize] {
            score += PAWN_TABLE[SQUARE_120_TO_64[*sq as usize]];
        }

        piece = Piece::BP;
        for sq in &self.piece_lists[piece as usize] {
            score -= PAWN_TABLE[MIRROR64[SQUARE_120_TO_64[*sq as usize]]];
        }

        piece = Piece::WN;
        for sq in &self.piece_lists[piece as usize] {
            score += KNIGHT_TABLE[SQUARE_120_TO_64[*sq as usize]];
        }

        piece = Piece::BN;
        for sq in &self.piece_lists[piece as usize] {
            score -= KNIGHT_TABLE[MIRROR64[SQUARE_120_TO_64[*sq as usize]]];
        }

        piece = Piece::WB;
        for sq in &self.piece_lists[piece as usize] {
            score += BISHOP_TABLE[SQUARE_120_TO_64[*sq as usize]];
        }

        piece = Piece::BB;
        for sq in &self.piece_lists[piece as usize] {
            score -= BISHOP_TABLE[MIRROR64[SQUARE_120_TO_64[*sq as usize]]];
        }

        piece = Piece::WR;
        for sq in &self.piece_lists[piece as usize] {
            score += ROOK_TABLE[SQUARE_120_TO_64[*sq as usize]];
        }

        piece = Piece::BR;
        for sq in &self.piece_lists[piece as usize] {
            score -= ROOK_TABLE[MIRROR64[SQUARE_120_TO_64[*sq as usize]]];
        }

        if self.side == WHITE {
            return score;
        } else {
            return -score;
        }
    }
}

const PAWN_TABLE: [i32; 64] = [
    0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,
    10 , 10 , 0 , -10 , -10 , 0 , 10 , 10 ,
    5 , 0 , 0 , 5 , 5 , 0 , 0 , 5 ,
    0 , 0 , 10 , 20 , 20 , 10 , 0 , 0 ,
    5 , 5 , 5 , 10 , 10 , 5 , 5 , 5 ,
    10 , 10 , 10 , 20 , 20 , 10 , 10 , 10 ,
    20 , 20 , 20 , 30 , 30 , 20 , 20 , 20 ,
    0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 
];

const KNIGHT_TABLE: [i32; 64] = [
    0 , -10 , 0 , 0 , 0 , 0 , -10 , 0 ,
    0 , 0 , 0 , 5 , 5 , 0 , 0 , 0 ,
    0 , 0 , 10 , 10 , 10 , 10 , 0 , 0 ,
    0 , 0 , 10 , 20 , 20 , 10 , 5 , 0 ,
    5 , 10 , 15 , 20 , 20 , 15 , 10 , 5 ,
    5 , 10 , 10 , 20 , 20 , 10 , 10 , 5 ,
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0 ,
    0 , 0 , 0 , 0 , 0 , 0 , 0 , 0  
];

const BISHOP_TABLE: [i32; 64] = [
    0 , 0 , -10 , 0 , 0 , -10 , 0 , 0 ,
    0 , 0 , 0 , 10 , 10 , 0 , 0 , 0 ,
    0 , 0 , 10 , 15 , 15 , 10 , 0 , 0 ,
    0 , 10 , 15 , 20 , 20 , 15 , 10 , 0 ,
    0 , 10 , 15 , 20 , 20 , 15 , 10 , 0 ,
    0 , 0 , 10 , 15 , 15 , 10 , 0 , 0 ,
    0 , 0 , 0 , 10 , 10 , 0 , 0 , 0 ,
    0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 
];

const ROOK_TABLE: [i32; 64] = [
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0 ,
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0 ,
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0 ,
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0 ,
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0 ,
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0 ,
    25 , 25 , 25 , 25 , 25 , 25 , 25 , 25 ,
    0 , 0 , 5 , 10 , 10 , 5 , 0 , 0  
];

const MIRROR64: [usize; 64] = [
    56 , 57 , 58 , 59 , 60 , 61 , 62 , 63 ,
    48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 ,
    40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 ,
    32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 ,
    24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,
    16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 ,
    8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,
    0 , 1 , 2 , 3 , 4 , 5 , 6 , 7
];
