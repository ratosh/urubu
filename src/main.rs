use std::time::Instant;
use urubu::types::board_move::BoardMove;
use urubu::types::square::Square;
use urubu::simplified::perft::Perft;
use urubu::simplified::position::Position;

fn main() {
    println!("Hello, world!");
    let mut position = Position::default();
    let timer = Instant::now();
    let nodes = Perft::new().perft(&mut position, 7);
    let duration = timer.elapsed();
    let dur = duration.as_millis() as u64;
    println!("perft result {}", nodes);
    println!("Time taken {} ms", dur);
    println!("nps {}", (nodes * 1000 /dur));
}
