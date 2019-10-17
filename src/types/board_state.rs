use crate::types::bitboard::Bitboard;
use crate::types::castling_rights::CastlingRights;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(Clone)]
pub struct BoardState {
    pub zobrist_key: u64,
    pub pawn_zobrist_key: u64,
    pub rule_50: u8,
    pub castling_rights: CastlingRights,
    pub ep_square: Square,
    pub captured_piece: PieceType,
    pub check_bitboard: u64,
    pub pinned_bitboard: u64,
    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS]
}