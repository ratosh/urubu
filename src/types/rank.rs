use crate::types::color::Color;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct Rank(pub i8);

impl Rank {

    pub const NUM_RANKS: usize = 8;

    pub const RANK_1: Rank = Rank(0);
    pub const RANK_2: Rank = Rank(1);
    pub const RANK_3: Rank = Rank(2);
    pub const RANK_4: Rank = Rank(3);
    pub const RANK_5: Rank = Rank(4);
    pub const RANK_6: Rank = Rank(5);
    pub const RANK_7: Rank = Rank(6);
    pub const RANK_8: Rank = Rank(7);

    pub const RANKS: [Rank; Rank::NUM_RANKS] = [
        Rank::RANK_1,
        Rank::RANK_2,
        Rank::RANK_3,
        Rank::RANK_4,
        Rank::RANK_5,
        Rank::RANK_6,
        Rank::RANK_7,
        Rank::RANK_8];
    pub const REPRESENTATION: [char; Rank::NUM_RANKS] = ['1', '2', '3', '4', '5', '6', '7', '8'];

    #[inline]
    pub fn to_usize(&self) -> usize {
        return self.0 as usize;
    }

    #[inline]
    pub fn to_char(&self) -> char {
        Rank::REPRESENTATION[self.to_usize()]
    }

    #[inline]
    pub fn reverse(&self) -> Rank {
        Rank(self.0 ^ Rank::RANK_8.0)
    }

    #[inline]
    pub fn relative(&self, color: Color) -> Rank {
        Rank(self.0 ^ (Rank::RANK_8.0 * color.to_i8()))
    }

    #[inline]
    pub fn distance(&self, other: &Self) -> u8 {
        (self.0 - other.0).abs() as u8
    }

    #[inline]
    pub fn from_char(input: char) -> Option<Rank> {
        return if input >= '1' && input <= '8' {
            Some(Rank(u8::from(input as u8 - b'1') as i8))
        } else {
            None
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(Rank::RANK_1.to_char(), '1');
        assert_eq!(Rank::RANK_2.to_char(), '2');
        assert_eq!(Rank::RANK_3.to_char(), '3');
        assert_eq!(Rank::RANK_4.to_char(), '4');
        assert_eq!(Rank::RANK_5.to_char(), '5');
        assert_eq!(Rank::RANK_6.to_char(), '6');
        assert_eq!(Rank::RANK_7.to_char(), '7');
        assert_eq!(Rank::RANK_8.to_char(), '8');
    }

    #[test]
    fn from_char() {
        assert_eq!(Rank::from_char('-').is_none(), true);
        assert_eq!(Rank::from_char('1').unwrap(), Rank::RANK_1);
        assert_eq!(Rank::from_char('2').unwrap(), Rank::RANK_2);
        assert_eq!(Rank::from_char('3').unwrap(), Rank::RANK_3);
        assert_eq!(Rank::from_char('4').unwrap(), Rank::RANK_4);
        assert_eq!(Rank::from_char('5').unwrap(), Rank::RANK_5);
        assert_eq!(Rank::from_char('6').unwrap(), Rank::RANK_6);
        assert_eq!(Rank::from_char('7').unwrap(), Rank::RANK_7);
        assert_eq!(Rank::from_char('8').unwrap(), Rank::RANK_8);
    }
}

