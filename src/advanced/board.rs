use crate::advanced::board_state::BoardState;
use crate::advanced::zobrist_key::ZobristKey;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::castling_rights::{CastlingIndex, CastlingRights, CastlingRightsIterator, CastlingSide};
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(Clone)]
pub struct Board {
    pub color_bitboard: [Bitboard; Color::NUM_COLORS],
    piece_bitboard: [Bitboard; PieceType::NUM_PIECE_TYPES],

    pub color_to_move: Color,

    pub move_number: u16,
    king_square: [Square; Color::NUM_COLORS],

    pub board_state: BoardState,
    pub board_state_history: [BoardState; Board::GAME_MAX_LENGTH],

    initial_rook_square: [Square; CastlingIndex::NUM_INDEXES],
    pub castling_rights_masks: [CastlingRights; Square::NUM_SQUARES],
}

impl Board {
    pub const GAME_MAX_LENGTH: usize = 4095;

    #[inline]
    pub fn new() -> Board {
        let mut result = Board {
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
            king_square: [Square::E1, Square::E8],
            board_state: BoardState::new(),
            board_state_history: [BoardState::new(); Board::GAME_MAX_LENGTH],
            initial_rook_square: [Square::H1, Square::A1, Square::H8, Square::A8],
            castling_rights_masks: [CastlingRights::NO_CASTLING; Square::NUM_SQUARES],
        };

        result.castling_rights_masks[result.king_square[Color::White.to_usize()].to_usize()] = CastlingRights::WHITE_RIGHTS;
        result.castling_rights_masks[result.king_square[Color::Black.to_usize()].to_usize()] = CastlingRights::BLACK_RIGHTS;

        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::WhiteA.to_usize()].to_usize()] = CastlingRights::WHITE_OO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::WhiteH.to_usize()].to_usize()] = CastlingRights::WHITE_OOO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::BlackA.to_usize()].to_usize()] = CastlingRights::BLACK_OO;
        result.castling_rights_masks[result.initial_rook_square[CastlingIndex::BlackH.to_usize()].to_usize()] = CastlingRights::BLACK_OOO;

        result
    }

    #[inline]
    pub fn game_bitboard(&self) -> Bitboard {
        self.piece_bitboard[PieceType::NONE.to_usize()]
    }

    #[inline]
    pub fn empty_bitboard(&self) -> Bitboard {
        self.game_bitboard().not()
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

        if !self.piece_bitboard[PieceType::NONE.to_usize()].has(&bitboard) {
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
        self.piece_bitboard[piece_type.to_usize()].intersect(&self.color_bitboard[color.to_usize()])
    }

    #[inline]
    pub fn next_color_to_move(&self) -> Color {
        self.color_to_move.invert()
    }

    #[inline]
    pub fn king_square(&self, color: &Color) -> Square {
        self.king_square[color.to_usize()]
    }


    #[inline]
    pub fn compute_zobrist(&mut self) {
        let mut zobrist_key = ZobristKey::new();
        let mut pawn_zobrist_key = ZobristKey::new();
        if self.color_to_move.is_white() {
            zobrist_key.set_color();
        }
        if self.board_state.ep_square.is_some() {
            zobrist_key.set_ep(&self.board_state.ep_square.unwrap());
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
        zobrist_key.set_castling_rights(&self.board_state.castling_rights);
        self.board_state.zkey = zobrist_key;
        self.board_state.zkey_pawn = pawn_zobrist_key;
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
    fn add_piece(&mut self, color: &Color, piece_type: &PieceType, square: &Square) {
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
    fn push_to_history(&mut self) {
        self.board_state_history[self.move_number as usize] = self.board_state;
        self.move_number += 1;
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
        let square_rook_to = CastlingIndex::SQUARE_ROOK_TO[castling_index.to_usize()];

        self.move_piece(color, &PieceType::KING, &square_from, &square_to);
        self.move_piece(color, &PieceType::ROOK, &square_rook_from, &square_rook_to);
    }

    #[inline]
    fn undo_castle(&mut self, color: &Color, square_from: &Square, square_to: &Square) {
        let castling_side = if square_to.0 > square_from.0 {
            CastlingSide::HSide
        } else {
            CastlingSide::ASide
        };
        let castling_index = CastlingIndex::from_color_side(color, &castling_side);
        let square_rook_from = self.initial_rook_square[castling_index.to_usize()];
        let square_rook_to = CastlingIndex::SQUARE_ROOK_TO[castling_index.to_usize()];

        self.move_piece(color, &PieceType::KING, &square_to, &square_from);
        self.move_piece(color, &PieceType::ROOK, &square_rook_to, &square_rook_from);
    }

    #[inline]
    fn clear_ep(&mut self) {
        if self.board_state.ep_square.is_some() {
            self.board_state.zkey.set_ep(&self.board_state.ep_square.unwrap());
            self.board_state.ep_square = None;
        }
    }

    #[inline]
    pub fn do_move(&mut self, board_move: &BoardMove) {
        self.push_to_history();
        let mut board_state = self.board_state;
        board_state.rule_50 += 1;

        let square_from = board_move.square_from();
        let square_to = board_move.square_to();
        let piece_type = self.piece_type(&square_from);
        let mut piece_captured = self.piece_type(&square_to);
        let move_type = board_move.move_type();

        let color_our = self.color_to_move;
        let color_their = color_our.invert();

        board_state.zkey.set_color();
        board_state.zkey.move_piece(&color_our, &piece_type, &square_from, &square_to);

        // Castling needs to move two pieces
        if move_type.is_castling() {
            self.do_castle(&color_our, &square_from, &square_to);
            self.board_state.zkey.move_piece(&color_our, &PieceType::ROOK, &square_from, &square_to);
        } else {
            let mut square_captured = square_to;
            if move_type.is_passant() {
                square_captured = square_captured.forward(&color_their);
                piece_captured = PieceType::PAWN;
            }
            if piece_captured != PieceType::NONE {
                if piece_captured == PieceType::PAWN {
                    board_state.zkey_pawn.change_piece(&color_their, &piece_captured, &square_captured);
                }
                self.remove_piece(&color_their, &piece_captured, &square_captured);
                board_state.zkey.change_piece(&color_their, &piece_captured, &square_captured);
                board_state.rule_50 = 0
            }

            self.move_piece(&color_our, &piece_type, &square_from, &square_to);
            board_state.piece_captured = piece_captured;
        }

        self.clear_ep();

        if piece_type == PieceType::PAWN {
            board_state.zkey_pawn.change_piece(&color_our, &piece_type, &square_from);
            let promoted_piece = move_type.promoted_piece_type();
            if promoted_piece != PieceType::NONE {
                self.remove_piece(&color_our, &PieceType::PAWN, &square_to);
                self.add_piece(&color_our, &promoted_piece, &square_to);
                board_state.zkey.change_piece(&color_our, &piece_type, &square_to);
                board_state.zkey.change_piece(&color_our, &promoted_piece, &square_to);
            } else {
                if square_from.0 ^ square_to.0 == 16 &&
                    square_to.neighbour().intersect(&self.piece_bitboard(&color_their, &PieceType::PAWN)).is_not_empty() {
                    let square_ep = square_from.forward(&color_our);
                    board_state.ep_square = Some(square_ep);
                    board_state.zkey.set_ep(&square_ep);
                }
                board_state.zkey_pawn.change_piece(&color_our, &piece_type, &square_to);
            }
            board_state.rule_50 = 0;
        } else if piece_type == PieceType::KING {
            self.king_square[color_our.to_usize()] = square_to;
        }

        board_state.update_castling_rights(self, &square_from, &square_to);
        self.color_to_move = color_their;

        self.piece_bitboard[PieceType::NONE.to_usize()] = self.color_bitboard[Color::White.to_usize()]
            .union(&self.color_bitboard[Color::Black.to_usize()]);
        board_state.first_pass(&self);
        self.board_state = board_state;
    }

    #[inline]
    fn pop_from_history(&mut self) {
        self.move_number -= 1;
        self.board_state = self.board_state_history[self.move_number as usize];
    }

    #[inline]
    pub fn undo_move(&mut self, board_move: &BoardMove) {
        let color_their = self.color_to_move;
        let color_our = color_their.invert();

        let square_from = board_move.square_from();
        let square_to = board_move.square_to();
        let mut piece_type = self.piece_type(&square_to);
        let move_type = board_move.move_type();
        let piece_captured = self.board_state.piece_captured;

        if move_type.is_promotion() {
            self.remove_piece(&color_our, &piece_type, &square_to);
            self.add_piece(&color_our, &PieceType::PAWN, &square_to);

            piece_type = PieceType::PAWN
        }

        if move_type.is_castling() {
            self.undo_castle(&color_our, &square_from, &square_to);
        } else {
            self.move_piece(&color_our, &piece_type, &square_to, &square_from);

            if piece_captured != PieceType::NONE {
                let mut square_capture = square_to;
                if move_type.is_passant() {
                    square_capture = square_capture.forward(&color_their);
                }
                self.add_piece(&color_their, &piece_captured, &square_capture);
            }
        }

        if piece_type == PieceType::KING {
            self.king_square[color_our.to_usize()] = square_from;
        }

        self.color_to_move = color_our;
        self.piece_bitboard[PieceType::NONE.to_usize()] = self.color_bitboard[Color::White.to_usize()]
            .union(&self.color_bitboard[Color::Black.to_usize()]);

        self.pop_from_history();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_board() {
        let board = Board::new();
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
}