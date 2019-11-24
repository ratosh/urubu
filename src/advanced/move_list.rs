use crate::types::board_move::BoardMove;
use crate::advanced::game::GAME_MAX_LENGTH;

pub struct MoveList {
    current_ply: usize,
    move_list: [BoardMove; GAME_MAX_LENGTH],
    next_to_move: [usize; MoveList::MAX_PLIES],
    next_to_generate: [usize; MoveList::MAX_PLIES],
}

#[allow(dead_code)]
impl MoveList {
    pub const MAX_PLIES: usize = 127;

    pub fn new() -> Self {
        MoveList {
            current_ply: 0,
            move_list: [BoardMove::NONE; GAME_MAX_LENGTH],
            next_to_move: [0; MoveList::MAX_PLIES],
            next_to_generate: [0; MoveList::MAX_PLIES],
        }
    }

    pub fn start_ply(&mut self) -> bool {
        if self.current_ply >= MoveList::MAX_PLIES - 1 {
            return false;
        }
        let next_to_gen = self.next_to_generate[self.current_ply];
        self.current_ply += 1;
        self.next_to_generate[self.current_ply] = next_to_gen;
        self.next_to_move[self.current_ply] = next_to_gen;
        return true;
    }

    pub fn end_ply(&mut self) {
        self.current_ply -= 1;
    }

    pub fn next(&mut self) -> BoardMove {
        let board_move = self.move_list[self.next_to_move[self.current_ply]];
        self.current_ply += 1;
        return board_move;
    }

    pub fn has_next(&self) -> bool {
        return self.next_to_generate[self.current_ply] != self.next_to_move[self.current_ply];
    }

    pub fn add_move(&mut self, board_move: BoardMove) {
        self.move_list[self.next_to_generate[self.current_ply]] = board_move;
        self.next_to_generate[self.current_ply] += 1;
    }

    pub fn skip_moves(&mut self) {
        self.next_to_move[self.current_ply] = self.next_to_generate[self.current_ply];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::square::Square;

    #[test]
    fn add_next() {
        let mut move_list = MoveList::new();
        let board_move = BoardMove::build_normal(&Square::A1, &Square::A2);
        move_list.add_move(board_move);
        assert_eq!(move_list.next(), board_move)
    }

    #[test]
    fn current_ply_change() {
        let mut move_list = MoveList::new();
        assert_eq!(move_list.current_ply, 0);
        move_list.start_ply();
        assert_eq!(move_list.current_ply, 1);
        move_list.end_ply();
        assert_eq!(move_list.current_ply, 0);
    }

    #[test]
    fn has_next() {
        let mut move_list = MoveList::new();
        assert_eq!(move_list.has_next(), false);
        let board_move = BoardMove::build_normal(&Square::A1, &Square::A2);
        move_list.add_move(board_move);
        assert_eq!(move_list.has_next(), true);
        move_list.start_ply();
        assert_eq!(move_list.has_next(), false);
        move_list.end_ply();
        assert_eq!(move_list.has_next(), true);
    }

    #[test]
    fn skip_moves() {
        let mut move_list = MoveList::new();
        assert_eq!(move_list.has_next(), false);
        let board_move = BoardMove::build_normal(&Square::A1, &Square::A2);
        move_list.start_ply();
        assert_eq!(move_list.has_next(), false);
        move_list.skip_moves();
        assert_eq!(move_list.has_next(), false);
        move_list.add_move(board_move);
        assert_eq!(move_list.has_next(), true);
        move_list.start_ply();
        assert_eq!(move_list.has_next(), false);
        move_list.end_ply();
        assert_eq!(move_list.has_next(), true);
        move_list.skip_moves();
        assert_eq!(move_list.has_next(), false);
    }
}