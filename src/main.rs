mod pieces;
mod board;
mod bitboard;
mod moves;
mod validate;
mod simpleloop;
mod version;
mod xboard;

fn main() {

    board::uci_loop();

}
