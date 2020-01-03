use criterion::{black_box, Criterion, criterion_group, criterion_main};

use urubu::types::color::Color;
use urubu::types::square::Square;
use urubu::simplified::position::Position;
use urubu::simplified::perft::Perft;

fn color_to_char(c: &mut Criterion) {
    c.bench_function("color_to_char", |b| {
        b.iter(|| Color::White.to_char());
    });
}

fn color_invert(c: &mut Criterion) {
    c.bench_function("color_invert", |b| {
        b.iter(|| Color::White.reverse());
    });
}

fn square_forward(c: &mut Criterion) {
    c.bench_function("square_forward", |b| {
        b.iter(|| Square::A1.forward(Color::White));
    });
}

fn perft(c: &mut Criterion) {

    let position = Position::default();
    let mut perft = Perft::new();
    c.bench_function("perft", |b| {
        b.iter(|| perft.perft(&mut position.clone(), 3));
    });
}

criterion_group!(benches, perft);
criterion_main!(benches);