use crate::types::piece_type::PieceType;
use crate::types::color::Color;
use crate::types::square::Square;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub struct ZobristKey(pub u64);

impl ZobristKey {
    /*
    #[inline]
    pub fn remove_piece(&self, color: &Color, piece: &PieceType, square: &Square) -> Self {}

    #[inline]
    pub fn add_piece(&self, color: &Color, piece: &PieceType, square: &Square) -> Self {}
    */
}