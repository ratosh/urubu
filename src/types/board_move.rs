use crate::types::move_type::MoveType;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct BoardMove(pub u16);

impl BoardMove {
    pub const TO_SHIFT: u16 = 6;
    pub const MOVE_TYPE_SHIFT: u16 = 12;

    #[inline]
    pub fn build_normal(square_from: &Square, square_to: &Square) -> BoardMove {
        BoardMove::build_move(square_from, square_to, &MoveType::NORMAL)
    }

    #[inline]
    pub fn build_passant(square_from: &Square, square_to: &Square) -> BoardMove {
        BoardMove::build_move(square_from, square_to, &MoveType::PASSANT)
    }

    #[inline]
    pub fn build_castling(square_from: &Square, square_to: &Square) -> BoardMove {
        BoardMove::build_move(square_from, square_to, &MoveType::CASTLING)
    }

    #[inline]
    pub fn build_move(square_from: &Square, square_to: &Square, move_type: &MoveType) -> BoardMove {
        BoardMove((square_from.to_u16() |
            square_to.to_u16() << BoardMove::TO_SHIFT |
            move_type.to_u16() << BoardMove::MOVE_TYPE_SHIFT) as u16)
    }

    #[inline]
    pub fn square_from(&self) -> Square {
        Square((self.0 & Square::H8.0 as u16) as i8)
    }

    #[inline]
    pub fn square_to(&self) -> Square {
        Square((self.0 >> BoardMove::TO_SHIFT & Square::H8.0 as u16) as i8)
    }

    #[inline]
    pub fn move_type(&self) -> MoveType {
        MoveType((self.0 >> BoardMove::MOVE_TYPE_SHIFT) as u8)
    }

    #[inline]
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(5);
        result.push_str(&self.square_from().to_string());
        result.push_str(&self.square_to().to_string());
        let promoted_piece = self.move_type().promoted_piece_type();
        if promoted_piece.0 > PieceType::PAWN.0 {
            result.push(promoted_piece.to_char());
        }
        return result;
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(BoardMove::build_normal(&Square::A1, &Square::A2).to_string(), "a1a2");
        assert_eq!(BoardMove::build_normal(&Square::B2, &Square::B8).to_string(), "b2b8");
        assert_eq!(BoardMove::build_normal(&Square::C3, &Square::D4).to_string(), "c3d4");
        assert_eq!(BoardMove::build_move(&Square::H7, &Square::H8, &MoveType::PROMOTION_QUEEN).to_string(), "h7h8q");
    }
}
