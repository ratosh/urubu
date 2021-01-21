use crate::advanced::attack_info::AttackInfo;
use crate::advanced::board::Board;
use crate::advanced::move_list::MoveList;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::color::Color;
use crate::types::move_type::MoveType;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[allow(dead_code)]
impl MoveList {
    #[inline]
    pub fn generate_quiets(&mut self, board: &Board, attack_info: &mut AttackInfo) {
        attack_info.update(board);
        let our_color = board.color_to_move;
        if board.check_bitboard.is_empty() {
            self.generate_castling_moves(board, attack_info);
        }

        let mask = attack_info.movement_mask(&our_color).intersect(board.empty_bitboard());
        if mask.is_not_empty() {
            self.generate_quiet_pawn_moves(board, mask);
            self.generate_moves(board, attack_info, PieceType::KNIGHT, mask);
            self.generate_moves(board, attack_info, PieceType::BISHOP, mask);
            self.generate_moves(board, attack_info, PieceType::ROOK, mask);
            self.generate_moves(board, attack_info, PieceType::QUEEN, mask);
        }
        self.generate_moves(board, attack_info, PieceType::KING, board.empty_bitboard());
    }

    #[inline]
    fn generate_castling_moves(&mut self, board: &Board, attack_info: &AttackInfo) {
        let our_color = board.color_to_move;
        let their_color = our_color.reverse();
        let possible_castling = board.castling_rights.color_filter(&our_color);
        let king_square = board.king_square(our_color);
        for castling_index in possible_castling.iterator() {
            let king_to = castling_index.square_king_to();
            let king_path = king_square.between(king_to)
                .union(Bitboard::from_square(king_to));

            let rook_from = board.initial_rook_square(castling_index);
            let rook_to = castling_index.square_rook_to();
            let rook_path = rook_from.between(rook_to);

            if king_path.union(rook_path).intersect(board.game_bitboard())
                .union(king_path.intersect(attack_info.all_attack_bitboard(their_color, PieceType::NONE))).is_empty() {
                let board_move = BoardMove::build_castling(king_square, king_to);
                self.add_move(board_move);
            }
        }
    }

    #[inline]
    fn generate_quiet_pawn_moves(&mut self, board: &Board, mask: Bitboard) {
        let color = board.color_to_move;
        let their_color = board.color_to_move.reverse();
        let pawn_bitboard = board.piece_bitboard(color, PieceType::PAWN)
            .intersect(board.empty_bitboard().pawn_forward(their_color))
            .difference(Bitboard::PROMOTION[color.to_usize()]);

        for square in pawn_bitboard.iterator() {
            let move_bitboard = if Bitboard::from_square(square).intersect(board.pinned_bitboard).is_not_empty() {
                square.pawn_move(color)
                    .union(square.pawn_double_move(color))
                    .intersect(mask)
                    .intersect(board.king_square(color).pinned_mask(square))
            } else {
                square.pawn_move(color)
                    .union(square.pawn_double_move(color))
                    .intersect(mask)
            };

            self.generate_moves_from_square(color, square, move_bitboard);
        }
    }

    #[inline]
    fn generate_moves(&mut self, board: &Board, attack_info: &AttackInfo, piece_type: PieceType, mask: Bitboard) {
        let color = board.color_to_move;
        let masked_move = mask.intersect(attack_info.pinned_attack_bitboard(color, piece_type));
        if masked_move.is_empty() {
            return;
        }
        for square in board.piece_bitboard(color, piece_type).iterator() {
            self.generate_moves_from_square(color, square, attack_info.movement(&square).intersect(mask))
        }
    }

    #[inline]
    fn generate_moves_from_square(&mut self, color: Color, square: Square, bitboard: Bitboard) {
        for square_to in bitboard.iterator() {
            let board_move = BoardMove::build_normal(square, square_to);
            self.add_move(board_move);
        }
    }

    #[inline]
    pub fn generate_noisy(&mut self, board: &Board, attack_info: &mut AttackInfo) {
        attack_info.update(board);
        let our_color = board.color_to_move;

        let mask = attack_info.movement_mask(&our_color).intersect(board.color_bitboard(our_color.reverse()));
        if mask.is_not_empty() {
            self.generate_capture_promotions(board, mask);
            self.generate_quiet_promotions(board, attack_info.movement_mask(&our_color).intersect(board.empty_bitboard()));
            self.generate_pawn_capture(board, mask);
            self.generate_moves(board, attack_info, PieceType::KNIGHT, mask);
            self.generate_moves(board, attack_info, PieceType::BISHOP, mask);
            self.generate_moves(board, attack_info, PieceType::ROOK, mask);
            self.generate_moves(board, attack_info, PieceType::QUEEN, mask);
            self.generate_ep_capture(board, attack_info.movement_mask(&our_color));
        }
        self.generate_moves(board, attack_info, PieceType::KING, board.color_bitboard(our_color.reverse()));
    }

    #[inline]
    fn generate_ep_capture(&mut self, board: &Board, mask: Bitboard) {
        if let Some(ep_square) = board.ep_square {

            let color = board.color_to_move;
            let their_color = color.reverse();

            if Bitboard::from_square(ep_square.forward(&their_color)).intersect(mask).is_empty() {
                return;
            }

            let bitboard = ep_square.pawn_attacks(their_color)
                .intersect(board.piece_bitboard(color, PieceType::PAWN));

            for square in bitboard.iterator() {
                let bitboard_from = Bitboard::from_square(square);
                let mut bitboard_to = Bitboard::from_square(ep_square);
                if bitboard_from.intersect(board.pinned_bitboard).is_not_empty() {
                    let king_square = board.king_square(color);
                    bitboard_to = bitboard_to.intersect(king_square.pinned_mask(square));
                }
                if bitboard_to.is_not_empty() {
                    let board_move = BoardMove::build_passant(square, ep_square);
                    self.add_move(board_move);
                }
            }
        }
    }

    #[inline]
    fn generate_pawn_capture(&mut self, board: &Board, mask: Bitboard) {
        let color = board.color_to_move;
        let their_color = board.color_to_move.reverse();
        let pawn_bitboard = board.piece_bitboard(color, PieceType::PAWN)
            .intersect(Bitboard::PROMOTION[color.to_usize()].reverse())
            .intersect(board.color_bitboard(their_color).intersect(mask).pawn_attacks(their_color));

        for square in pawn_bitboard.iterator() {
            let bitboard_from = Bitboard::from_square(square);
            let mut bitboard_to = square.pawn_attacks(color)
                .intersect(board.color_bitboard(their_color))
                .intersect(mask);
            if bitboard_from.intersect(board.pinned_bitboard).is_not_empty() {
                let king_square = board.king_square(color);
                bitboard_to = bitboard_to.intersect(king_square.pinned_mask(square));
            }
            self.generate_moves_from_square(color, square, bitboard_to);
        }
    }

    #[inline]
    fn generate_capture_promotions(&mut self, board: &Board, mask: Bitboard) {
        let color = board.color_to_move;
        let their_color = board.color_to_move.reverse();
        let pawn_bitboard = board.piece_bitboard(color, PieceType::PAWN)
            .intersect(Bitboard::PROMOTION[color.to_usize()])
            .intersect(board.color_bitboard(their_color).intersect(mask).pawn_attacks(their_color));

        for square in pawn_bitboard.iterator() {
            let bitboard_from = Bitboard::from_square(square);
            let mut bitboard_to = square.pawn_attacks(color)
                .intersect(board.color_bitboard(their_color))
                .intersect(mask);
            if bitboard_from.intersect(board.pinned_bitboard).is_not_empty() {
                let king_square = board.king_square(color);
                bitboard_to = bitboard_to.intersect(king_square.pinned_mask(square));
            }
            self.generate_promotions(color, square, bitboard_to);
        }
    }

    #[inline]
    fn generate_quiet_promotions(&mut self, board: &Board, mask: Bitboard) {
        let color = board.color_to_move;
        let their_color = board.color_to_move.reverse();
        let pawn_bitboard = board.piece_bitboard(color, PieceType::PAWN)
            .difference(board.pinned_bitboard)
            .intersect(board.empty_bitboard().intersect(mask).pawn_forward(their_color))
            .intersect(Bitboard::PROMOTION[color.to_usize()]);

        for square in pawn_bitboard.iterator() {
            self.generate_promotion(color, square, square.forward(&color));
        }
    }

    #[inline]
    fn generate_promotions(&mut self, color: Color, square_from: Square, bitboard_to: Bitboard) {
        for square in bitboard_to.iterator() {
            self.generate_promotion(color, square_from, square);
        }
    }

    #[inline]
    fn generate_promotion(&mut self, color: Color, square_from: Square, square_to: Square) {
        self.add_move(BoardMove::build_move(square_from, square_to, MoveType::PROMOTION_QUEEN));
        self.add_move(BoardMove::build_move(square_from, square_to, MoveType::PROMOTION_ROOK));
        self.add_move(BoardMove::build_move(square_from, square_to, MoveType::PROMOTION_BISHOP));
        self.add_move(BoardMove::build_move(square_from, square_to, MoveType::PROMOTION_KNIGHT));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn count_moves(fen: &str) -> u32 {
        let board = Board::from_fen(fen);
        let mut move_list = MoveList::new();
        let mut attack_info = AttackInfo::new();
        move_list.generate_quiets(&board, &mut attack_info);
        move_list.generate_noisy(&board, &mut attack_info);
        let mut legal_moves = 0;
        while move_list.has_next() {
            let board_move = move_list.next();
            if board.clone().do_move(&board_move) {
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