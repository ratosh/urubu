use crate::simplified::position::Position;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::color::Color;
use crate::types::move_list::MoveList;
use crate::types::move_type::MoveType;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[allow(dead_code)]
impl MoveList {
    #[inline]
    pub fn generate_quiets(&mut self, position: &Position) {
        let our_color = position.ctm();
        if position.state().check_bitboard.is_empty() {
            self.generate_castling_moves(position, &our_color);
        }

        let mask = &position.get_mask(our_color).intersect(&position.empty_bitboard());
        if mask.is_not_empty() {
            self.generate_quiet_pawn_moves(position, our_color, mask);
            self.generate_moves(position,  our_color, &PieceType::KNIGHT, mask);
            self.generate_moves(position,  our_color, &PieceType::BISHOP, mask);
            self.generate_moves(position,  our_color, &PieceType::ROOK, mask);
            self.generate_moves(position,  our_color, &PieceType::QUEEN, mask);
        }
        self.generate_moves(position, our_color, &PieceType::KING, &position.empty_bitboard());
    }

    #[inline]
    fn generate_castling_moves(&mut self, position: &Position, color: &Color) {
        let possible_castling = position.state().castling_rights.color_filter(&color);
        let king_square = position.king_square(&color);
        for castling_index in possible_castling.iterator() {
            let king_to = castling_index.square_king_to();
            self.add_move(BoardMove::build_castling(&king_square, &king_to));
        }
    }

    #[inline]
    fn generate_quiet_pawn_moves(&mut self, position: &Position, color: &Color, mask: &Bitboard) {
        let their_color = color.reverse();
        let pawn_bitboard = position.piece_bitboard(&color, &PieceType::PAWN)
            .intersect(&position.empty_bitboard().pawn_forward(&their_color))
            .difference(&Bitboard::PROMOTION[color]);

        for square in pawn_bitboard.iterator() {
            let move_bitboard = square.pawn_move(&color)
                    .union(&square.pawn_double_move(&color))
                    .intersect(mask);

            self.register_moves_from_square(&color, &square, &move_bitboard);
        }
    }

    #[inline]
    fn generate_moves(&mut self, position: &Position, color: &Color, piece_type: &PieceType, mask: &Bitboard) {
        for square in position.piece_bitboard(&color, piece_type).iterator() {
            let bitboard = match piece_type {
                &PieceType::KNIGHT => {
                    square.knight_moves()
                },
                &PieceType::BISHOP => {
                    square.bishop_moves(position.game_bitboard())
                },
                &PieceType::ROOK => {
                    square.rook_moves(position.game_bitboard())
                },
                &PieceType::QUEEN => {
                    square.bishop_moves(position.game_bitboard()).union(&square.rook_moves(position.game_bitboard()))
                },
                &PieceType::KING => {
                    square.king_moves()
                },
                _ => {
                    Bitboard::EMPTY
                }
            };
            self.register_moves_from_square(&color, &square, &mask.intersect(&bitboard))
        }
    }

    #[inline]
    fn register_moves_from_square(&mut self, color: &Color, square: &Square, bitboard: &Bitboard) {
        for square_to in bitboard.iterator() {
            let board_move = BoardMove::build_normal(square, &square_to);
            self.add_move(board_move);
        }
    }

    #[inline]
    pub fn generate_noisy(&mut self, position: &Position) {
        let color = position.ctm();
        let their_color = color.reverse();
        let mask = &position.get_mask(color);
        let capture_mask = &mask.intersect(&position.color_bitboard(&their_color));
        if mask.is_not_empty() {
            self.generate_capture_promotions(position, color, capture_mask);
            self.generate_quiet_promotions(position, color, mask);
            self.generate_pawn_capture(position, color, capture_mask);
            self.generate_moves(position, color, &PieceType::KNIGHT, capture_mask);
            self.generate_moves(position, color, &PieceType::BISHOP, capture_mask);
            self.generate_moves(position, color, &PieceType::ROOK, capture_mask);
            self.generate_moves(position, color, &PieceType::QUEEN, capture_mask);
            self.generate_ep_capture(position, &mask);
        }
        self.generate_moves(position, color, &PieceType::KING, &position.color_bitboard(&their_color));
    }

    #[inline]
    fn generate_ep_capture(&mut self, position: &Position, mask: &Bitboard) {
        if let Some(ep_square) = position.state().ep_square {
            let color = position.ctm();
            let their_color = color.reverse();

            if Bitboard::from(&ep_square.forward(&their_color)).intersect(mask).is_empty() {
                return;
            }

            let bitboard = ep_square.pawn_attacks(&their_color)
                .intersect(&position.piece_bitboard(&color, &PieceType::PAWN));

            for square in bitboard.iterator() {
                let board_move = BoardMove::build_passant(&square, &ep_square);
                self.add_move(board_move);
            }
        }
    }

    #[inline]
    fn generate_pawn_capture(&mut self, position: &Position, color: &Color, mask: &Bitboard) {
        let their_color = color.reverse();
        let pawn_bitboard = position.piece_bitboard(&color, &PieceType::PAWN)
            .intersect(&Bitboard::PROMOTION[color.to_usize()].reverse())
            .intersect(&position.color_bitboard(&their_color).intersect(mask).pawn_attacks(&their_color));

        for square in pawn_bitboard.iterator() {
            let bitboard_from = Bitboard::from(&square);
            let mut bitboard_to = square.pawn_attacks(&color).intersect(mask);

            self.register_moves_from_square(&color, &square, &bitboard_to);
        }
    }

    #[inline]
    fn generate_capture_promotions(&mut self, position: &Position, color: &Color, mask: &Bitboard) {
        let their_color = color.reverse();
        let pawn_bitboard = position.piece_bitboard(&color, &PieceType::PAWN)
            .intersect(&Bitboard::PROMOTION[color])
            .intersect(&position.color_bitboard(&their_color).intersect(mask).pawn_attacks(&their_color));

        for square in pawn_bitboard.iterator() {
            let bitboard_to = square.pawn_attacks(&color)
                .intersect(&position.color_bitboard(&their_color))
                .intersect(mask);
            self.generate_promotions(&color, &square, &bitboard_to);
        }
    }

    #[inline]
    fn generate_quiet_promotions(&mut self, position: &Position, color: &Color, mask: &Bitboard) {
        let their_color = color.reverse();
        let pawn_bitboard = position.piece_bitboard(&color, &PieceType::PAWN)
            .intersect(&position.empty_bitboard().intersect(mask).pawn_forward(&their_color))
            .intersect(&Bitboard::PROMOTION[color]);

        for square in pawn_bitboard.iterator() {
            self.generate_promotion(&color, &square, &square.forward(&color));
        }
    }

    #[inline]
    fn generate_promotions(&mut self, color: &Color, square_from: &Square, bitboard_to: &Bitboard) {
        for square in bitboard_to.iterator() {
            self.generate_promotion(color, square_from, &square);
        }
    }

    #[inline]
    fn generate_promotion(&mut self, color: &Color, square_from: &Square, square_to: &Square) {
        self.add_move(BoardMove::build_move(square_from, square_to, &MoveType::PROMOTION_QUEEN));
        self.add_move(BoardMove::build_move(square_from, square_to, &MoveType::PROMOTION_ROOK));
        self.add_move(BoardMove::build_move(square_from, square_to, &MoveType::PROMOTION_BISHOP));
        self.add_move(BoardMove::build_move(square_from, square_to, &MoveType::PROMOTION_KNIGHT));
    }
}

#[cfg(test)]
mod test {
    use crate::simplified::position::Position;
    use crate::types::move_list::MoveList;

    fn count_moves(fen: &str) -> u32 {
        let position = Position::from_fen(fen);
        let mut move_list = MoveList::new();
        move_list.generate_quiets(&position);
        move_list.generate_noisy(&position);
        let mut legal_moves = 0;
        while move_list.has_next() {
            let board_move = move_list.next();
            if position.is_legal_move(&board_move) {
                println!("move {}", board_move.to_string());
                legal_moves += 1;
            } else {
                println!("Invalid {}", board_move.to_string());
            }
        }
        return legal_moves;
    }

    #[test]
    fn pawn_move() {
        let legal_moves = count_moves("5k2/8/8/8/3P4/8/8/5K2 w - -");
        assert_eq!(legal_moves, 6);
    }

    #[test]
    fn pawn_double_move() {
        let legal_moves = count_moves("5k2/8/8/8/8/8/3P4/5K2 w - -");
        assert_eq!(legal_moves, 7);
    }

    #[test]
    fn pawn_blocked_move() {
        let legal_moves = count_moves("5k2/8/8/3p4/3P4/8/8/5K2 w - -");
        assert_eq!(legal_moves, 5);
    }

    #[test]
    fn pawn_blocked_move_and_double_move() {
        let legal_moves = count_moves("5k2/8/8/8/8/3p4/3P4/5K2 w - -");
        assert_eq!(legal_moves, 4);
    }

    #[test]
    fn pawn_blocked_double_move() {
        let legal_moves = count_moves("5k2/8/8/8/3p4/8/3P4/5K2 w - -");
        assert_eq!(legal_moves, 6);
    }

    #[test]
    fn pinned_pawn_cannot_move() {
        let legal_moves = count_moves("5k2/8/8/1b6/8/3P4/8/5K2 w - -");
        assert_eq!(legal_moves, 5);
    }

    #[test]
    fn pawn_move_blocking_check() {
        let legal_moves = count_moves("5k2/8/8/8/2b5/8/3P4/5K2 w - -");
        assert_eq!(legal_moves, 5);
    }

    #[test]
    fn pawn_double_move_blocking_check() {
        let legal_moves = count_moves("5k2/8/8/2b5/8/8/3P4/6K1 w - -");
        assert_eq!(legal_moves, 5);
    }

    #[test]
    fn castling() {
        let legal_moves = count_moves("5k2/8/8/8/8/8/8/R3K3 w Q -");
        assert_eq!(legal_moves, 16);
    }

    #[test]
    fn castling_in_check() {
        let legal_moves = count_moves("3r1k2/8/8/8/8/8/8/R3K3 w Q -");
        assert_eq!(legal_moves, 13);
    }

    #[test]
    fn promotion() {
        let legal_moves = count_moves("5k2/2P5/8/8/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 9)
    }

    #[test]
    fn blocked_promotion() {
        let legal_moves = count_moves("2r2k2/2P5/8/8/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 5)
    }

    #[test]
    fn check_block_promotion() {
        let legal_moves = count_moves("5k2/3P4/1q6/8/8/8/5K2/8 w - -");
        assert_eq!(legal_moves, 6)
    }

    #[test]
    fn check_capture_promotion() {
        let legal_moves = count_moves("1q3K2/2P5/8/8/5k2/8/8/8 w - -");
        assert_eq!(legal_moves, 11)
    }

    #[test]
    fn pinned_capture_promotion() {
        let legal_moves = count_moves("1b3k2/2P5/8/8/5K2/8/8/8 w - -");
        assert_eq!(legal_moves, 12)
    }

    #[test]
    fn knight_moves() {
        let legal_moves = count_moves("5k2/8/8/8/2N5/8/8/5K2 w - -");
        assert_eq!(legal_moves, 13)
    }

    #[test]
    fn knight_capture() {
        let legal_moves = count_moves("3r1k2/8/2N5/8/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 13)
    }


    #[test]
    fn knight_pinned_moves() {
        let legal_moves = count_moves("5k2/8/8/3b4/8/8/6N1/7K w - -");
        assert_eq!(legal_moves, 2)
    }

    #[test]
    fn bishop_moves() {
        let legal_moves = count_moves("5k2/8/8/3B4/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 18)
    }

    #[test]
    fn bishop_capture() {
        let legal_moves = count_moves("3r1k2/8/1B6/8/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 14)
    }

    #[test]
    fn bishop_pinned_moves() {
        let legal_moves = count_moves("5k2/8/8/3b4/8/8/6B1/7K w - -");
        assert_eq!(legal_moves, 5)
    }

    #[test]
    fn rook_moves() {
        let legal_moves = count_moves("5k2/8/8/3R4/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 19)
    }

    #[test]
    fn rook_capture() {
        let legal_moves = count_moves("3r1k2/8/8/3R4/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 19)
    }

    #[test]
    fn rook_pinned_moves() {
        let legal_moves = count_moves("5k2/5r2/8/8/8/8/5R2/5K2 w - -");
        assert_eq!(legal_moves, 9)
    }

    #[test]
    fn queen_moves() {
        let legal_moves = count_moves("5k2/8/8/3Q4/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 32)
    }

    #[test]
    fn queen_pinned_moves() {
        let legal_moves = count_moves("5k2/5r2/8/8/8/8/5Q2/5K2 w - -");
        assert_eq!(legal_moves, 9)
    }

    #[test]
    fn king_moves() {
        let legal_moves = count_moves("5k2/8/8/8/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 5)
    }

    #[test]
    fn king_capture() {
        let legal_moves = count_moves("5k2/8/8/8/8/8/8/4qK2 w - -");
        assert_eq!(legal_moves, 2)
    }

    #[test]
    fn king_safe_moves() {
        let legal_moves = count_moves("5k2/5r2/8/8/8/8/8/5K2 w - -");
        assert_eq!(legal_moves, 4)
    }

    #[test]
    fn ep_capture() {
        let legal_moves = count_moves("5k2/8/8/3Pp3/8/8/8/4K3 w - e6");
        assert_eq!(legal_moves, 7)
    }

    #[test]
    fn ep_pin_capture() {
        let legal_moves = count_moves("5kb1/8/8/3Pp3/2K5/8/8/8 w - e6");
        assert_eq!(legal_moves, 7)
    }

    #[test]
    fn p4_issue() {
        let legal_moves = count_moves("r1bqkbnr/pppppppp/2n5/8/Q7/2P5/PP1PPPPP/RNB1KBNR b KQkq -");
        assert_eq!(legal_moves, 22)
    }

    #[test]
    fn p4_issue2() {
        let legal_moves = count_moves("rnbqkbnr/1ppppppp/8/p7/1P6/7P/P1PPPPP1/RNBQKBNR b KQkq -");
        assert_eq!(legal_moves, 22)
    }

    #[test]
    fn p4_issue3() {
        let legal_moves = count_moves("rnbqkbnr/ppp1pppp/3p4/8/Q7/2P5/PP1PPPPP/RNB1KBNR b KQkq -");
        assert_eq!(legal_moves, 6)
    }

    #[test]
    fn p5_issue1() {
        let legal_moves = count_moves("rnbqkbnr/1ppppp1p/6p1/p7/8/1P6/PBPPPPPP/RN1QKBNR w KQkq -");
        assert_eq!(legal_moves, 28)
    }

    #[test]
    fn p438() {
        let legal_moves = count_moves("1rb3rk/1p6/n7/Bqbpp1p1/P1p3Qp/5NPK/2P3nP/RN3R2 w - -");
        assert_eq!(legal_moves, 33)
    }

    #[test]
    fn p2762() {
        let legal_moves = count_moves("8/p4Q2/P6k/2P5/8/1P6/1r4RK/r7 w - -");
        assert_eq!(legal_moves, 30)
    }

    #[test]
    fn gen916() {
        let legal_moves = count_moves("r3qbn1/4k1p1/B4n2/1p3p1r/p1ppPP1P/P1P1Q3/1P6/RNBbK2R w KQ -");
        assert_eq!(legal_moves, 29)
    }

    #[test]
    fn gen2535() {
        let legal_moves = count_moves("rnbqk1n1/pp1pppb1/2p5/6p1/8/NP1P4/P1PQPPP1/R3KBNr w Qq -");
        assert_eq!(legal_moves, 28)
    }

    #[test]
    fn gen683_3() {
        let legal_moves = count_moves("5b2/8/rp3qN1/p1k2p1r/PpbpP3/7P/2QP1PB1/RN2K2R w KQ -");
        assert_eq!(legal_moves, 30)
    }
}

impl Position {
    #[inline]
    fn get_mask(&self, color: &Color) -> Bitboard {
        return if self.state().check_bitboard.is_empty() {
            Bitboard::ALL
        } else if self.state().check_bitboard.one_element() {
            let check_square = self.state().check_bitboard.to_square();
            let king_square = self.king_square(color);
            check_square.between(&king_square).union(&self.state().check_bitboard)
        } else {
            Bitboard::EMPTY
        };
    }
}