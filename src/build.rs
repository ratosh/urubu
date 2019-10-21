pub mod movegen;
pub mod types;
pub mod utils;

use std::io;

use crate::movegen::generator::generate_movegen_file;

fn main() -> io::Result<()> {
    generate_movegen_file()?;
    Ok(())
}
