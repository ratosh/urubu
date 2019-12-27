use crate::advanced::zobrist_key::ZobristKey;
use crate::types::color::Color;
use crate::types::castling_rights::CastlingRights;
use crate::types::square::Square;
use crate::types::bitboard::Bitboard;
use crate::types::piece_type::PieceType;

#[derive(Clone)]
pub struct PositionState {
    pub zkey: ZobristKey,
    pub zkey_pawn: ZobristKey,

    pub color_to_move: Color,
    pub move_number: usize,
    pub rule_50: usize,

    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,

    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub check_bitboard: Bitboard,
}

impl PositionState {

    #[inline]
    pub fn new() -> Self {
        Self {
            zkey: ZobristKey::new(),
            zkey_pawn: ZobristKey::new(),
            color_to_move: Color::White,
            move_number: 0,
            rule_50: 0,
            castling_rights: CastlingRights::NO_CASTLING,
            ep_square: None,
            danger_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
            check_bitboard: Bitboard::EMPTY
        }
    }
}