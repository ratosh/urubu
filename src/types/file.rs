#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
pub struct File(pub i8);

impl File {
    pub const NUM_FILES: usize = 8;

    pub const FILE_A: File = File(0);
    pub const FILE_B: File = File(1);
    pub const FILE_C: File = File(2);
    pub const FILE_D: File = File(3);
    pub const FILE_E: File = File(4);
    pub const FILE_F: File = File(5);
    pub const FILE_G: File = File(6);
    pub const FILE_H: File = File(7);

    pub const FILES: [File; File::NUM_FILES] = [
        File::FILE_A,
        File::FILE_B,
        File::FILE_C,
        File::FILE_D,
        File::FILE_E,
        File::FILE_F,
        File::FILE_G,
        File::FILE_H,
    ];

    pub const REPRESENTATION: [char; File::NUM_FILES] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    #[inline]
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_char(self) -> char {
        File::REPRESENTATION[self.to_usize()]
    }

    #[inline]
    pub fn from_char(input: char) -> Option<File> {
        if input >= 'a' && input <= 'h' {
            Some(File((input as u8 - b'a') as i8))
        } else {
            None
        }
    }

    #[inline]
    pub fn reverse(self) -> File {
        File(self.0 ^ File::FILE_H.0)
    }

    #[inline]
    pub fn distance(self, other: Self) -> u8 {
        (self.0 - other.0).abs() as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(File::FILE_A.to_char(), 'a');
        assert_eq!(File::FILE_B.to_char(), 'b');
        assert_eq!(File::FILE_C.to_char(), 'c');
        assert_eq!(File::FILE_D.to_char(), 'd');
        assert_eq!(File::FILE_E.to_char(), 'e');
        assert_eq!(File::FILE_F.to_char(), 'f');
        assert_eq!(File::FILE_G.to_char(), 'g');
        assert_eq!(File::FILE_H.to_char(), 'h');
    }

    #[test]
    fn reverse() {
        assert_eq!(File::FILE_A.reverse(), File::FILE_H);
        assert_eq!(File::FILE_B.reverse(), File::FILE_G);
        assert_eq!(File::FILE_C.reverse(), File::FILE_F);
        assert_eq!(File::FILE_D.reverse(), File::FILE_E);
        assert_eq!(File::FILE_E.reverse(), File::FILE_D);
        assert_eq!(File::FILE_F.reverse(), File::FILE_C);
        assert_eq!(File::FILE_G.reverse(), File::FILE_B);
        assert_eq!(File::FILE_H.reverse(), File::FILE_A);
    }
}
