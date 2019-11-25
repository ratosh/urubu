use crate::advanced::board::Board;
use crate::types::square::Square;
use crate::types::file::File;
use crate::types::rank::Rank;
use crate::types::piece_type::PieceType;
use crate::types::color::Color;
use crate::types::castling_rights::CastlingRights;
use std::cmp::max;

impl Board {
    const SPLITTER: char = '/';

    pub fn from_fen(fen: &str) -> Self {
        let mut result = Board::empty();
        let mut file:usize = File::FILE_A.to_usize();
        let mut rank:usize = Rank::RANK_8.to_usize();

        let mut tokens = fen.split(' ');

        for token in tokens.next().unwrap().chars() {
            if token == Board::SPLITTER {
                file = File::FILE_A.to_usize();
                rank -= 1;
            } else if token >= '1' && token <= '8' {
                let skip = (token as u8 - b'0') as usize;
                file += skip;
            } else {
                let piece_type = PieceType::from_char(token);
                let color = PieceType::piece_color(token);
                result.add_piece(&color, &piece_type, &Square::from_file_rank(&File::FILES[file], &Rank::RANKS[rank]));
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        let default = Board::default();
        let from_fen = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(default, from_fen);
    }

    #[test]
    fn partial_fen() {
        let default = Board::default();
        let from_fen = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
        assert_eq!(default, from_fen);
    }
}