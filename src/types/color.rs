#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Color(pub i8);

impl Color {
    pub const NUM_COLORS: usize = 2;
    pub const WHITE: Color = Color(0);
    pub const BLACK: Color = Color(1);

    pub const COLORS: [Color; Color::NUM_COLORS] = [Color::WHITE, Color::BLACK];
    pub const REPRESENTATION: [char; Color::NUM_COLORS] = ['w', 'b'];

    #[inline]
    pub fn to_u8(&self) -> u8 {
        return self.0 as u8;
    }

    #[inline]
    pub fn to_usize(&self) -> usize {
        return self.0 as usize;
    }

    #[inline]
    pub fn to_char(&self) -> char {
        Color::REPRESENTATION[self.to_usize()]
    }

    #[inline]
    pub fn from_char(ch: char) -> Option<Color> {
        match ch {
            'w' => Some(Color::WHITE),
            'b' => Some(Color::BLACK),
            _ => None,
        }
    }

    #[inline]
    pub fn invert(&self) -> Color {
        Color(self.0 ^ Color::BLACK.0)
    }

    #[inline]
    pub fn multiplier(&self) -> i8 {
        -2 * self.0 + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(Color::WHITE.to_char(), 'w');
        assert_eq!(Color::BLACK.to_char(), 'b');
    }

    #[test]
    fn from_char() {
        assert_eq!(Color::from_char('w').unwrap(), Color::WHITE);
        assert_eq!(Color::from_char('b').unwrap(), Color::BLACK);
        assert_eq!(Color::from_char('-'), None);
    }

    #[test]
    fn invert() {
        assert_eq!(Color::WHITE.invert(), Color::BLACK);
        assert_eq!(Color::BLACK.invert(), Color::WHITE);
    }
}