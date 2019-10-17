use std::convert::From;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct File(pub i8);

impl File {
    pub const NUM_FILES: usize = 8;

    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);

    pub const FILES: [File; File::NUM_FILES] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H];

    pub const REPRESENTATION: [char; File::NUM_FILES] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    #[inline]
    pub fn to_usize(&self) -> usize {
        return self.0 as usize;
    }

    #[inline]
    pub fn to_char(&self) -> char {
        File::REPRESENTATION[self.to_usize()]
    }

    #[inline]
    pub fn from_char(input: char) -> Option<File> {
        if input >= 'a' && input <= 'h' {
            Some(File(u8::from(input as u8 - b'a') as i8));
        }
        None
    }

    #[inline]
    pub fn invert(&self) -> File {
        File(self.0 ^ File::H.0)
    }

    #[inline]
    pub fn distance(&self, other: &Self) -> u8 {
        (self.0 - other.0).abs() as u8
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
    fn invert() {
        assert_eq!(File::A.invert(), File::H);
        assert_eq!(File::B.invert(), File::G);
        assert_eq!(File::C.invert(), File::F);
        assert_eq!(File::D.invert(), File::E);
        assert_eq!(File::E.invert(), File::D);
        assert_eq!(File::F.invert(), File::C);
        assert_eq!(File::G.invert(), File::B);
        assert_eq!(File::H.invert(), File::A);
    }
}

