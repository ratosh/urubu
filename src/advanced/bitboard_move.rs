use std::ops::{Shl, Shr};

use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::magic::Magic;
use crate::types::square::Square;

include!(concat!(env!("OUT_DIR"), "/bitboard_generated.rs"));

const NORTH: i8 = 8;

impl Bitboard {
    #[inline]
    pub fn pawn_forward(self, color: Color) -> Bitboard {
        if color == Color::White {
            self.mvl(NORTH as u64)
        } else {
            self.mvr(NORTH as u64)
        }
    }

    #[inline]
    pub fn mvl(self, offset: u64) -> Bitboard {
        Bitboard(self.0.shl(offset))
    }

    #[inline]
    pub fn mvr(self, offset: u64) -> Bitboard {
        Bitboard(self.0.shr(offset))
    }

    #[inline]
    pub fn pawn_attacks(self, color: Color) -> Bitboard {
        match color {
            Color::White => self
                .difference(Bitboard::FILE_A)
                .mvl(7)
                .union(self.difference(Bitboard::FILE_H).mvl(9)),
            Color::Black => self
                .difference(Bitboard::FILE_A)
                .mvr(9)
                .union(self.difference(Bitboard::FILE_H).mvr(7)),
        }
    }
}

impl Square {
    #[inline]
    pub fn pawn_attacks(self, color: Color) -> Bitboard {
        unsafe {
            *PAWN_ATTACKS.get_unchecked(color.to_usize()).get_unchecked(self.to_usize())
        }
    }

    #[inline]
    pub fn pawn_move(self, color: Color) -> Bitboard {
        unsafe {
            *PAWN_MOVES.get_unchecked(color.to_usize()).get_unchecked(self.to_usize())
        }
    }

    #[inline]
    pub fn pawn_double_move(self, color: Color) -> Bitboard {
        unsafe {
            *PAWN_DOUBLE_MOVES.get_unchecked(color.to_usize()).get_unchecked(self.to_usize())
        }
    }

    #[inline]
    pub fn between(self, other: Square) -> Bitboard {
        unsafe {
            *BETWEEN.get_unchecked(self.to_usize()).get_unchecked(other.to_usize())
        }
    }

    #[inline]
    pub fn neighbour(self) -> Bitboard {
        NEIGHBOUR[self]
    }

    #[inline]
    pub fn pinned_mask(self, other: Square) -> Bitboard {
        unsafe {
            *PINNED_MASK.get_unchecked(self.to_usize()).get_unchecked(other.to_usize())
        }
    }

    #[inline]
    pub fn knight_moves(self) -> Bitboard {
        KNIGHT_MOVES[self]
    }

    #[inline]
    pub fn king_moves(self) -> Bitboard {
        KING_MOVES[self]
    }

    #[inline]
    pub fn pseudo_bishop_moves(self) -> Bitboard {
        PSEUDO_BISHOP[self]
    }

    #[inline]
    pub fn bishop_moves(self, occupied: Bitboard) -> Bitboard {
        let magic = &Magic::BISHOP[self];
        let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as usize
            >> (Square::NUM_SQUARES - Magic::BISHOP_SHIFT) as usize)
            + magic.offset;

        unsafe {
            *ATTACKS.get_unchecked(index as usize)
        }
    }

    #[inline]
    pub fn pseudo_rook_moves(self) -> Bitboard {
        PSEUDO_ROOK[self]
    }

    #[inline]
    pub fn rook_moves(self, occupied: Bitboard) -> Bitboard {
        let magic = &Magic::ROOK[self];
        let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as usize
            >> (Square::NUM_SQUARES - Magic::ROOK_SHIFT) as usize)
            + magic.offset;
        unsafe {
            *ATTACKS.get_unchecked(index as usize)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knight_moves() {
        assert_eq!(Square::A1.knight_moves(), Bitboard::B3.union(Bitboard::C2));
        assert_eq!(
            Square::B2.knight_moves(),
            Bitboard::A4
                .union(Bitboard::C4)
                .union(Bitboard::D3)
                .union(Bitboard::D1)
        );
    }

    #[test]
    fn king_moves() {
        assert_eq!(
            Square::A1.king_moves(),
            Bitboard::A2.union(Bitboard::B1).union(Bitboard::B2)
        );
        assert_eq!(
            Square::B2.king_moves(),
            Bitboard::A1
                .union(Bitboard::A2)
                .union(Bitboard::A3)
                .union(Bitboard::B1)
                .union(Bitboard::B3)
                .union(Bitboard::C1)
                .union(Bitboard::C2)
                .union(Bitboard::C3)
        );
    }

    #[test]
    fn pawn_bitboard_forward() {
        assert_eq!(Bitboard::A2.pawn_forward(Color::White), Bitboard::A3);
        assert_eq!(Bitboard::A2.pawn_forward(Color::Black), Bitboard::A1);
    }

    #[test]
    fn bitboard_pawn_attacks() {
        assert_eq!(Bitboard::A5.pawn_attacks(Color::White), Bitboard::B6);
        assert_eq!(Bitboard::A5.pawn_attacks(Color::Black), Bitboard::B4);
        assert_eq!(Bitboard::H3.pawn_attacks(Color::White), Bitboard::G4);
        assert_eq!(Bitboard::H3.pawn_attacks(Color::Black), Bitboard::G2);
    }

    #[test]
    fn square_pawn_attacks() {
        assert_eq!(Square::A5.pawn_attacks(Color::White), Bitboard::B6);
        assert_eq!(Square::A5.pawn_attacks(Color::Black), Bitboard::B4);
        assert_eq!(Square::H3.pawn_attacks(Color::White), Bitboard::G4);
        assert_eq!(Square::H3.pawn_attacks(Color::Black), Bitboard::G2);
    }

    #[test]
    fn pawn_move() {
        assert_eq!(Square::A2.pawn_move(Color::White), Bitboard::A3);
        assert_eq!(Square::A8.pawn_move(Color::White), Bitboard::EMPTY);
        assert_eq!(Square::A2.pawn_move(Color::Black), Bitboard::A1);
        assert_eq!(Square::A1.pawn_move(Color::Black), Bitboard::EMPTY);
    }

    #[test]
    fn pawn_double_move() {
        assert_eq!(Square::A2.pawn_double_move(Color::White), Bitboard::A4);
        assert_eq!(Square::A3.pawn_double_move(Color::White), Bitboard::EMPTY);
        assert_eq!(Square::A2.pawn_double_move(Color::Black), Bitboard::EMPTY);
        assert_eq!(Square::A7.pawn_double_move(Color::Black), Bitboard::A5);
    }

    #[test]
    fn neighbour() {
        assert_eq!(Square::A2.neighbour(), Bitboard::B2);
        assert_eq!(Square::D3.neighbour(), Bitboard::C3.union(Bitboard::E3));
    }

    #[test]
    fn bishop_move() {
        assert_eq!(
            Square::A1.pseudo_bishop_moves(),
            Bitboard::B2
                .union(Bitboard::C3)
                .union(Bitboard::D4)
                .union(Bitboard::E5)
                .union(Bitboard::F6)
                .union(Bitboard::G7)
                .union(Bitboard::H8)
        );
        assert_eq!(
            Square::A1.bishop_moves(Bitboard::EMPTY),
            Square::A1.pseudo_bishop_moves()
        );
    }

    #[test]
    fn rook_move() {
        assert_eq!(
            Square::A1.rook_moves(Bitboard::EMPTY),
            Square::A1.pseudo_rook_moves()
        );
        assert_eq!(
            Square::A1.rook_moves(Bitboard::EMPTY),
            Bitboard::A2
                .union(Bitboard::A3)
                .union(Bitboard::A4)
                .union(Bitboard::A5)
                .union(Bitboard::A6)
                .union(Bitboard::A7)
                .union(Bitboard::A8)
                .union(Bitboard::B1)
                .union(Bitboard::C1)
                .union(Bitboard::D1)
                .union(Bitboard::E1)
                .union(Bitboard::F1)
                .union(Bitboard::G1)
                .union(Bitboard::H1)
        );
    }

    #[test]
    fn pinned_mask() {
        assert_eq!(
            Square::H2.pinned_mask(Square::G2),
            Bitboard::G2
                .union(Bitboard::F2)
                .union(Bitboard::E2)
                .union(Bitboard::D2)
                .union(Bitboard::C2)
                .union(Bitboard::B2)
                .union(Bitboard::A2)
        );
        assert_eq!(
            Square::H3.pinned_mask(Square::G4),
            Bitboard::G4
                .union(Bitboard::F5)
                .union(Bitboard::E6)
                .union(Bitboard::D7)
                .union(Bitboard::C8)
        );
    }
}
