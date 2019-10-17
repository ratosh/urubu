use crate::types::color::Color;
use crate::types::file::File;
use crate::types::rank::Rank;
use std::cmp;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Square(pub i8);

impl Square {
    pub const NUM_SQUARES: usize = 64;

    const REPRESENTATION: [&'static str; Square::NUM_SQUARES] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];

    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);

    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);

    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);

    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);

    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);

    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);

    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    pub const SQUARES: [Square; Square::NUM_SQUARES] = [
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8];

    #[inline]
    pub fn to_usize(&self) -> usize {
        return self.0 as usize;
    }

    #[inline]
    pub fn to_u8(&self) -> u8 {
        return self.0 as u8;
    }

    #[inline]
    pub fn to_u16(&self) -> u16 {
        return self.0 as u16;
    }

    #[inline]
    pub fn to_u64(&self) -> u64 {
        return self.0 as u64;
    }

    #[inline]
    pub fn invert(&self) -> Square {
        Square(self.0 ^ Square::A8.0)
    }

    #[inline]
    pub fn relative(&self, color: &Color) -> Square {
        Square(self.0 ^ (Square::A8.0 * color.0))
    }

    #[inline]
    pub fn new(file: &File, rank: &Rank) -> Square {
        Square(file.0 << 3 + rank.0)
    }

    #[inline]
    pub fn rank(&self) -> Rank {
        Rank(self.0 >> 3)
    }

    #[inline]
    pub fn file(&self) -> File {
        File(self.0 & 7)
    }

    #[inline]
    pub fn to_string(&self) -> String {
        Square::REPRESENTATION[self.to_usize()].to_string()
    }

    pub fn forward(&self, color: &Color) -> Self {
        let result = self.0 + 8 * color.multiplier();
        Square(result)
    }

    pub fn square_dist(&self, other: &Self) -> u8 {
        cmp::max(
            self.rank_dist(other),
            self.file_dist(other)
        )
    }

    pub fn rank_dist(&self, other: &Self) -> u8 {
        self.rank().distance(&other.rank())
    }

    pub fn file_dist(&self, other: &Self) -> u8 {
        self.file().distance(&other.file())
    }


    pub fn offset(&self, value: &i8) -> Option<Self> {
        let result = self.0 + *value;
        if result >= Square::A1.0 && result <= Square::H8.0 {
            return Some(Square(result));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::types::color::Color;

    use super::*;

    #[test]
    fn invert() {
        assert_eq!(Square::A1.invert(), Square::A8);
        assert_eq!(Square::B2.invert(), Square::B7);
        assert_eq!(Square::C3.invert(), Square::C6);
        assert_eq!(Square::D4.invert(), Square::D5);
        assert_eq!(Square::E8.invert(), Square::E1);
        assert_eq!(Square::F7.invert(), Square::F2);
        assert_eq!(Square::G6.invert(), Square::G3);
        assert_eq!(Square::H5.invert(), Square::H4);
    }

    #[test]
    fn relative() {
        assert_eq!(Square::A1.relative(&Color::WHITE), Square::A1);
        assert_eq!(Square::A1.relative(&Color::BLACK), Square::A8);
        assert_eq!(Square::B2.relative(&Color::WHITE), Square::B2);
        assert_eq!(Square::B2.relative(&Color::BLACK), Square::B7);
        assert_eq!(Square::C3.relative(&Color::WHITE), Square::C3);
        assert_eq!(Square::C3.relative(&Color::BLACK), Square::C6);
        assert_eq!(Square::D4.relative(&Color::WHITE), Square::D4);
        assert_eq!(Square::D4.relative(&Color::BLACK), Square::D5);
    }

    #[test]
    fn forward() {
        assert_eq!(Square::A1.forward(&Color::WHITE), Square::A2);
        assert_eq!(Square::B2.forward(&Color::WHITE), Square::B3);
        assert_eq!(Square::B2.forward(&Color::BLACK), Square::B1);
        assert_eq!(Square::C3.forward(&Color::WHITE), Square::C4);
        assert_eq!(Square::C3.forward(&Color::BLACK), Square::C2);
        assert_eq!(Square::D4.forward(&Color::WHITE), Square::D5);
        assert_eq!(Square::D4.forward(&Color::BLACK), Square::D3);
        assert_eq!(Square::E5.forward(&Color::WHITE), Square::E6);
        assert_eq!(Square::E5.forward(&Color::BLACK), Square::E4);
        assert_eq!(Square::F6.forward(&Color::WHITE), Square::F7);
        assert_eq!(Square::F6.forward(&Color::BLACK), Square::F5);
        assert_eq!(Square::G7.forward(&Color::WHITE), Square::G8);
        assert_eq!(Square::G7.forward(&Color::BLACK), Square::G6);
        assert_eq!(Square::H8.forward(&Color::BLACK), Square::H7);
    }
}