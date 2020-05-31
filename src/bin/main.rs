use chareth::{board,benchmarks,xboard,simpleloop};

use std::io::{self, Write};
use std::env;

fn main() {

    chareth::initialize();

    // Run a benchmark search if indicated by the command arguments:
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "b" {
        let nodes = board::benchmark_search(args[2].parse().unwrap());
        println!("nodes: {}", nodes);

        // benchmarks::benchmark_move_gen(args[2].parse().unwrap());
        // benchmarks::benchmark_eval(args[2].parse().unwrap());
        
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
