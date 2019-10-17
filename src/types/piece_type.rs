#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct PieceType(pub u8);

impl PieceType {

    pub const NUM_PIECE_TYPES: usize = 7;

    pub const TYPE_ARRAY: [PieceType; PieceType::NUM_PIECE_TYPES] = [
        PieceType::NONE,
        PieceType::PAWN,
        PieceType::KNIGHT,
        PieceType::BISHOP,
        PieceType::ROOK,
        PieceType::QUEEN,
        PieceType::KING];
    pub const REPRESENTATION: [char; 7] = ['-', 'p', 'n', 'b', 'r', 'q', 'k'];

    pub const NONE: PieceType = PieceType(0);
    pub const PAWN: PieceType = PieceType(1);
    pub const KNIGHT: PieceType = PieceType(2);
    pub const BISHOP: PieceType = PieceType(3);
    pub const ROOK: PieceType = PieceType(4);
    pub const QUEEN: PieceType = PieceType(5);
    pub const KING: PieceType = PieceType(6);

    #[inline]
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn to_char(&self) -> char {
        PieceType::REPRESENTATION[self.to_usize()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(PieceType::PAWN.to_char(), 'p');
        assert_eq!(PieceType::KNIGHT.to_char(), 'n');
        assert_eq!(PieceType::BISHOP.to_char(), 'b');
        assert_eq!(PieceType::ROOK.to_char(), 'r');
        assert_eq!(PieceType::QUEEN.to_char(), 'q');
        assert_eq!(PieceType::KING.to_char(), 'k');
    }

}

