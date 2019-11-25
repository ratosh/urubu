use std::{env, io};
use std::path::Path;

use crate::types::bitboard::Bitboard;
use crate::types::castling_rights::CastlingIndex;
use crate::types::color::Color;
use crate::types::file::File;
use crate::types::rank::Rank;
use crate::types::square::Square;
use crate::utils::file_writer::{write_u64, write_u64_array, write_3d_u64_array};
use crate::utils::random::Random;
use crate::types::piece_type::PieceType;
use crate::types::castling_rights::CastlingRights;

pub fn generate_zobrist_file() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("got OUT_DIR");
    let magic_path = Path::new(&out_dir).join("zobrist_generated.rs");
    let mut file = std::fs::File::create(&magic_path).expect("Created file");

    let mut random = Random::new();

    write_3d_u64_array(&mut file, "PSQT", &init_zobrist_psqt(&mut random))?;
    write_u64_array(&mut file, "EP", &init_zobrist_ep(&mut random))?;
    write_u64_array(&mut file, "CASTLING", &init_zobrist_castling(&mut random))?;
    write_u64(&mut file, "COLOR", &init_zobrist_color(&mut random))?;
    Ok(())
}

fn init_zobrist_psqt(random: &mut Random) -> [[[u64; Square::NUM_SQUARES];PieceType::NUM_PIECE_TYPES];Color::NUM_COLORS] {
    let mut result = [[[0 as u64; Square::NUM_SQUARES];PieceType::NUM_PIECE_TYPES];Color::NUM_COLORS];
    for color in Color::COLORS.iter() {
        for piece_type in PieceType::PIECE_TYPES.iter() {
            if piece_type == &PieceType::NONE {
                continue;
            }
            for sq in Square::SQUARES.iter() {
                result[color.to_usize()][piece_type.to_usize()][sq.to_usize()] = random.next();
            }
        }
    }
    result
}

fn init_zobrist_ep(random: &mut Random) -> [u64; File::NUM_FILES] {
    let mut result: [u64;File::NUM_FILES] = [0; File::NUM_FILES];
    for file in File::FILES.iter() {
        result[file.to_usize()] = random.next();
    }
    result
}

fn init_zobrist_castling(random: &mut Random) -> [u64; CastlingRights::NUM_RIGHTS] {
    let mut result: [u64;CastlingRights::NUM_RIGHTS] = [0; CastlingRights::NUM_RIGHTS];
    // NOTE: No right representation should have no zobrist key change
    for castling_right in 1..CastlingRights::NUM_RIGHTS {
        result[castling_right] = random.next();
    }
    result

}

fn init_zobrist_color(random: &mut Random) -> u64 {
    random.next()
}