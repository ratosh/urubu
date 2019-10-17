use crate::types::color::Color;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug, Eq)]
pub struct CastlingRights(pub u8);

#[derive(PartialOrd, PartialEq, Copy, Clone, Eq)]
pub struct CastlingIndex(pub u8);

pub struct CastlingRightsIterator {
    castling_rights: CastlingRights
}

impl CastlingIndex {
    const REPRESENTATION: [char; 5] = ['K', 'Q', 'k', 'q', '-'];
    pub const NO_CASTLING: CastlingIndex = CastlingIndex(4);

    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_char(&self) -> char {
        CastlingIndex::REPRESENTATION[self.to_usize()]
    }
}

impl CastlingRights {
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
        match *self {
            CastlingRights::NO_CASTLING => CastlingIndex::NO_CASTLING.to_char().to_string(),
            _ => {
                let mut alg = String::with_capacity(4);
                for index in 0..4 {
                    let right = CastlingRights(1 << index);
                    if self.0 & right.0 == right.0 {
                        alg.push(CastlingIndex::REPRESENTATION[index])
                    }
                }
                alg
            }
        }
    }

    #[inline]
    pub fn to_string2(&self) -> String {
        if self.0 == CastlingRights::NO_CASTLING.0 {
            CastlingIndex::NO_CASTLING.to_char().to_string()
        } else {
            let mut alg = String::with_capacity(4);
            for entry in self.iterator() {
                alg.push(entry.to_char())
            }
            alg
        }
    }

    #[inline]
    pub fn filter(&self, color: &Color) -> Self {
        CastlingRights(self.0 & (CastlingRights::WHITE_RIGHTS.0 << (color.to_u8() * 2)))
    }

    #[inline]
    pub fn add_right(&self, other: &Self) -> Self {
        CastlingRights(self.0 | other.0)
    }

    #[inline]
    pub fn rem_right(&self, other: &Self) -> Self {
        CastlingRights(self.0 & !other.0)
    }

    pub fn iterator(self) -> CastlingRightsIterator {
        CastlingRightsIterator { castling_rights: self }
    }
}

impl Iterator for CastlingRightsIterator {
    type Item = CastlingIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.castling_rights.0 > 0 {
            let item = CastlingIndex(self.castling_rights.0.trailing_zeros() as u8);
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
        assert_eq!(CastlingRights::WHITE_OO.filter(&Color::WHITE), CastlingRights::WHITE_OO);
        assert_eq!(CastlingRights::WHITE_OOO.filter(&Color::WHITE), CastlingRights::WHITE_OOO);
        assert_eq!(CastlingRights::WHITE_OO.filter(&Color::BLACK), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::WHITE_OOO.filter(&Color::BLACK), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.filter(&Color::WHITE), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::ANY_CASTLING.filter(&Color::BLACK), CastlingRights::BLACK_RIGHTS);
    }

    #[test]
    fn add_right() {
        assert_eq!(CastlingRights::ANY_CASTLING.add_right(&CastlingRights::WHITE_OO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.add_right(&CastlingRights::WHITE_OOO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.add_right(&CastlingRights::BLACK_OO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.add_right(&CastlingRights::BLACK_OOO), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.add_right(&CastlingRights::WHITE_RIGHTS), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::ANY_CASTLING.add_right(&CastlingRights::BLACK_RIGHTS), CastlingRights::ANY_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.add_right(&CastlingRights::WHITE_OO), CastlingRights::WHITE_OO);
        assert_eq!(CastlingRights::NO_CASTLING.add_right(&CastlingRights::WHITE_OOO), CastlingRights::WHITE_OOO);
        assert_eq!(CastlingRights::NO_CASTLING.add_right(&CastlingRights::BLACK_OO), CastlingRights::BLACK_OO);
        assert_eq!(CastlingRights::NO_CASTLING.add_right(&CastlingRights::BLACK_OOO), CastlingRights::BLACK_OOO);
        assert_eq!(CastlingRights::NO_CASTLING.add_right(&CastlingRights::WHITE_RIGHTS), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::NO_CASTLING.add_right(&CastlingRights::BLACK_RIGHTS), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::WHITE_RIGHTS.add_right(&CastlingRights::WHITE_OO), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::WHITE_RIGHTS.add_right(&CastlingRights::WHITE_OOO), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::BLACK_RIGHTS.add_right(&CastlingRights::BLACK_OO), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::BLACK_RIGHTS.add_right(&CastlingRights::BLACK_OOO), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::BLACK_RIGHTS.add_right(&CastlingRights::WHITE_RIGHTS), CastlingRights::ANY_CASTLING);
    }

    #[test]
    fn rem_right() {
        assert_eq!(CastlingRights::ANY_CASTLING.rem_right(&CastlingRights::WHITE_RIGHTS), CastlingRights::BLACK_RIGHTS);
        assert_eq!(CastlingRights::ANY_CASTLING.rem_right(&CastlingRights::BLACK_RIGHTS), CastlingRights::WHITE_RIGHTS);
        assert_eq!(CastlingRights::NO_CASTLING.rem_right(&CastlingRights::WHITE_OO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.rem_right(&CastlingRights::WHITE_OOO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.rem_right(&CastlingRights::BLACK_OO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.rem_right(&CastlingRights::BLACK_OOO), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.rem_right(&CastlingRights::WHITE_RIGHTS), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::NO_CASTLING.rem_right(&CastlingRights::BLACK_RIGHTS), CastlingRights::NO_CASTLING);
        assert_eq!(CastlingRights::WHITE_RIGHTS.rem_right(&CastlingRights::WHITE_OO), CastlingRights::WHITE_OOO);
        assert_eq!(CastlingRights::WHITE_RIGHTS.rem_right(&CastlingRights::WHITE_OOO), CastlingRights::WHITE_OO);
        assert_eq!(CastlingRights::BLACK_RIGHTS.rem_right(&CastlingRights::BLACK_OO), CastlingRights::BLACK_OOO);
        assert_eq!(CastlingRights::BLACK_RIGHTS.rem_right(&CastlingRights::BLACK_OOO), CastlingRights::BLACK_OO);
        assert_eq!(CastlingRights::BLACK_RIGHTS.rem_right(&CastlingRights::WHITE_RIGHTS), CastlingRights::BLACK_RIGHTS);
    }
}