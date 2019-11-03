use crate::types::bitboard::Bitboard;
use crate::types::castling_rights::CastlingRights;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;
use crate::types::zobrist_key::ZobristKey;
use crate::movegen::board::Board;
use crate::movegen::bitboard_move::{pawn_attacks, bishop_moves, knight_moves, rook_moves};

#[derive(Clone)]
pub struct BoardState {
    pub zobrist_key: ZobristKey,
    pub pawn_zobrist_key: ZobristKey,
    pub rule_50: u8,
    pub castling_rights: CastlingRights,
    pub ep_square: Square,
    pub captured_piece: PieceType,

    pub check_bitboard: Bitboard,
    pub pinned_bitboard: Bitboard,
    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
}

impl BoardState {
    pub fn update(&mut self, board: &Board) {
        self.update_danger_bitboard(board, &Color::WHITE);
        self.update_danger_bitboard(board, &Color::BLACK);
    }

    fn update_danger_bitboard(&mut self, board: &Board, color: &Color) {
        let king_square = &board.king_square[color.to_usize()];

        self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] = pawn_attacks(color, king_square);
        self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()] = knight_moves(king_square);
        self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()] = bishop_moves(king_square, &board.game_bitboard);
        self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()] = rook_moves(king_square, &board.game_bitboard);

        self.danger_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .add(&self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()])
                .add(&self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()])
                .add(&self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()])
    }
}