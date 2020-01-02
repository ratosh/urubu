use crate::types::move_list::MoveList;
use crate::simplified::position::Position;

pub struct Perft {
    move_list: MoveList,
    invalid_moves: u64,
    valid_moves: u64
}

impl Perft {
    pub fn new() -> Self {
        Self {
            move_list: MoveList::new(),
            invalid_moves: 0,
            valid_moves: 0,
        }
    }

    pub fn divide(&mut self, position: &mut Position, depth: u8) {
        if depth == 0 {
            return;
        }
        self.move_list.start_ply();
        self.move_list.generate_quiets(position);
        self.move_list.generate_noisy(position);

        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            if position.is_legal_move(&board_move) {
                let mut clone = position.clone();
                clone.do_move(&board_move);
                println!("{} -> {}", board_move.to_string(), self.perft(&mut clone, depth - 1));
            }
        }
        self.move_list.end_ply();
    }

    pub fn perft(&mut self, position: &mut Position, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        self.move_list.start_ply();
        self.move_list.generate_quiets(position);
        self.move_list.generate_noisy(position);

        let mut result = 0;

        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            let mut clone = position.clone();
            if clone.do_move(&board_move) {
                result += self.perft(&mut clone, depth - 1);
                self.valid_moves += 1;
            } else {
                self.invalid_moves += 1;
            }
        }
        self.move_list.end_ply();

        return result;
    }
}



#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use crate::types::board_move::BoardMove;
    use crate::types::square::Square;
    use crate::simplified::perft::Perft;
    use crate::simplified::position::Position;

    fn check_perft_file(path: &str, depth_limit: u8) {
        let file = File::open(path).expect("failed to open test suite");
        let reader = BufReader::new(file);

        let mut position = Position::empty();
        let mut perft = Perft::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            let mut slices = line.trim().splitn(2, ' ');

            match slices.next() {
                Some("epd") => {
                    let fen = slices.next().expect("expected position");
                    println!("position {}", fen);
                    position = Position::from_fen(fen);
                }
                Some("perft") => {
                    let mut params = slices.next().expect("expected perft params").splitn(2, ' ');
                    let depth: u8 = params.next().expect("expected perft depth").parse().expect("expected integer value");

                    let nodes: u64 = params.next().expect("expected perft nodes").parse().expect("expected integer value");

                    if depth <= depth_limit {
                        assert_eq!(perft.perft(&mut position, depth), nodes);
                    }
                }
                _ => {}
            }
        }
        println!("valid nodes {}", perft.valid_moves);
        println!("invalid nodes {}", perft.invalid_moves);
    }

    #[test]
//    #[ignore]
    fn test_random() {
        check_perft_file("G:/chess/epds/random.perft", 5);
    }
}
