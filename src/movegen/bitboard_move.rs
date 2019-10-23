use crate::movegen::magic::Magic;
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
pub fn bishop_moves(square: &Square, occupied: &Bitboard) -> Bitboard {
    let magic = &Magic::BISHOP[square.to_usize()];
    let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as u64 >> (Square::NUM_SQUARES - Magic::BISHOP_SHIFT) as u64) + magic.offset;
    println!("index {}", index);
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
}