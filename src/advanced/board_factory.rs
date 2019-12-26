use crate::advanced::board::Board;
use crate::types::square::Square;
use crate::types::file::File;
use crate::types::rank::Rank;
use crate::types::piece_type::PieceType;
use crate::types::color::Color;
use crate::types::castling_rights::CastlingRights;
use std::cmp;
use std::cmp::max;
use crate::types::bitboard::Bitboard;
use crate::advanced::board_state::BoardState;
use crate::advanced::position::Position;

impl Board {

    #[inline]
    pub fn from_fen(fen: &str) -> Self {
        Board::from_position(&Position::from_fen(fen))
    }

    #[inline]
    pub fn to_fen(&self) -> String {
        self.position.to_fen()
    }
}

impl Position {
    const SEPARATOR: char = '/';

    pub fn from_fen(fen: &str) -> Self {
        let mut position = Position::empty();
        let mut file:usize = File::FILE_A.to_usize();
        let mut rank:usize = Rank::RANK_8.to_usize();

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
                position.add_piece(color, piece_type, Square::from_file_rank(File::FILES[file], Rank::RANKS[rank]));
                file += 1;
            }
        }

        let color_to_move = Color::from_string(tokens.next().unwrap());

        let castling_rights = CastlingRights::from_string(tokens.next().unwrap());

        let ep_square = Square::from_string(tokens.next().unwrap());

        let rule50 = tokens.next();
        if rule50.is_some() {
            position.current_state.rule_50 = rule50.unwrap().parse().unwrap();
        }

        let move_number = tokens.next();
        if move_number.is_some() {
            position.current_state.move_number = max(Color::NUM_COLORS * (move_number.unwrap().parse::<usize>().unwrap().wrapping_sub(Color::Black.to_usize())), 0) as u16 + color_to_move.unwrap().to_u16();
        }

        position.current_state.color_to_move = color_to_move.unwrap();
        position.current_state.castling_rights = castling_rights;
        position.current_state.ep_square = ep_square;
        position.setup();

        return position;
    }

    #[inline]
    pub fn to_fen(&self) -> String {
        let mut result = String::new();

        let mut empty_squares: u32 = 0;

        // Board piece representation
        for &rank in Rank::RANKS.iter().rev() {
            for &file in File::FILES.iter() {
                let square = Square::from_file_rank(file, rank);
                let bitboard = Bitboard::from_square(square);
                let piece_type = self.piece_type(square);
                if piece_type == PieceType::NONE {
                    empty_squares += 1;
                    continue
                }
                if empty_squares > 0 {
                    result.push(std::char::from_digit(empty_squares, 10).unwrap());
                    empty_squares = 0
                }

                result.push(piece_type.to_char_colored(self.color_at(square).unwrap()));
            }
            if empty_squares > 0 {
                result.push(std::char::from_digit(empty_squares, 10).unwrap());
                empty_squares = 0
            }

            if rank.0 != Rank::RANK_1.0 {
                result.push(Position::SEPARATOR);
            }
        }

        result.push_str(self.current_state.to_fen().as_str());
        return result;
    }
}

impl BoardState {
    const EMPTY_SPACE: char = ' ';

    #[inline]
    pub fn to_fen(&self) -> String {
        let mut result = String::new();

        result.push(BoardState::EMPTY_SPACE);
        result.push(self.color_to_move.to_char());
        result.push(BoardState::EMPTY_SPACE);
        result.push_str(self.castling_rights.to_string().as_str());
        result.push(BoardState::EMPTY_SPACE);
        if let Some(ep_square) = self.ep_square {
            result.push_str(ep_square.to_string().as_str());
        } else {
            result.push_str("-");
        }
        result.push(BoardState::EMPTY_SPACE);
        result.push_str(self.rule_50.to_string().as_str());
        result.push(BoardState::EMPTY_SPACE);
        result.push_str(cmp::max(1, (self.move_number as i16 - self.color_to_move.to_i16()) / 2).to_string().as_str());
        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_fen(board: Board, fen: &str) {
        let from_fen = Board::from_fen(fen);
        assert_eq!(fen, from_fen.to_fen());
        assert_eq!(board, from_fen);
    }

    #[test]
    fn default() {
        check_fen(Board::default(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[test]
    fn partial_fen() {
        assert_eq!(Board::default(), Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"));
    }
}