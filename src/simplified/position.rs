use std::cmp;
use std::cmp::max;

use crate::advanced::zobrist_key::ZobristKey;
use crate::simplified::position_state::PositionState;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::castling_rights::{CastlingIndex, CastlingRights, CastlingSide};
use crate::types::color::Color;
use crate::types::file::File;
use crate::types::move_type::MoveType;
use crate::types::piece_type::PieceType;
use crate::types::rank::Rank;
use crate::types::square::Square;

// Position encodes all positional information
#[derive(Clone)]
pub struct Position {
    // Position castling information
    initial_rook_square: [Square; CastlingIndex::NUM_INDEXES],
    castling_rights_masks: [CastlingRights; Square::NUM_SQUARES],

    color_to_move: Color,
    move_number: u16,

    piece_type_board: [PieceType; Square::NUM_SQUARES],

    color_bitboard: [Bitboard; Color::NUM_COLORS],
    piece_bitboard: [Bitboard; PieceType::NUM_PIECE_TYPES],

    king_square: [Square; Color::NUM_COLORS],

    // Irreversible position info (should be cached in history)
    state: PositionState,
}

impl Position {
    pub const DEFAULT_FEN: &'static str = "rnbqkbnr/pppppp1p/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq -";
    const EMPTY_SPACE: char = ' ';
    const SEPARATOR: char = '/';

    #[inline]
    pub fn empty() -> Self {
        let mut result = Self {
            initial_rook_square: [Square::H1, Square::A1, Square::H8, Square::A8],
            castling_rights_masks: [CastlingRights::NO_CASTLING; Square::NUM_SQUARES],

            color_to_move: Color::White,
            move_number: 0,

            piece_type_board: [PieceType::NONE; Square::NUM_SQUARES],
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

            state: PositionState::new(),
        };
        return result;
    }

    pub fn default() -> Self {
        Position::from_fen(Position::DEFAULT_FEN)
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut result = Position::empty();
        let mut file: usize = File::FILE_A.to_usize();
        let mut rank: usize = Rank::RANK_8.to_usize();

        let mut tokens = fen.split(Position::EMPTY_SPACE);

        for token in tokens.next().unwrap().chars() {
            if token == Position::SEPARATOR {
                file = File::FILE_A.to_usize();
                rank -= 1;
            } else if token >= '1' && token <= '8' {
                let skip = (token as u8 - b'0') as usize;
                file += skip;
            } else {
                let (color, piece_type) = PieceType::from_char(token);
                result.add_piece(color, piece_type, Square::from_file_rank(File::FILES[file], Rank::RANKS[rank]));
                file += 1;
            }
        }

        let color_to_move = Color::from_string(tokens.next().unwrap()).unwrap();

        let castling_rights = CastlingRights::from_string(tokens.next().unwrap());

        let ep_square = Square::from_string(tokens.next().unwrap());

        let rule50 = tokens.next();
        if rule50.is_some() {
            result.state.rule_50 = rule50.unwrap().parse().unwrap();
        }

        let move_number = tokens.next();
        if move_number.is_some() {
            result.move_number = max(Color::NUM_COLORS as u16 * (move_number.unwrap().parse::<u16>().unwrap().wrapping_sub(Color::Black.to_u16())), 0) as u16 + color_to_move.to_u16();
        }

        result.color_to_move = color_to_move;
        result.state.castling_rights = castling_rights;
        result.state.ep_square = ep_square;

        result.setup();

        return result;
    }

//    pub fn to_fen(&self) -> String {
//        let mut result = String::new();
//
//        let mut empty_squares: u32 = 0;
//
//        // Board piece representation
//        for rank in Rank::RANKS.iter().rev() {
//            for file in File::FILES.iter() {
//                let square = Square::from_file_rank(file, rank);
//                let bitboard = Bitboard::from_square(&square);
//                let piece_type = self.piece_type(&square);
//                if piece_type == PieceType::NONE {
//                    empty_squares += 1;
//                    continue;
//                }
//                if empty_squares > 0 {
//                    result.push(std::char::from_digit(empty_squares, 10).unwrap());
//                    empty_squares = 0
//                }
//
//
//                result.push(piece_type.to_char_colored(self.color_at(&square).unwrap()));
//            }
//            if empty_squares > 0 {
//                result.push(std::char::from_digit(empty_squares, 10).unwrap());
//                empty_squares = 0
//            }
//
//            if rank.0 != Rank::RANK_1.0 {
//                result.push(Position::SEPARATOR);
//            }
//        }
//
//        result.push(Position::EMPTY_SPACE);
//        result.push(self.color_to_move.to_char());
//        result.push(Position::EMPTY_SPACE);
//        result.push_str(self.castling_rights.to_string().as_str());
//        result.push(Position::EMPTY_SPACE);
//        if let Some(ep_square) = self.ep_square {
//            result.push_str(ep_square.to_string().as_str());
//        } else {
//            result.push_str("-");
//        }
//        result.push(Position::EMPTY_SPACE);
//        result.push_str(self.rule_50.to_string().as_str());
//        result.push(Position::EMPTY_SPACE);
//        result.push_str(cmp::max(1, (self.move_number as i16 - self.color_to_move.to_i16()) / 2).to_string().as_str());
//        return result;
//    }

    pub fn setup(&mut self) {
        self.castling_rights_masks[self.king_square[Color::White.to_usize()].to_usize()] = CastlingRights::WHITE_RIGHTS;
        self.castling_rights_masks[self.king_square[Color::Black.to_usize()].to_usize()] = CastlingRights::BLACK_RIGHTS;

        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::WhiteA.to_usize()].to_usize()] = CastlingRights::WHITE_OOO;
        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::WhiteH.to_usize()].to_usize()] = CastlingRights::WHITE_OO;
        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::BlackA.to_usize()].to_usize()] = CastlingRights::BLACK_OOO;
        self.castling_rights_masks[self.initial_rook_square[CastlingIndex::BlackH.to_usize()].to_usize()] = CastlingRights::BLACK_OO;
    }

    #[inline]
    fn remove_piece(&mut self, color: Color, piece_type: PieceType, square: Square) {
        let bitboard = Bitboard::from(square);
        self.piece_bitboard[piece_type] = self.piece_bitboard[piece_type].difference(bitboard);
        self.color_bitboard[color] = self.color_bitboard[color].difference(bitboard);

        self.piece_type_board[square] = PieceType::NONE;

        // Updating zobrist info
        self.state.change_piece(color, piece_type, square);
    }

    #[inline]
    pub fn add_piece(&mut self, color: Color, piece_type: PieceType, square: Square) {
        let bitboard = Bitboard::from(square);
        self.piece_bitboard[piece_type] = self.piece_bitboard[piece_type].union(bitboard);
        self.color_bitboard[color] = self.color_bitboard[color].union(bitboard);

        self.piece_type_board[square] = piece_type;

        // Updating zobrist info
        self.state.change_piece(color, piece_type, square);
        if piece_type == PieceType::KING {
            self.king_square[color] = square;
        }
    }

    #[inline]
    fn move_piece(&mut self, color: Color, piece_type: PieceType, from: Square, to: Square) {
        let bitboard = Bitboard::from(from).union(Bitboard::from(to));
        self.piece_bitboard[piece_type] = self.piece_bitboard[piece_type].invert(bitboard);
        self.color_bitboard[color] = self.color_bitboard[color].invert(bitboard);

        self.piece_type_board[from] = PieceType::NONE;
        self.piece_type_board[to] = piece_type;

        // Updating zobrist info
        self.state.move_piece(color, piece_type, from, to);
        if piece_type == PieceType::KING {
            self.king_square[color] = to;
        }
    }

    #[inline]
    pub fn color_at(&self, square: Square) -> Option<Color> {
        return if self.color_bitboard[Color::White] == Bitboard::from(square) {
            Some(Color::White)
        } else if self.color_bitboard[Color::Black] == Bitboard::from(square) {
            Some(Color::Black)
        } else {
            None
        };
    }

    #[inline]
    pub fn piece_bitboard(&self, color: Color, piece_type: PieceType) -> Bitboard {
        self.piece_bitboard[piece_type].intersect(self.color_bitboard(color))
    }

    #[inline]
    pub fn color_bitboard(&self, color: Color) -> Bitboard {
        self.color_bitboard[color]
    }

    #[inline]
    pub fn piece_at(&self, square: Square) -> PieceType {
        self.piece_type_board[square]
    }

    #[inline]
    pub fn update_castling_rights(&mut self, square_from: Square, square_to: Square) {
        let right_change = self.castling_rights_masks[square_from.to_usize()]
            .union(self.castling_rights_masks[square_to.to_usize()]);

        self.state.update_castling_rights(right_change);
    }

    // Validate pseudo legal moves (Castle, Passant, Leave king under check)
    #[inline]
    pub fn is_legal_move(&self, board_move: &BoardMove) -> bool {
        let move_type = board_move.move_type();
        let color_our = self.color_to_move;
        let color_their = color_our.reverse();
        return match move_type {
            MoveType::NORMAL => {
                //Check if king is under attack after board changes
                let piece_type = self.piece_type_board[board_move.square_from()];
                if piece_type == PieceType::KING {
                    let our_bitboard = self.color_bitboard(color_our).invert(Bitboard::from(board_move.square_from()));
                    let their_bitboard = self.color_bitboard(color_their);
                    board_move.square_to().attacks_to(self, color_our, our_bitboard, their_bitboard).is_empty()
                } else {
                    let our_bitboard = self.color_bitboard(color_our)
                        .invert(Bitboard::from(board_move.square_from()))
                        .union(Bitboard::from(board_move.square_to()));
                    let their_bitboard = self.color_bitboard(color_their)
                        .difference(Bitboard::from(board_move.square_to()));
                    self.king_square[color_our].attacks_to(self, color_our, our_bitboard, their_bitboard).is_empty()
                }
            }
            MoveType::PASSANT => {
                //Check if king is under attack after board changes
                let our_bitboard = self.color_bitboard(color_our)
                    .invert(Bitboard::from(board_move.square_from()))
                    .union(Bitboard::from(board_move.square_to()));
                let their_bitboard = self.color_bitboard(color_their)
                    .difference(Bitboard::from(board_move.square_to().forward(color_their)));
                self.king_square[color_our].attacks_to(self, color_our, our_bitboard, their_bitboard).is_empty()
            }
            MoveType::CASTLING => {
                let path = board_move.square_from().between(board_move.square_to())
                    .with_square(board_move.square_to());
                !path.attacks_to(self, color_our, self.color_bitboard[color_our], self.color_bitboard[color_their])
            }
            // PROMOTIONS
            _ => {
                let our_bitboard = self.color_bitboard(color_our)
                    .invert(Bitboard::from(board_move.square_from()))
                    .union(Bitboard::from(board_move.square_to()));
                let their_bitboard = self.color_bitboard(color_their)
                    .difference(Bitboard::from(board_move.square_to()));
                self.king_square[color_our].attacks_to(self, color_our, our_bitboard, their_bitboard).is_empty()
            }
        };
    }

    #[inline]
    pub fn do_move(&mut self, board_move: &BoardMove) -> bool {
        self.state.rule_50 += 1;

        let square_from = board_move.square_from();
        let square_to = board_move.square_to();
        let piece_type = self.piece_type_board[square_from];
        let move_type = board_move.move_type();

        let color_our = self.color_to_move;
        let color_their = color_our.reverse();

        match move_type {
            MoveType::NORMAL => {
                self.state.clear_ep();
                let piece_captured = self.piece_type_board[square_to];
                if piece_captured != PieceType::NONE {
                    self.remove_piece(color_their, piece_captured, square_to);
                    self.state.rule_50 = 0;
                }
                self.move_piece(color_our, piece_type, square_from, square_to);
                if piece_type == PieceType::PAWN {
                    self.state.rule_50 = 0;
                    if square_from.0 ^ square_to.0 == 16 {
                        self.state.set_ep(square_from.forward(color_our));
                    }
                }
            }
            MoveType::PASSANT => {
                debug_assert!(self.state.ep_square.is_some());
                self.remove_piece(color_their, PieceType::PAWN, square_to.forward(color_their));
                self.move_piece(color_our, PieceType::PAWN, square_from, square_to);
                self.state.clear_ep();
            }
            MoveType::CASTLING => {
                self.state.clear_ep();
                self.do_castle(color_our, square_from, square_to);
            }
            // PROMOTIONS
            _ => {
                self.state.clear_ep();
                let promoted_piece = move_type.promoted_piece_type();
                let piece_captured = self.piece_type_board[square_to];
                debug_assert_ne!(promoted_piece, PieceType::NONE);
                if piece_captured != PieceType::NONE {
                    self.remove_piece(color_their, piece_captured, square_to);
                    self.state.rule_50 = 0;
                }
                self.remove_piece(color_our, PieceType::PAWN, square_from);
                self.add_piece(color_our, promoted_piece, square_to);
                self.state.rule_50 = 0;
            }
        }
        self.update_castling_rights(square_from, square_to);

        self.color_to_move = color_their;
        self.state.zkey.change_color();

        self.set_checkbitboard();
        return true;
    }

    #[inline]
    fn do_castle(&mut self, color: Color, square_from: Square, square_to: Square) {
        let castling_side = if square_to.0 > square_from.0 {
            CastlingSide::HSide
        } else {
            CastlingSide::ASide
        };
        let castling_index = CastlingIndex::from_color_side(color, castling_side);
        let square_rook_from = self.initial_rook_square[castling_index.to_usize()];
        let square_rook_to = castling_index.square_rook_to();

        self.move_piece(color, PieceType::KING, square_from, square_to);
        self.move_piece(color, PieceType::ROOK, square_rook_from, square_rook_to);
    }

    #[inline]
    fn set_checkbitboard(&mut self) {
        self.state.check_bitboard = self.king_square[self.color_to_move]
            .attacks_to(self, self.color_to_move, self.color_bitboard[self.color_to_move], self.color_bitboard[self.color_to_move.reverse()]);
    }

    #[inline]
    fn bishop_like_pieces(&self) -> Bitboard {
        self.piece_bitboard[PieceType::BISHOP].union(self.piece_bitboard[PieceType::QUEEN])
    }

    #[inline]
    fn rook_like_pieces(&self) -> Bitboard {
        self.piece_bitboard[PieceType::ROOK].union(self.piece_bitboard[PieceType::QUEEN])
    }

    #[inline]
    pub fn state(&self) -> &PositionState {
        &self.state
    }

    #[inline]
    pub fn ctm(&self) -> Color {
        self.color_to_move
    }

    #[inline]
    pub fn game_bitboard(&self) -> Bitboard {
        self.color_bitboard[Color::White].union(self.color_bitboard[Color::Black])
    }

    #[inline]
    pub fn empty_bitboard(&self) -> Bitboard {
        self.game_bitboard().reverse()
    }

    #[inline]
    pub fn king_square(&self, color: Color) -> Square {
        self.king_square[color]
    }

    #[inline]
    pub fn rook_from(&self, castling_index: CastlingIndex) -> Square {
        self.initial_rook_square[castling_index.to_usize()]
    }
}

impl Bitboard {

    #[inline]
    pub fn attacks_to(&self, position: &Position, defending_color: Color, our_bitboard: Bitboard, their_bitboard: Bitboard) -> bool {
        for square in self.iterator() {
            if square.attacks_to(position, defending_color, our_bitboard, their_bitboard).is_not_empty() {
                return true;
            }
        }
        return false;
    }
}

impl Square {

    #[inline]
    pub fn attacks_to(&self, position: &Position, defending_color: Color, our_bitboard: Bitboard, their_bitboard: Bitboard) -> Bitboard {
        let bitboard = our_bitboard.union(their_bitboard);
        self.pawn_attacks(defending_color).intersect(position.piece_bitboard[PieceType::PAWN])
            .union(self.knight_moves().intersect(position.piece_bitboard[PieceType::KNIGHT]))
            .union(self.bishop_moves(bitboard).intersect(position.bishop_like_pieces()))
            .union(self.rook_moves(bitboard).intersect(position.rook_like_pieces()))
            .union(self.king_moves().intersect(position.piece_bitboard[PieceType::KING]))
            .intersect(their_bitboard)
    }
}


#[cfg(test)]
mod test {
    use crate::types::bitboard::Bitboard;
    use crate::types::color::Color;
    use crate::types::piece_type::PieceType;

    use super::*;

    #[test]
    fn default_board() {
        let position = Position::from_fen(Position::DEFAULT_FEN);
        assert_eq!(position.color_to_move, Color::White);

        assert_eq!(position.color_at(Square::A1).unwrap(), Color::White);
        assert_eq!(position.color_at(Square::A2).unwrap(), Color::White);
        assert_eq!(position.color_at(Square::A3), None);
        assert_eq!(position.color_at(Square::A4), None);
        assert_eq!(position.color_at(Square::A5), None);
        assert_eq!(position.color_at(Square::A6), None);
        assert_eq!(position.color_at(Square::A7).unwrap(), Color::Black);
        assert_eq!(position.color_at(Square::A8).unwrap(), Color::Black);

        assert_eq!(position.piece_at(Square::A1), PieceType::ROOK);
        assert_eq!(position.piece_at(Square::A2), PieceType::PAWN);
        assert_eq!(position.piece_at(Square::A3), PieceType::NONE);
        assert_eq!(position.piece_at(Square::A4), PieceType::NONE);
        assert_eq!(position.piece_at(Square::A5), PieceType::NONE);
        assert_eq!(position.piece_at(Square::A6), PieceType::NONE);
        assert_eq!(position.piece_at(Square::A7), PieceType::PAWN);
        assert_eq!(position.piece_at(Square::A8), PieceType::ROOK);
    }

    #[test]
    fn move_piece() {
        let mut position = Position::from_fen(Position::DEFAULT_FEN);
        let zkey = position.state.zkey.clone();
        position.move_piece(Color::White, PieceType::ROOK, Square::A1, Square::A3);
        assert_ne!(zkey, position.state.zkey);
        assert_eq!(position.color_at(Square::A1), None);
        assert_eq!(position.piece_at(Square::A1), PieceType::NONE);

        assert_eq!(position.color_at(Square::A3).unwrap(), Color::White);
        assert_eq!(position.piece_at(Square::A3), PieceType::ROOK);
    }

    #[test]
    fn move_pawn() {
        let mut position = Position::from_fen(Position::DEFAULT_FEN);
        let zkey_pawn = position.state.zkey_pawn.clone();
        position.move_piece(Color::White, PieceType::PAWN, Square::A2, Square::A3);
        assert_ne!(zkey_pawn, position.state.zkey_pawn);
        assert_eq!(position.color_at(Square::A2), None);
        assert_eq!(position.piece_at(Square::A2), PieceType::NONE);

        assert_eq!(position.color_at(Square::A3).unwrap(), Color::White);
        assert_eq!(position.piece_at(Square::A3), PieceType::PAWN);
    }
}