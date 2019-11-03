use crate::types::square::Square;
use crate::types::color::Color;
use crate::types::bitboard::Bitboard;
use crate::types::piece_type::PieceType;
use crate::movegen::board::Board;

#[derive(Clone)]
pub struct AttackInfo {
    pub attack_bitboard: [[Bitboard; PieceType::NUM_PIECE_TYPES]; Color::NUM_COLORS],
    pub piece_movement: [[Bitboard; Square::NUM_SQUARES]; Color::NUM_COLORS],

    pub movement_mask: [Bitboard; Color::NUM_COLORS],

    //pub zobrist_key: [Zobrist; Color::NUM_COLORS],
}

impl AttackInfo {
    pub fn update(&mut self, board: &Board, color: &Color) {
        /*
        // Check zobrist
        // Update zobrist
        for sq in Square::SQUARES {
            pieceMovement[color][sq.to_usize()] = Bitboard::EMPTY;
        }
        val checkBitboard = board.basicEvalInfo.checkBitboard

        val mask = when {
            checkBitboard == Bitboard.EMPTY -> {
            Bitboard.ALL
            }
            Bitboard.oneElement(checkBitboard) -> {
            val square = Square.getSquare(checkBitboard)
            val kingSquare = board.kingSquare[color]
            BitboardMove.BETWEEN_BITBOARD[kingSquare][square] or checkBitboard
            }
            else -> {
            Bitboard.EMPTY
            }
        }
        if (mask != Bitboard.EMPTY) {
            pawnAttacks(board, color, mask)
            knightMoves(board, color, mask)
            bishopMoves(board, color, mask)
            rookMoves(board, color, mask)
        }
        kingMoves(board, color)

        movementMask[color] = mask

        attacksBitboard[color][Piece.NONE] = attacksBitboard[color][Piece.PAWN] or
        attacksBitboard[color][Piece.KNIGHT] or
        attacksBitboard[color][Piece.BISHOP] or
        attacksBitboard[color][Piece.ROOK] or
        attacksBitboard[color][Piece.QUEEN] or
        attacksBitboard[color][Piece.KING]
        */

    }
}