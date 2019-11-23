pub mod generator;
pub mod types;
pub mod utils;

use std::io;

use crate::generator::bitboard_generator::generate_bitboard_file;
use crate::generator::zobrist_generator::generate_zobrist_file;

fn main() -> io::Result<()> {
    generate_bitboard_file()?;
    generate_zobrist_file()?;
    Ok(())
}
