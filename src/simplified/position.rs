use std::cmp;
use std::cmp::max;

use crate::advanced::zobrist_key::ZobristKey;
use crate::simplified::position_state::PositionState;
use crate::types::bitboard::Bitboard;
use crate::types::castling_rights::{CastlingIndex, CastlingRights};
use crate::types::color::Color;
use crate::types::file::File;
use crate::types::piece_type::PieceType;
use crate::types::rank::Rank;
use crate::types::square::Square;

// Position encodes all positional information
#[derive(Clone)]
pub struct Position {
    // Position castling information
    initial_rook_square: [Square; CastlingIndex::NUM_INDEXES],
    castling_rights_masks: [CastlingRights; Square::NUM_SQUARES],

    color_bitboard: [Bitboard; Color::NUM_COLORS],
    piece_bitboard: [Bitboard; PieceType::NUM_PIECE_TYPES],

    king_square: [Square; Color::NUM_COLORS],

    state: PositionState,
}

impl Position {
    const EMPTY_SPACE: char = ' ';
    const SEPARATOR: char = '/';

    #[inline]
    pub fn empty() -> Self {
        let mut result = Self {
            state: PositionState::new(),
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
            initial_rook_square: [Square::H1, Square::A1, Square::H8, Square::A8],
            castling_rights_masks: [CastlingRights::NO_CASTLING; Square::NUM_SQUARES],
        };
        return result;
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut result = Position::empty();
        let mut file: usize = File::FILE_A.to_usize();
        let mut rank: usize = Rank::RANK_8.to_usize();

        let mut tokens = fen.split(' ');

        for token in tokens.next().unwrap().chars() {
            if token == Position::SEPARATOR {
                file = File::FILE_A.to_usize();
                rank -= 1;
            } else if token >= '1' && token <= '8' {
                let skip = (token as u8 - b'0') as usize;
                file += skip;
            } else {
                let (piece_type, color) = PieceType::from_char(token);
                result.add_piece(&color, &piece_type, &Square::from_file_rank(&File::FILES[file], &Rank::RANKS[rank]));
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
            result.state.move_number = max(Color::NUM_COLORS * (move_number.unwrap().parse::<usize>().unwrap().wrapping_sub(Color::Black.to_usize())), 0) as usize + color_to_move.to_usize();
        }

        result.state.color_to_move = color_to_move;
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

//        result.compute_zobrist();
//        result.initial_pass();
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
}