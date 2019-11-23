use crate::advanced::board::Board;
use crate::types::bitboard::Bitboard;
use crate::types::castling_rights::CastlingRights;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;
use crate::advanced::zobrist_key::ZobristKey;
use std::thread::current;

#[derive(Clone, Copy)]
pub struct BoardState {
    pub zkey: ZobristKey,
    pub zkey_pawn: ZobristKey,
    pub rule_50: u8,
    pub castling_rights: CastlingRights,
    pub ep_square: Option<Square>,
    pub piece_captured: PieceType,

    pub pinned_bitboard: Bitboard,
    pub danger_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub check_bitboard: Bitboard,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            zkey: ZobristKey::new(),
            zkey_pawn: ZobristKey::new(),
            rule_50: 0,
            castling_rights: CastlingRights::ANY_CASTLING,
            ep_square: None,
            piece_captured: PieceType::NONE,
            check_bitboard: Bitboard::EMPTY,
            pinned_bitboard: Bitboard::EMPTY,
            danger_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
        }
    }

    #[inline]
    pub fn first_pass(&mut self, board: &Board) {
        let previous_color = &board.color_to_move.invert();
        self.update_danger_bitboard(board, previous_color);
        self.set_check_bitboard(board, previous_color);
    }

    #[inline]
    pub fn second_pass(&mut self, board: &Board) {
        self.pinned_bitboard.clear();
        let current_color = &board.color_to_move;
        for color in Color::COLORS.iter() {
            self.set_pinned(board, &color);
        }
        self.update_danger_bitboard(board, current_color);
        self.set_check_bitboard(board, current_color);
    }

    #[inline]
    fn set_pinned(&mut self, board: &Board, color: &Color) {
        let their_color = color.invert();
        if board.slider_pieces(&their_color).is_not_empty() {
            let our_bitboard = board.color_bitboard[their_color.to_usize()];
            let their_bitboard = board.color_bitboard[color.invert().to_usize()];
            let mut pinned = Bitboard::EMPTY;
            let king_square = board.king_square(color);

            let game_bitboard = board.game_bitboard();

            let between_pieces = board
                .bishop_like_pieces(&their_color)
                .intersect(&king_square.pseudo_bishop_moves())
                .union(&board.rook_like_pieces(&their_color).intersect(&king_square.pseudo_rook_moves()));

            for square in between_pieces.iterator() {
                let between_piece = king_square.between(&square).intersect(&game_bitboard);
                if between_piece.is_not_empty() && between_piece.one_element() {
                    pinned = pinned.union(&between_piece.intersect(&our_bitboard))
                }
            }
            self.pinned_bitboard = self.pinned_bitboard.union(&pinned)
        }
    }

    #[inline]
    fn update_danger_bitboard(&mut self, board: &Board, color: &Color) {
        let king_square = board.king_square(color);

        self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] = king_square.pawn_attacks(&color.invert());
        self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()] = king_square.knight_moves();
        self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()] = king_square.bishop_moves(&board.game_bitboard());
        self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()] = king_square.rook_moves(&board.game_bitboard());
        self.danger_bitboard[color.to_usize()][PieceType::QUEEN.to_usize()] =
            self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()].union(
                &self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()]
            );

        self.danger_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.danger_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .union(&self.danger_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()])
                .union(&self.danger_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()])
                .union(&self.danger_bitboard[color.to_usize()][PieceType::ROOK.to_usize()])
    }

    fn set_check_bitboard(&mut self, board: &Board, color: &Color) {
        let our_color = color;
        let their_color = our_color.invert();
        self.check_bitboard = self.danger_bitboard[our_color.to_usize()][PieceType::PAWN.to_usize()]
            .intersect(&board.piece_bitboard(&their_color, &PieceType::PAWN))
            .union(&self.danger_bitboard[our_color.to_usize()][PieceType::KNIGHT.to_usize()]
                .intersect(&board.piece_bitboard(&their_color, &PieceType::KNIGHT)))
            .union(&self.danger_bitboard[our_color.to_usize()][PieceType::BISHOP.to_usize()]
                .intersect(&board.bishop_like_pieces(&their_color)))
            .union(&self.danger_bitboard[our_color.to_usize()][PieceType::ROOK.to_usize()]
                .intersect(&board.rook_like_pieces(&their_color)));
    }

    #[inline]
    pub fn update_castling_rights(&mut self, board: &Board, square_from: &Square, square_to: &Square) {
        self.zkey.set_castling_rights(&self.castling_rights);

        let right_change = board.castling_rights_masks[square_from.to_usize()]
            .union(&board.castling_rights_masks[square_to.to_usize()]);
        self.castling_rights = self.castling_rights.difference(&right_change);
        self.zkey.set_castling_rights(&self.castling_rights);
    }

}

#[cfg(test)]
mod test {
    use crate::advanced::board::Board;
    use crate::advanced::board_state::BoardState;
    use crate::types::bitboard::Bitboard;
    use crate::types::color::Color;
    use crate::types::piece_type::PieceType;

    #[test]
    fn pinned() {
        let mut state = BoardState::new();
        state.set_check_bitboard(&Board::new(), &Color::White);
        assert_eq!(state.check_bitboard.is_empty(), true);
    }

    #[test]
    fn danger() {
        let mut state = BoardState::new();
        let board = Board::new();
        state.set_pinned(&board, &Color::White);
        assert_eq!(state.pinned_bitboard, Bitboard::EMPTY);
        state.pinned_bitboard.clear();
        state.set_pinned(&board, &Color::Black);
        assert_eq!(state.pinned_bitboard, Bitboard::EMPTY);
        state.pinned_bitboard.clear();
    }

    #[test]
    fn check_bitboard() {
        let mut state = BoardState::new();
        let board = Board::new();
        state.update_danger_bitboard(&board, &Color::White);
        state.update_danger_bitboard(&board, &Color::Black);
        assert_eq!(state.danger_bitboard[Color::White.to_usize()][PieceType::PAWN.to_usize()],
                   board.king_square(&Color::White).pawn_attacks(&Color::Black));
        assert_eq!(state.danger_bitboard[Color::White.to_usize()][PieceType::KNIGHT.to_usize()],
                   board.king_square(&Color::White).knight_moves());
    }
}