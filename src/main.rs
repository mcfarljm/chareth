mod board;
mod bitboard;

use board::{ranks,files};

fn main() {
    println!("Hello, world!");

    for r in ranks().rev() {
        println!("r: {}", r);
    }

    for f in files() {
        println!("f: {}", f);
    }
}
