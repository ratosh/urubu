use crate::advanced::attack_info::AttackInfo;
use crate::advanced::board::Board;
use crate::advanced::move_list::MoveList;

pub struct Perft {
    attack_info: AttackInfo,
    move_list: MoveList,
}

impl Perft {
    pub fn new() -> Self {
        Self {
            attack_info: AttackInfo::new(),
            move_list: MoveList::new(),
        }
    }

    pub fn divide(&mut self, board: &mut Board, depth: u8) {
        if depth == 0 {
            return;
        }
        self.move_list.start_ply();
        self.move_list.generate_quiets(board, &mut self.attack_info);
        self.move_list.generate_noisy(board, &mut self.attack_info);

        let mut result = 0;

        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            let mut clone = board.clone();
            if clone.do_move(&board_move) {
                println!("{} -> {}", board_move.to_string(), self.perft(&mut clone, depth - 1));
            }
        }
        self.move_list.end_ply();
    }

    pub fn perft(&mut self, board: &mut Board, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        self.move_list.start_ply();
        self.move_list.generate_quiets(board, &mut self.attack_info);
        self.move_list.generate_noisy(board, &mut self.attack_info);

        let mut result = 0;

        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            let mut clone = board.clone();
            if clone.do_move(&board_move) {
                result += self.perft(&mut clone, depth - 1);
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
    use crate::advanced::board::Board;
    use crate::advanced::perft::Perft;

    fn check_perft_file(path: &str, depth_limit: u8) {
        let file = File::open(path).expect("failed to open test suite");
        let reader = BufReader::new(file);

        let mut board = Board::default();
        let mut perft = Perft::new();

        for line in reader.lines().map(|l| l.unwrap()) {
            let mut slices = line.trim().splitn(2, ' ');

            match slices.next() {
                Some("epd") => {
                    let position = slices.next().expect("expected position");
                    println!("position {}", position);
                    board = Board::from_fen(position);
                }
                Some("perft") => {
                    let mut params = slices.next().expect("expected perft params").splitn(2, ' ');
                    let depth: u8 = params.next().expect("expected perft depth").parse().expect("expected integer value");

                    let nodes: u64 = params.next().expect("expected perft nodes").parse().expect("expected integer value");

                    if depth <= depth_limit {
                        assert_eq!(perft.perft(&mut board, depth), nodes);
                    }
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_random() {
        check_perft_file("G:/chess/epds/random.perft", 6);
    }
}
