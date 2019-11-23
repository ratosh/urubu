use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::square::Square;
use crate::types::magic::Magic;
use std::ops::{Shr, Shl};

include!(concat!(env!("OUT_DIR"), "/bitboard_generated.rs"));

const NORTH: i8 = 8;

const EAST: i8 = 1;
const WEST: i8 = -EAST;

const PAWN_ATTACK_STEP: [i8; 2] = [
    NORTH + WEST,
    NORTH + EAST];

impl Bitboard {
    #[inline]
    pub fn pawn_forward(&self, color: &Color) -> Bitboard {
        self.pawn_move(color, NORTH)
    }

    #[inline]
    pub fn pawn_attacks_left(&self, color: &Color) -> Bitboard {
        self.pawn_move(color, PAWN_ATTACK_STEP[0])
    }

    #[inline]
    pub fn pawn_attacks_right(&self, color: &Color) -> Bitboard {
        self.pawn_move(color, PAWN_ATTACK_STEP[1])
    }

    #[inline]
    fn pawn_move(&self, color: &Color, offset: i8) -> Bitboard {
        Bitboard(if color.is_white() {
            self.0.shl(offset as u64)
        } else {
            self.0.shr(offset as u64)
        })
    }
}

impl Square {

    #[inline]
    pub fn pawn_attacks(&self, color: &Color) -> Bitboard {
        return PAWN_ATTACKS[color.to_usize()][self.to_usize()]
    }

    #[inline]
    pub fn pawn_move(&self, color: &Color) -> Bitboard {
        PAWN_MOVES[color.to_usize()][self.to_usize()]
    }

    #[inline]
    pub fn pawn_double_move(&self, color: &Color) -> Bitboard {
        PAWN_DOUBLE_MOVES[color.to_usize()][self.to_usize()]
    }

    #[inline]
    pub fn between(&self, other: &Square) -> Bitboard {
        BETWEEN[self.to_usize()][other.to_usize()]
    }

    #[inline]
    pub fn neighbour(&self) -> Bitboard {
        NEIGHBOUR[self.to_usize()]
    }

    #[inline]
    pub fn pinned_mask(&self, other: &Square) -> Bitboard {
        PINNED_MASK[self.to_usize()][other.to_usize()]
    }

    #[inline]
    pub fn knight_moves(&self) -> Bitboard {
        KNIGHT_MOVES[self.to_usize()]
    }

    #[inline]
    pub fn king_moves(&self) -> Bitboard {
        KING_MOVES[self.to_usize()]
    }

    #[inline]
    pub fn pseudo_bishop_moves(&self) -> Bitboard {
        PSEUDO_BISHOP[self.to_usize()]
    }

    #[inline]
    pub fn bishop_moves(&self, occupied: &Bitboard) -> Bitboard {
        let magic = &Magic::BISHOP[self.to_usize()];
        let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as u64 >> (Square::NUM_SQUARES - Magic::BISHOP_SHIFT) as u64) + magic.offset;
        return ATTACKS[index as usize];
    }

    #[inline]
    pub fn pseudo_rook_moves(&self) -> Bitboard {
        PSEUDO_ROOK[self.to_usize()]
    }

    #[inline]
    pub fn rook_moves(&self, occupied: &Bitboard) -> Bitboard {
        let magic = &Magic::ROOK[self.to_usize()];
        let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as u64 >> (Square::NUM_SQUARES - Magic::ROOK_SHIFT) as u64) + magic.offset;
        return ATTACKS[index as usize];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knight_moves() {
        assert_eq!(Square::A1.knight_moves(), Bitboard::B3.union(&Bitboard::C2));
        assert_eq!(Square::B2.knight_moves(), Bitboard::A4.union(&Bitboard::C4).union(&Bitboard::D3).union(&Bitboard::D1));
    }

    #[test]
    fn king_moves() {
        assert_eq!(Square::A1.king_moves(), Bitboard::A2.union(&Bitboard::B1).union(&Bitboard::B2));
        assert_eq!(Square::B2.king_moves(), Bitboard::A1.union(&Bitboard::A2).union(&Bitboard::A3)
            .union(&Bitboard::B1).union(&Bitboard::B3).union(&Bitboard::C1)
            .union(&Bitboard::C2).union(&Bitboard::C3));
    }

    #[test]
    fn pawn_bitboard_forward() {
        assert_eq!(Bitboard::A2.pawn_forward(&Color::White), Bitboard::A3);
        assert_eq!(Bitboard::A2.pawn_forward(&Color::Black), Bitboard::A1);
    }

    #[test]
    fn pawn_move() {
        assert_eq!(Square::A2.pawn_move(&Color::White), Bitboard::A3);
        assert_eq!(Square::A8.pawn_move(&Color::White), Bitboard::EMPTY);
        assert_eq!(Square::A2.pawn_move(&Color::Black), Bitboard::A1);
        assert_eq!(Square::A1.pawn_move(&Color::Black), Bitboard::EMPTY);
    }

    #[test]
    fn pawn_double_move() {
        assert_eq!(Square::A2.pawn_double_move(&Color::White), Bitboard::A4);
        assert_eq!(Square::A3.pawn_double_move(&Color::White), Bitboard::EMPTY);
        assert_eq!(Square::A2.pawn_double_move(&Color::Black), Bitboard::EMPTY);
        assert_eq!(Square::A7.pawn_double_move(&Color::Black), Bitboard::A5);
    }


    #[test]
    fn neighbour() {
        assert_eq!(Square::A2.neighbour(), Bitboard::B2);
        assert_eq!(Square::D3.neighbour(), Bitboard::C3.union(&Bitboard::E3));
    }

    #[test]
    fn bishop_move() {
        assert_eq!(Square::A1.bishop_moves(&Bitboard::EMPTY), Bitboard::B2.union(&Bitboard::C3)
            .union(&Bitboard::D4).union(&Bitboard::E5).union(&Bitboard::F6).union(&Bitboard::G7)
            .union(&Bitboard::H8));
    }

    #[test]
    fn rook_move() {
        assert_eq!(Square::A1.rook_moves(&Bitboard::EMPTY), Bitboard::A2.union(&Bitboard::A3)
            .union(&Bitboard::A4).union(&Bitboard::A5).union(&Bitboard::A6).union(&Bitboard::A7)
            .union(&Bitboard::A8).union(&Bitboard::B1).union(&Bitboard::C1).union(&Bitboard::D1)
            .union(&Bitboard::E1).union(&Bitboard::F1).union(&Bitboard::G1).union(&Bitboard::H1));
    }
}