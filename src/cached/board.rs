use std::fmt::{Debug, Error, Formatter};

use crate::advanced::zobrist_key::ZobristKey;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::castling_rights::{CastlingIndex, CastlingRights, CastlingSide};
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(Clone)]
pub struct Board {
    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,

    color_bitboard: [Bitboard; Color::NUM_COLORS],
    piece_bitboard: [Bitboard; PieceType::NUM_PIECE_TYPES],

    pub color_to_move: Color,
    pub move_number: u16,
    pub rule_50: u16,

    // Extra info
    pub zkey: ZobristKey,
    pub zkey_pawn: ZobristKey,

    king_square: [Square; Color::NUM_COLORS],

    initial_rook_square: [Square; CastlingIndex::NUM_INDEXES],
    castling_rights_masks: [CastlingRights; Square::NUM_SQUARES],

    pub pinned_bitboard: Bitboard,
    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub check_bitboard: Bitboard,
}

impl Board {
    #[inline]
    pub fn empty() -> Self {
        let mut result = Self {
            zkey: ZobristKey::new(),
            zkey_pawn: ZobristKey::new(),
            castling_rights: CastlingRights::NO_CASTLING,
            ep_square: None,
            color_bitboard: [
                Bitboard(0),
                Bitboard(0),
            ],
            piece_bitboard: [
                Bitboard(0),
                Bitboard(0),
                Bitboard(0),
                Bitboard(0),
                Bitboard(0),
                Bitboard(0),
                Bitboard(0),
            ],
            color_to_move: Color::White,
            move_number: 0,
            rule_50: 0,
            king_square: [Square::E1, Square::E8],

            initial_rook_square: [Square::H1, Square::A1, Square::H8, Square::A8],
            castling_rights_masks: [CastlingRights::NO_CASTLING; Square::NUM_SQUARES],

            check_bitboard: Bitboard::EMPTY,
            pinned_bitboard: Bitboard::EMPTY,
            danger_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
        };

        result.castling_rights_masks[result.king_square(&Color::White).to_usize()] = CastlingRights::WHITE_RIGHTS;
        result.castling_rights_masks[result.king_square(&Color::Black).to_usize()] = CastlingRights::BLACK_RIGHTS;

        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::WhiteA.to_usize()].to_usize()] = CastlingRights::WHITE_OOO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::WhiteH.to_usize()].to_usize()] = CastlingRights::WHITE_OO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::BlackA.to_usize()].to_usize()] = CastlingRights::BLACK_OOO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::BlackH.to_usize()].to_usize()] = CastlingRights::BLACK_OO;

        result.compute_zobrist();
        result.initial_pass();
        return result;
    }

    #[inline]
    pub fn default() -> Self {
        let mut result = Self {
            zkey: ZobristKey::new(),
            zkey_pawn: ZobristKey::new(),
            castling_rights: CastlingRights::ANY_CASTLING,
            ep_square: None,
            color_bitboard: [
                Bitboard(0x0000_0000_0000_FFFF),
                Bitboard(0xFFFF_0000_0000_0000),
            ],
            piece_bitboard: [
                Bitboard(0xffff_0000_0000_ffff),
                Bitboard(0x00ff_0000_0000_ff00),
                Bitboard(0x4200_0000_0000_0042),
                Bitboard(0x2400_0000_0000_0024),
                Bitboard(0x8100_0000_0000_0081),
                Bitboard(0x0800_0000_0000_0008),
                Bitboard(0x1000_0000_0000_0010),
            ],
            color_to_move: Color::White,
            move_number: 0,
            rule_50: 0,
            king_square: [Square::E1, Square::E8],

            initial_rook_square: [Square::H1, Square::A1, Square::H8, Square::A8],
            castling_rights_masks: [CastlingRights::NO_CASTLING; Square::NUM_SQUARES],

            check_bitboard: Bitboard::EMPTY,
            pinned_bitboard: Bitboard::EMPTY,
            danger_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
        };

        result.castling_rights_masks[result.king_square(&Color::White).to_usize()] = CastlingRights::WHITE_RIGHTS;
        result.castling_rights_masks[result.king_square(&Color::Black).to_usize()] = CastlingRights::BLACK_RIGHTS;

        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::WhiteA.to_usize()].to_usize()] = CastlingRights::WHITE_OO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::WhiteH.to_usize()].to_usize()] = CastlingRights::WHITE_OOO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::BlackA.to_usize()].to_usize()] = CastlingRights::BLACK_OO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::BlackH.to_usize()].to_usize()] = CastlingRights::BLACK_OOO;

        result.compute_zobrist();
        result.initial_pass();
        return result;
    }

    pub fn compute_king_square(&mut self) {
        for color in Color::COLORS.iter() {
            self.king_square[color.to_usize()] = self.piece_bitboard(&color, &PieceType::KING).to_square();
        }
    }

    #[inline]
    pub fn game_bitboard(&self) -> Bitboard {
        self.piece_bitboard[PieceType::NONE.to_usize()]
    }

    #[inline]
    pub fn empty_bitboard(&self) -> Bitboard {
        self.game_bitboard().reverse()
    }

    #[inline]
    pub fn slider_pieces(&self, color: &Color) -> Bitboard {
        self.piece_bitboard[PieceType::BISHOP.to_usize()]
            .union(&self.piece_bitboard[PieceType::ROOK.to_usize()])
            .union(&self.piece_bitboard[PieceType::QUEEN.to_usize()])
            .intersect(&self.color_bitboard[color.to_usize()])
    }

    #[inline]
    pub fn bishop_like_pieces(&self, color: &Color) -> Bitboard {
        self.piece_bitboard[PieceType::BISHOP.to_usize()]
            .union(&self.piece_bitboard[PieceType::QUEEN.to_usize()])
            .intersect(&self.color_bitboard[color.to_usize()])
    }

    #[inline]
    pub fn rook_like_pieces(&self, color: &Color) -> Bitboard {
        self.piece_bitboard[PieceType::ROOK.to_usize()]
            .union(&self.piece_bitboard[PieceType::QUEEN.to_usize()])
            .intersect(&self.color_bitboard[color.to_usize()])
    }

    #[inline]
    pub fn piece_type(&self, square: &Square) -> PieceType {
        let bitboard = Bitboard::from_square(square);

        if self.piece_bitboard[PieceType::NONE.to_usize()].intersect(&bitboard).is_empty() {
            return PieceType::NONE;
        } else if self.piece_bitboard[PieceType::PAWN.to_usize()].has(&bitboard) {
            return PieceType::PAWN;
        } else if self.piece_bitboard[PieceType::KNIGHT.to_usize()].has(&bitboard) {
            return PieceType::KNIGHT;
        } else if self.piece_bitboard[PieceType::BISHOP.to_usize()].has(&bitboard) {
            return PieceType::BISHOP;
        } else if self.piece_bitboard[PieceType::ROOK.to_usize()].has(&bitboard) {
            return PieceType::ROOK;
        } else if self.piece_bitboard[PieceType::QUEEN.to_usize()].has(&bitboard) {
            return PieceType::QUEEN;
        }
        return PieceType::KING;
    }

    #[inline]
    pub fn color_at(&self, square: &Square) -> Option<Color> {
        let bitboard = Bitboard::from_square(square);
        if self.color_bitboard[Color::White.to_usize()].has(&bitboard) {
            return Some(Color::White);
        } else if self.color_bitboard[Color::Black.to_usize()].has(&bitboard) {
            return Some(Color::Black);
        }
        None
    }

    #[inline]
    pub fn piece_bitboard(&self, color: &Color, piece_type: &PieceType) -> Bitboard {
        self.piece_bitboard[piece_type.to_usize()].intersect(&self.color_bitboard(color))
    }

    #[inline]
    pub fn color_bitboard(&self, color: &Color) -> Bitboard {
        self.color_bitboard[color.to_usize()]
    }

    #[inline]
    pub fn next_color_to_move(&self) -> Color {
        self.color_to_move.reverse()
    }

    #[inline]
    pub fn king_square(&self, color: &Color) -> Square {
        self.king_square[color.to_usize()]
    }

    #[inline]
    pub fn initial_rook_square(&self, castling_index: &CastlingIndex) -> Square {
        self.initial_rook_square[castling_index.to_usize()]
    }

    #[inline]
    fn remove_piece(&mut self, color: &Color, piece_type: &PieceType, square: &Square) {
        let bitboard = Bitboard::from_square(square);
        self.piece_bitboard[piece_type.to_usize()] =
            self.piece_bitboard[piece_type.to_usize()].difference(&bitboard);
        self.color_bitboard[color.to_usize()] =
            self.color_bitboard[color.to_usize()].difference(&bitboard);
    }

    #[inline]
    pub fn add_piece(&mut self, color: &Color, piece_type: &PieceType, square: &Square) {
        let bitboard = Bitboard::from_square(square);
        self.piece_bitboard[piece_type.to_usize()] =
            self.piece_bitboard[piece_type.to_usize()].union(&bitboard);
        self.color_bitboard[color.to_usize()] =
            self.color_bitboard[color.to_usize()].union(&bitboard);
    }

    #[inline]
    fn move_piece(&mut self, color: &Color, piece_type: &PieceType, square_from: &Square, square_to: &Square) {
        let bitboard = Bitboard::from_square(square_from)
            .union(&Bitboard::from_square(square_to));
        self.piece_bitboard[piece_type.to_usize()] =
            self.piece_bitboard[piece_type.to_usize()].invert(&bitboard);
        self.color_bitboard[color.to_usize()] =
            self.color_bitboard[color.to_usize()].invert(&bitboard);
    }

    #[inline]
    pub fn compute_zobrist(&mut self) {
        let mut zobrist_key = ZobristKey::new();
        let mut pawn_zobrist_key = ZobristKey::new();
        if self.color_to_move.is_white() {
            zobrist_key.set_color();
        }
        if self.ep_square.is_some() {
            zobrist_key.set_ep(&self.ep_square.unwrap());
        }
        for square in Square::SQUARES.iter() {
            let piece_type = self.piece_type(square);
            if piece_type == PieceType::PAWN {
                zobrist_key.change_piece(&self.color_at(square).unwrap(), &piece_type, square);
                pawn_zobrist_key.change_piece(&self.color_at(square).unwrap(), &piece_type, square);
            } else if piece_type != PieceType::NONE {
                zobrist_key.change_piece(&self.color_at(square).unwrap(), &piece_type, square);
            }
        }
        zobrist_key.set_castling_rights(&self.castling_rights);
        self.zkey = zobrist_key;
        self.zkey_pawn = pawn_zobrist_key;
    }

    #[inline]
    fn do_castle(&mut self, color: &Color, square_from: &Square, square_to: &Square) {
        let castling_side = if square_to.0 > square_from.0 {
            CastlingSide::HSide
        } else {
            CastlingSide::ASide
        };
        let castling_index = CastlingIndex::from_color_side(color, &castling_side);
        let square_rook_from = self.initial_rook_square[castling_index.to_usize()];
        let square_rook_to = castling_index.square_rook_to();

        self.move_piece(color, &PieceType::KING, &square_from, &square_to);
        self.move_piece(color, &PieceType::ROOK, &square_rook_from, &square_rook_to);
    }

    #[inline]
    fn clear_ep(&mut self) {
        if self.ep_square.is_some() {
            self.zkey.set_ep(&self.ep_square.unwrap());
            self.ep_square = None;
        }
    }

    pub fn do_move(&mut self, board_move: &BoardMove) -> bool {
        self.rule_50 += 1;

        let square_from = board_move.square_from();
        let square_to = board_move.square_to();
        let piece_type = self.piece_type(&square_from);
        let mut piece_captured = self.piece_type(&square_to);
        let move_type = board_move.move_type();

        let color_our = self.color_to_move;
        let color_their = color_our.reverse();

        self.zkey.set_color();
        self.zkey.move_piece(&color_our, &piece_type, &square_from, &square_to);

        // Castling needs to move two pieces
        if move_type.is_castling() {
            self.do_castle(&color_our, &square_from, &square_to);
            self.zkey.move_piece(&color_our, &PieceType::ROOK, &square_from, &square_to);
        } else {
            let mut square_captured = square_to;
            if move_type.is_passant() {
                square_captured = square_captured.forward(&color_their);
                piece_captured = PieceType::PAWN;
            }
            if piece_captured != PieceType::NONE {
                if piece_captured == PieceType::PAWN {
                    self.zkey_pawn.change_piece(&color_their, &piece_captured, &square_captured);
                }
                self.remove_piece(&color_their, &piece_captured, &square_captured);
                self.zkey.change_piece(&color_their, &piece_captured, &square_captured);
                self.rule_50 = 0
            }

            self.move_piece(&color_our, &piece_type, &square_from, &square_to);
        }

        self.clear_ep();

        if piece_type == PieceType::PAWN {
            self.zkey_pawn.change_piece(&color_our, &piece_type, &square_from);
            let promoted_piece = move_type.promoted_piece_type();
            if promoted_piece != PieceType::NONE {
                self.remove_piece(&color_our, &PieceType::PAWN, &square_to);
                self.add_piece(&color_our, &promoted_piece, &square_to);
                self.zkey.change_piece(&color_our, &piece_type, &square_to);
                self.zkey.change_piece(&color_our, &promoted_piece, &square_to);
            } else {
                if square_from.0 ^ square_to.0 == 16 &&
                    square_to.neighbour().intersect(&self.piece_bitboard(&color_their, &PieceType::PAWN)).is_not_empty() {
                    let square_ep = square_from.forward(&color_our);
                    self.ep_square = Some(square_ep);
                    self.zkey.set_ep(&square_ep);
                }
                self.zkey_pawn.change_piece(&color_our, &piece_type, &square_to);
            }
            self.rule_50 = 0;
        } else if piece_type == PieceType::KING {
            self.king_square[color_our.to_usize()] = square_to;
        }

        self.update_castling_rights(&square_from, &square_to);
        self.color_to_move = color_their;

        self.first_pass();
        if self.check_bitboard.is_not_empty() {
            return false;
        }
        self.second_pass();
        return true;
    }

    #[inline]
    pub fn initial_pass(&mut self) {
        self.first_pass();
        self.second_pass();
    }

    #[inline]
    fn first_pass(&mut self) {
        self.piece_bitboard[PieceType::NONE.to_usize()] = self.color_bitboard[Color::White.to_usize()]
            .union(&self.color_bitboard[Color::Black.to_usize()]);
        let previous_color = &self.color_to_move.reverse();
        self.update_danger_bitboard(previous_color);
        self.set_check_bitboard(previous_color);
    }

    #[inline]
    fn second_pass(&mut self) {
        self.pinned_bitboard.clear();
        for color in Color::COLORS.iter() {
            self.set_pinned(&color);
        }
        let color = self.color_to_move;
        self.update_danger_bitboard(&color);
        self.set_check_bitboard(&color);
    }

    #[inline]
    fn set_pinned(&mut self, color: &Color) {
        let their_color = color.reverse();
        if self.slider_pieces(&their_color).is_not_empty() {
            let our_bitboard = self.color_bitboard(color);
            let mut pinned = Bitboard::EMPTY;
            let king_square = self.king_square(color);

            let game_bitboard = self.game_bitboard();

            let possible_pin = self
                .bishop_like_pieces(&their_color)
                .intersect(&king_square.pseudo_bishop_moves())
                .union(&self.rook_like_pieces(&their_color).intersect(&king_square.pseudo_rook_moves()));

            for square in possible_pin.iterator() {
                let between_piece = king_square.between(&square).intersect(&game_bitboard);
                if between_piece.is_not_empty() && between_piece.one_element() {
                    pinned = pinned.union(&between_piece.intersect(&our_bitboard))
                }
            }
            self.pinned_bitboard = self.pinned_bitboard.union(&pinned)
        }
    }

    #[inline]
    fn update_danger_bitboard(&mut self, color: &Color) {
        let king_square = self.king_square(color);

        self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] = king_square.pawn_attacks(&color);
        self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()] = king_square.knight_moves();
        self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()] = king_square.bishop_moves(&self.game_bitboard());
        self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()] = king_square.rook_moves(&self.game_bitboard());
        self.danger_bitboard[color.to_usize()][PieceType::QUEEN.to_usize()] =
            self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()].union(
                &self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()]
            );

        self.danger_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .union(&self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()])
                .union(&self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()])
                .union(&self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()])
    }

    #[inline]
    fn set_check_bitboard(&mut self, color: &Color) {
        let our_color = color;
        let their_color = our_color.reverse();
        self.check_bitboard = self.danger_bitboard[our_color.to_usize()][PieceType::PAWN.to_usize()]
            .intersect(&self.piece_bitboard(&their_color, &PieceType::PAWN))
            .union(&self.danger_bitboard[our_color.to_usize()][PieceType::KNIGHT.to_usize()]
                .intersect(&self.piece_bitboard(&their_color, &PieceType::KNIGHT)))
            .union(&self.danger_bitboard[our_color.to_usize()][PieceType::BISHOP.to_usize()]
                .intersect(&self.bishop_like_pieces(&their_color)))
            .union(&self.danger_bitboard[our_color.to_usize()][PieceType::ROOK.to_usize()]
                .intersect(&self.rook_like_pieces(&their_color)));
    }

    #[inline]
    pub fn update_castling_rights(&mut self, square_from: &Square, square_to: &Square) {
        self.zkey.set_castling_rights(&self.castling_rights);

        let right_change = self.castling_rights_masks[square_from.to_usize()]
            .union(&self.castling_rights_masks[square_to.to_usize()]);
        self.castling_rights = self.castling_rights.difference(&right_change);
        self.zkey.set_castling_rights(&self.castling_rights);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.color_bitboard == other.color_bitboard &&
            self.piece_bitboard == other.piece_bitboard &&
            self.castling_rights == other.castling_rights &&
            self.color_to_move == other.color_to_move &&
            self.move_number == other.move_number &&
            self.rule_50 == other.rule_50
    }
}

impl Eq for Board {}

impl Debug for Board {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let mut res_str: String = String::new();
        res_str.push_str(&format!("Color: {}\n", self.color_to_move.to_char()));
        res_str.push_str(&format!("White BB: {}\n", self.color_bitboard(&Color::White).to_string()));
        res_str.push_str(&format!("Black BB: {}\n", self.color_bitboard(&Color::Black).to_string()));
        write!(formatter, "{}", res_str)
    }
}

#[cfg(test)]
mod test {
    use crate::cached::board::Board;
    use crate::types::bitboard::Bitboard;
    use crate::types::color::Color;
    use crate::types::piece_type::PieceType;

    use super::*;

    #[test]
    fn default_board() {
        let board = Board::default();
        assert_eq!(board.color_to_move.is_white(), true);

        assert_eq!(board.color_at(&Square::A1).unwrap().is_white(), true);
        assert_eq!(board.color_at(&Square::A2).unwrap().is_white(), true);
        assert_eq!(board.color_at(&Square::A3).is_none(), true);
        assert_eq!(board.color_at(&Square::A4).is_none(), true);
        assert_eq!(board.color_at(&Square::A5).is_none(), true);
        assert_eq!(board.color_at(&Square::A6).is_none(), true);
        assert_eq!(board.color_at(&Square::A7).unwrap().is_white(), false);
        assert_eq!(board.color_at(&Square::A8).unwrap().is_white(), false);

        assert_eq!(board.piece_type(&Square::A1), PieceType::ROOK);
        assert_eq!(board.piece_type(&Square::A2), PieceType::PAWN);
        assert_eq!(board.piece_type(&Square::A3), PieceType::NONE);
        assert_eq!(board.piece_type(&Square::A4), PieceType::NONE);
        assert_eq!(board.piece_type(&Square::A5), PieceType::NONE);
        assert_eq!(board.piece_type(&Square::A6), PieceType::NONE);
        assert_eq!(board.piece_type(&Square::A7), PieceType::PAWN);
        assert_eq!(board.piece_type(&Square::A8), PieceType::ROOK);
    }

    #[test]
    fn pinned() {
        let mut board = Board::default();
        board.set_check_bitboard(&Color::White);
        assert_eq!(board.check_bitboard.is_empty(), true);
    }

    #[test]
    fn danger() {
        let mut board = Board::default();
        board.set_pinned(&Color::White);
        assert_eq!(board.pinned_bitboard, Bitboard::EMPTY);
        board.pinned_bitboard.clear();
        board.set_pinned(&Color::Black);
        assert_eq!(board.pinned_bitboard, Bitboard::EMPTY);
        board.pinned_bitboard.clear();
    }

    #[test]
    fn check_bitboard() {
        let mut board = Board::default();
        board.update_danger_bitboard(&Color::White);
        board.update_danger_bitboard(&Color::Black);
        assert_eq!(board.danger_bitboard[Color::White.to_usize()][PieceType::PAWN.to_usize()],
                   board.king_square(&Color::White).pawn_attacks(&Color::White));
        assert_eq!(board.danger_bitboard[Color::White.to_usize()][PieceType::KNIGHT.to_usize()],
                   board.king_square(&Color::White).knight_moves());
    }
}