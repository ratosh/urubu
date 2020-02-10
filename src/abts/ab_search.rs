use crate::simplified::position::Position;
use crate::types::move_list::MoveList;
use crate::types::board_move::BoardMove;
use crate::simplified::game::Game;

const SCORE_MAX : i16 = 32000;
const SCORE_MIN : i16 = -SCORE_MAX;

pub struct SearchInfo {
    pub node_count: u64,
    move_list: MoveList
}

impl SearchInfo {

    fn qsearch(&mut self,
        game: &mut Game,
        ply: u8,
        alpha: i16,
        beta: i16) -> (i16, BoardMove) {
        self.node_count += 1;

        let eval = game.evaluate() * game.position.ctm().multiplier() as i16;

        if eval >= beta {
            return (eval, BoardMove::NONE);
        }

        let mut best_score = eval.max(alpha);
        let mut best_move = BoardMove::NONE;

        self.move_list.start_ply();
        self.move_list.generate_noisy(&game.position);
        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            if !game.do_move(board_move) {
                continue;
            }
            let (search_score, _) = self.qsearch(game, ply + 1, -beta, -best_score);
            game.undo_move(board_move);

            if -search_score > best_score {
                best_score = -search_score;
                best_move = board_move;
            }
        }
        self.move_list.end_ply();
        (best_score, best_move)
    }

    fn absearch(&mut self,
        game: &mut Game,
        depth: u8, 
        ply: u8,
        alpha: i16,
        beta: i16) -> (i16, BoardMove) {
        let new_depth = depth - 1;
        if new_depth == 0 {
            return self.qsearch(game, ply, alpha, beta)
        }
        self.node_count += 1;

        let current_alpha = alpha.max(SCORE_MIN + ply as i16);
        let current_beta = beta.min(SCORE_MAX - (ply + 1) as i16);

        let eval = game.evaluate() * game.position.ctm().multiplier() as i16;
        let mut best_score = eval;
        let mut best_move = BoardMove::NONE;

        self.move_list.start_ply();
        self.move_list.generate_quiets(&game.position);
        self.move_list.generate_noisy(&game.position);
        while self.move_list.has_next() {
            let board_move = self.move_list.next();
            if !game.do_move(board_move) {
                continue;
            }
            let (search_score, _) = self.absearch(game, depth - 1, ply + 1, -current_beta, -current_alpha);
            game.undo_move(board_move);

            if -search_score > best_score {
                best_score = -search_score;
                best_move = board_move;
            }
        }
        self.move_list.end_ply();
        (best_score, best_move)
    }

    pub fn search(&mut self, game: &mut Game) -> BoardMove {
        let mut depth = 1;
        let mut best_move = BoardMove::NONE;
        let alpha = SCORE_MIN;
        let beta = SCORE_MAX;

        while depth <= 2 {
            let (_, inner_best_move) = self.absearch(game, depth, 0, alpha, beta);
            best_move = inner_best_move;
            depth += 1;
        }
        return best_move;
    }
}

impl Default for SearchInfo {
    fn default() -> Self {
        SearchInfo {
            node_count: 0,
            move_list: MoveList::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_qsearch(fen: &str, min_diff: i16, max_diff: i16) {
        let mut game = Game::from_fen(fen);
        let eval_value:i16 = game.evaluate();
        let mut search = SearchInfo {
            node_count: 0,
            move_list: MoveList::default()
        };
        let search_value = search.qsearch(
            &mut game,
            0,
            SCORE_MIN,
            SCORE_MAX
        );
        println!("node count {}", search.node_count);

        println!("{} {} {} {}", eval_value, search_value.0, min_diff, max_diff);
        assert!(eval_value + min_diff <= search_value.0);
        assert!(eval_value + max_diff >= search_value.0);
    }

    fn test_search(fen: &str, depth: u8) {
        let mut game = Game::from_fen(fen);
        let mut search = SearchInfo {
            node_count: 0,
            move_list: MoveList::default()
        };
        let search_value = search.absearch(
            &mut game,
            depth,
            0,
            SCORE_MIN,
            SCORE_MAX
        );
        println!("node count {}", search.node_count);
        println!("search result {} {}", search_value.0, search_value.1.to_string());
    }

    #[test]
    fn test_qsearch_capture() {
        test_qsearch("1k6/8/8/4p3/8/8/8/2K1R3 w - -", 50, 250);
    }

    #[test]
    fn test_search_start() {
        test_search(Position::DEFAULT_FEN, 2);
    }

    #[test]
    fn test_search_1() {
        test_search("1k6/8/8/4p3/8/8/8/2K1R3 w - -", 3);
    }
}