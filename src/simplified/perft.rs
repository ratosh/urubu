use crate::simplified::position::Position;
use crate::types::move_list::MoveList;

pub struct Perft {
    move_list: MoveList,
    invalid_moves: u64,
    valid_moves: u64,
}

impl Perft {
    #[inline]
    pub fn divide(&mut self, position: &mut Position, depth: u8) {
        if depth == 0 {
            return;
        }
        self.move_list.start_ply();
        self.move_list.generate_noisy(position);
        self.move_list.generate_quiets(position);

        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            if position.is_legal_move(board_move) {
                let mut clone = position.clone();
                clone.do_move(board_move);
                println!(
                    "{} -> {}",
                    board_move.to_string(),
                    self.perft(&mut clone, depth - 1)
                );
            } else {
                println!("illegal -> {}", board_move.to_string());
            }
        }
        self.move_list.end_ply();
    }

    #[inline]
    pub fn perft(&mut self, position: &mut Position, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        self.move_list.start_ply();
        self.move_list.generate_noisy(position);
        self.move_list.generate_quiets(position);

        let mut result = 0;

        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            if position.is_legal_move(board_move) {
                let mut clone = position.clone();
                clone.do_move(board_move);
                result += self.perft(&mut clone, depth - 1);
                self.valid_moves += 1;
            } else {
                self.invalid_moves += 1;
            }
        }
        self.move_list.end_ply();

        result
    }
}

impl Default for Perft {
    fn default() -> Self {
        Self {
            move_list: MoveList::default(),
            invalid_moves: 0,
            valid_moves: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::simplified::perft::Perft;
    use crate::simplified::position::Position;
    use crate::types::board_move::BoardMove;
    use crate::types::move_type::MoveType;
    use crate::types::square::Square;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn check_perft_file(path: &str, depth_limit: u8) {
        let file = File::open(path).expect("failed to open test suite");
        let reader = BufReader::new(file);

        let mut position = Position::empty();
        let mut perft = Perft::default();

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
                    let depth: u8 = params
                        .next()
                        .expect("expected perft depth")
                        .parse()
                        .expect("expected integer value");

                    let nodes: u64 = params
                        .next()
                        .expect("expected perft nodes")
                        .parse()
                        .expect("expected integer value");

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

    #[test]
    fn test_perft() {
        let position = Position::default();
        let mut perft = Perft::default();
        let result = perft.perft(&mut position.clone(), 6);
        println!("Perft is {}", result);
    }

    #[test]
    fn test_divide() {
        let position = Position::default();
        let mut perft = Perft::default();
        perft.divide(&mut position.clone(), 6);
    }

    #[test]
    fn test_divide1() {
        let mut position =
            Position::from_fen("2bqk1nr/p5bp/n2p1P2/p1pPp3/P5p1/RrP5/1P1NPP1P/2B1KBNR w Kk -");
        position.do_move(BoardMove::build_normal(Square::F6, Square::G7));
        position.do_move(BoardMove::build_normal(Square::G8, Square::F6));
        position.do_move(BoardMove::build_move(
            Square::G7,
            Square::H8,
            MoveType::PROMOTION_BISHOP,
        ));
        let mut perft = Perft::default();
        perft.divide(&mut position, 1);
    }
}
