use crate::advanced::zobrist_key::ZobristKey;
use crate::cached::board::Board;
use crate::types::bitboard::Bitboard;
use crate::types::color::Color;
use crate::types::piece_type::PieceType;
use crate::types::square::Square;

#[derive(Clone)]
pub struct AttackInfo {
    all_attack_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    // Consider pin when creating attack bitboard
    special_attack_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    piece_movement: [Bitboard; Square::NUM_SQUARES],

    movement_mask: [Bitboard; Color::NUM_COLORS],

    zobrist_key: ZobristKey,
}

impl AttackInfo {
    pub fn new() -> Self {
        AttackInfo {
            all_attack_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
            special_attack_bitboard: [[Bitboard::EMPTY; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
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

        self.first_pass(board, &Color::White);
        self.first_pass(board, &Color::Black);

        self.second_pass(board, &Color::White);
        self.second_pass(board, &Color::Black);
    }

    fn first_pass(&mut self, board: &Board, color: &Color) {
        for piece_type in PieceType::PIECE_TYPES.iter() {
            self.all_attack_bitboard[color.to_usize()][piece_type.to_usize()] = Bitboard::EMPTY;
            self.special_attack_bitboard[color.to_usize()][piece_type.to_usize()] = Bitboard::EMPTY;
        }

        let check_bitboard = board.check_bitboard.intersect(&board.color_bitboard(&color.reverse()));

        let mask = if check_bitboard.is_empty() {
            Bitboard::ALL
        } else if check_bitboard.one_element() {
            let check_square = check_bitboard.to_square();
            let king_square = board.king_square(color);
            check_square.between(&king_square).union(&check_bitboard)
        } else {
            Bitboard::EMPTY
        };
        self.movement_mask[color.to_usize()] = mask;

        if mask.is_not_empty() {
            self.pawn_attacks(board, color, &mask);
            self.knight_moves(board, color, &mask);
            self.bishop_moves(board, color, &mask);
            self.rook_moves(board, color, &mask);
            self.queen_moves(board, color, &mask);
        }

        self.all_attack_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.all_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .union(&self.all_attack_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()])
                .union(&self.all_attack_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()])
                .union(&self.all_attack_bitboard[color.to_usize()][PieceType::ROOK.to_usize()])
                .union(&self.all_attack_bitboard[color.to_usize()][PieceType::QUEEN.to_usize()]);

        self.special_attack_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.special_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                .union(&self.special_attack_bitboard[color.to_usize()][PieceType::KNIGHT.to_usize()])
                .union(&self.special_attack_bitboard[color.to_usize()][PieceType::BISHOP.to_usize()])
                .union(&self.special_attack_bitboard[color.to_usize()][PieceType::ROOK.to_usize()])
                .union(&self.special_attack_bitboard[color.to_usize()][PieceType::QUEEN.to_usize()]);
    }

    fn second_pass(&mut self, board: &Board, color: &Color) {
        self.king_moves(board, color);
        self.all_attack_bitboard[color.to_usize()][PieceType::NONE.to_usize()] =
            self.all_attack_bitboard[color.to_usize()][PieceType::NONE.to_usize()]
                .union(&self.all_attack_bitboard[color.to_usize()][PieceType::KING.to_usize()])
    }

    fn pawn_attacks(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let unpinned_pawns = board.piece_bitboard(color, &PieceType::PAWN)
            .intersect(&board.pinned_bitboard.reverse());
        let unpinned_attacks = unpinned_pawns.pawn_attacks(color).intersect(mask);
        self.all_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] =
            unpinned_attacks;
        self.special_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] =
            unpinned_attacks;

        let pinned_pawns = board.piece_bitboard(color, &PieceType::PAWN)
            .intersect(&board.pinned_bitboard);
        let king_square = board.king_square(color);

        for square in pinned_pawns.iterator() {
            let pinned_mask = king_square.pinned_mask(&square);
            let bitboard = square.pawn_attacks(color)
                .intersect(mask);

            self.all_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] =
                self.all_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                    .union(&bitboard);

            self.special_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()] =
                self.special_attack_bitboard[color.to_usize()][PieceType::PAWN.to_usize()]
                    .union(&bitboard.intersect(&pinned_mask));
        }
    }

    #[inline]
    fn knight_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let knights = board.piece_bitboard(color, &PieceType::KNIGHT);

        for square in knights.iterator() {
            let bitboard = square.knight_moves().intersect(mask);
            let pinned_bitboard = pinned_mask(board, color, &square, bitboard);
            self.register_bitboard(color, &PieceType::KNIGHT, &square, &bitboard, &pinned_bitboard);
        }
    }

    #[inline]
    fn bishop_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let bishops = board.piece_bitboard(color, &PieceType::BISHOP);
        let king_square = board.king_square(color);

        for square in bishops.iterator() {
            let bitboard = square.bishop_moves(&board.game_bitboard()).intersect(mask);
            let pinned_bitboard = pinned_mask(board, color, &square, bitboard);
            self.register_bitboard(color, &PieceType::BISHOP, &square, &bitboard, &pinned_bitboard);
        }
    }

    #[inline]
    fn rook_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let rooks = board.piece_bitboard(color, &PieceType::ROOK);
        let king_square = board.king_square(color);

        for square in rooks.iterator() {
            let from_bitboard = Bitboard::from_square(&square);
            let bitboard = square.rook_moves(&board.game_bitboard()).intersect(mask);
            let pinned_bitboard = pinned_mask(board, color, &square, bitboard);
            self.register_bitboard(color, &PieceType::ROOK, &square, &bitboard, &pinned_bitboard);
        }
    }

    #[inline]
    fn queen_moves(&mut self, board: &Board, color: &Color, mask: &Bitboard) {
        let queens = board.piece_bitboard(color, &PieceType::QUEEN);
        let king_square = board.king_square(color);

        for square in queens.iterator() {
            let from_bitboard = Bitboard::from_square(&square);
            let bitboard =
                square.bishop_moves(&board.game_bitboard())
                    .union(&square.rook_moves(&board.game_bitboard()))
                    .intersect(mask);
            let pinned_bitboard = pinned_mask(board, color, &square, bitboard);
            self.register_bitboard(color, &PieceType::QUEEN, &square, &bitboard, &pinned_bitboard);
        }
    }

    #[inline]
    fn king_moves(&mut self, board: &Board, color: &Color) {
        let their_color = color.reverse();
        let king_square = board.king_square(color);
        let their_square = board.king_square(&their_color);

        let mut king_move_mask = Bitboard::EMPTY;
        for square in board.check_bitboard.intersect(&board.slider_pieces(&their_color)).iterator() {
            king_move_mask = king_move_mask.union(&square.pinned_mask(&king_square))
        }
        let bitboard = king_square.king_moves()
            .difference(&their_square.king_moves())
            .difference(&king_move_mask);

        let bitboard_safe = bitboard.difference(&self.all_attack_bitboard[their_color.to_usize()][PieceType::NONE.to_usize()]);

        self.register_bitboard(color, &PieceType::KING, &king_square, &bitboard, &bitboard_safe);
    }

    #[inline]
    fn register_bitboard(&mut self, color: &Color, piece_type: &PieceType, square: &Square, bitboard: &Bitboard, pin_bitboard: &Bitboard) {
        self.piece_movement[square.to_usize()] = *pin_bitboard;
        self.all_attack_bitboard[color.to_usize()][piece_type.to_usize()] =
            self.all_attack_bitboard(&color, piece_type).union(bitboard);
        self.special_attack_bitboard[color.to_usize()][piece_type.to_usize()] =
            self.pinned_attack_bitboard(&color, piece_type).union(pin_bitboard);
    }

    #[inline]
    pub fn movement_mask(&self, color: &Color) -> Bitboard {
        self.movement_mask[color.to_usize()]
    }

    #[inline]
    pub fn pinned_attack_bitboard(&self, color: &Color, piece_type: &PieceType) -> Bitboard {
        self.special_attack_bitboard[color.to_usize()][piece_type.to_usize()]
    }

    #[inline]
    pub fn all_attack_bitboard(&self, color: &Color, piece_type: &PieceType) -> Bitboard {
        self.all_attack_bitboard[color.to_usize()][piece_type.to_usize()]
    }

    #[inline]
    pub fn movement(&self, square: &Square) -> Bitboard {
        self.piece_movement[square.to_usize()]
    }
}

#[inline]
fn pinned_mask(board: &Board, color: &Color, square: &Square, bitboard: Bitboard) -> Bitboard {
    let from_bitboard = Bitboard::from_square(&square);
    return if from_bitboard.intersect(&board.pinned_bitboard).is_empty() {
        bitboard
    } else {
        let king_square = board.king_square(color);
        bitboard.intersect(&king_square.pinned_mask(&square))
    };
}