use std::{env, io};
use std::fs::File;
use std::path::Path;

use crate::generator::magic::Magic;
use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::rank::Rank;
use crate::types::square::Square;
use crate::utils::file_writer::{write_2d_array, write_array};

pub fn generate_bitboard_file() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("got OUT_DIR");
    let magic_path = Path::new(&out_dir).join("zobrist_generated.rs");
    let mut file = File::create(&magic_path).expect("Created file");

    write_array(&mut file, "KNIGHT_MOVES", &init_knight_moves())?;
    Ok(())
}