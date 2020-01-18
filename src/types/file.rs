use std::ops;

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub const NUM_FILES: usize = 8;

    pub const FILES: [File; File::NUM_FILES] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];

    pub const REPRESENTATION: [char; File::NUM_FILES] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    #[inline]
    pub fn new(value: i8) -> File {
        assert!(value < 8);
        unsafe {
            ::std::mem::transmute(value as u8)
        }
    }

    #[inline]
    pub fn to_usize(self) -> usize {
        self as usize
    }

    #[inline]
    pub fn to_i8(self) -> i8 {
        self as i8
    }

    #[inline]
    pub fn to_u32(self) -> u32 {
        self as u32
    }

    #[inline]
    pub fn to_char(self) -> char {
        File::REPRESENTATION[self]
    }

    #[inline]
    pub fn from_char(input: char) -> Option<File> {
        if input >= 'a' && input <= 'h' {
            Some(File::new((input as u8 - b'a') as i8))
        } else {
            None
        }
    }

    #[inline]
    pub fn reverse(self) -> File {
        File::new(self.to_i8() ^ 7)
    }

    #[inline]
    pub fn distance(self, other: Self) -> u8 {
        (self.to_i8() - other.to_i8()).abs() as u8
    }
}

impl ops::Index<File> for [char] {
    type Output = char;

    fn index(&self, index: File) -> &Self::Output {
        unsafe { self.get_unchecked(index.to_usize()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(File::A.to_char(), 'a');
        assert_eq!(File::B.to_char(), 'b');
        assert_eq!(File::C.to_char(), 'c');
        assert_eq!(File::D.to_char(), 'd');
        assert_eq!(File::E.to_char(), 'e');
        assert_eq!(File::F.to_char(), 'f');
        assert_eq!(File::G.to_char(), 'g');
        assert_eq!(File::H.to_char(), 'h');
    }

    #[test]
    fn reverse() {
        assert_eq!(File::A.reverse(), File::H);
        assert_eq!(File::B.reverse(), File::G);
        assert_eq!(File::C.reverse(), File::F);
        assert_eq!(File::D.reverse(), File::E);
        assert_eq!(File::E.reverse(), File::D);
        assert_eq!(File::F.reverse(), File::C);
        assert_eq!(File::G.reverse(), File::B);
        assert_eq!(File::H.reverse(), File::A);
    }
}
