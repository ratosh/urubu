use std::fs::File;
use std::path::Path;
use std::{env, io};

use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::file;
use crate::types::magic::Magic;
use crate::types::rank::Rank;
use crate::types::square::Square;
use crate::utils::file_writer::{write_2d_bitboard_array, write_bitboard_array};

const PAWN_FORWARD: [i8; Color::NUM_COLORS] = [NORTH, SOUTH];
const PAWN_ATTACK_LEFT: [i8; Color::NUM_COLORS] = [NORTH + WEST, SOUTH + WEST];
const PAWN_ATTACK_RIGHT: [i8; Color::NUM_COLORS] = [NORTH + EAST, SOUTH + EAST];

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
    SOUTH * 2 + EAST,
];

const BISHOP_MOVE_STEPS: [i8; 4] = [NORTH + EAST, NORTH + WEST, SOUTH + EAST, SOUTH + WEST];

const ROOK_MOVE_STEPS: [i8; 4] = [NORTH, EAST, WEST, SOUTH];

const KING_MOVE_STEPS: [i8; 8] = [
    NORTH + EAST,
    NORTH,
    NORTH + WEST,
    EAST,
    WEST,
    SOUTH + EAST,
    SOUTH,
    SOUTH + WEST,
];

pub fn generate_bitboard_file() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("got OUT_DIR");
    let magic_path = Path::new(&out_dir).join("bitboard_generated.rs");
    let mut file = File::create(&magic_path).expect("Created file");

    write_bitboard_array(&mut file, "KNIGHT_MOVES", &init_knight_moves())?;
    write_bitboard_array(&mut file, "KING_MOVES", &init_king_moves())?;
    write_bitboard_array(&mut file, "PSEUDO_BISHOP", &init_pseudo_bishop())?;
    write_bitboard_array(&mut file, "PSEUDO_ROOK", &init_pseudo_rook())?;
    write_bitboard_array(&mut file, "ATTACKS", &init_magic())?;
    write_bitboard_array(&mut file, "NEIGHBOUR", &init_neighbour())?;
    write_2d_bitboard_array(&mut file, "BETWEEN", &init_between())?;
    write_2d_bitboard_array(&mut file, "PAWN_ATTACKS", &init_pawn_attacks())?;
    write_2d_bitboard_array(&mut file, "PAWN_MOVES", &init_pawn_moves())?;
    write_2d_bitboard_array(&mut file, "PAWN_DOUBLE_MOVES", &init_pawn_double_moves())?;
    write_2d_bitboard_array(&mut file, "PINNED_MASK", &init_pinned_mask())?;
    Ok(())
}

#[inline]
fn init_knight_moves() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(*square, &KNIGHT_MOVE_STEPS, Bitboard::ALL);
    }
    res
}

#[inline]
fn init_king_moves() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(*square, &KING_MOVE_STEPS, Bitboard::ALL);
    }
    res
}

#[inline]
fn init_pseudo_bishop() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(*square, &BISHOP_MOVE_STEPS, Bitboard::EMPTY);
    }
    res
}

#[inline]
fn init_pseudo_rook() -> [Bitboard; Square::NUM_SQUARES] {
    let mut res = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        res[square.to_usize()] = slide_moves(*square, &ROOK_MOVE_STEPS, Bitboard::EMPTY);
    }
    res
}

#[inline]
fn init_magic() -> [Bitboard; Magic::SIZE] {
    let mut result = [Bitboard::EMPTY; Magic::SIZE];
    for square in Square::SQUARES.iter() {
        get_magic(
            *square,
            &BISHOP_MOVE_STEPS,
            &Magic::BISHOP[square.to_usize()],
            Magic::BISHOP_SHIFT,
            &mut result,
        );
        get_magic(
            *square,
            &ROOK_MOVE_STEPS,
            &Magic::ROOK[square.to_usize()],
            Magic::ROOK_SHIFT,
            &mut result,
        );
    }
    result
}

#[inline]
fn init_neighbour() -> [Bitboard; Square::NUM_SQUARES] {
    let mut result = [Bitboard::EMPTY; Square::NUM_SQUARES];
    for square in Square::SQUARES.iter() {
        let file = square.to_file();

        let bitboard_bounds = match file {
            crate::types::file::File::FILE_H => Bitboard::FILE_A.reverse(),
            crate::types::file::File::FILE_A => Bitboard::FILE_H.reverse(),
            _ => Bitboard::ALL,
        };

        let mut possible_neighbours = Bitboard::EMPTY;
        let west_square = square.offset(WEST);
        if let Some(square) = west_square {
            possible_neighbours = Bitboard::from(square);
        }
        let east_square = square.offset(EAST);
        if let Some(square) = east_square {
            possible_neighbours = possible_neighbours.union(Bitboard::from(square));
        }
        result[square.to_usize()] = bitboard_bounds.intersect(possible_neighbours);
    }
    result
}

#[inline]
pub fn get_magic(
    square: Square,
    move_steps: &[i8],
    magic: &Magic,
    shift: usize,
    attacks: &mut [Bitboard],
) {
    let mut mutable_subset = Bitboard::EMPTY;
    let mut over = false;
    while !over {
        let attack = slide_moves(square, move_steps, mutable_subset);
        let idx = ((magic.factor.wrapping_mul(mutable_subset.to_u64())
            >> (Square::NUM_SQUARES - shift) as u64)
            + magic.offset) as usize;
        attacks[idx] = attack;
        mutable_subset =
            Bitboard::new((mutable_subset.to_u64().wrapping_sub(magic.mask)) & magic.mask);
        over = mutable_subset == Bitboard::EMPTY;
    }
}

fn init_between() -> [[Bitboard; Square::NUM_SQUARES]; Square::NUM_SQUARES] {
    let mut result = [[Bitboard::EMPTY; Square::NUM_SQUARES]; Square::NUM_SQUARES];
    let direction_array: [[i8; 1]; 4] = [[7], [9], [1], [8]];
    let border_array = [
        Bitboard::FILE_A.union(Bitboard::RANK_8),
        Bitboard::FILE_H.union(Bitboard::RANK_8),
        Bitboard::FILE_H,
        Bitboard::RANK_8,
    ];
    for start_square in Square::SQUARES.iter() {
        for (index, direction) in direction_array.iter().enumerate() {
            let border = border_array[index];
            let mut moving_square: Option<Square> = Some(*start_square);
            let mut bitboard = Bitboard::from(*start_square);
            while bitboard.intersect(border) == Bitboard::EMPTY {
                moving_square = moving_square.unwrap().offset(direction[0 as usize]);
                if moving_square.is_none() {
                    break;
                }
                let final_square = moving_square.unwrap();
                bitboard = Bitboard::from(final_square);
                let between = slide_moves(*start_square, direction, bitboard.union(border))
                    .difference(bitboard);
                result[start_square.to_usize()][final_square.to_usize()] = between;
                result[final_square.to_usize()][start_square.to_usize()] = between;
            }
        }
    }
    result
}

fn slide_moves(square: Square, slide_values: &[i8], limit: Bitboard) -> Bitboard {
    let mut result = Bitboard::EMPTY;
    for slide in slide_values {
        let moves = slide_move(square, *slide, limit);
        result = result.union(moves);
    }
    return result;
}

fn slide_move(square: Square, slide_value: i8, limit: Bitboard) -> Bitboard {
    let mut result = Bitboard::EMPTY;
    let mut old_square = square;
    while let Some(new_square) = old_square.offset(slide_value) {
        if old_square.square_dist(new_square) > 2 {
            break;
        }

        result = result.with_square(new_square);

        if limit.is_set(new_square) {
            break;
        }
        old_square = new_square;
    }
    return result;
}

fn init_pawn_attacks() -> [[Bitboard; Square::NUM_SQUARES]; Color::NUM_COLORS] {
    let mut result = [[Bitboard::EMPTY; Square::NUM_SQUARES]; Color::NUM_COLORS];
    for square in Square::SQUARES.iter() {
        result[Color::White.to_usize()][square.to_usize()] =
            init_pawn_attack(*square, Color::White);
        result[Color::Black.to_usize()][square.to_usize()] =
            init_pawn_attack(*square, Color::Black);
    }
    return result;
}

fn init_pawn_attack(square: Square, color: Color) -> Bitboard {
    let mut result = Bitboard::EMPTY;
    if square.to_file() != file::File::FILE_A {
        if let Some(final_square) = square.offset(PAWN_ATTACK_LEFT[color.to_usize()]) {
            result = result.union(Bitboard::from(final_square));
        }
    }
    if square.to_file() != file::File::FILE_H {
        if let Some(final_square) = square.offset(PAWN_ATTACK_RIGHT[color.to_usize()]) {
            result = result.union(Bitboard::from(final_square));
        }
    }
    return result;
}

fn init_pawn_moves() -> [[Bitboard; Square::NUM_SQUARES]; Color::NUM_COLORS] {
    let mut result = [[Bitboard::EMPTY; Square::NUM_SQUARES]; Color::NUM_COLORS];
    for square in Square::SQUARES.iter() {
        result[Color::White.to_usize()][square.to_usize()] = init_pawn_move(*square, Color::White);
        result[Color::Black.to_usize()][square.to_usize()] = init_pawn_move(*square, Color::Black);
    }
    return result;
}

fn init_pawn_move(square: Square, color: Color) -> Bitboard {
    let forward = square.offset(PAWN_FORWARD[color.to_usize()]);
    if forward != None {
        return Bitboard::from(forward.unwrap());
    }
    return Bitboard::EMPTY;
}

fn init_pawn_double_moves() -> [[Bitboard; Square::NUM_SQUARES]; Color::NUM_COLORS] {
    let mut result = [[Bitboard::EMPTY; Square::NUM_SQUARES]; Color::NUM_COLORS];
    for square in Square::SQUARES.iter() {
        result[Color::White.to_usize()][square.to_usize()] =
            init_pawn_double_move(*square, Color::White);
        result[Color::Black.to_usize()][square.to_usize()] =
            init_pawn_double_move(*square, Color::Black);
    }
    return result;
}

fn init_pawn_double_move(square: Square, color: Color) -> Bitboard {
    if square.to_rank().relative(color) == Rank::RANK_2 {
        let forward = square.offset(PAWN_FORWARD[color.to_usize()] * 2);
        if forward != None {
            return Bitboard::from(forward.unwrap());
        }
    }
    return Bitboard::EMPTY;
}

fn init_pinned_mask() -> [[Bitboard; Square::NUM_SQUARES]; Square::NUM_SQUARES] {
    let mut result = [[Bitboard::EMPTY; Square::NUM_SQUARES]; Square::NUM_SQUARES];
    let direction_array: [[i8; 1]; 8] = [[7], [9], [1], [8], [-7], [-9], [-1], [-8]];
    let border_array = [
        Bitboard::FILE_A.union(Bitboard::RANK_8),
        Bitboard::FILE_H.union(Bitboard::RANK_8),
        Bitboard::FILE_H,
        Bitboard::RANK_8,
        Bitboard::FILE_H.union(Bitboard::RANK_1),
        Bitboard::FILE_A.union(Bitboard::RANK_1),
        Bitboard::FILE_A,
        Bitboard::RANK_1,
    ];
    for start_square in Square::SQUARES.iter() {
        for (index, direction) in direction_array.iter().enumerate() {
            let border = border_array[index];
            let mut moving_square: Option<Square> = Some(*start_square);
            let mut bitboard = Bitboard::EMPTY;
            let pinned_mask = slide_moves(*start_square, direction, border);
            if pinned_mask.is_not_empty() {
                while bitboard.intersect(border).is_empty() {
                    moving_square = moving_square.unwrap().offset(direction[0 as usize]);
                    if moving_square.is_none() {
                        break;
                    }
                    let final_square = moving_square.unwrap();
                    result[start_square.to_usize()][final_square.to_usize()] = pinned_mask;
                    bitboard = Bitboard::from(final_square);
                }
            }
        }
    }
    result
}
