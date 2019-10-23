use std::{env, io};
use std::fs::File;
use std::path::Path;

use crate::movegen::magic::Magic;
use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::square::Square;
use crate::utils::file_writer::write_array;

const NORTH: i8 = 8;
const SOUTH: i8 = -8;

const EAST: i8 = 1;
const WEST: i8 = -1;

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

pub fn generate_movegen_file() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("got OUT_DIR");;
    let magic_path = Path::new(&out_dir).join("bitboard_move.rs");
    let mut file = File::create(&magic_path).expect("Created file");

    write_array(&mut file, "KNIGHT_MOVES", &init_knight_moves())?;
    write_array(&mut file, "KING_MOVES", &init_king_moves())?;
    write_array(&mut file, "ATTACKS", &init_magic())?;
    Ok(())
}

fn init_knight_moves() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for sq in Square::SQUARES.iter() {
        res[sq.to_usize()] = slide_moves(sq, &KNIGHT_MOVE_STEPS, &Bitboard::ALL);
    }
    res
}

fn init_king_moves() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(square, &KING_MOVE_STEPS, &Bitboard::ALL);
    }
    res
}

fn init_magic() -> [Bitboard; Magic::SIZE] {
    let mut result = [Bitboard::EMPTY; Magic::SIZE];
    dbg!("init");
    for square in Square::SQUARES.iter() {
        get_magic(square, &BISHOP_MOVE_STEPS, &Magic::BISHOP[square.to_usize()], Magic::BISHOP_SHIFT, &mut result);
        get_magic(square, &ROOK_MOVE_STEPS, &Magic::ROOK[square.to_usize()], Magic::ROOK_SHIFT, &mut result);
    }
    result
}

#[inline]
pub fn get_magic(square: &Square, move_steps: &[i8], magic: &Magic, shift: usize, attacks: &mut [Bitboard]) {
    let mut mutable_subset = Bitboard::EMPTY;
    let mut over = false;
    while !over {
        let attack = slide_moves(square, move_steps, &mutable_subset);
        let idx = ((magic.factor.wrapping_mul(mutable_subset.0) >> (Square::NUM_SQUARES - shift) as u64) + magic.offset) as usize;
        attacks[idx] = attack;
        mutable_subset = Bitboard((mutable_subset.0.wrapping_sub(magic.mask)) & magic.mask);
        over = mutable_subset == Bitboard::EMPTY;
    }
}

fn slide_moves(square: &Square, slide_values: &[i8], limit: &Bitboard) -> Bitboard {
    let mut result = Bitboard::EMPTY;
    for slide in slide_values {
        let moves = slide_move(square, slide, limit);
        result.add(&moves);
    }
    return result;
}

fn slide_move(square: &Square, slide_value: &i8, limit: &Bitboard) -> Bitboard {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knight_moves_test() {
        let knight_moves = init_knight_moves();
        assert_eq!(knight_moves[Square::A1.to_usize()], Bitboard::B3.merge(&Bitboard::C2));
        assert_eq!(knight_moves[Square::B2.to_usize()], Bitboard::A4.merge(&Bitboard::C4).merge(&Bitboard::D3).merge(&Bitboard::D1));
    }

    #[test]
    fn king_moves_test() {
        let king_moves = init_king_moves();
        assert_eq!(king_moves[Square::A1.to_usize()], Bitboard::A2.merge(&Bitboard::B1).merge(&Bitboard::B2));
        assert_eq!(king_moves[Square::B2.to_usize()], Bitboard::A1.merge(&Bitboard::A2).merge(&Bitboard::A3)
            .merge(&Bitboard::B1).merge(&Bitboard::B3).merge(&Bitboard::C1)
            .merge(&Bitboard::C2).merge(&Bitboard::C3));
    }

    #[test]
    fn init_magic_test() {
        let attacks = init_magic();
        for (index, entry) in attacks.iter().enumerate() {
            if *entry == Bitboard::EMPTY {
                println!("{}", index);
            }
        }
    }
}