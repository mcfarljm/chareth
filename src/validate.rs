use crate::pieces;

pub fn side_valid(side: usize) -> bool {
    side == pieces::WHITE || side == pieces::BLACK
}

pub fn piece_valid(piece: usize) -> bool {
    piece >= pieces::Piece::WP && piece <= pieces::Piece::BK
}

pub fn piece_valid_or_empty(piece: usize) -> bool {
    piece >= pieces::Piece::EMPTY && piece <= pieces::Piece::BK
}
