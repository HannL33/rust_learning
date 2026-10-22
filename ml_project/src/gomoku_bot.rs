use crate::{gomoku_engine as ge, gomoku_evaluation::evaluate, tic_tac_engine::Cell::Taken};
const CLOSE_RANGE_PARAM: usize = 2;
const FORCING_BUDGET: usize = 5;

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

            if !board.win_in_one(player).is_empty() {
                if player == bot {
                    return 100;
                } else {
                    return -100;
                }
            }
            let mut best = if is_maximizing { i32::MIN } else { i32::MAX };
            let opponent_wins = board.win_in_one(player.opposite());
            let mut moves = match opponent_wins.len() {
                0 => {
                    if depth == 0 {
                        return evaluate(&board, bot) as i32;
                    }
                    board.available_empty_moves_close(2)
                }
                1 => opponent_wins,
                _ => {
                    if player == bot {
                        return -100;
                    } else {
                        return 100;
                    }
                }
            };

            let center = board.size() / 2;
            moves.sort_by_key(|(row, col)| row.abs_diff(center) + col.abs_diff(center));
            if moves.is_empty() {
                return evaluate(&board, bot) as i32;
            }

            for coor in moves {
                let new_board = board
                    .apply_move(coor, player)
                    .expect("Expect the best right now!");
                let score = minimax_alpha_beta(
                    new_board,
                    !is_maximizing,
                    bot,
                    depth.saturating_sub(1),
                    alpha,
                    beta,
                );
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

    if let Some(position) = board.win_in_one(player).first() {
        return Some(*position);
    }
    let opponent_wins = board.win_in_one(player.opposite());
    if opponent_wins.len() == 1 {
        return Some(opponent_wins[0]);
    }
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
    let mut moves = board.available_empty_moves_close(CLOSE_RANGE_PARAM);

    moves.sort_by_key(|&(row, col)| row.abs_diff(center) + col.abs_diff(center));

    for coor in moves {
        let new_board = board.apply_move(coor, player).expect("Expect the best");
        let score;
        if search_forcing_line(&new_board, player, player.opposite(), FORCING_BUDGET)
            == ForcingResult::ProvenWin
        {
            score = 100
        } else if search_forcing_line(
            &new_board,
            player.opposite(),
            player.opposite(),
            FORCING_BUDGET,
        ) == ForcingResult::ProvenWin
        {
            score = -100
        } else {
            score = minimax_alpha_beta(new_board, false, player, depth - 1, alpha, beta);
        }
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

#[derive(Debug, PartialEq)]
enum ForcingResult {
    ProvenWin,
    Unknown,
}

/**
We only find moves, that are threats for the opponent
We do not know if the threat lead to the win
*/
fn forcing_moves(board: &ge::Board, player: ge::Player) -> Vec<ge::Position> {
    let mut res_vec: Vec<ge::Position> = Vec::new();
    let available_close_moves = board.available_empty_moves_close(CLOSE_RANGE_PARAM);
    for av_move in available_close_moves {
        let next_board = board
            .apply_move(av_move, player)
            .expect("Expected the possible move to play in forcing moves.");
        if !next_board.win_in_one(player).is_empty() {
            res_vec.push(av_move);
        }
    }

    res_vec
}

pub fn search_forcing_line(
    board: &ge::Board,
    attacker: ge::Player,
    to_move: ge::Player,
    budget: usize,
) -> ForcingResult {
    if board.check_result() == ge::GameResult::Win(attacker) {
        return ForcingResult::ProvenWin;
    } else if board.check_result() == ge::GameResult::Draw
        || board.check_result() == ge::GameResult::Win(attacker.opposite())
    {
        return ForcingResult::Unknown;
    }

    if !board.win_in_one(to_move).is_empty() {
        if to_move == attacker {
            return ForcingResult::ProvenWin;
        } else {
            return ForcingResult::Unknown;
        }
    }
    let n_wins = board.win_in_one(to_move.opposite()).len();
    if n_wins >= 2 {
        if to_move.opposite() == attacker {
            return ForcingResult::ProvenWin;
        } else {
            return ForcingResult::Unknown;
        }
    };
    if n_wins == 1 && budget >= 1 {
        let next_forcing_move = board.win_in_one(to_move.opposite())[0];
        let next_board = board
            .apply_move(next_forcing_move, to_move)
            .expect("We expect in search forcing line to apply move correctly!");
        return search_forcing_line(&next_board, attacker, to_move.opposite(), budget - 1);
    };

    if to_move == attacker && budget >= 1 {
        for threat_move in forcing_moves(board, attacker) {
            let next_board = board
                .apply_move(threat_move, to_move)
                .expect("We expect in search forcing line to apply move correctly!");
            if search_forcing_line(&next_board, attacker, to_move.opposite(), budget - 1)
                == ForcingResult::ProvenWin
            {
                return ForcingResult::ProvenWin;
            };
        }
    }

    ForcingResult::Unknown
}

#[cfg(test)]
mod tests;
