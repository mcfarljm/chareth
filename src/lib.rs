mod pieces;
pub mod board;
mod bitboard;
mod moves;
mod validate;
pub mod simpleloop;
mod version;
pub mod xboard;
pub mod benchmarks;

#[macro_use]
extern crate lazy_static;

pub fn initialize() {
    board::init_mvv_lva();
    bitboard::init_eval_masks();
    bitboard::init_obs_diff_masks();
    pieces::init_move_tables();
}
