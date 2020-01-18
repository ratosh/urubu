use criterion::{criterion_group, criterion_main, Criterion};

use urubu::simplified::perft::Perft;
use urubu::simplified::position::Position;
use urubu::simplified::game::Game;

fn perft(c: &mut Criterion) {
    let game = Game::default();
    let mut perft = Perft::default();
    c.bench_function("perft", |b| {
        b.iter(|| perft.perft(&mut game.clone(), 3));
    });
}

criterion_group!(benches, perft);
criterion_main!(benches);
