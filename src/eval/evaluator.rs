use crate::types::color::Color;
use crate::simplified::game::Game;
use crate::eval::eval_constants::MATERIAL_SCORE;

impl Game {
    pub fn evaluate(&self) -> i16 {
        let mut result: i16 = 0;
        for square in self.position.game_bitboard().iterator() {
            let piece_type = self.position.piece_at(square);
            let color = self.position.color_at(square).unwrap();

            result += MATERIAL_SCORE[piece_type] * color.multiplier() as i16;
        }

        result
    }
}