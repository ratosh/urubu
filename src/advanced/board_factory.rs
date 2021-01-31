use crate::advanced::board::Board;
use crate::types::square::Square;
use crate::types::file::File;
use crate::types::rank::Rank;
use crate::types::piece_type::PieceType;
use crate::types::color::Color;
use crate::types::castling_rights::CastlingRights;
use std::cmp;
use std::cmp::max;

impl Board {
    const EMPTY_SPACE: char = ' ';
    const SEPARATOR: char = '/';

    pub fn from_fen(fen: &str) -> Self {
        let mut result = Board::empty();
        let mut file:usize = File::FILE_A.to_usize();
        let mut rank:usize = Rank::RANK_8.to_usize();

        let mut tokens = fen.split(' ');

        for token in tokens.next().unwrap().chars() {
            if token == Board::SEPARATOR {
                file = File::FILE_A.to_usize();
                rank -= 1;
            } else if token >= '1' && token <= '8' {
                let skip = (token as u8 - b'0') as usize;
                file += skip;
            } else {
                let (piece_type, color) = PieceType::from_char(token);
                result.add_piece(color, piece_type, Square::from_file_rank(&File::FILES[file], &Rank::RANKS[rank]));
                file += 1;
            }
        }

        let color_to_move = Color::from_string(tokens.next().unwrap());

        let castling_rights = CastlingRights::from_string(tokens.next().unwrap());

        let ep_square = Square::from_string(tokens.next().unwrap());

        let rule50 = tokens.next();
        if rule50.is_some() {
            result.rule_50 = rule50.unwrap().parse().unwrap();
        }

        let move_number = tokens.next();
        if move_number.is_some() {
            result.move_number = max(Color::NUM_COLORS * (move_number.unwrap().parse::<usize>().unwrap().wrapping_sub(Color::Black.to_usize())), 0) as u16 + color_to_move.unwrap().to_u16();
        }

        result.color_to_move = color_to_move.unwrap();
        result.castling_rights = castling_rights;
        result.ep_square = ep_square;

        result.compute_king_square();
        result.compute_zobrist();
        result.initial_pass();

        return result;
    }

    pub fn to_fen(&self) -> String {
        let mut result = String::new();

        let mut empty_squares: u32 = 0;

        // Board piece representation
        for rank in Rank::RANKS.iter().rev() {
            for file in File::FILES.iter() {
                let square = Square::from_file_rank(file, rank);
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
                result.push(Board::SEPARATOR);
            }
        }

        result.push(Board::EMPTY_SPACE);
        result.push(self.color_to_move.to_char());
        result.push(Board::EMPTY_SPACE);
        result.push_str(self.castling_rights.to_string().as_str());
        result.push(Board::EMPTY_SPACE);
        if let Some(ep_square) = self.ep_square {
            result.push_str(ep_square.to_string().as_str());
        } else {
            result.push_str("-");
        }
        result.push(Board::EMPTY_SPACE);
        result.push_str(self.rule_50.to_string().as_str());
        result.push(Board::EMPTY_SPACE);
        result.push_str(cmp::max(1, (self.move_number as i16 - self.color_to_move.to_i16()) / 2).to_string().as_str());
        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_fen(board: Board, fen: &str) {
        let from_fen = Board::from_fen(fen);
        assert_eq!(board, from_fen);
        assert_eq!(from_fen.to_fen(), fen);
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