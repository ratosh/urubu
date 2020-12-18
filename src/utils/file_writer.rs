use std::fs::File;
use std::io;
use std::io::Write;
use std::iter::Iterator;

use crate::types::bitboard::Bitboard;

pub fn write_bitboard_array(file: &mut File, name: &str, array: &[Bitboard]) -> io::Result<()> {
    write!(file, "const {}: [Bitboard; {}] = [", name, array.len())?;
    for entry in array.iter() {
        write!(file, "  Bitboard({}), ", entry.to_usize())?;
    }
    write!(file, "];\n")?;
    Ok(())
}

pub fn write_2d_bitboard_array(file: &mut File, name: &str, array: &[[Bitboard; 64]]) -> io::Result<()> {
    let inner_size = array[0].len();
    let outer_size = array.len();
    write!(file, "const {}: [[Bitboard; {}];{}] = [", name, inner_size, outer_size)?;
    for entry in array.iter() {
        write!(file, "\n[")?;
        for inner_entry in entry.iter() {
            write!(file, "  Bitboard({}), ", inner_entry.to_usize())?;
        }
        write!(file, "],")?;
    }
    write!(file, "];\n")?;
    Ok(())
}

pub fn write_3d_u64_array(file: &mut File, name: &str, array: &[[[u64; 64]; 7]]) -> io::Result<()> {
    let first_size = array.len();
    let second_size = array[0].len();
    let third_size = array[0][0].len();
    write!(file, "const {}: [[[u64; {}];{}];{}] = [", name, third_size, second_size, first_size)?;
    for entry1 in array.iter() {
        write!(file, "\n[")?;
        for entry2 in entry1.iter() {
            write!(file, "\n[")?;
            for entry3 in entry2.iter() {
                write!(file, "  {}, ", entry3)?;
            }
            write!(file, "],")?;
        }
        write!(file, "],")?;
    }
    write!(file, "];\n")?;
    Ok(())
}


pub fn write_u64_array(file: &mut File, name: &str, array: &[u64]) -> io::Result<()> {
    write!(file, "const {}: [u64; {}] = [", name, array.len())?;
    for entry in array.iter() {
        write!(file, "  {}, ", entry)?;
    }
    write!(file, "];\n")?;
    Ok(())
}


pub fn write_u64(file: &mut File, name: &str, value: &u64) -> io::Result<()> {
    write!(file, "const {}: u64 = {};", name, value)?;
    Ok(())
}