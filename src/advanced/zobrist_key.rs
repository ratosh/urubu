use crate::types::castling_rights::CastlingRights;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

include!(concat!(env!("OUT_DIR"), "/zobrist_generated.rs"));

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct ZobristKey(pub u64);

impl ZobristKey {
    #[inline]
    pub fn to_u64(&self) -> u64 {
        self.0 as u64
    }

    #[inline]
    pub fn new() -> Self {
        ZobristKey(0)
    }

    #[inline]
    pub fn move_piece(&mut self, color: Color, piece_type: PieceType, from: Square, to: Square) {
        self.change_piece(color, piece_type, from);
        self.change_piece(color, piece_type, to);
    }

    #[inline]
    pub fn change_piece(&mut self, color: Color, piece_type: PieceType, square: Square) {
        self.0 ^= PSQT[color.to_usize()][piece_type.to_usize()][square.to_usize()];
    }

    #[inline]
    pub fn set_ep(&mut self, square: Square) {
        self.0 ^= EP[square.to_file().to_usize()];
    }

    #[inline]
    pub fn set_castling_rights(&mut self, castling_rights: CastlingRights) {
        self.0 ^= CASTLING[castling_rights];
    }

    #[inline]
    pub fn change_color(&mut self) {
        self.0 ^= COLOR;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_piece() {
        let mut key = ZobristKey::new();
        let init_key = key.to_u64();
        key.move_piece(Color::White, PieceType::PAWN, Square::A1, Square::A2);
        let k1 = key.to_u64();
        key.move_piece(Color::White, PieceType::PAWN, Square::A2, Square::A1);
        let k2 = key.to_u64();
        assert_ne!(init_key, k1);
        assert_eq!(init_key, k2);
    }

    #[test]
    fn change_piece() {
        let mut key = ZobristKey::new();
        let init_key = key.to_u64();
        key.change_piece(Color::White, PieceType::PAWN, Square::A1);
        let k1 = key.to_u64();
        key.change_piece(Color::White, PieceType::PAWN, Square::A1);
        let k2 = key.to_u64();
        assert_ne!(init_key, k1);
        assert_eq!(init_key, k2);
    }

    #[test]
    fn set_ep() {
        let mut key = ZobristKey::new();
        let init_key = key.to_u64();
        key.set_ep(Square::A2);
        let k1 = key.to_u64();
        key.set_ep(Square::A2);
        let k2 = key.to_u64();
        assert_ne!(init_key, k1);
        assert_eq!(init_key, k2);
    }

    #[test]
    fn set_castling_rights() {
        let mut key = ZobristKey::new();
        let init_key = key.to_u64();
        key.set_castling_rights(CastlingRights::NO_CASTLING);
        let k1 = key.to_u64();
        key.set_castling_rights(CastlingRights::ANY_CASTLING);
        let k2 = key.to_u64();
        key.set_castling_rights(CastlingRights::ANY_CASTLING);
        let k3 = key.to_u64();
        assert_eq!(init_key, k1);
        assert_ne!(init_key, k2);
        assert_eq!(init_key, k3);
    }

    #[test]
    fn update_color() {
        let mut key = ZobristKey::new();
        let init_key = key.to_u64();
        key.change_color();
        let k1 = key.to_u64();
        key.change_color();
        let k2 = key.to_u64();
        assert_ne!(init_key, k1);
        assert_eq!(init_key, k2);
    }
}