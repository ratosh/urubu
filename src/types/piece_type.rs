use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct PieceType(pub u8);

impl PieceType {
    pub const NUM_PIECE_TYPES: usize = 7;

    pub const PIECE_TYPES: [PieceType; PieceType::NUM_PIECE_TYPES] = [
        PieceType::NONE,
        PieceType::PAWN,
        PieceType::KNIGHT,
        PieceType::BISHOP,
        PieceType::ROOK,
        PieceType::QUEEN,
        PieceType::KING,
    ];
    pub const REPRESENTATION: [char; PieceType::NUM_PIECE_TYPES] =
        ['-', 'p', 'n', 'b', 'r', 'q', 'k'];

    pub const NONE: PieceType = PieceType(0);
    pub const PAWN: PieceType = PieceType(1);
    pub const KNIGHT: PieceType = PieceType(2);
    pub const BISHOP: PieceType = PieceType(3);
    pub const ROOK: PieceType = PieceType(4);
    pub const QUEEN: PieceType = PieceType(5);
    pub const KING: PieceType = PieceType(6);

    #[inline]
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_char(self) -> char {
        PieceType::REPRESENTATION[self.to_usize()]
    }

    #[inline]
    pub fn to_char_colored(self, color: Color) -> char {
        if color == Color::White {
            self.to_char().to_ascii_uppercase()
        } else {
            self.to_char()
        }
    }

    #[inline]
    pub fn from_char(c: char) -> (Color, PieceType) {
        let lower_c = c.to_lowercase().nth(0).unwrap();
        if let Some(index) = PieceType::REPRESENTATION.iter().position(|&s| s == lower_c) {
            (PieceType::get_color(c), PieceType(index as u8))
        } else {
            (Color::White, PieceType::NONE)
        }
    }

    #[inline]
    fn get_color(c: char) -> Color {
        if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl Index<PieceType> for [Bitboard] {
    type Output = Bitboard;

    fn index(&self, index: PieceType) -> &Self::Output {
        unsafe {
            self.get_unchecked(index.to_usize())
        }
    }
}

impl IndexMut<PieceType> for [Bitboard] {
    fn index_mut(&mut self, index: PieceType) -> &mut Bitboard {
        unsafe {
            self.get_unchecked_mut(index.to_usize())
        }
    }
}

impl Index<PieceType> for [i16] {
    type Output = i16;

    fn index(&self, index: PieceType) -> &Self::Output {
        unsafe {
            self.get_unchecked(index.to_usize())
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(PieceType::PAWN.to_char(), 'p');
        assert_eq!(PieceType::KNIGHT.to_char(), 'n');
        assert_eq!(PieceType::BISHOP.to_char(), 'b');
        assert_eq!(PieceType::ROOK.to_char(), 'r');
        assert_eq!(PieceType::QUEEN.to_char(), 'q');
        assert_eq!(PieceType::KING.to_char(), 'k');
    }
}
