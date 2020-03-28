# Chareth

Chareth is an xboard and UCI compatible chess engine written in Rust, based on the VICE engine and [video series](https://www.youtube.com/watch?v=bGAfaepBco4&list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg) by Bluefever Software.  It was developed as a way to learn about both the Rust programming language and chess programming.

## Usage

* Install Rust: https://www.rust-lang.org/tools/install
* Clone this repository
* Run `cargo build --release` from the root directory

This will create the executable `chareth` in the `target/release` directory.  Now simply configure your chess GUI application to point to this executable.
