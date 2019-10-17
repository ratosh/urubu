use crate::movegen::magic::Magic;
use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::square::Square;

const NORTH: i8 = 8;
const SOUTH: i8 = -8;

const EAST: i8 = 1;
const WEST: i8 = -1;

const PAWN_FORWARD: [i8; Color::NUM_COLORS] = [NORTH, SOUTH];
const PAWN_ATTACK_STEP: [i8; 2] = [
    NORTH + WEST,
    NORTH + EAST];

const KNIGHT_MOVE_STEPS: [i8; 8] = [
    NORTH * 2 + EAST,
    NORTH * 2 + WEST,
    NORTH + 2 * EAST,
    NORTH + 2 * WEST,
    SOUTH + 2 * EAST,
    SOUTH + 2 * WEST,
    SOUTH * 2 + WEST,
    SOUTH * 2 + EAST];
const BISHOP_MOVE_STEPS: [i8; 4] = [
    NORTH + EAST,
    NORTH + WEST,
    SOUTH + EAST,
    SOUTH + WEST];
const ROOK_MOVE_STEPS: [i8; 4] = [
    NORTH,
    EAST,
    WEST,
    SOUTH];
const KING_MOVE_STEPS: [i8; 8] = [
    NORTH + EAST,
    NORTH,
    NORTH + WEST,
    EAST, WEST,
    SOUTH + EAST,
    SOUTH,
    SOUTH + WEST];

lazy_static! {
    pub static ref KNIGHT_MOVES: [Bitboard; Square::NUM_SQUARES] = init_knight_attacks();
    pub static ref KING_MOVES: [Bitboard; Square::NUM_SQUARES] = init_king_attacks();
    pub static ref MAGIC_ATTACKS: [Bitboard; Magic::SIZE] = init_magic();
//    pub static ref BISHOP_PSEUDO_MOVES: [Bitboard; Square::NUM_SQUARES] = bishop_pseudo_attacks();
//    pub static ref ROOK_PSEUDO_MOVES: [Bitboard; Square::NUM_SQUARES] = rook_pseudo_attacks();
}

fn init_knight_attacks() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for sq in Square::SQUARES.iter() {
        res[sq.to_usize()] = slide_moves(sq, &KNIGHT_MOVE_STEPS, &Bitboard::ALL);
    }
    res
}

fn init_king_attacks() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(square, &KING_MOVE_STEPS, &Bitboard::ALL);
    }
    res
}

fn init_magic() -> [Bitboard; Magic::SIZE] {
    println!("Init magic");
    let mut result = [Bitboard::EMPTY; Magic::SIZE];
//    for square in Square::SQUARES.iter() {
//        get_magic(square, &BISHOP_MOVE_STEPS, &Magic::BISHOP[square.to_usize()], Magic::BISHOP_SHIFT, &mut result);
//        get_magic(square, &ROOK_MOVE_STEPS, &Magic::ROOK[square.to_usize()], Magic::ROOK_SHIFT, &mut result);
//    }
    result
}

fn bishop_pseudo_attacks() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(square, &BISHOP_MOVE_STEPS, &Bitboard::EMPTY);
    }
    res
}

fn rook_pseudo_attacks() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(square, &ROOK_MOVE_STEPS, &Bitboard::EMPTY);
    }
    res
}

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
pub fn get_magic(square: &Square, move_steps: &[i8], magic: &Magic, shift: usize, attacks: &mut [Bitboard]) {
    let mut mutable_subset = Bitboard::EMPTY;
    let mut over = false;
//    for square in Bitboard(magic.mask).iterator() {
//        let attack = slide_moves(&square, move_steps, &mutable_subset);
//        let idx = (magic.factor.wrapping_mul(mutable_subset.0) >> (Square::NUM_SQUARES - shift) as u64) + magic.offset;
//        attacks[idx as usize] = attack;
//        mutable_subset = Bitboard((mutable_subset.0.wrapping_sub(magic.mask)) & magic.mask);
//        over = mutable_subset == Bitboard::EMPTY;
//    }
}

pub fn slide_moves(square: &Square, slide_values: &[i8], limit: &Bitboard) -> Bitboard {
    let mut result = Bitboard::EMPTY;
    for slide in slide_values {
        let moves = slide_move(square, slide, limit);
        result.add(&moves);
    }
    return result;
}

pub fn slide_move(square: &Square, slide_value: &i8, limit: &Bitboard) -> Bitboard {
    let mut result = Bitboard::EMPTY;
    let mut old_square = *square;
    while let Some(new_square) = old_square.offset(slide_value) {
        if old_square.square_dist(&new_square) > 2 {
            break;
        }

        result.add_square(&new_square);

        if limit.is_set(&new_square) {
            break;
        }
        old_square = new_square;
    }
    return result;
}

#[inline]
pub fn knight_moves(square: &Square) -> Bitboard {
    KNIGHT_MOVES[square.to_usize()]
}

#[inline]
pub fn bishop_moves(square: &Square, occupied: &Bitboard) -> Bitboard {
    println!("magic {}", std::mem::size_of::<Magic>());
    let magic = &Magic::BISHOP[square.to_usize()];
//    let index = 0;
    let index = ((magic.factor.wrapping_mul(occupied.0 & magic.mask)) as u64 >> (Square::NUM_SQUARES - Magic::BISHOP_SHIFT) as u64) + magic.offset;
    println!("index {}", index);
    return MAGIC_ATTACKS[index as usize];
//    return Bitboard::EMPTY;
}

#[inline]
pub fn king_moves(square: &Square) -> Bitboard {
    KING_MOVES[square.to_usize()]
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
    fn knight_moves_test() {
        assert_eq!(knight_moves(&Square::A1), Bitboard::B3.merge(&Bitboard::C2));
        assert_eq!(knight_moves(&Square::B2), Bitboard::A4.merge(&Bitboard::C4).merge(&Bitboard::D3).merge(&Bitboard::D1));
    }

    #[test]
    fn bishop_moves_test() {
        assert_eq!(bishop_moves(&Square::A1, &Bitboard::EMPTY),
                   Bitboard::B2.merge(&Bitboard::C3).merge(&Bitboard::D4)
                       .merge(&Bitboard::E5).merge(&Bitboard::F6).merge(&Bitboard::G7)
                       .merge(&Bitboard::H8));
    }

    #[test]
    fn fucking_test() {
        let mut result = [Bitboard::EMPTY; Magic::SIZE];
    }

//    #[test]
//    fn king_moves() {
//        assert_eq!(Square::A1.king_moves(), Bitboard::A2.merge(&Bitboard::B1).merge(&Bitboard::B2));
//        assert_eq!(Square::B2.king_moves(), Bitboard::A1.merge(&Bitboard::A2).merge(&Bitboard::A3)
//            .merge(&Bitboard::B1).merge(&Bitboard::B3).merge(&Bitboard::C1)
//            .merge(&Bitboard::C2).merge(&Bitboard::C3));
//    }
}