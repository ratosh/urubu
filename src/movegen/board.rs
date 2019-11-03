use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::square::Square;
use crate::types::piece_type::PieceType;
use crate::movegen::board_state::BoardState;

#[derive(Clone)]
pub struct Board {

    pub game_bitboard: Bitboard,
    pub piece_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub color_bitboard: [Bitboard; Color::NUM_COLORS],

    pub piece_type_board: [PieceType; Square::NUM_SQUARES],

    pub color_to_move: Color,

    pub move_number: u16,
    pub king_square: [Square; Color::NUM_COLORS],

    pub board_state: [BoardState;Board::GAME_MAX_LENGTH]
}

impl Board {
    pub const GAME_MAX_LENGTH:usize = 4095;

    #[inline]
    pub fn empty_bitboard(&self) -> Bitboard {
        self.game_bitboard.invert()
    }
}