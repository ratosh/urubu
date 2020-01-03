use criterion::{criterion_group, criterion_main, Criterion};

use urubu::simplified::perft::Perft;
use urubu::simplified::position::Position;

fn perft(c: &mut Criterion) {
    let position = Position::default();
    let mut perft = Perft::default();
    c.bench_function("perft", |b| {
        b.iter(|| perft.perft(&mut position.clone(), 3));
    });
}

criterion_group!(benches, perft);
criterion_main!(benches);
