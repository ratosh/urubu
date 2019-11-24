use crate::advanced::attack_info::AttackInfo;
use crate::advanced::move_list::MoveList;
use crate::advanced::board::Board;

struct Perft {
    attack_info: AttackInfo,
    move_list: MoveList,
}

impl Perft {
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