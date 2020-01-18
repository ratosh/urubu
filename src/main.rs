use std::time::Instant;
use urubu::simplified::perft::Perft;
use urubu::simplified::game::Game;

fn main() {
    println!("Hello, world!");
    let mut game = Game::default();
    let timer = Instant::now();
    let nodes = Perft::default().perft(&mut game, 6);
    let duration = timer.elapsed();
    let dur = duration.as_millis() as u64;
    println!("perft result {}", nodes);
    println!("Time taken {} ms", dur);
    println!("nps {}", (nodes * 1000 / dur));
}
