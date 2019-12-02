use urubu::advanced::perft::Perft;
use urubu::advanced::board::Board;
use std::time::Instant;
use urubu::types::board_move::BoardMove;
use urubu::types::square::Square;

fn main() {
    println!("Hello, world!");
    let mut board = Board::default();
//    board.do_move(&BoardMove::build_normal(&Square::C2, &Square::C3));
//    board.do_move(&BoardMove::build_normal(&Square::D7, &Square::D6));
    let before_time = Instant::now();
    let nodes = Perft::new().perft(&mut board, 5);
    let after_time = Instant::now();
    let dur = after_time.duration_since(before_time).as_millis() as u64;
    println!("perft result {}", nodes);
    println!("Time taken {} ms", dur);
//    println!("nps {}", (nodes * 1000 /dur));
    Perft::new().divide(&mut board, 5);
}
