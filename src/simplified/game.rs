use crate::types::square::Square;
use crate::types::castling_rights::{CastlingIndex, CastlingRights};
use crate::simplified::position::Position;
use crate::advanced::zobrist_key::ZobristKey;
use crate::types::color::Color;

#[derive(Clone)]
pub struct Game {
    position: Position,
    stack: Vec<StackElem>,
}

#[derive(Clone)]
struct HistoryStackElement {
    pub key: ZobristKey,
    pub piece_type_captured: Piece,
    pub square_captured: Square,
    pub position: Position,
}

impl Game {
    pub fn from_fen(fen: &str) -> Game {
        let position = Position::from_fen(fen).unwrap();

        Game {
            position,
            stack: Vec::new(),
        }
    }

    pub fn make_move(&mut self, board_move: &BoardMove) -> bool {
        let mut clone = self.position.clone;
        let make = clone.
        clone.
    }
}