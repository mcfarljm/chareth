use crate::pieces;

pub fn side_valid(side: usize) -> bool {
    side == pieces::WHITE || side == pieces::BLACK
}
