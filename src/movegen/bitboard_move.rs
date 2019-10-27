use crate::generator::magic::Magic;
use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::square::Square;

include!(concat!(env!("OUT_DIR"), "/bitboard_move.rs"));

const NORTH: i8 = 8;
const SOUTH: i8 = -8;

const EAST: i8 = 1;
const WEST: i8 = -1;

const PAWN_FORWARD: [i8; Color::NUM_COLORS] = [NORTH, SOUTH];

const PAWN_ATTACK_STEP: [i8; 2] = [
    NORTH + WEST,
    NORTH + EAST];

#[inline]
pub fn pawn_forward(bitboard: &Bitboard, color: &Color) -> Bitboard {
    Bitboard(match color {
        &Color::WHITE => bitboard.0 << NORTH as u64,
        _ => bitboard.0 >> NORTH as u64,
    })
}

#[inline]
pub fn pawn_attacks_left(bitboard: &Bitboard, color: &Color) -> Bitboard {
    Bitboard(match color {
        &Color::WHITE => bitboard.0 << PAWN_ATTACK_STEP[0] as u64,
        _ => bitboard.0 >> PAWN_ATTACK_STEP[0] as u64,
    })
}

#[inline]
pub fn pawn_attacks_right(bitboard: &Bitboard, color: &Color) -> Bitboard {
    Bitboard(match color {
        &Color::WHITE => bitboard.0 << PAWN_ATTACK_STEP[1] as u64,
        _ => bitboard.0 >> PAWN_ATTACK_STEP[1] as u64,
    })
}

#[inline]
pub fn knight_moves(square: &Square) -> Bitboard {
    return KNIGHT_MOVES[square.to_usize()];
}

#[inline]
pub fn king_moves(square: &Square) -> Bitboard {
    return KING_MOVES[square.to_usize()];
}

#[inline]
pub fn bishop_moves(square: &Square, occupied: &Bitboard) -> Bitboard {
    let magic = &Magic::BISHOP[square.to_usize()];
    let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as u64 >> (Square::NUM_SQUARES - Magic::BISHOP_SHIFT) as u64) + magic.offset;
    return ATTACKS[index as usize];
}

#[inline]
pub fn rook_moves(square: &Square, occupied: &Bitboard) -> Bitboard {
    let magic = &Magic::ROOK[square.to_usize()];
    let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as u64 >> (Square::NUM_SQUARES - Magic::ROOK_SHIFT) as u64) + magic.offset;
    return ATTACKS[index as usize];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pawn_forward_test() {
        assert_eq!(pawn_forward(&Bitboard::A2, &Color::WHITE), Bitboard::A3);
        assert_eq!(pawn_forward(&Bitboard::A2, &Color::BLACK), Bitboard::A1);
    }

    #[test]
    fn bishop_move_test() {
        assert_eq!(bishop_moves(&Square::A1, &Bitboard::EMPTY), Bitboard::B2.union(&Bitboard::C3)
            .union(&Bitboard::D4).union(&Bitboard::E5).union(&Bitboard::F6).union(&Bitboard::G7)
            .union(&Bitboard::H8));
    }

    #[test]
    fn rook_move_test() {
        assert_eq!(bishop_moves(&Square::A1, &Bitboard::EMPTY), Bitboard::A2.union(&Bitboard::A3)
            .union(&Bitboard::A4).union(&Bitboard::A5).union(&Bitboard::A6).union(&Bitboard::A7)
            .union(&Bitboard::A8).union(&Bitboard::B1).union(&Bitboard::C1).union(&Bitboard::D1)
            .union(&Bitboard::E1).union(&Bitboard::F1).union(&Bitboard::G1).union(&Bitboard::H1));
    }
}