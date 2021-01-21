use crate::types::color::Color;
use std::mem::transmute;
use crate::types::square::Square;

#[allow(missing_docs)]
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug, Eq)]
pub enum CastlingSide {
    HSide = 0,
    ASide = 1,
}

impl CastlingSide {
    #[inline]
    pub fn to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }
}

#[allow(missing_docs)]
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug, Eq)]
pub enum CastlingIndex {
    WhiteH = 0,
    WhiteA = 1,
    BlackH = 2,
    BlackA = 3,
}

#[allow(missing_docs)]
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug, Eq)]
pub struct CastlingRights(pub u8);

pub struct CastlingRightsIterator {
    castling_rights: CastlingRights
}

impl CastlingIndex {
    pub const NUM_INDEXES: usize = 4;
    pub const SQUARE_ROOK_TO: [Square; CastlingIndex::NUM_INDEXES] = [Square::F1, Square::D1, Square::F8, Square::D8];
    pub const SQUARE_KING_TO: [Square; CastlingIndex::NUM_INDEXES] = [Square::G1, Square::C1, Square::G8, Square::C8];

    const REPRESENTATION: [char; CastlingIndex::NUM_INDEXES] = ['K', 'Q', 'k', 'q'];

    #[inline]
    pub fn to_usize(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    pub fn unsafe_creation(value: u32) -> Self {
        unsafe {
            return transmute(value as u8);
        }
    }

    #[inline]
    pub fn to_char(&self) -> char {
        CastlingIndex::REPRESENTATION[self.to_usize()]
    }

    #[inline]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'K' => Some(CastlingIndex::WhiteH),
            'Q' => Some(CastlingIndex::WhiteA),
            'k' => Some(CastlingIndex::BlackH),
            'q' => Some(CastlingIndex::BlackA),
            _ => None,
        }
    }

    #[inline]
    pub fn from_color_side(color: Color, castling_side: CastlingSide) -> Self {
        CastlingIndex::unsafe_creation(castling_side.to_u32() + 2 * color.to_u32())
    }

    #[inline]
    pub fn square_king_to(&self) -> Square {
        CastlingIndex::SQUARE_KING_TO[self.to_usize()]
    }

    #[inline]
    pub fn square_rook_to(&self) -> Square {
        CastlingIndex::SQUARE_ROOK_TO[self.to_usize()]
    }
}

impl CastlingRights {
    pub const NUM_RIGHTS: usize = 16;

    pub const NO_CASTLING: CastlingRights = CastlingRights(0);
    pub const WHITE_OO: CastlingRights = CastlingRights(1);
    pub const WHITE_OOO: CastlingRights = CastlingRights(2);
    pub const BLACK_OO: CastlingRights = CastlingRights(4);
    pub const BLACK_OOO: CastlingRights = CastlingRights(8);

    pub const ANY_CASTLING: CastlingRights = CastlingRights(1 | 2 | 4 | 8);
    pub const WHITE_RIGHTS: CastlingRights = CastlingRights(1 | 2);
    pub const BLACK_RIGHTS: CastlingRights = CastlingRights(4 | 8);

    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    #[inline]
    pub fn to_string(&self) -> String {
        if self.0 == CastlingRights::NO_CASTLING.0 {
            return "-".to_string();
        } else {
            let mut alg = String::with_capacity(CastlingIndex::NUM_INDEXES);
            for entry in self.iterator() {
                alg.push(entry.to_char())
            }
            alg
        }
    }

    #[inline]
    pub fn from_string(st: &str) -> Self {
        let mut result = CastlingRights::NO_CASTLING;
        for c in st.chars() {
            if let Some(index) = CastlingIndex::from_char(c) {
                result = result.add_index(&index);
            }
        }
        return result
    }

    #[inline]
    pub fn color_filter(&self, color: &Color) -> Self {
        CastlingRights(self.0 & (CastlingRights::WHITE_RIGHTS.0 << (color.to_u8() * 2)))
    }

    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        CastlingRights(self.0 | other.0)
    }

    #[inline]
    pub fn intersect(&self, other: &Self) -> Self {
        CastlingRights(self.0 & other.0)
    }

    #[inline]
    pub fn difference(&self, other: &Self) -> Self {
        CastlingRights(self.0 & !other.0)
    }

    #[inline]
    pub fn add_index(&self, index: &CastlingIndex) -> Self {
        return CastlingRights(self.0 | 1 << index.to_u8())
    }

    pub fn iterator(self) -> CastlingRightsIterator {
        CastlingRightsIterator { castling_rights: self }
    }
}

impl Iterator for CastlingRightsIterator {
    type Item = CastlingIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.castling_rights.0 > 0 {
            let item = CastlingIndex::unsafe_creation(self.castling_rights.0.trailing_zeros());
            self.castling_rights.0 &= self.castling_rights.0 - 1;
            return Some(item);
        }
        None
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::types::color::Color;

    #[test]
    fn castling_index_unsafe_creation() {
        assert_eq!(CastlingIndex::unsafe_creation(0), CastlingIndex::WhiteA);
        assert_eq!(CastlingIndex::unsafe_creation(1), CastlingIndex::WhiteH);
        assert_eq!(CastlingIndex::unsafe_creation(2), CastlingIndex::BlackA);
        assert_eq!(CastlingIndex::unsafe_creation(3), CastlingIndex::BlackH);
    }

    #[test]
    fn castling_index_from_char() {
        assert_eq!(CastlingIndex::from_char('-'), None);
        assert_eq!(CastlingIndex::from_char('K').unwrap(), CastlingIndex::WhiteA);
        assert_eq!(CastlingIndex::from_char('Q').unwrap(), CastlingIndex::WhiteH);
        assert_eq!(CastlingIndex::from_char('k').unwrap(), CastlingIndex::BlackA);
        assert_eq!(CastlingIndex::from_char('q').unwrap(), CastlingIndex::BlackH);
    }

    #[test]
    fn castling_index_to_char() {
        assert_eq!(CastlingIndex::WhiteA.to_char(), 'K');
        assert_eq!(CastlingIndex::WhiteH.to_char(), 'Q');
        assert_eq!(CastlingIndex::BlackA.to_char(), 'k');
        assert_eq!(CastlingIndex::BlackH.to_char(), 'q');
    }

    #[test]
    fn to_string() {
        assert_eq!(CastlingRights::WHITE_OO.to_string(), "K");
        assert_eq!(CastlingRights::WHITE_OOO.to_string(), "Q");
        assert_eq!(CastlingRights::BLACK_OO.to_string(), "k");
        assert_eq!(CastlingRights::BLACK_OOO.to_string(), "q");
        assert_eq!(CastlingRights::WHITE_RIGHTS.to_string(), "KQ");
        assert_eq!(CastlingRights::BLACK_RIGHTS.to_string(), "kq");
        assert_eq!(CastlingRights::ANY_CASTLING.to_string(), "KQkq");
        assert_eq!(CastlingRights::NO_CASTLING.to_string(), "-");
        assert_eq!(CastlingRights(0).to_string(), "-");
    }

    #[test]
    fn filter() {
        assert_eq!(CastlingRights::WHITE_OO.color_filter(&Color::White), CastlingRights::WHITE_OO);
        assert_eq!(CastlingRights::WHITE_OOO.color_filter(&Color::White), CastlingRights::WHITE_OOO);
        assert_eq!(CastlingRights::WHITE_OO.color_filter(&Color::Black), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::WHITE_OOO.color_filter(&Color::Black), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.color_filter(&Color::White), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::ANY_CASTLING.color_filter(&Color::Black), CastlingRights::BLACK_RIGHTS);
    }

    #[test]
    fn add_right() {
        assert_eq!(CastlingRights::ANY_CASTLING.union(&CastlingRights::WHITE_OO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.union(&CastlingRights::WHITE_OOO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.union(&CastlingRights::BLACK_OO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.union(&CastlingRights::BLACK_OOO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.union(&CastlingRights::WHITE_RIGHTS), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.union(&CastlingRights::BLACK_RIGHTS), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.union(&CastlingRights::WHITE_OO), CastlingRights::WHITE_OO);
        assert_eq!(CastlingRights::NO_CASTLING.union(&CastlingRights::WHITE_OOO), CastlingRights::WHITE_OOO);
        assert_eq!(CastlingRights::NO_CASTLING.union(&CastlingRights::BLACK_OO), CastlingRights::BLACK_OO);
        assert_eq!(CastlingRights::NO_CASTLING.union(&CastlingRights::BLACK_OOO), CastlingRights::BLACK_OOO);
        assert_eq!(CastlingRights::NO_CASTLING.union(&CastlingRights::WHITE_RIGHTS), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::NO_CASTLING.union(&CastlingRights::BLACK_RIGHTS), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::WHITE_RIGHTS.union(&CastlingRights::WHITE_OO), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::WHITE_RIGHTS.union(&CastlingRights::WHITE_OOO), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::BLACK_RIGHTS.union(&CastlingRights::BLACK_OO), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::BLACK_RIGHTS.union(&CastlingRights::BLACK_OOO), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::BLACK_RIGHTS.union(&CastlingRights::WHITE_RIGHTS), CastlingRights::ANY_CASTLING);
    }

    #[test]
    fn rem_right() {
        assert_eq!(CastlingRights::ANY_CASTLING.difference(&CastlingRights::WHITE_RIGHTS), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::ANY_CASTLING.difference(&CastlingRights::BLACK_RIGHTS), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::NO_CASTLING.difference(&CastlingRights::WHITE_OO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.difference(&CastlingRights::WHITE_OOO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.difference(&CastlingRights::BLACK_OO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.difference(&CastlingRights::BLACK_OOO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.difference(&CastlingRights::WHITE_RIGHTS), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.difference(&CastlingRights::BLACK_RIGHTS), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::WHITE_RIGHTS.difference(&CastlingRights::WHITE_OO), CastlingRights::WHITE_OOO);
        assert_eq!(CastlingRights::WHITE_RIGHTS.difference(&CastlingRights::WHITE_OOO), CastlingRights::WHITE_OO);
        assert_eq!(CastlingRights::BLACK_RIGHTS.difference(&CastlingRights::BLACK_OO), CastlingRights::BLACK_OOO);
        assert_eq!(CastlingRights::BLACK_RIGHTS.difference(&CastlingRights::BLACK_OOO), CastlingRights::BLACK_OO);
        assert_eq!(CastlingRights::BLACK_RIGHTS.difference(&CastlingRights::WHITE_RIGHTS), CastlingRights::BLACK_RIGHTS);
    }
}