use urubu::advanced::perft::Perft;
use urubu::advanced::board::Board;
use std::time::Instant;
use urubu::types::board_move::BoardMove;
use urubu::types::square::Square;
use urubu::advanced::game::Game;

fn main() {
    println!("Hello, world!");
    let mut game = Game::default();
    let before_time = Instant::now();
    let nodes = Perft::new().perft(&mut game, 6);
    let after_time = Instant::now();
    let dur = after_time.duration_since(before_time).as_millis() as u64;
    println!("perft result {}", nodes);
    println!("Time taken {} ms", dur);
    println!("nps {}", (nodes * 1000 /dur));
}
