use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::square::Square;
use crate::types::magic::Magic;

include!(concat!(env!("OUT_DIR"), "/bitboard_generated.rs"));

const NORTH: i8 = 8;
const SOUTH: i8 = -8;

const EAST: i8 = 1;
const WEST: i8 = -1;

const PAWN_ATTACK_STEP: [i8; 2] = [
    NORTH + WEST,
    NORTH + EAST];

#[inline]
pub fn pawn_forward(color: &Color, bitboard: &Bitboard) -> Bitboard {
    Bitboard(match color {
        &Color::WHITE => bitboard.0 << NORTH as u64,
        _ => bitboard.0 >> NORTH as u64,
    })
}

#[inline]
pub fn pawn_attacks_left(color: &Color, bitboard: &Bitboard) -> Bitboard {
    Bitboard(match color {
        &Color::WHITE => bitboard.0 << PAWN_ATTACK_STEP[0] as u64,
        _ => bitboard.0 >> PAWN_ATTACK_STEP[0] as u64,
    })
}

#[inline]
pub fn pawn_attacks_right(color: &Color, bitboard: &Bitboard) -> Bitboard {
    Bitboard(match color {
        &Color::WHITE => bitboard.0 << PAWN_ATTACK_STEP[1] as u64,
        _ => bitboard.0 >> PAWN_ATTACK_STEP[1] as u64,
    })
}

#[inline]
pub fn pawn_attacks(color: &Color, square: &Square) -> Bitboard {
    return PAWN_ATTACKS[color.to_usize()][square.to_usize()]
}

#[inline]
pub fn pawn_move(color: &Color, square: &Square) -> Bitboard {
    return PAWN_MOVES[color.to_usize()][square.to_usize()]
}

#[inline]
pub fn pawn_double_move(color: &Color, square: &Square) -> Bitboard {
    return PAWN_DOUBLE_MOVES[color.to_usize()][square.to_usize()]
}

#[inline]
pub fn between(square1: &Square, square2: &Square) -> Bitboard {
    return BETWEEN[square1.to_usize()][square2.to_usize()]
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
    fn knight_moves_test() {
        assert_eq!(KNIGHT_MOVES[Square::A1.to_usize()], Bitboard::B3.add(&Bitboard::C2));
        assert_eq!(KNIGHT_MOVES[Square::B2.to_usize()], Bitboard::A4.add(&Bitboard::C4).add(&Bitboard::D3).add(&Bitboard::D1));
    }

    #[test]
    fn king_moves_test() {
        assert_eq!(KING_MOVES[Square::A1.to_usize()], Bitboard::A2.add(&Bitboard::B1).add(&Bitboard::B2));
        assert_eq!(KING_MOVES[Square::B2.to_usize()], Bitboard::A1.add(&Bitboard::A2).add(&Bitboard::A3)
            .add(&Bitboard::B1).add(&Bitboard::B3).add(&Bitboard::C1)
            .add(&Bitboard::C2).add(&Bitboard::C3));
    }

    #[test]
    fn pawn_bitboard_forward_test() {
        assert_eq!(pawn_forward(&Color::WHITE, &Bitboard::A2), Bitboard::A3);
        assert_eq!(pawn_forward(&Color::BLACK, &Bitboard::A2), Bitboard::A1);
    }

    #[test]
    fn pawn_move_test() {
        assert_eq!(PAWN_MOVES[Color::WHITE.to_usize()][Square::A2.to_usize()], Bitboard::A3);
        assert_eq!(PAWN_MOVES[Color::WHITE.to_usize()][Square::A8.to_usize()], Bitboard::EMPTY);
        assert_eq!(PAWN_MOVES[Color::BLACK.to_usize()][Square::A2.to_usize()], Bitboard::A1);
        assert_eq!(PAWN_MOVES[Color::BLACK.to_usize()][Square::A1.to_usize()], Bitboard::EMPTY);
    }

    #[test]
    fn pawn_double_move_test() {
        assert_eq!(PAWN_DOUBLE_MOVES[Color::WHITE.to_usize()][Square::A2.to_usize()], Bitboard::A4);
        assert_eq!(PAWN_DOUBLE_MOVES[Color::WHITE.to_usize()][Square::A3.to_usize()], Bitboard::EMPTY);
        assert_eq!(PAWN_DOUBLE_MOVES[Color::BLACK.to_usize()][Square::A2.to_usize()], Bitboard::EMPTY);
        assert_eq!(PAWN_DOUBLE_MOVES[Color::BLACK.to_usize()][Square::A7.to_usize()], Bitboard::A5);
    }

    #[test]
    fn bishop_move_test() {
        assert_eq!(bishop_moves(&Square::A1, &Bitboard::EMPTY), Bitboard::B2.add(&Bitboard::C3)
            .add(&Bitboard::D4).add(&Bitboard::E5).add(&Bitboard::F6).add(&Bitboard::G7)
            .add(&Bitboard::H8));
    }

    #[test]
    fn rook_move_test() {
        assert_eq!(bishop_moves(&Square::A1, &Bitboard::EMPTY), Bitboard::A2.add(&Bitboard::A3)
            .add(&Bitboard::A4).add(&Bitboard::A5).add(&Bitboard::A6).add(&Bitboard::A7)
            .add(&Bitboard::A8).add(&Bitboard::B1).add(&Bitboard::C1).add(&Bitboard::D1)
            .add(&Bitboard::E1).add(&Bitboard::F1).add(&Bitboard::G1).add(&Bitboard::H1));
    }
}