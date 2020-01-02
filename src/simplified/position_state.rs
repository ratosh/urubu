use crate::advanced::zobrist_key::ZobristKey;
use crate::types::color::Color;
use crate::types::castling_rights::CastlingRights;
use crate::types::square::Square;
use crate::types::bitboard::Bitboard;
use crate::types::piece_type::PieceType;

#[derive(Clone, Copy)]
pub struct PositionState {
    // Extra info
    pub zkey: ZobristKey,
    pub zkey_pawn: ZobristKey,

    pub rule_50: usize,

    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,

    pub check_bitboard: Bitboard,
}

impl PositionState {

    #[inline]
    pub fn new() -> Self {
        Self {
            zkey: ZobristKey::new(),
            zkey_pawn: ZobristKey::new(),
            rule_50: 0,
            castling_rights: CastlingRights::NO_CASTLING,
            ep_square: None,
            check_bitboard: Bitboard::EMPTY
        }
    }

    #[inline]
    pub fn change_piece(&mut self, color: Color, piece_type: PieceType, square: Square) {
        // Updating zobrist info
        self.zkey.change_piece(color, piece_type, square);
        if piece_type == PieceType::PAWN {
            self.zkey_pawn.change_piece(color, piece_type, square)
        }
    }

    pub fn move_piece(&mut self, color: Color, piece_type: PieceType, from: Square, to: Square) {
        // Updating zobrist info
        self.zkey.move_piece(color, piece_type, from, to);
        if piece_type == PieceType::PAWN {
            self.zkey_pawn.move_piece(color, piece_type, from, to)
        }
    }

    #[inline]
    pub fn update_castling_rights(&mut self, right_change: CastlingRights) {
        if right_change != CastlingRights::NO_CASTLING {
            self.zkey.set_castling_rights(self.castling_rights);
            self.castling_rights = self.castling_rights.difference(right_change);
            self.zkey.set_castling_rights(self.castling_rights);
        }
    }

    #[inline]
    pub fn clear_ep(&mut self) {
        if self.ep_square.is_some() {
            self.zkey.set_ep(self.ep_square.unwrap());
            self.ep_square = None;
        }
    }

    #[inline]
    pub fn set_ep(&mut self, square: Square) {
        self.ep_square = Some(square);
        self.zkey.set_ep(square);
    }
}