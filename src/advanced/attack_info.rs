use crate::advanced::board::Board;
use crate::advanced::zobrist_key::ZobristKey;
use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(Clone)]
pub struct AttackInfo {
    attack_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    piece_movement: [Bitboard; Square::NUM_SQUARES],

    movement_mask: [Bitboard; Color::NUM_COLORS],

    zobrist_key: ZobristKey,
}

impl AttackInfo {
    pub fn new() -> Self {
        AttackInfo {
            attack_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
            piece_movement: [Bitboard::EMPTY; Square::NUM_SQUARES],
            movement_mask: [Bitboard::EMPTY; Color::NUM_COLORS],
            zobrist_key: ZobristKey::new(),
        }
    }

    pub fn update(&mut self, board: &Board) {
        if board.zkey == self.zobrist_key {
            return;
        }
        self.zobrist_key = board.zkey;
        for square in Square::SQUARES.iter() {
            self.piece_movement[square.to_usize()] = Bitboard::EMPTY;
        }

        self.update_color(board, &Color::White);
        self.update_color(board, &Color::Black);
    }

    fn update_color(&mut self, board: &Board, color: &Color) {
        for piece_type in PieceType::PIECE_TYPES.iter() {
            self.attack_bitboard[color.to_usize()][piece_type.to_usize()] = Bitboard::EMPTY;
        }

        let check_bitboard = board.check_bitboard;

        let mask = if check_bitboard.is_empty() {
            Bitboard::ALL
        } else if check_bitboard.one_element() {
            let check_square = check_bitboard.to_square();
            let king_square = board.king_square(color);
            check_square.between(&king_square).union(&check_bitboard)
        } else {
            Bitboard::EMPTY
        };

        if mask.is_not_empty() {
            self.pawn_attacks(board, color, &mask);
            self.knight_moves(board, color, &mask);
            self.bishop_moves(board, color, &mask);
            self.rook_moves(board, color, &mask);
            self.queen_moves(board, color, &mask);
        }
        self.king_moves(board, color);

        self.attack_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .union(&self.attack_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()])
                .union(&self.attack_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()])
                .union(&self.attack_bitboard[color.to_usize()][PieceType::ROOK.to_usize()])
                .union(&self.attack_bitboard[color.to_usize()][PieceType::QUEEN.to_usize()])
                .union(&self.attack_bitboard[color.to_usize()][PieceType::KING.to_usize()]);
    }

    fn pawn_attacks(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let unpinned_pawns = board.piece_bitboard(color, &PieceType::PAWN)
            .intersect(&board.pinned_bitboard.not());
        self.attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] =
            unpinned_pawns.pawn_attacks_left(color)
                .union(&unpinned_pawns.pawn_attacks_right(color))
                .intersect(mask);

        let pinned_pawns = board.piece_bitboard(color, &PieceType::PAWN)
            .intersect(&board.pinned_bitboard);
        let king_square = board.king_square(color);

        for square in pinned_pawns.iterator() {
            let pinned_mask = king_square.pinned_mask(&square);
            let bitboard = square.pawn_attacks(color)
                .intersect(mask)
                .intersect(&pinned_mask);

            self.attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] =
                self.attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()].union(&bitboard);
        }
    }

    fn knight_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let unpinned_knights = board.piece_bitboard(color, &PieceType::KNIGHT)
            .intersect(&board.pinned_bitboard.not());

        for square in unpinned_knights.iterator() {
            let bitboard = square.knight_moves().intersect(mask);
            self.register_bitboard(color, &PieceType::KNIGHT, &square, &bitboard);
        }
    }

    fn bishop_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let bishops = board.piece_bitboard(color, &PieceType::BISHOP);
        let king_square = board.king_square(color);

        for square in bishops.iterator() {
            let from_bitboard = Bitboard::from_square(&square);
            let bitboard =
                if from_bitboard.intersect(&board.pinned_bitboard).is_not_empty() {
                    square.bishop_moves(&board.game_bitboard())
                        .intersect(mask)
                        .intersect(&king_square.pinned_mask(&square))
                } else {
                    square.bishop_moves(&board.game_bitboard()).intersect(mask)
                };
            self.register_bitboard(color, &PieceType::BISHOP, &square, &bitboard);
        }
    }

    fn rook_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let rooks = board.piece_bitboard(color, &PieceType::ROOK);
        let king_square = board.king_square(color);

        for square in rooks.iterator() {
            let from_bitboard = Bitboard::from_square(&square);
            let bitboard =
                if from_bitboard.intersect(&board.pinned_bitboard).is_not_empty() {
                    square.rook_moves(&board.game_bitboard())
                        .intersect(mask)
                        .intersect(&king_square.pinned_mask(&square))
                } else {
                    square.rook_moves(&board.game_bitboard())
                        .intersect(mask)
                };
            self.register_bitboard(color, &PieceType::BISHOP, &square, &bitboard);
        }
    }

    fn queen_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let queens = board.piece_bitboard(color, &PieceType::QUEEN);
        let king_square = board.king_square(color);

        for square in queens.iterator() {
            let from_bitboard = Bitboard::from_square(&square);
            let bitboard =
                if from_bitboard.intersect(&board.pinned_bitboard).is_not_empty() {
                    square.bishop_moves(&board.game_bitboard())
                        .union(&square.rook_moves(&board.game_bitboard()))
                        .intersect(mask)
                        .intersect(&king_square.pinned_mask(&square))
                } else {
                    square.bishop_moves(&board.game_bitboard())
                        .union(&square.rook_moves(&board.game_bitboard()))
                        .intersect(mask)
                };
            self.register_bitboard(color, &PieceType::BISHOP, &square, &bitboard);
        }
    }

    fn king_moves(&mut self, board: &Board, color: &Color) {
        let king_square = board.king_square(color);
        let their_square = board.king_square(&color.invert());
        let bitboard = king_square.king_moves()
            .intersect(&their_square.king_moves().not());

        self.register_bitboard(color, &PieceType::KING, &king_square, &bitboard);
    }

    fn register_bitboard(&mut self, color: &Color, piece_type: &PieceType, square: &Square, bitboard: &Bitboard) {
        self.piece_movement[square.to_usize()] = *bitboard;
        self.attack_bitboard[color.to_usize()][piece_type.to_usize()] =
            self.attack_bitboard(&color, piece_type).union(bitboard);
    }

    pub fn movement_mask(&self, color: &Color) -> Bitboard {
        self.movement_mask[color.to_usize()]
    }

    pub fn attack_bitboard(&self, color: &Color, piece_type: &PieceType) -> Bitboard {
        self.attack_bitboard[color.to_usize()][piece_type.to_usize()]
    }
}