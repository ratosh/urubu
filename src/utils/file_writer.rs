use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::iter::Iterator;

use crate::types::bitboard::Bitboard;

pub fn write_array(file: &mut File, name: &str, array: &[Bitboard]) -> io::Result<()> {
    write!(file, "const {}: [Bitboard; {}] = [", name, array.len())?;
    for entry in array.iter() {
        write!(file, "  Bitboard({}), ", entry.0)?;
    }
    write!(file, "];\n")?;
    Ok(())
}


pub fn write_2d_array(file: &mut File, name: &str, array: &[[Bitboard;64];64]) -> io::Result<()> {
    write!(file, "const {}: [[Bitboard; {}];{}] = [[", name, array.len(), array.len())?;
    for (index, entry) in array.iter().enumerate() {
        for inner_entry in entry.iter() {
            write!(file, "  Bitboard({}), ", inner_entry.0)?;
        }
        if index < 63 {
            write!(file, "],[\n")?;
        }
    }
    write!(file, "]];\n")?;
    Ok(())
}
