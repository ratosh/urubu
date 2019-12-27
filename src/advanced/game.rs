use crate::advanced::board::Board;
use crate::types::board_move::BoardMove;

#[derive(Clone)]
pub struct Game {
    pub board_history: Vec<Board>,
    pub board: Board,
}

impl Game {

    pub const MAX_LENGTH: usize = 4095;
    #[inline]
    pub fn from_board(board: &Board) -> Self {
        let mut result = Self {
            board_history: Vec::new(),
            board: board.clone(),
        };
        result.setup();
        return result;
    }

    #[inline]
    pub fn default() -> Self {
        let mut result = Self {
            board_history: Vec::new(),
            board: Board::default(),
        };
        result.setup();
        return result;
    }

    #[inline]
    pub fn reset(&mut self, board: &Board) {
        self.board_history.clear();
        self.board = board.clone();
        self.setup();
    }

    #[inline]
    pub fn setup(&mut self) {
        self.board.setup();
    }

    #[inline]
    pub fn do_move(&mut self, board_move: &BoardMove) -> bool {
        self.push_to_history(self.board.clone());
        return self.board.do_move(board_move);
    }

    #[inline]
    pub fn undo_move(&mut self) {
        self.board = self.pop_from_history();
    }

    #[inline]
    fn push_to_history(&mut self, board: Board) {
        self.board_history.push(board);
    }

    #[inline]
    fn pop_from_history(&mut self) -> Board {
        self.board_history.pop().unwrap()
    }
}