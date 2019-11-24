use crate::advanced::move_list::MoveList;
use crate::advanced::board::Board;
use crate::advanced::attack_info::AttackInfo;
use crate::types::board_move::BoardMove;
use crate::types::piece_type::PieceType;
use crate::types::bitboard::Bitboard;

#[allow(dead_code)]
impl MoveList {
    pub fn generate_quiets(&mut self, board: &Board, attack_info: &mut AttackInfo) {
        attack_info.update(board);
        let our_color = board.color_to_move;
        if board.check_bitboard.is_empty() {
            self.generate_castling_moves(board, attack_info);
        }

        let mask = attack_info.movement_mask(&our_color).intersect(&board.empty_bitboard());
//        if mask.is_not_empty() {
//            self.generate_quiet_pawn_moves(board, attack_info, mask);
//            self.generate_quiet_moves(board, attack_info, PieceType::KNIGHT, mask);
//            self.generate_quiet_moves(board, attack_info, PieceType::BISHOP, mask);
//            self.generate_quiet_moves(board, attack_info, PieceType::ROOK, mask);
//            self.generate_quiet_moves(board, attack_info, PieceType::QUEEN, mask);
//        }
//        self.generate_quiet_moves(board, attack_info, PieceType::KING, board.empty_bitboard());
    }

    fn generate_castling_moves(&mut self, board: &Board, attack_info: &AttackInfo) {
        let our_color = board.color_to_move;
        let their_color = our_color.invert();
        let possible_castling = board.castling_rights.color_filter(&our_color);
        let king_square = board.king_square(&our_color);
        for castling_index in possible_castling.iterator() {
            let king_to = castling_index.square_king_to();
            let king_path = king_square.between(&king_to)
                .union(&Bitboard::from_square(&king_to));

            let rook_from = board.initial_rook_square(&castling_index);
            let rook_to = castling_index.square_rook_to();
            let rook_path = rook_from.between(&rook_to);

            if king_path.union(&rook_path).intersect(&board.game_bitboard())
                .union(&king_path.intersect(&attack_info.attack_bitboard(&their_color, &PieceType::NONE))).is_empty() {

                let board_move = BoardMove::build_castling(&king_square, &king_to);
                self.add_move(board_move);
            }
        }
    }

    pub fn generate_noisy(&mut self, board: &Board, attack_info: &mut AttackInfo) {
        attack_info.update(board);
    }
}