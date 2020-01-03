use crate::types::castling_rights::CastlingRights;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

include!(concat!(env!("OUT_DIR"), "/zobrist_generated.rs"));

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct ZobristKey(pub u64);

impl ZobristKey {
    #[inline]
    pub fn to_u64(self) -> u64 {
        self.0 as u64
    }

    #[inline]
    pub fn move_piece(self, color: Color, piece_type: PieceType, from: Square, to: Square) -> ZobristKey {
        self.change_piece(color, piece_type, to).change_piece(color, piece_type, from)
    }

    #[inline]
    pub fn change_piece(self, color: Color, piece_type: PieceType, square: Square) -> ZobristKey {
        ZobristKey(self.0 ^ PSQT[color.to_usize()][piece_type.to_usize()][square.to_usize()])
    }

    #[inline]
    pub fn set_ep(self, square: Square) -> ZobristKey {
        ZobristKey(self.0 ^ EP[square.to_file().to_usize()])
    }

    #[inline]
    pub fn set_castling_rights(self, castling_rights: CastlingRights) -> ZobristKey {
        ZobristKey(self.0 ^ CASTLING[castling_rights])
    }

    #[inline]
    pub fn change_color(self) -> ZobristKey {
        ZobristKey(self.0 ^ COLOR)
    }
}

impl Default for ZobristKey {
    fn default() -> Self {
        ZobristKey(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_piece() {
        let key = ZobristKey::default();
        let k1 = key.move_piece(Color::White, PieceType::PAWN, Square::A1, Square::A2);
        let k2 = key.move_piece(Color::White, PieceType::PAWN, Square::A2, Square::A1);
        assert_ne!(key, k1);
        assert_eq!(key, k2);
    }

    #[test]
    fn change_piece() {
        let key = ZobristKey::default();
        let k1 = key.change_piece(Color::White, PieceType::PAWN, Square::A1);
        let k2 = key.change_piece(Color::White, PieceType::PAWN, Square::A1);
        assert_ne!(key, k1);
        assert_eq!(key, k2);
    }

    #[test]
    fn set_ep() {
        let key = ZobristKey::default();
        let k1 = key.set_ep(Square::A2);
        let k2 = key.set_ep(Square::A2);
        assert_ne!(key, k1);
        assert_eq!(key, k2);
    }

    #[test]
    fn set_castling_rights() {
        let key = ZobristKey::default();
        let k1 = key.set_castling_rights(CastlingRights::NO_CASTLING);
        let k2 = key.set_castling_rights(CastlingRights::ANY_CASTLING);
        let k3 = key.set_castling_rights(CastlingRights::ANY_CASTLING);
        assert_eq!(key, k1);
        assert_ne!(key, k2);
        assert_eq!(key, k3);
    }

    #[test]
    fn update_color() {
        let key = ZobristKey::default();
        let k1 = key.change_color();
        let k2 = key.change_color();
        assert_ne!(key, k1);
        assert_eq!(key, k2);
    }
}
