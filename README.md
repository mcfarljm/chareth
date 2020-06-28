# Chareth

Chareth is an xboard and UCI compatible chess engine written in Rust, based on the VICE engine and [video series](https://www.youtube.com/watch?v=bGAfaepBco4&list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg) by Bluefever Software.  It was developed as a way to learn about both the Rust programming language and chess programming.

## Features

* Bitboard representation for all pieces.  Stored as individual bitboards for all 12 colored pieces, in addition to side and overall occupation bitboards.
* A length-64 square-to-piece mapping is also stored, in addition to an integer (square) representation of each side's king location.
* Obstruction difference for sliding piece move generation.
* Standard alpha beta and quiescence search with iterative deepening.
* Move ordering using principal variation and search killers, following VICE video series.
* Transposition table: to be implemented.
* Null move pruning: still being tested.

## Comparison to VICE

Chareth implementation originally followed the VICE video series very closely through video 82, although I have tried to take advantage of some of Rust's higher-level features where possible, such as structures, vectors, and hash maps.  However, even after trying basic optimizations in the code, the best I could get in terms of search speed was still about 20% slower than VICE (which is written in C).  Chareth was rewritten to use bitboard representations from all pieces (instead of for pawns only, as in VICE), along with the associated bitboard techniques for move generation.  With these changes, my testing shows it to be slightly (about 15%) faster in search than the corresponding VICE code.

Some of the features from the later VICE videos, such as transposition tables and other evaluation improvements are not yet implemented.

## Usage

* Install Rust: https://www.rust-lang.org/tools/install
* Clone this repository
* Run `cargo build --release` from the root directory

This will create the executable `chareth` in the `target/release` directory.  Now simply configure your chess GUI application to point to this executable.
