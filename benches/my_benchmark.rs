use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chareth::board;

pub fn benchmark_move_gen(c: &mut Criterion) {
    chareth::initialize();
    let board = board::Board::from_fen(board::START_FEN);

    c.bench_function("move gen", |b| b.iter(|| board.generate_all_moves()));
}

pub fn benchmark_eval(c: &mut Criterion) {
    chareth::initialize();
    let board = board::Board::from_fen(board::START_FEN);

    c.bench_function("evaluate", |b| b.iter(|| board.evaluate()));
}

criterion_group!(benches, benchmark_move_gen, benchmark_eval);
criterion_main!(benches);
