use criterion::{criterion_group, criterion_main, Criterion};
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

pub fn benchmark_search(c: &mut Criterion) {
    chareth::initialize();
    
    let mut board = board::Board::from_fen(board::START_FEN);
    let mut info = board::SearchInfo::new(4, board::GameMode::None);
    info.set_show_thinking(false);

    c.bench_function("search", |b| b.iter(|| board.search(&mut info)));
}

criterion_group!(benches, benchmark_move_gen, benchmark_eval, benchmark_search);
criterion_main!(benches);
