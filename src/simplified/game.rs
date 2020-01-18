use crate::advanced::zobrist_key::ZobristKey;
use crate::simplified::position::Position;
use crate::simplified::position_state::PositionState;
use crate::types::board_move::BoardMove;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(Clone)]
pub struct Game {
    pub position: Position,
    stack: Vec<HistoryStackElement>,
}

#[derive(Clone)]
struct HistoryStackElement {
    pub piece_type_captured: PieceType,
    pub square_captured: Square,
    pub state: PositionState,
}

impl Game {
    pub fn from_fen(fen: &str) -> Game {
        Game {
            position: Position::from_fen(fen),
            stack: Vec::new(),
        }
    }

    pub fn do_move(&mut self, board_move: BoardMove) -> bool {
        return if self.position.is_legal_move(board_move) {
            let state = self.position.state().clone();
            let (piece_type_captured, square_captured) = self.position.do_move(board_move);
            self.stack.push(HistoryStackElement {
                state,
                piece_type_captured,
                square_captured,
            });
            true
        } else {
            false
        };
    }

    pub fn undo_move(&mut self, board_move: BoardMove) {
        let history = self.stack.pop().unwrap();
        self.position.undo_move(board_move, history.piece_type_captured, history.square_captured, &history.state);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            position: Position::default(),
            stack: Vec::new()
        }
    }
}

