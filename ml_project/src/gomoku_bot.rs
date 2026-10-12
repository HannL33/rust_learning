use crate::{gomoku_engine as ge, gomoku_evaluation::evaluate};

// evaluation in [-100, 100]
pub fn minimax(board: ge::Board, is_maximizing: bool, bot: ge::Player, depth: usize) -> i32 {
    match board.check_result() {
        ge::GameResult::Win(_player) => {
            if _player == bot {
                return 100;
            } else {
                return -100;
            }
        }
        ge::GameResult::Draw => return 0,
        ge::GameResult::InProgress => {
            let player = if is_maximizing { bot } else { bot.opposite() };
            let mut best = if is_maximizing { i32::MIN } else { i32::MAX };
            if depth == 0 {
                evaluate(&board, bot) as i32
            } else {
                for coor in board.available_empty_moves() {
                    let new_board = board
                        .apply_move(coor, player)
                        .expect("Expect the best right now!");
                    let score = minimax(new_board, !is_maximizing, bot, depth - 1);
                    best = if is_maximizing {
                        best.max(score)
                    } else {
                        best.min(score)
                    };
                }
                best
            }
        }
    }
}
pub fn minimax_alpha_beta(
    board: ge::Board,
    is_maximizing: bool,
    bot: ge::Player,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    match board.check_result() {
        ge::GameResult::Win(_player) => {
            if _player == bot {
                return 100;
            } else {
                return -100;
            }
        }
        ge::GameResult::Draw => return 0,
        ge::GameResult::InProgress => {
            let player = if is_maximizing { bot } else { bot.opposite() };
            let mut best = if is_maximizing { i32::MIN } else { i32::MAX };
            if depth == 0 {
                evaluate(&board, bot) as i32
            } else {
                let center = board.size() / 2;
                let mut moves = board.available_empty_moves_close(2);
                moves.sort_by_key(|(row, col)| row.abs_diff(center) + col.abs_diff(center));
                if moves.is_empty() {
                    return evaluate(&board, bot) as i32;
                }

                for coor in moves {
                    // 2 is for now hyperparameter
                    let new_board = board
                        .apply_move(coor, player)
                        .expect("Expect the best right now!");
                    let score =
                        minimax_alpha_beta(new_board, !is_maximizing, bot, depth - 1, alpha, beta);
                    if is_maximizing {
                        best = best.max(score);
                        alpha = alpha.max(best);
                    } else {
                        best = best.min(score);
                        beta = beta.min(best);
                    };
                    if alpha >= beta {
                        break;
                    }
                }
                best
            }
        }
    }
}

pub fn best_move_alpha_beta(
    board: ge::Board,
    player: ge::Player,
    depth: usize,
) -> Option<(usize, usize)> {
    if matches!(
        board.check_result(),
        ge::GameResult::Draw | ge::GameResult::Win(_)
    ) {
        return None;
    }
    assert!(depth > 0, "Search depth must be at least 1");

    let center = board.size() / 2;
    let mut best = i32::MIN;
    let mut chosen = None;
    let mut alpha = i32::MIN;
    let beta = i32::MAX;
    if board
        .cells()
        .iter()
        .flatten()
        .all(|cell| *cell == ge::Cell::Empty)
    {
        return Some((board.size() / 2, board.size() / 2));
    }
    let mut moves = board.available_empty_moves_close(2);

    moves.sort_by_key(|&(row, col)| row.abs_diff(center) + col.abs_diff(center));

    for coor in moves {
        let new_board = board.apply_move(coor, player).expect("Expect the best");
        let score = minimax_alpha_beta(new_board, false, player, depth - 1, alpha, beta);
        if score > best {
            best = score;
            chosen = Some(coor);
        }
        alpha = alpha.max(best);
        if best == 100 {
            break;
        }
    }
    println!("Curren evaluation by bot: {:?}", best);
    Some(chosen).flatten()
}

pub fn best_move(board: ge::Board, player: ge::Player, depth: usize) -> Option<(usize, usize)> {
    if matches!(
        board.check_result(),
        ge::GameResult::Draw | ge::GameResult::Win(_)
    ) {
        return None;
    }
    assert!(depth > 0, "Search depth must be at least 1");

    let mut best = i32::MIN;
    let mut chosen = None;

    let center = board.size() / 2;
    if board
        .cells()
        .iter()
        .flatten()
        .all(|cell| *cell == ge::Cell::Empty)
    {
        return Some((center, center));
    };
    let mut moves = board.available_empty_moves();
    moves.sort_by_key(|&(row, col)| row.abs_diff(center) + col.abs_diff(center));

    for coor in moves {
        let new_board = board.apply_move(coor, player).expect("Expect the best");
        let score = minimax(new_board, false, player, depth - 1);
        if score > best {
            best = score;
            chosen = Some(coor);
        }
        if best == 100 {
            break;
        }
    }
    println!("Curren evaluation by bot: {:?}", best);
    Some(chosen).flatten()
}

#[cfg(test)]
mod tests;
