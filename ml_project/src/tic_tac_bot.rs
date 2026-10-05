use crate::tic_tac_engine as tte;

/// Searches to the end of the game and scores the position from `bot`'s perspective.
/// `is_maximizing` is true on the bots turn and false on the opponents turn.
pub fn minimax(board: tte::Board, is_maximizing: bool, bot: tte::Player) -> i32 {
    match tte::check_winner(&board) {
        tte::GameResult::InProgress => {
            if is_maximizing {
                let mut score_vec = Vec::new();
                for (idx, cell) in board.cells().enumerate() {
                    if cell == tte::Cell::Empty {
                        let board = tte::apply_move(board, idx, bot).expect(
                            "expected the best after the unpacking apply move error in minimax",
                        );
                        score_vec.push(minimax(board, !is_maximizing, bot))
                    }
                }
                let result = *score_vec
                    .iter()
                    .max()
                    .expect("There is always maximum i think");
                result
            } else {
                let mut score_vec = Vec::new();
                for (idx, cell) in board.cells().enumerate() {
                    if cell == tte::Cell::Empty {
                        let board = tte::apply_move(board, idx, bot.opposite()).expect(
                            "expected the best after the unpacking apply move error in minimax",
                        );
                        score_vec.push(minimax(board, !is_maximizing, bot))
                    }
                }
                let result = *score_vec
                    .iter()
                    .min()
                    .expect("There is always minimum i think");
                result
            }
        }
        tte::GameResult::Draw => return 0,
        tte::GameResult::Win(winner) => {
            if winner == bot {
                10
            } else {
                -10
            }
        }
    }
}

/// Returns an optimal move, or `None` if the game has already ended.
/// The board must be a legal position with player on turn.
pub fn best_move(board: tte::Board, player: tte::Player) -> Option<usize> {
    if tte::check_winner(&board) != tte::GameResult::InProgress {
        return None;
    }
    let mut best_eval_move = (-10, None);
    for (idx, cell) in board.cells().enumerate() {
        if cell == tte::Cell::Empty {
            let _board = tte::apply_move(board, idx, player)
                .expect("Something wrong happen during the apply move in best move!");
            let eval = minimax(_board, false, player);
            if eval == 10 {
                return idx;
            } else if eval >= best_eval_move.0 {
                best_eval_move.0 = eval;
                best_eval_move.1 = idx;
            }
        }
    }
    best_eval_move.1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_after(moves: &[usize]) -> tte::Game {
        let mut game = tte::Game::new();
        for &position in moves {
            game.play(position).expect("test moves must be legal");
        }
        game
    }

    #[test]
    fn best_move_takes_an_immediate_win_for_either_player() {
        for moves in [&[0, 3, 1, 4, 6, 7][..], &[3, 0, 4, 1, 6, 5, 8][..]] {
            let mut game = game_after(moves);
            let bot = game.turn();
            assert_eq!(minimax(*game.board(), true, bot), 10);
            assert_eq!(best_move(*game.board(), bot), Some(2));
            game.play(2).unwrap();
            assert_eq!(game.result(), tte::GameResult::Win(bot));
        }
    }

    #[test]
    fn best_move_blocks_an_immediate_loss() {
        // X X . / . O . / . . . — O must block the top row.
        let mut game = game_after(&[0, 4, 1]);
        let bot = game.turn();
        assert_eq!(minimax(*game.board(), true, bot), 0);
        assert_eq!(best_move(*game.board(), bot), Some(2));
        game.play(2).unwrap();
        assert_eq!(game.result(), tte::GameResult::InProgress);
    }

    #[test]
    fn best_move_returns_none_after_a_win_or_draw() {
        let won = game_after(&[0, 3, 1, 4, 2]);
        let drawn = game_after(&[0, 1, 2, 4, 3, 5, 7, 6, 8]);
        for game in [won, drawn] {
            for player in [tte::Player::X, tte::Player::O] {
                assert_eq!(best_move(*game.board(), player), None);
            }
        }
    }

    #[test]
    fn two_optimal_bots_finish_in_a_draw() {
        let mut game = tte::Game::new();
        for _ in 0..9 {
            if game.result() != tte::GameResult::InProgress {
                break;
            }
            let position = best_move(*game.board(), game.turn())
                .expect("an unfinished game must have a legal move");
            game.play(position)
                .expect("the bot must choose a legal move");
        }
        assert_eq!(game.result(), tte::GameResult::Draw);
    }
}
