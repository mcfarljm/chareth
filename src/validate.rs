use crate::pieces;
use crate::board;

pub fn square_on_board(sq: usize) -> bool {
    board::SQUARE_120_TO_64[sq] <= 63
}

pub fn side_valid(side: usize) -> bool {
    side == pieces::WHITE || side == pieces::BLACK
}

pub fn piece_valid(piece: usize) -> bool {
    piece >= pieces::Piece::WP && piece <= pieces::Piece::BK
}

pub fn piece_valid_or_empty(piece: usize) -> bool {
    piece >= pieces::Piece::EMPTY && piece <= pieces::Piece::BK
}
