use std::fmt::{Debug, Error, Formatter};

use crate::advanced::zobrist_key::ZobristKey;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::castling_rights::{CastlingIndex, CastlingRights, CastlingSide};
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;
use crate::advanced::game::GAME_MAX_LENGTH;
use crate::advanced::board_state::BoardState;
use crate::advanced::position::Position;

#[derive(Clone)]
pub struct GameInfo {
    pub initial_rook_square: [Square; CastlingIndex::NUM_INDEXES],
    pub castling_rights_masks: [CastlingRights; Square::NUM_SQUARES],
}

impl GameInfo {

    #[inline]
    pub fn default() -> Self {
        Self {
            initial_rook_square: [Square::H1, Square::A1, Square::H8, Square::A8],
            castling_rights_masks: [CastlingRights::NO_CASTLING; Square::NUM_SQUARES]
        }
    }

    #[inline]
    pub fn setup(&mut self, position: &Position) {
        self.castling_rights_masks[position.king_square(Color::White).to_usize()] = CastlingRights::WHITE_RIGHTS;
        self.castling_rights_masks[position.king_square(Color::Black).to_usize()] = CastlingRights::BLACK_RIGHTS;

        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::WhiteA.to_usize()].to_usize()] = CastlingRights::WHITE_OOO;
        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::WhiteH.to_usize()].to_usize()] = CastlingRights::WHITE_OO;
        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::BlackA.to_usize()].to_usize()] = CastlingRights::BLACK_OOO;
        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::BlackH.to_usize()].to_usize()] = CastlingRights::BLACK_OO;
    }

    #[inline]
    pub fn initial_rook_square(&self, castling_index: &CastlingIndex) -> Square {
        self.initial_rook_square[castling_index.to_usize()]
    }
}

#[derive(Clone, Copy)]
pub struct MoveStack {
    pub state: BoardState,
    pub piece_type_captured: PieceType,
    pub square_captured: Square,
}

impl MoveStack {

    #[inline]
    pub fn default() -> Self {
        Self {
            state: BoardState::new(),
            piece_type_captured: PieceType::NONE,
            square_captured: Square::A1,
        }
    }
}


#[derive(Clone)]
pub struct Board {
    pub game_info: GameInfo,
    pub move_history: Vec<MoveStack>,
    pub position: Position
}


impl Board {
    #[inline]
    pub fn from_position(position: &Position) -> Self {
        let mut result = Self {
            game_info: GameInfo::default(),
            move_history: Vec::new(),
            position: position.clone()
        };
        result.setup();
        return result;
    }

    #[inline]
    pub fn default() -> Self {
        let mut result = Self {
            game_info: GameInfo::default(),
            move_history: Vec::new(),
            position: Position::default()
        };
        result.setup();
        return result;
    }

    #[inline]
    pub fn setup(&mut self) {
        self.position.setup();
        self.game_info.setup(&self.position);
    }

    #[inline]
    pub fn do_move(&mut self, board_move: BoardMove) -> bool {
        let before_state = self.position.current_state.clone();
        let (piece_type_captured, square_captured, valid) = self.position.do_move(board_move, &self.game_info);
        self.push_to_history(before_state, piece_type_captured, square_captured);
        return valid;
    }

    #[inline]
    pub fn undo_move(&mut self, board_move: BoardMove) {
        let move_stack = self.pop_from_history();
        self.position.undo_move(board_move, move_stack.state, move_stack.piece_type_captured, move_stack.square_captured, &self.game_info);
    }

    #[inline]
    fn push_to_history(&mut self, state: BoardState, piece_type_captured: PieceType, square_captured: Square)  {
        self.move_history.push(MoveStack {
            piece_type_captured,
            square_captured,
            state,
        })
    }

    #[inline]
    fn pop_from_history(&mut self) -> MoveStack {
        self.move_history.pop().unwrap()
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Board {}

impl Debug for Board {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let mut res_str: String = String::new();
        res_str.push_str(&format!("Position: {}\n", self.position.to_fen()));
        write!(formatter, "{}", res_str)
    }
}

#[cfg(test)]
mod test {
    use crate::advanced::board::Board;
    use crate::types::bitboard::Bitboard;
    use crate::types::color::Color;
    use crate::types::piece_type::PieceType;

    use super::*;

    #[test]
    fn default_board() {
        let board = Board::default();
        assert_eq!(board.position.current_state.color_to_move.is_white(), true);

        assert_eq!(board.position.color_at(Square::A1).unwrap().is_white(), true);
        assert_eq!(board.position.color_at(Square::A2).unwrap().is_white(), true);
        assert_eq!(board.position.color_at(Square::A3).is_none(), true);
        assert_eq!(board.position.color_at(Square::A4).is_none(), true);
        assert_eq!(board.position.color_at(Square::A5).is_none(), true);
        assert_eq!(board.position.color_at(Square::A6).is_none(), true);
        assert_eq!(board.position.color_at(Square::A7).unwrap().is_white(), false);
        assert_eq!(board.position.color_at(Square::A8).unwrap().is_white(), false);

        assert_eq!(board.position.piece_type(Square::A1), PieceType::ROOK);
        assert_eq!(board.position.piece_type(Square::A2), PieceType::PAWN);
        assert_eq!(board.position.piece_type(Square::A3), PieceType::NONE);
        assert_eq!(board.position.piece_type(Square::A4), PieceType::NONE);
        assert_eq!(board.position.piece_type(Square::A5), PieceType::NONE);
        assert_eq!(board.position.piece_type(Square::A6), PieceType::NONE);
        assert_eq!(board.position.piece_type(Square::A7), PieceType::PAWN);
        assert_eq!(board.position.piece_type(Square::A8), PieceType::ROOK);
    }
}