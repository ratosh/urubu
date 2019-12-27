use crate::types::color::Color;
use crate::types::piece_type::PieceType;

#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub const NUM_PIECES: usize = 13;
    pub const PIECES: [Piece; Piece::NUM_PIECES] = [
        Piece::NONE,
        Piece::WHITE_PAWN,
        Piece::BLACK_PAWN,
        Piece::WHITE_KNIGHT,
        Piece::BLACK_KNIGHT,
        Piece::WHITE_BISHOP,
        Piece::BLACK_BISHOP,
        Piece::WHITE_ROOK,
        Piece::BLACK_ROOK,
        Piece::WHITE_QUEEN,
        Piece::BLACK_QUEEN,
        Piece::WHITE_KING,
        Piece::BLACK_KING];
    pub const REPRESENTATION: [char; Piece::NUM_PIECES] = ['-', 'P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k'];

    pub const NONE: Piece = Piece {color: Color::White, piece_type: PieceType::NONE};
    pub const WHITE_PAWN: Piece = Piece {color: Color::White, piece_type: PieceType::PAWN};
    pub const BLACK_PAWN: Piece = Piece {color: Color::Black, piece_type: PieceType::PAWN};
    pub const WHITE_KNIGHT: Piece = Piece {color: Color::White, piece_type: PieceType::KNIGHT};
    pub const BLACK_KNIGHT: Piece = Piece {color: Color::Black, piece_type: PieceType::KNIGHT};
    pub const WHITE_BISHOP: Piece = Piece {color: Color::White, piece_type: PieceType::BISHOP};
    pub const BLACK_BISHOP: Piece = Piece {color: Color::Black, piece_type: PieceType::BISHOP};
    pub const WHITE_ROOK: Piece = Piece {color: Color::White, piece_type: PieceType::ROOK};
    pub const BLACK_ROOK: Piece = Piece {color: Color::Black, piece_type: PieceType::ROOK};
    pub const WHITE_QUEEN: Piece = Piece {color: Color::White, piece_type: PieceType::QUEEN};
    pub const BLACK_QUEEN: Piece = Piece {color: Color::Black, piece_type: PieceType::QUEEN};
    pub const WHITE_KING: Piece = Piece {color: Color::White, piece_type: PieceType::KING};
    pub const BLACK_KING: Piece = Piece {color: Color::Black, piece_type: PieceType::KING};


    #[inline]
    pub fn to_char(&self) -> char {
        if let Some(index) = Piece::PIECES.iter().position(|&s| s == *self) {
            return Piece::REPRESENTATION[index];
        }
        return Piece::REPRESENTATION[0];
    }

    #[inline]
    pub fn from_char(c: char) -> Piece {
        if let Some(index) = Piece::REPRESENTATION.iter().position(|&s| s == c) {
            return Piece::PIECES[index];
        }
        return Piece::PIECES[0];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(Piece::WHITE_PAWN.to_char(), 'P');
        assert_eq!(Piece::BLACK_PAWN.to_char(), 'p');
        assert_eq!(Piece::WHITE_KNIGHT.to_char(), 'N');
        assert_eq!(Piece::BLACK_KNIGHT.to_char(), 'n');
    }

    #[test]
    fn from_char() {
        assert_eq!(Piece::from_char('P'), Piece::WHITE_PAWN);
        assert_eq!(Piece::from_char('p'), Piece::BLACK_PAWN);
        assert_eq!(Piece::from_char('N'), Piece::WHITE_KNIGHT);
        assert_eq!(Piece::from_char('n'), Piece::BLACK_KNIGHT);
    }
}

