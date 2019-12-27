use crate::types::castling_rights::{CastlingIndex, CastlingRights};
use crate::types::square::Square;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::bitboard::Bitboard;

/// Position encodes all positional information
pub struct Position {
    // Position castling information
    initial_rook_square: [Square; CastlingIndex::NUM_INDEXES],
    castling_rights_masks: [CastlingRights; Square::NUM_SQUARES],

    bb_sides: [Bitboard; Color::NUM_COLORS],
    bb_pieces: [Bitboard; PieceType::NUM_PIECE_TYPES],

    state: PositionState,

    king_square: [Square; Color::NUM_COLORS],
}

pub struct PositionState {
    zkey: Zobrist,
    zkey_pawn: Zobrist,

    color_to_move: Color,
    move_number: usize,
    rule_50: usize,

    castling_rights: CastlingRights,
    ep_square: Option<Square>,

    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub check_bitboard: Bitboard,
}