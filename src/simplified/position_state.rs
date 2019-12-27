use crate::advanced::zobrist_key::ZobristKey;
use crate::types::color::Color;
use crate::types::castling_rights::CastlingRights;
use crate::types::square::Square;
use crate::types::bitboard::Bitboard;
use crate::types::piece_type::PieceType;

#[derive(Clone)]
pub struct PositionState {
    zkey: ZobristKey,
    zkey_pawn: ZobristKey,

    color_to_move: Color,
    move_number: usize,
    rule_50: usize,

    castling_rights: CastlingRights,
    ep_square: Option<Square>,

    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub check_bitboard: Bitboard,
}
