pub mod board;
pub mod xboard;
pub mod simpleloop;
pub mod benchmarks;

mod pieces;
mod bitboard;
mod moves;
mod validate;
mod version;

#[macro_use]
extern crate lazy_static;

pub fn initialize() {
    board::init_mvv_lva();
    bitboard::init_eval_masks();
    bitboard::init_obs_diff_masks();
    pieces::init_move_tables();
}
