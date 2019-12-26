use criterion::{black_box, Criterion, criterion_group, criterion_main};

use urubu::types::color::Color;
use urubu::types::square::Square;

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

criterion_group!(benches, color_to_char, color_invert, square_forward);
criterion_main!(benches);