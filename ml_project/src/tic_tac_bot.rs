use crate::tic_tac_engine as tte;

/// A selected move and the number of search nodes visited while choosing it.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchResult {
    pub position: Option<usize>,
    /// Includes terminal nodes in candidate subtrees, but not the root board.
    /// Zero when the game has already ended.
    pub visited_nodes: usize,
}

/// Searches to the end of the game and scores the position from `bot`'s perspective.
/// `is_maximizing` is true on the bot's turn and false on the opponent's turn.
/// `count` counts visited nodes, including terminal positions.
pub fn minimax(board: tte::Board, is_maximizing: bool, bot: tte::Player, count: &mut usize) -> i32 {
    *count += 1;
    match tte::check_winner(&board) {
        tte::GameResult::InProgress => {
            let player = if is_maximizing { bot } else { bot.opposite() };
            let mut best = if is_maximizing { i32::MIN } else { i32::MAX };
            for (idx, cell) in board.cells().enumerate() {
                if cell == tte::Cell::Empty {
                    let next = tte::apply_move(board, idx, player)
                        .expect("an empty cell must accept a move");
                    let score = minimax(next, !is_maximizing, bot, count);
                    best = if is_maximizing {
                        best.max(score)
                    } else {
                        best.min(score)
                    };
                }
            }
            best
        }
        tte::GameResult::Draw => 0,
        tte::GameResult::Win(winner) => {
            if winner == bot {
                10
            } else {
                -10
            }
        }
    }
}

pub fn minimax_alpha_beta(
    board: tte::Board,
    is_maximizing: bool,
    bot: tte::Player,
    mut alpha: i32,
    mut beta: i32,
    count: &mut usize,
) -> i32 {
    *count += 1;
    match tte::check_winner(&board) {
        tte::GameResult::InProgress => {
            let player = if is_maximizing { bot } else { bot.opposite() };
            let mut best = if is_maximizing { i32::MIN } else { i32::MAX };
            for (idx, cell) in board.cells().enumerate() {
                if cell == tte::Cell::Empty {
                    let next = tte::apply_move(board, idx, player)
                        .expect("an empty cell must accept a move");
                    let score = minimax_alpha_beta(next, !is_maximizing, bot, alpha, beta, count);
                    if is_maximizing {
                        best = best.max(score);
                        alpha = alpha.max(best);
                    } else {
                        best = best.min(score);
                        beta = beta.min(best);
                    }
                    if alpha >= beta {
                        break;
                    }
                }
            }
            best
        }
        tte::GameResult::Draw => 0,
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
/// The board must be a legal position with `player` on turn.
pub fn best_move(board: tte::Board, player: tte::Player) -> Option<usize> {
    best_move_with_stats(board, player).position
}

/// Selects an optimal move using minimax and returns search statistics.
/// The board must be a legal position with `player` on turn.
pub fn best_move_with_stats(board: tte::Board, player: tte::Player) -> SearchResult {
    if tte::check_winner(&board) != tte::GameResult::InProgress {
        return SearchResult::default();
    }
    let mut best = i32::MIN;
    let mut chosen = None;
    let mut count = 0;
    for (idx, cell) in board.cells().enumerate() {
        if cell == tte::Cell::Empty {
            let next =
                tte::apply_move(board, idx, player).expect("an empty cell must accept a move");
            let score = minimax(next, false, player, &mut count);
            if score >= best {
                best = score;
                chosen = Some(idx);
            }
            if best == 10 {
                break;
            }
        }
    }
    SearchResult {
        position: chosen,
        visited_nodes: count,
    }
}

/// Returns an optimal move using alpha-beta pruning, or `None` after the game ends.
/// The board must be a legal position with `player` on turn.
pub fn best_move_alpha_beta(board: tte::Board, player: tte::Player) -> Option<usize> {
    best_move_alpha_beta_with_stats(board, player).position
}

/// Selects an optimal move using alpha-beta pruning and returns search statistics.
/// The board must be a legal position with `player` on turn.
pub fn best_move_alpha_beta_with_stats(board: tte::Board, player: tte::Player) -> SearchResult {
    if tte::check_winner(&board) != tte::GameResult::InProgress {
        return SearchResult::default();
    }
    let mut best = i32::MIN;
    let mut chosen = None;
    let mut count = 0;
    let mut alpha = i32::MIN;
    let beta = i32::MAX;

    for (idx, cell) in board.cells().enumerate() {
        if cell == tte::Cell::Empty {
            let next =
                tte::apply_move(board, idx, player).expect("an empty cell must accept a move");
            let score = minimax_alpha_beta(next, false, player, alpha, beta, &mut count);
            // A score equal to alpha may only be an upper bound after a cutoff.
            // Keep the previously proven move unless the new score is strictly better.
            if score > best {
                best = score;
                chosen = Some(idx);
            }
            alpha = alpha.max(best);
            if best == 10 {
                break;
            }
        }
    }
    SearchResult {
        position: chosen,
        visited_nodes: count,
    }
}

/// Test written by AI
#[cfg(test)]
mod tests;
