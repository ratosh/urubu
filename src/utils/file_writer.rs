use std::env;
use std::fmt::LowerHex;
use std::fs::File;
use std::io;
use std::io::Write;
use std::iter::Iterator;
use std::path::Path;

use crate::types::bitboard::Bitboard;

pub fn write_array(file: &mut File, name: &str, array: &[Bitboard]) -> io::Result<()> {
    write!(file, "#[inline]\n")?;
    write!(file, "const {}: [Bitboard; {}] = [", name, array.len())?;
    for entry in array.iter() {
        write!(file, "  Bitboard({}), ", entry.0)?;
    }
    write!(file, "];\n")?;
    Ok(())
}
