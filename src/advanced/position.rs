use std::fmt::{Debug, Error, Formatter};

use crate::advanced::board::GameInfo;
use crate::advanced::board_state::BoardState;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::castling_rights::{CastlingIndex, CastlingSide};
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;
use crate::advanced::zobrist_key::ZobristKey;

#[derive(Clone)]
pub struct Position {
    color_bitboard: [Bitboard; Color::NUM_COLORS],
    piece_bitboard: [Bitboard; PieceType::NUM_PIECE_TYPES],

    pub current_state: BoardState,

    king_square: [Square; Color::NUM_COLORS],
}

impl Position {
    #[inline]
    pub fn empty() -> Self {
        let mut result = Self {
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
            king_square: [Square::E1, Square::E8],
            current_state: BoardState::new(),
        };
        return result;
    }

    #[inline]
    pub fn default() -> Self {
        let mut result = Self {
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
            king_square: [Square::E1, Square::E8],
            current_state: BoardState::new(),
        };

        result.setup();
        return result;
    }

    pub fn setup(&mut self) {
        self.compute_king_square();
        self.first_pass();
        self.second_pass();
        self.compute_zobrist();
    }

    #[inline]
    pub fn compute_zobrist(&mut self) {
        let mut zobrist_key = ZobristKey::new();
        let mut pawn_zobrist_key = ZobristKey::new();
        if self.current_state.color_to_move.is_white() {
            zobrist_key.set_color();
        }
        if self.current_state.ep_square.is_some() {
            zobrist_key.set_ep(self.current_state.ep_square.unwrap());
        }
        for &square in Square::SQUARES.iter() {
            let piece_type = self.piece_type(square);
            if piece_type == PieceType::PAWN {
                let color_at = self.color_at(square).unwrap();
                zobrist_key.change_piece(color_at, piece_type, square);
                pawn_zobrist_key.change_piece(color_at, piece_type, square);
            } else if piece_type != PieceType::NONE {
                let color_at = self.color_at(square).unwrap();
                zobrist_key.change_piece(color_at, piece_type, square);
            }
        }
        zobrist_key.set_castling_rights(&self.current_state.castling_rights);

        self.current_state.zkey = zobrist_key;
        self.current_state.zkey_pawn = pawn_zobrist_key;
    }

    #[inline]
    pub fn king_square(&self, color: Color) -> Square {
        self.king_square[color.to_usize()]
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
    pub fn piece_bitboard(&self, color: Color, piece_type: PieceType) -> Bitboard {
        self.piece_bitboard[piece_type.to_usize()].intersect(self.color_bitboard(color))
    }

    #[inline]
    pub fn color_bitboard(&self, color: Color) -> Bitboard {
        self.color_bitboard[color.to_usize()]
    }

    #[inline]
    pub fn slider_pieces(&self, color: Color) -> Bitboard {
        self.piece_bitboard[PieceType::BISHOP.to_usize()]
            .union(self.piece_bitboard[PieceType::ROOK.to_usize()])
            .union(self.piece_bitboard[PieceType::QUEEN.to_usize()])
            .intersect(self.color_bitboard[color.to_usize()])
    }

    #[inline]
    pub fn bishop_like_pieces(&self, color: Color) -> Bitboard {
        self.piece_bitboard[PieceType::BISHOP.to_usize()]
            .union(self.piece_bitboard[PieceType::QUEEN.to_usize()])
            .intersect(self.color_bitboard[color.to_usize()])
    }

    #[inline]
    pub fn rook_like_pieces(&self, color: Color) -> Bitboard {
        self.piece_bitboard[PieceType::ROOK.to_usize()]
            .union(self.piece_bitboard[PieceType::QUEEN.to_usize()])
            .intersect(self.color_bitboard[color.to_usize()])
    }

    #[inline]
    pub fn piece_type(&self, square: Square) -> PieceType {
        let bitboard = Bitboard::from_square(square);

        if self.piece_bitboard[PieceType::NONE.to_usize()].intersect(bitboard).is_empty() {
            return PieceType::NONE;
        } else if self.piece_bitboard[PieceType::PAWN.to_usize()].has(bitboard) {
            return PieceType::PAWN;
        } else if self.piece_bitboard[PieceType::KNIGHT.to_usize()].has(bitboard) {
            return PieceType::KNIGHT;
        } else if self.piece_bitboard[PieceType::BISHOP.to_usize()].has(bitboard) {
            return PieceType::BISHOP;
        } else if self.piece_bitboard[PieceType::ROOK.to_usize()].has(bitboard) {
            return PieceType::ROOK;
        } else if self.piece_bitboard[PieceType::QUEEN.to_usize()].has(bitboard) {
            return PieceType::QUEEN;
        }
        return PieceType::KING;
    }

    #[inline]
    pub fn color_at(&self, square: Square) -> Option<Color> {
        let bitboard = Bitboard::from_square(square);
        if self.color_bitboard[Color::White.to_usize()].has(bitboard) {
            return Some(Color::White);
        } else if self.color_bitboard[Color::Black.to_usize()].has(bitboard) {
            return Some(Color::Black);
        }
        None
    }

    #[inline]
    fn remove_piece(&mut self, color: Color, piece_type: PieceType, square: Square) {
        let bitboard = Bitboard::from_square(square);
        self.piece_bitboard[piece_type.to_usize()] =
            self.piece_bitboard[piece_type.to_usize()].difference(bitboard);
        self.color_bitboard[color.to_usize()] =
            self.color_bitboard[color.to_usize()].difference(bitboard);
    }

    #[inline]
    pub fn add_piece(&mut self, color: Color, piece_type: PieceType, square: Square) {
        let bitboard = Bitboard::from_square(square);
        self.piece_bitboard[piece_type.to_usize()] =
            self.piece_bitboard[piece_type.to_usize()].union(bitboard);
        self.color_bitboard[color.to_usize()] =
            self.color_bitboard[color.to_usize()].union(bitboard);
    }

    #[inline]
    fn move_piece(&mut self, color: Color, piece_type: PieceType, square_from: Square, square_to: Square) {
        let bitboard = Bitboard::from_square(square_from)
            .union(Bitboard::from_square(square_to));
        self.piece_bitboard[piece_type.to_usize()] =
            self.piece_bitboard[piece_type.to_usize()].invert(bitboard);
        self.color_bitboard[color.to_usize()] =
            self.color_bitboard[color.to_usize()].invert(bitboard);
    }

    pub fn compute_king_square(&mut self) {
        for &color in Color::COLORS.iter() {
            self.king_square[color.to_usize()] = self.piece_bitboard(color, PieceType::KING).to_square();
        }
    }

    #[inline]
    fn do_castle(&mut self, color: Color, square_from: Square, square_to: Square, game_info: &GameInfo) {
        let castling_side = if square_to.0 > square_from.0 {
            CastlingSide::HSide
        } else {
            CastlingSide::ASide
        };
        let castling_index = CastlingIndex::from_color_side(color, &castling_side);
        let square_rook_from = game_info.initial_rook_square[castling_index.to_usize()];
        let square_rook_to = castling_index.square_rook_to();

        self.move_piece(color, PieceType::KING, square_from, square_to);
        self.move_piece(color, PieceType::ROOK, square_rook_from, square_rook_to);
    }

    #[inline]
    fn undo_castle(&mut self, color: Color, square_from: Square, square_to: Square, game_info: &GameInfo) {
        let castling_side = if square_to.0 > square_from.0 {
            CastlingSide::HSide
        } else {
            CastlingSide::ASide
        };
        let castling_index = CastlingIndex::from_color_side(color, &castling_side);
        let square_rook_from = game_info.initial_rook_square[castling_index.to_usize()];
        let square_rook_to = castling_index.square_rook_to();

        self.move_piece(color, PieceType::KING, square_to, square_from);
        self.move_piece(color, PieceType::ROOK, square_rook_to, square_rook_from);
    }

    pub fn do_move(&mut self, board_move: BoardMove, game_info: &GameInfo) -> (PieceType, Square, bool) {
        self.current_state.rule_50 += 1;

        let square_from = board_move.square_from();
        let square_to = board_move.square_to();
        let piece_type = self.piece_type(square_from);
        let mut piece_captured = self.piece_type(square_to);
        let move_type = board_move.move_type();

        let color_our = self.current_state.color_to_move;
        let color_their = color_our.reverse();

        self.current_state.zkey.set_color();
        self.current_state.zkey.move_piece(color_our, piece_type, square_from, square_to);

        // Castling needs to move two pieces
        let mut square_captured = square_to;
        if move_type.is_castling() {
            self.do_castle(color_our, square_from, square_to, game_info);
            self.current_state.zkey.move_piece(color_our, PieceType::ROOK, square_from, square_to);
        } else {
            if move_type.is_passant() {
                square_captured = square_captured.forward(color_their);
                piece_captured = PieceType::PAWN;
            }
            if piece_captured != PieceType::NONE {
                if piece_captured == PieceType::PAWN {
                    self.current_state.zkey_pawn.change_piece(color_their, piece_captured, square_captured);
                }
                self.remove_piece(color_their, piece_captured, square_captured);
                self.current_state.zkey.change_piece(color_their, piece_captured, square_captured);
                self.current_state.rule_50 = 0
            }

            self.move_piece(color_our, piece_type, square_from, square_to);
        }

        self.current_state.clear_ep();

        if piece_type == PieceType::PAWN {
            self.current_state.zkey_pawn.change_piece(color_our, piece_type, square_from);
            let promoted_piece = move_type.promoted_piece_type();
            if promoted_piece != PieceType::NONE {
                self.remove_piece(color_our, PieceType::PAWN, square_to);
                self.add_piece(color_our, promoted_piece, square_to);
                self.current_state.zkey.change_piece(color_our, piece_type, square_to);
                self.current_state.zkey.change_piece(color_our, promoted_piece, square_to);
            } else {
                if square_from.0 ^ square_to.0 == 16 &&
                    square_to.neighbour().intersect(self.piece_bitboard(color_their, PieceType::PAWN)).is_not_empty() {
                    let square_ep = square_from.forward(color_our);
                    self.current_state.ep_square = Some(square_ep);
                    self.current_state.zkey.set_ep(square_ep);
                }
                self.current_state.zkey_pawn.change_piece(color_our, piece_type, square_to);
            }
            self.current_state.rule_50 = 0;
        } else if piece_type == PieceType::KING {
            self.king_square[color_our.to_usize()] = square_to;
        }

        self.update_castling_rights(square_from, square_to, game_info);
        self.current_state.color_to_move = color_their;

        self.first_pass();
        if self.current_state.check_bitboard.is_not_empty() {
            return (piece_captured, square_captured, false);
        }
        self.second_pass();
        return (piece_captured, square_captured, true);
    }

    pub fn undo_move(&mut self, board_move: BoardMove, state: BoardState, piece_type_captured: PieceType, square_captured: Square, game_info: &GameInfo) {
        let color_to_move = state.color_to_move;

        let square_to = board_move.square_to();
        let square_from = board_move.square_from();
        let mut piece_type_moved = self.piece_type(square_to);
        let move_type = board_move.move_type();

        if move_type.is_promotion() {
            self.remove_piece(color_to_move, piece_type_moved, square_to);
            self.add_piece(color_to_move, PieceType::PAWN, square_to);
            piece_type_moved = PieceType::PAWN;
        }
        if move_type.is_castling() {
            self.undo_castle(color_to_move, square_from, square_to, game_info);
        } else {
            self.move_piece(color_to_move, piece_type_moved, square_to, square_from);

            if piece_type_captured != PieceType::NONE {
                self.add_piece(color_to_move.reverse(), piece_type_captured, square_captured);
            }
        }
        if piece_type_moved == PieceType::KING {
            self.king_square[color_to_move] = square_from;
        }

        self.piece_bitboard[PieceType::NONE.to_usize()] = self.color_bitboard[Color::White.to_usize()]
            .union(self.color_bitboard[Color::Black.to_usize()]);
        self.current_state = state;
    }

    #[inline]
    fn first_pass(&mut self) {
        self.piece_bitboard[PieceType::NONE.to_usize()] = self.color_bitboard[Color::White.to_usize()]
            .union(self.color_bitboard[Color::Black.to_usize()]);
        let previous_color = self.current_state.color_to_move.reverse();
        self.update_danger_bitboard(previous_color);
        self.set_check_bitboard(previous_color);
    }

    #[inline]
    fn second_pass(&mut self) {
        self.current_state.pinned_bitboard.clear();
        for &color in Color::COLORS.iter() {
            self.set_pinned(color);
        }
        let color = self.current_state.color_to_move;
        self.update_danger_bitboard(color);
        self.set_check_bitboard(color);
    }

    #[inline]
    pub fn update_castling_rights(&mut self, square_from: Square, square_to: Square, game_info: &GameInfo) {
        self.current_state.zkey.set_castling_rights(&self.current_state.castling_rights);

        let right_change = game_info.castling_rights_masks[square_from.to_usize()]
            .union(&game_info.castling_rights_masks[square_to.to_usize()]);
        self.current_state.castling_rights = self.current_state.castling_rights.difference(&right_change);
        self.current_state.zkey.set_castling_rights(&self.current_state.castling_rights);
    }

    #[inline]
    pub fn update_danger_bitboard(&mut self, color: Color) {
        self.current_state.update_danger_bitboard(self.king_square(color), self.game_bitboard(), color);
    }

    #[inline]
    pub fn set_check_bitboard(&mut self, color: Color) {
        self.current_state.set_check_bitboard(&self.piece_bitboard, self.color_bitboard[color.reverse()], color);
    }

    #[inline]
    pub fn set_pinned(&mut self, color: Color) {
        self.current_state.set_pinned(self.king_square(color), &self.piece_bitboard, self.color_bitboard[color], color);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.color_bitboard == other.color_bitboard &&
            self.piece_bitboard == other.piece_bitboard &&
            self.current_state == other.current_state
    }
}

impl Eq for Position {}

impl Debug for Position {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let mut res_str: String = String::new();
        res_str.push_str(&format!("Position: {}\n", self.to_fen()));
        write!(formatter, "{}", res_str)
    }
}

#[cfg(test)]
mod test {
    use crate::advanced::board::Board;
    use crate::advanced::position::Position;
    use crate::types::bitboard::Bitboard;
    use crate::types::color::Color;
    use crate::types::piece_type::PieceType;
    use crate::types::square::Square;

    #[test]
    fn check() {
        let mut position = Position::default();
        position.set_check_bitboard(Color::White);
        assert_eq!(position.current_state.check_bitboard.is_empty(), true);
    }

    #[test]
    fn pinned() {
        let mut position = Position::default();
        position.set_pinned(Color::White);
        assert_eq!(position.current_state.pinned_bitboard, Bitboard::EMPTY);
        position.current_state.pinned_bitboard.clear();
        position.set_pinned(Color::Black);
        assert_eq!(position.current_state.pinned_bitboard, Bitboard::EMPTY);
        position.current_state.pinned_bitboard.clear();
    }

    #[test]
    fn danger() {
        let mut position = Position::default();
        position.update_danger_bitboard(Color::White);
        position.update_danger_bitboard(Color::Black);
        assert_eq!(position.current_state.danger_bitboard[Color::White.to_usize()][PieceType::PAWN.to_usize()],
                   position.king_square(Color::White).pawn_attacks(Color::White));
        assert_eq!(position.current_state.danger_bitboard[Color::White.to_usize()][PieceType::KNIGHT.to_usize()],
                   position.king_square(Color::White).knight_moves());
    }

    #[test]
    fn pinned_pawn() {
        let mut position = Position::from_fen("rnb1kbnr/ppq1pppp/2pp4/8/Q5P1/2P5/PP1PPPBP/RNB1K1NR b KQkq -");
        position.set_pinned(Color::Black);
        assert_eq!(position.current_state.pinned_bitboard, Bitboard::C6);
    }
}
