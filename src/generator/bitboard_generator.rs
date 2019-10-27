use std::{env, io};
use std::fs::File;
use std::path::Path;

use crate::generator::magic::Magic;
use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::square::Square;
use crate::utils::file_writer::{write_2d_array, write_array};

const PAWN_FORWARD: [i8; Color::NUM_COLORS] = [NORTH, SOUTH];

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

pub fn generate_bitboard_file() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("got OUT_DIR");
    let magic_path = Path::new(&out_dir).join("bitboard_move.rs");
    let mut file = File::create(&magic_path).expect("Created file");

    write_array(&mut file, "KNIGHT_MOVES", &init_knight_moves())?;
    write_array(&mut file, "KING_MOVES", &init_king_moves())?;
    write_array(&mut file, "ATTACKS", &init_magic())?;
    write_2d_array(&mut file, "BETWEEN", &init_between())?;
    write_2d_array(&mut file, "PAWN_MOVES", &init_pawn_moves())?;
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

fn init_between() -> [[Bitboard; Square::NUM_SQUARES]; Square::NUM_SQUARES] {
    let mut result = [[Bitboard::EMPTY; Square::NUM_SQUARES]; Square::NUM_SQUARES];
    let direction_array: [[i8; 1]; 4] = [[7], [9], [1], [8]];
    let border_array = [Bitboard::FILE_A.union(&Bitboard::RANK_8),
        Bitboard::FILE_H.union(&Bitboard::RANK_8),
        Bitboard::FILE_H,
        Bitboard::RANK_8];
    for start_square in Square::SQUARES.iter() {
        for (index, direction) in direction_array.iter().enumerate() {
            let border = border_array[index];
            let mut moving_square: Option<Square> = Some(*start_square);
            let mut bitboard = Bitboard::from_square(&start_square);
            while bitboard.intersect(&border) == Bitboard::EMPTY {
                moving_square = moving_square.unwrap().offset(&direction[0 as usize]);
                if moving_square.is_none() {
                    break;
                }
                let final_square = moving_square.unwrap();
                bitboard = Bitboard::from_square(&final_square);
                let between = slide_moves(start_square, direction, &bitboard.union(&border)).not(&bitboard);
                result[start_square.to_usize()][final_square.to_usize()] = between;
                result[final_square.to_usize()][start_square.to_usize()] = between;
            }
        }
    }
    result
}

fn init_pawn_moves() -> [[Bitboard; Square::NUM_SQUARES]; Color::NUM_COLORS] {
    let mut result = [[Bitboard::EMPTY; Square::NUM_SQUARES]; Color::NUM_COLORS];
    for square in Square::SQUARES.iter() {
        result[Color::WHITE.to_usize()][square.to_usize()] = init_pawn_move(&square, &Color::WHITE);
        result[Color::BLACK.to_usize()][square.to_usize()] = init_pawn_move(&square, &Color::BLACK);
    }
    return result;
}

fn init_pawn_move(square: &Square, color: &Color) -> Bitboard {
    let forward = square.offset(&PAWN_FORWARD[color.to_usize()]);
    if forward != None {
        return Bitboard::from_square(&forward.unwrap());
    }
    return Bitboard::EMPTY;
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