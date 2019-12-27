use crate::advanced::attack_info::AttackInfo;
use crate::advanced::move_list::MoveList;
use crate::advanced::game::Game;

pub struct Perft {
    attack_info: AttackInfo,
    move_list: MoveList,
    invalid_moves: u64,
    valid_moves: u64
}

impl Perft {
    pub fn new() -> Self {
        Self {
            attack_info: AttackInfo::new(),
            move_list: MoveList::new(),
            invalid_moves: 0,
            valid_moves: 0,
        }
    }

    pub fn divide(&mut self, game: &mut Game, depth: u8) {
        if depth == 0 {
            return;
        }
        self.move_list.start_ply();
        self.move_list.generate_quiets(&game.board, &mut self.attack_info);
        self.move_list.generate_noisy(&game.board, &mut self.attack_info);

        while self.move_list.has_next() {
            let board_move = self.move_list.next();

            if game.do_move(&board_move) {
                println!("{} -> {}", board_move.to_string(), self.perft(game, depth - 1));
            }
            game.undo_move();
        }
        self.move_list.end_ply();
    }

    pub fn perft(&mut self, game: &mut Game, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        self.move_list.start_ply();
        self.move_list.generate_quiets(&game.board, &mut self.attack_info);
        self.move_list.generate_noisy(&game.board, &mut self.attack_info);

        let mut result = 0;

        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            if game.do_move(&board_move) {
                result += self.perft(game, depth - 1);
                self.valid_moves += 1;
            } else {
                self.invalid_moves += 1;
            }
            game.undo_move();
        }
        self.move_list.end_ply();

        return result;
    }
}



#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use crate::advanced::board::Board;
    use crate::advanced::perft::Perft;
    use crate::types::board_move::BoardMove;
    use crate::types::square::Square;
    use crate::advanced::game::Game;

    fn check_perft_file(path: &str, depth_limit: u8) {
        let file = File::open(path).expect("failed to open test suite");
        let reader = BufReader::new(file);

        let mut game = Game::default();
        let mut perft = Perft::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            let mut slices = line.trim().splitn(2, ' ');

            match slices.next() {
                Some("epd") => {
                    let position = slices.next().expect("expected position");
                    println!("position {}", position);
                    game.reset(&Board::from_fen(position));
                }
                Some("perft") => {
                    let mut params = slices.next().expect("expected perft params").splitn(2, ' ');
                    let depth: u8 = params.next().expect("expected perft depth").parse().expect("expected integer value");

                    let nodes: u64 = params.next().expect("expected perft nodes").parse().expect("expected integer value");

                    if depth <= depth_limit {
                        assert_eq!(perft.perft(&mut game, depth), nodes);
                    }
                }
                _ => {}
            }
        }
        println!("valid nodes {}", perft.valid_moves);
        println!("invalid nodes {}", perft.invalid_moves);
    }

    #[test]
    #[ignore]
    fn test_random() {
        check_perft_file("G:/chess/epds/random.perft", 5);
    }
}
