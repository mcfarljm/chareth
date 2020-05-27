mod pieces;
mod board;
mod bitboard;
mod moves;
mod validate;
mod simpleloop;
mod version;
mod xboard;

use std::io::{self, Write};
use std::env;

#[macro_use]
extern crate lazy_static;

fn main() {

    board::init_mvv_lva();
    bitboard::init_eval_masks();

    // Run a benchmark search if indicated by the command arguments:
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "b" {
        let nodes = board::benchmark_search(args[2].parse().unwrap());
        println!("nodes: {}", nodes);
        std::process::exit(0);
    }

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
