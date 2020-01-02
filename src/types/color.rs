use std::mem::transmute;
use std::ops;
use crate::types::bitboard::Bitboard;
use crate::types::square::Square;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub const NUM_COLORS: usize = 2;

    pub const COLORS: [Color; Color::NUM_COLORS] = [Color::White, Color::Black];
    pub const REPRESENTATION: [char; Color::NUM_COLORS] = ['w', 'b'];

    #[inline]
    pub fn to_u8(&self) -> u8 {
        return *self as u8;
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        return *self as u32;
    }

    #[inline]
    pub fn to_u16(&self) -> u16 {
        return *self as u16;
    }
    #[inline]
    pub fn to_i16(&self) -> i16 {
        return *self as i16;
    }

    #[inline]
    pub fn to_i8(&self) -> i8 {
        return *self as i8;
    }

    #[inline]
    pub fn to_usize(&self) -> usize {
        return *self as usize;
    }

    #[inline]
    pub fn to_char(&self) -> char {
        Color::REPRESENTATION[self.to_usize()]
    }

    #[inline]
    pub fn unsafe_creation(value: i8) -> Color {
        unsafe {
            return transmute(value as u8);
        }
    }

    pub fn from_char(c: char) -> Option<Color> {
        match c {
            'w' => Some(Color::White),
            'b' => Some(Color::Black),
            _ => None,
        }
    }


    pub fn from_string(st: &str) -> Option<Color> {
        Color::from_char(st.chars().next().unwrap())
    }

    #[inline]
    pub fn reverse(&self) -> Color {
        Color::unsafe_creation(self.to_i8() ^ 1)
    }

    #[inline]
    pub fn multiplier(&self) -> i8 {
        -2 * self.to_i8() + 1
    }
}

impl ops::Index<&Color> for [Bitboard] {
    type Output = Bitboard;

    fn index(&self, index: &Color) -> &Self::Output {
        &self[index.to_usize()]
    }
}

impl ops::IndexMut<&Color> for [Bitboard] {
    fn index_mut(&mut self, index: &Color) -> &mut Bitboard {
        &mut self[index.to_usize()]
    }
}

impl ops::Index<&Color> for [Square] {
    type Output = Square;

    fn index(&self, index: &Color) -> &Self::Output {
        &self[index.to_usize()]
    }
}

impl ops::IndexMut<&Color> for [Square] {
    fn index_mut(&mut self, index: &Color) -> &mut Square {
        &mut self[index.to_usize()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(Color::White.to_char(), 'w');
        assert_eq!(Color::Black.to_char(), 'b');
    }

    #[test]
    fn from_char() {
        assert_eq!(Color::from_char('w').unwrap(), Color::White);
        assert_eq!(Color::from_char('b').unwrap(), Color::Black);
        assert_eq!(Color::from_char('-'), None);
    }

    #[test]
    fn reverse() {
        assert_eq!(Color::White.reverse(), Color::Black);
        assert_eq!(Color::Black.reverse(), Color::White);
    }
}