mod pieces;
mod board;
mod bitboard;
mod moves;
mod validate;
mod simpleloop;
mod version;
mod xboard;

use std::io::{self, Write};

fn main() {

    bitboard::init_eval_masks();

    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.starts_with("uci") {
            board::uci_loop();
        } else if input.starts_with("xboard") {
            xboard::xboard_loop();
        }
        else if input.starts_with("console") {
            simpleloop::simple_loop();
        }

        // Could be modified to only break if quit flag (or fn return value) is set
        break;
    }

}
