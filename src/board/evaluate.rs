use crate::board::*;
use crate::pieces::Piece;
use crate::moves::square_string;

const PAWN_ISOLATED_SCORE: i32 = -10;
// Passed pawn bonus indexed by rank
const PAWN_PASSED_SCORE: [i32; 8] = [0, 5, 10, 20, 35, 60, 100, 200];

impl Board {
    // Evaluate position for side to move
    pub fn evaluate(&self) -> i32 {
        // Score is counted for white, and then return negative if
        // black is to move
        let mut score = self.material[WHITE] - self.material[BLACK];

        let mut piece;

        piece = Piece::WP;
        for sq in &self.piece_lists[piece as usize] {
            let sq64 = SQUARE_120_TO_64[*sq as usize];
            score += PAWN_TABLE[sq64];

            if self.pawns[WHITE].isolated_pawn(sq64) {
                // println!("wP Iso: {}", square_string(*sq));
                score += PAWN_ISOLATED_SCORE;
            }

            if self.pawns[BLACK].passed_pawn(sq64, WHITE) {
                // println!("wP Passed: {}", square_string(*sq));
                score += PAWN_PASSED_SCORE[RANKS[*sq as usize] as usize];
            }
        }

        piece = Piece::BP;
        for sq in &self.piece_lists[piece as usize] {
            let sq64 = SQUARE_120_TO_64[*sq as usize];
            score -= PAWN_TABLE[MIRROR64[sq64]];

            if self.pawns[BLACK].isolated_pawn(sq64) {
                // println!("bP Iso: {}", square_string(*sq));
                score -= PAWN_ISOLATED_SCORE;
            }

            if self.pawns[WHITE].passed_pawn(sq64, BLACK) {
                // println!("bP Passed: {}", square_string(*sq));
                score -= PAWN_PASSED_SCORE[7 - RANKS[*sq as usize] as usize];
            }
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

pub const MIRROR64: [usize; 64] = [
    56 , 57 , 58 , 59 , 60 , 61 , 62 , 63 ,
    48 , 49 , 50 , 51 , 52 , 53 , 54 , 55 ,
    40 , 41 , 42 , 43 , 44 , 45 , 46 , 47 ,
    32 , 33 , 34 , 35 , 36 , 37 , 38 , 39 ,
    24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,
    16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 ,
    8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,
    0 , 1 , 2 , 3 , 4 , 5 , 6 , 7
];


#[cfg(test)]
mod tests {
    use crate::board::*;

    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;

    fn mirror_test(fen: &str) {
        let mut board = Board::from_fen(fen);
        let score = board.evaluate();
        board = board.mirror();
        assert_eq!(score, board.evaluate())
    }

    // Test to make sure that evaluation function is symmetric.
    // Similar idea to what is shown in VICE videos 79 and 80,
    // although it is a much smaller set of positions.
    #[test]
    fn mirror_test_suite() {
        let f = File::open("perftsuite.txt").expect("error opening perftsuite.txt in mirror_test_suite");
        let f = BufReader::new(f);        
        for line in f.lines() {
            mirror_test(line.unwrap().as_str());
        }
    }

    #[test]
    fn pawn_eval() {
        let fen = "2k1r2r/Bpq3pp/3b4/3Bp3/8/7b/PPP1QP2/R3R1K1 w - - 0 1";
        let mut board = Board::from_fen(fen);
        assert_eq!(-20, board.evaluate());
        board = board.mirror();
        assert_eq!(-20, board.evaluate());
    }
}
