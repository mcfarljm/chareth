use crate::board::*;
use crate::pieces::Piece;

const PAWN_ISOLATED_SCORE: i32 = -10;
// Passed pawn bonus indexed by rank
const PAWN_PASSED_SCORE: [i32; 8] = [0, 5, 10, 20, 35, 60, 100, 200];

const ENDGAME_MATERIAL: i32 = ROOK_VAL + 2*KNIGHT_VAL + 2*PAWN_VAL + KING_VAL;

impl Board {
    // Evaluate position for side to move
    pub fn evaluate(&self) -> i32 {
        // Score is counted for white, and then return negative if
        // black is to move
        let mut score = self.material[WHITE] - self.material[BLACK];

        let mut piece;

        for sq64 in self.bitboards[Piece::WP as usize].into_iter() {
            score += PAWN_TABLE[sq64];

            if self.bitboards[Piece::WP as usize].isolated_pawn(sq64) {
                score += PAWN_ISOLATED_SCORE;
            }

            if self.bitboards[Piece::BP as usize].passed_pawn(sq64, WHITE) {
                // Todo: formalize RANKS for sq64
                score += PAWN_PASSED_SCORE[sq64/8];
            }
        }

        for sq64 in self.bitboards[Piece::BP as usize].into_iter() {
            score -= PAWN_TABLE[MIRROR64[sq64]];

            if self.bitboards[Piece::BP as usize].isolated_pawn(sq64) {
                score -= PAWN_ISOLATED_SCORE;
            }

            if self.bitboards[Piece::WP as usize].passed_pawn(sq64, BLACK) {
                // Todo: formalize RANKS for sq64
                score -= PAWN_PASSED_SCORE[7 - sq64/8];
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

        if self.material[BLACK] <= ENDGAME_MATERIAL {
            score += KING_END_TABLE[SQUARE_120_TO_64[self.king_sq[WHITE] as usize]];
        } else {
            score += KING_OPEN_TABLE[SQUARE_120_TO_64[self.king_sq[WHITE] as usize]];
        }

        if self.material[WHITE] <= ENDGAME_MATERIAL {
            score -= KING_END_TABLE[MIRROR64[SQUARE_120_TO_64[self.king_sq[BLACK] as usize]]];
        } else {
            score -= KING_OPEN_TABLE[MIRROR64[SQUARE_120_TO_64[self.king_sq[BLACK] as usize]]];
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

const KING_END_TABLE: [i32; 64] = [
    -50 , -10 , 0 , 0 , 0 , 0 , -10 , -50 ,
    -10, 0 , 10 , 10 , 10 , 10 , 0 , -10 ,
    0 , 10 , 15 , 15 , 15 , 15 , 10 , 0 ,
    0 , 10 , 15 , 20 , 20 , 15 , 10 , 0 ,
    0 , 10 , 15 , 20 , 20 , 15 , 10 , 0 ,
    0 , 10 , 15 , 15 , 15 , 15 , 10 , 0 ,
    -10, 0 , 10 , 10 , 10 , 10 , 0 , -10 ,
    -50 , -10 , 0 , 0 , 0 , 0 , -10 , -50
];

const KING_OPEN_TABLE: [i32; 64] = [
    0 , 5 , 5 , -10 , -10 , 0 , 10 , 5 ,
    -30 , -30 , -30 , -30 , -30 , -30 , -30 , -30 ,
    -50 , -50 , -50 , -50 , -50 , -50 , -50 , -50 ,
    -70 , -70 , -70 , -70 , -70 , -70 , -70 , -70 ,
    -70 , -70 , -70 , -70 , -70 , -70 , -70 , -70 ,
    -70 , -70 , -70 , -70 , -70 , -70 , -70 , -70 ,
    -70 , -70 , -70 , -70 , -70 , -70 , -70 , -70 ,
    -70 , -70 , -70 , -70 , -70 , -70 , -70 , -70
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
        assert_eq!(-15, board.evaluate());
        board = board.mirror();
        assert_eq!(-15, board.evaluate());
    }
}
