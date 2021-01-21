use crate::types::square::Square;
use crate::types::color::Color;

pub struct BitboardIterator {
    bitboard: Bitboard
}

impl Iterator for BitboardIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard.0 > 0 {
            let square = self.bitboard.to_square();
            self.bitboard.0 &= self.bitboard.0 - 1;
            return Some(square);
        }
        None
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const ALL: Bitboard = Bitboard(0xFFFFFFFFFFFFFFFFu64);
    pub const EMPTY: Bitboard = Bitboard(0u64);
    pub const PROMOTION: [Bitboard; Color::NUM_COLORS] = [Bitboard::RANK_7, Bitboard::RANK_2];

    pub const FILE_A: Bitboard = Bitboard(0x101010101010101u64);
    pub const FILE_H: Bitboard = Bitboard(0x8080808080808080u64);

    pub const RANK_1: Bitboard = Bitboard(0xFFu64);
    pub const RANK_2: Bitboard = Bitboard(0xFF00u64);
    pub const RANK_7: Bitboard = Bitboard(0xFF000000000000u64);
    pub const RANK_8: Bitboard = Bitboard(0xFF00000000000000u64);

    pub const A1: Bitboard = Bitboard(1u64 << 0);
    pub const B1: Bitboard = Bitboard(1u64 << 1);
    pub const C1: Bitboard = Bitboard(1u64 << 2);
    pub const D1: Bitboard = Bitboard(1u64 << 3);
    pub const E1: Bitboard = Bitboard(1u64 << 4);
    pub const F1: Bitboard = Bitboard(1u64 << 5);
    pub const G1: Bitboard = Bitboard(1u64 << 6);
    pub const H1: Bitboard = Bitboard(1u64 << 7);

    pub const A2: Bitboard = Bitboard(1u64 << 8);
    pub const B2: Bitboard = Bitboard(1u64 << 9);
    pub const C2: Bitboard = Bitboard(1u64 << 10);
    pub const D2: Bitboard = Bitboard(1u64 << 11);
    pub const E2: Bitboard = Bitboard(1u64 << 12);
    pub const F2: Bitboard = Bitboard(1u64 << 13);
    pub const G2: Bitboard = Bitboard(1u64 << 14);
    pub const H2: Bitboard = Bitboard(1u64 << 15);

    pub const A3: Bitboard = Bitboard(1u64 << 16);
    pub const B3: Bitboard = Bitboard(1u64 << 17);
    pub const C3: Bitboard = Bitboard(1u64 << 18);
    pub const D3: Bitboard = Bitboard(1u64 << 19);
    pub const E3: Bitboard = Bitboard(1u64 << 20);
    pub const F3: Bitboard = Bitboard(1u64 << 21);
    pub const G3: Bitboard = Bitboard(1u64 << 22);
    pub const H3: Bitboard = Bitboard(1u64 << 23);

    pub const A4: Bitboard = Bitboard(1u64 << 24);
    pub const B4: Bitboard = Bitboard(1u64 << 25);
    pub const C4: Bitboard = Bitboard(1u64 << 26);
    pub const D4: Bitboard = Bitboard(1u64 << 27);
    pub const E4: Bitboard = Bitboard(1u64 << 28);
    pub const F4: Bitboard = Bitboard(1u64 << 29);
    pub const G4: Bitboard = Bitboard(1u64 << 30);
    pub const H4: Bitboard = Bitboard(1u64 << 31);

    pub const A5: Bitboard = Bitboard(1u64 << 32);
    pub const B5: Bitboard = Bitboard(1u64 << 33);
    pub const C5: Bitboard = Bitboard(1u64 << 34);
    pub const D5: Bitboard = Bitboard(1u64 << 35);
    pub const E5: Bitboard = Bitboard(1u64 << 36);
    pub const F5: Bitboard = Bitboard(1u64 << 37);
    pub const G5: Bitboard = Bitboard(1u64 << 38);
    pub const H5: Bitboard = Bitboard(1u64 << 39);

    pub const A6: Bitboard = Bitboard(1u64 << 40);
    pub const B6: Bitboard = Bitboard(1u64 << 41);
    pub const C6: Bitboard = Bitboard(1u64 << 42);
    pub const D6: Bitboard = Bitboard(1u64 << 43);
    pub const E6: Bitboard = Bitboard(1u64 << 44);
    pub const F6: Bitboard = Bitboard(1u64 << 45);
    pub const G6: Bitboard = Bitboard(1u64 << 46);
    pub const H6: Bitboard = Bitboard(1u64 << 47);

    pub const A7: Bitboard = Bitboard(1u64 << 48);
    pub const B7: Bitboard = Bitboard(1u64 << 49);
    pub const C7: Bitboard = Bitboard(1u64 << 50);
    pub const D7: Bitboard = Bitboard(1u64 << 51);
    pub const E7: Bitboard = Bitboard(1u64 << 52);
    pub const F7: Bitboard = Bitboard(1u64 << 53);
    pub const G7: Bitboard = Bitboard(1u64 << 54);
    pub const H7: Bitboard = Bitboard(1u64 << 55);

    pub const A8: Bitboard = Bitboard(1u64 << 56);
    pub const B8: Bitboard = Bitboard(1u64 << 57);
    pub const C8: Bitboard = Bitboard(1u64 << 58);
    pub const D8: Bitboard = Bitboard(1u64 << 59);
    pub const E8: Bitboard = Bitboard(1u64 << 60);
    pub const F8: Bitboard = Bitboard(1u64 << 61);
    pub const G8: Bitboard = Bitboard(1u64 << 62);
    pub const H8: Bitboard = Bitboard(1u64 << 63);

    #[inline]
    pub fn new(bitboard: u64) -> Self {
        Bitboard(bitboard)
    }

    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_u64(&self) -> u64 {
        self.0
    }

    #[inline]
    pub fn from_square(square: Square) -> Self {
        Bitboard(1u64 << square.to_u64())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn one_element(&self) -> bool {
        (self.0 & (self.0 - 1)) == 0
    }

    #[inline]
    pub fn is_set(&self, square: Square) -> bool {
        (1u64 << square.to_u64()) & self.0 != Bitboard::EMPTY.0
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    #[inline]
    pub fn intersect(&self, other: Self) -> Self {
        Bitboard(self.0 & other.0)
    }

    #[inline]
    pub fn union(&self, other: Self) -> Self {
        Bitboard(self.0 | other.0)
    }

    #[inline]
    pub fn difference(&self, other: Self) -> Self {
        Bitboard(self.0 & !other.0)
    }

    #[inline]
    pub fn invert(&self, other: Self) -> Self {
        Bitboard(self.0 ^ other.0)
    }

    #[inline]
    pub fn reverse(&self) -> Self {
        Bitboard(!self.0)
    }

    #[inline]
    pub fn with_square(&self, square: Square) -> Self {
        self.union(Bitboard::from_square(square))
    }

    #[inline]
    pub fn to_string(&self) -> String {
        format!("{:#066b}", self.0)
    }

    #[inline]
    pub fn has(&self, other: Bitboard) -> bool {
        self.intersect(other).is_not_empty()
    }

    #[inline]
    pub fn to_square(&self) -> Square {
        Square(self.0.trailing_zeros() as i8)
    }

    pub fn iterator(self) -> BitboardIterator {
        BitboardIterator { bitboard: self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_square() {
        assert_eq!(Bitboard::from_square(Square::A1), Bitboard::A1);
        assert_eq!(Bitboard::from_square(Square::A8), Bitboard::A8);
        assert_eq!(Bitboard::from_square(Square::H1), Bitboard::H1);
        assert_eq!(Bitboard::from_square(Square::H8), Bitboard::H8);
    }

    #[test]
    fn is_set() {
        assert_eq!(Bitboard::A1.is_set(Square::A1), true);
        assert_eq!(Bitboard::H1.is_set(Square::H1), true);
        assert_eq!(Bitboard::A2.is_set(Square::A2), true);
        assert_eq!(Bitboard::G2.is_set(Square::G2), true);
    }
}