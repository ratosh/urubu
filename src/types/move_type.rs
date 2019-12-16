use crate::types::piece_type::PieceType;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct MoveType(pub u8);

impl MoveType {
    pub const NORMAL: MoveType = MoveType(0);

    pub const PASSANT: MoveType = MoveType(1);
    pub const CASTLING: MoveType = MoveType(2);

    pub const PROMOTION_KNIGHT: MoveType = MoveType(4);
    pub const PROMOTION_BISHOP: MoveType = MoveType(5);
    pub const PROMOTION_ROOK: MoveType = MoveType(6);
    pub const PROMOTION_QUEEN: MoveType = MoveType(7);

    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_u16(&self) -> u16 {
        self.0 as u16
    }

    #[inline]
    pub fn is_promotion(&self) -> bool {
        self.0 & MoveType::PROMOTION_KNIGHT.0 != 0
    }

    #[inline]
    pub fn promoted_piece_type(&self) -> PieceType {
        if self.is_promotion() {
            PieceType(self.0 - 2)
        } else {
            PieceType::NONE
        }
    }

    #[inline]
    pub fn is_castling(&self) -> bool {
        self.0 == MoveType::CASTLING.0
    }

    #[inline]
    pub fn is_passant(&self) -> bool {
        self.0 == MoveType::PASSANT.0
    }

    #[inline]
    pub fn to_string(&self) -> char {
        self.promoted_piece_type().to_char()
    }
}