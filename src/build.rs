pub mod generator;
pub mod magic;
pub mod types;
pub mod utils;

use std::io;

use crate::generator::generate_movegen_file;

fn main() -> io::Result<()> {
    generate_movegen_file()?;
    Ok(())
}
