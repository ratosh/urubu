use crate::advanced::zobrist_key::ZobristKey;
use crate::simplified::position::Position;
use crate::types::board_move::BoardMove;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(Clone)]
pub struct Game {
    position: Position,
    stack: Vec<HistoryStackElement>,
}

#[derive(Clone)]
struct HistoryStackElement {
    pub key: ZobristKey,
    pub piece_type_captured: PieceType,
    pub square_captured: Square,
    pub position: Position,
}

impl Game {
    pub fn from_fen(fen: &str) -> Game {
        Game {
            position: Position::from_fen(fen),
            stack: Vec::new(),
        }
    }

    pub fn make_move(&mut self, board_move: BoardMove) -> bool {
        println!("{}", board_move.to_string());
        true
    }
}
