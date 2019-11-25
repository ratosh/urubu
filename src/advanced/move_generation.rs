use crate::advanced::attack_info::AttackInfo;
use crate::advanced::board::Board;
use crate::advanced::move_list::MoveList;
use crate::types::bitboard::Bitboard;
use crate::types::board_move::BoardMove;
use crate::types::piece_type::PieceType;

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
        println!("possible {}", possible_castling.to_string());
        for castling_index in possible_castling.iterator() {
            println!("index {}", castling_index.to_char());
            let king_to = castling_index.square_king_to();
            let king_path = king_square.between(&king_to)
                .union(&Bitboard::from_square(&king_to));

            let rook_from = board.initial_rook_square(&castling_index);
            let rook_to = castling_index.square_rook_to();
            let rook_path = rook_from.between(&rook_to);
            println!("king path {}", king_path.to_string());
            println!("rook path {}", rook_path.to_string());
            println!("game bitboard {}", board.game_bitboard().to_string());
            println!("attack bitboard {}", attack_info.attack_bitboard(&their_color, &PieceType::NONE).to_string());

            println!("p1 {}", king_path.union(&rook_path).intersect(&board.game_bitboard()).to_string());
            println!("p2 {}", king_path.intersect(&attack_info.attack_bitboard(&their_color, &PieceType::NONE)).to_string());

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

#[cfg(test)]
mod test {
    use super::*;

    fn count_moves(fen: &str) -> u32 {
        let board = Board::from_fen(fen);
        let mut move_list = MoveList::new();
        let mut attack_info = AttackInfo::new();
        move_list.generate_quiets(&board, &mut attack_info);
        let mut legal_moves = 0;
        while move_list.has_next() {
            let board_move = move_list.next();
            println!("move {}", board_move.to_string());
            if board.clone().do_move(&board_move) {
                legal_moves += 1;
            }
        }
        return legal_moves;
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
}