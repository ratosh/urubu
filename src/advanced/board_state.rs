use crate::advanced::board::Board;
use crate::advanced::position::Position;
use crate::advanced::zobrist_key::ZobristKey;
use crate::types::bitboard::Bitboard;
use crate::types::castling_rights::{CastlingIndex, CastlingRights, CastlingSide};
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;
use std::fmt::{Debug, Formatter, Error};

#[derive(Clone, Copy)]
// Board state information
pub struct BoardState {
    pub move_number: u16,
    pub rule_50: u16,

    pub color_to_move: Color,

    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,

    pub zkey: ZobristKey,
    pub zkey_pawn: ZobristKey,

    // Cache info
    pub pinned_bitboard: Bitboard,
    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub check_bitboard: Bitboard,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            move_number: 0,
            castling_rights: CastlingRights::ANY_CASTLING,
            ep_square: None,
            rule_50: 0,
            zkey: ZobristKey::new(),
            zkey_pawn: ZobristKey::new(),
            pinned_bitboard: Bitboard::EMPTY,
            danger_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
            check_bitboard: Bitboard::EMPTY,
            color_to_move: Color::White,
        }
    }

    #[inline]
    pub fn next_color_to_move(&self) -> Color {
        self.color_to_move.reverse()
    }

    #[inline]
    pub fn clear_ep(&mut self) {
        if self.ep_square.is_some() {
            self.zkey.set_ep(self.ep_square.unwrap());
            self.ep_square = None;
        }
    }

    #[inline]
    pub fn update_danger_bitboard(&mut self, king_square: Square, game_bitboard: Bitboard, color: Color) {
        self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] = king_square.pawn_attacks(color);
        self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()] = king_square.knight_moves();
        self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()] = king_square.bishop_moves(game_bitboard);
        self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()] = king_square.rook_moves(game_bitboard);
        self.danger_bitboard[color.to_usize()][PieceType::QUEEN.to_usize()] =
            self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()].union(
                self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()]
            );

        self.danger_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .union(self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()])
                .union(self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()])
                .union(self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()])
    }

    #[inline]
    pub fn set_check_bitboard(&mut self, pieces: &[Bitboard], opposite_color_bitboard: Bitboard, color: Color) {
        self.check_bitboard = opposite_color_bitboard.intersect(
            self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .intersect(pieces[PieceType::PAWN.to_usize()])
                .union(self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()]
                    .intersect(pieces[PieceType::KNIGHT.to_usize()]))
                .union(self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()]
                    .intersect(pieces[PieceType::BISHOP.to_usize()].union(pieces[PieceType::QUEEN.to_usize()])))
                .union(self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()]
                    .intersect(pieces[PieceType::ROOK.to_usize()].union(pieces[PieceType::QUEEN.to_usize()]))));
    }

    pub fn set_pinned(&mut self, king_square: Square, pieces: &[Bitboard], color_bitboard: Bitboard, color: Color) {
        let bishop_like_pieces = pieces[PieceType::BISHOP].union(pieces[PieceType::QUEEN]).difference(color_bitboard);
        let rook_like_pieces = pieces[PieceType::ROOK].union(pieces[PieceType::QUEEN]).difference(color_bitboard);
        if bishop_like_pieces.union(rook_like_pieces).is_not_empty() {
            let mut pinned = Bitboard::EMPTY;

            let game_bitboard = pieces[PieceType::NONE];

            let possible_pin = bishop_like_pieces
                .intersect(king_square.pseudo_bishop_moves())
                .union(rook_like_pieces.intersect(king_square.pseudo_rook_moves()));

            for square in possible_pin.iterator() {
                let between_piece = square.between(king_square).intersect(game_bitboard);
                if between_piece.is_not_empty() && between_piece.one_element() {
                    pinned = pinned.union(between_piece.intersect(color_bitboard))
                }
            }
            self.pinned_bitboard = self.pinned_bitboard.union(pinned)
        }
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.zkey == other.zkey &&
            self.zkey_pawn == other.zkey_pawn
    }
}

impl Eq for BoardState {}

impl Debug for BoardState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let mut res_str: String = String::new();
        res_str.push_str(&format!("zkey: {}\n", self.zkey.to_u64()));
        res_str.push_str(&format!("p_zkey: {}\n", self.zkey_pawn.to_u64()));
        write!(formatter, "{}", res_str)
    }
}