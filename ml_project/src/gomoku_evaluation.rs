use crate::gomoku_engine as ge;

pub fn evaluate(board: &ge::Board, player: ge::Player) -> f32 {
    // simple heuristics for evaluating the position of the game of gomoku
    match board.check_result() {
        ge::GameResult::Draw => return 0.0,
        ge::GameResult::Win(_player) => {
            if player == _player {
                100_f32
            } else {
                -100_f32
            }
        }
        ge::GameResult::InProgress => {
            pattern_score(board, player) - pattern_score(board, player.opposite())
        }
    }
}

fn pattern_score(board: &ge::Board, player: ge::Player) -> f32 {
    let mut best_eval: f32 = 0.0;

    for stone_run in check_exactly_n_stones(board, player, 4) {
        if stone_run.closed_ends == 0 {
            best_eval += (100_f32 - best_eval) * 85_f32 / 100_f32;
        } else if stone_run.closed_ends == 1 {
            best_eval += (100_f32 - best_eval) * 55_f32 / 100_f32;
        }
    }
    // check for 3 stones in any direction
    for stone_run in check_exactly_n_stones(board, player, 3) {
        if stone_run.closed_ends == 0 {
            best_eval += (100_f32 - best_eval) * 50_f32 / 100_f32;
        }
        if stone_run.closed_ends == 1 {
            best_eval += (100_f32 - best_eval) * 30_f32 / 100_f32;
        }
    }
    for stone_run in check_exactly_n_stones(board, player, 2) {
        if stone_run.closed_ends == 0 {
            best_eval += (100_f32 - best_eval) * 5_f32 / 100_f32;
        }
    }
    best_eval.min(99.0)
}

struct StonesFollowingLayout {
    coordinates: Vec<(usize, usize)>,
    closed_ends: usize, // 0, 1, 2
}

fn check_exactly_n_stones(
    board: &ge::Board,
    player: ge::Player,
    n_stones: usize,
) -> Vec<StonesFollowingLayout> {
    // check if there is any configuration for `n` stones following each other
    // in any direction
    // but it should check if those configurations have opposite player neighboardhood!
    if n_stones >= board.size() || n_stones < 1 {
        return Vec::new();
    }
    let mut result: Vec<StonesFollowingLayout> = Vec::new();
    for (r_idx, row) in board.cells().iter().enumerate() {
        for (c_idx, _) in row.iter().enumerate() {
            // check horizontals
            if c_idx + n_stones <= board.size()
                && row[c_idx..c_idx + n_stones]
                    .iter()
                    .all(|&x| x == ge::Cell::Taken(player))
                // we need to find exactly n_stones, not *n_stones - 1* inside n_stones sequence
                && !((c_idx > 0 && row[c_idx - 1] == ge::Cell::Taken(player))
                    || (c_idx + n_stones < board.size()
                        && row[c_idx + n_stones] == ge::Cell::Taken(player)))
            {
                let mut to_add = StonesFollowingLayout {
                    coordinates: Vec::new(),
                    closed_ends: 0,
                };
                // add all of the found stones
                for idx in c_idx..c_idx + n_stones {
                    to_add.coordinates.push((r_idx, idx));
                }
                // now check the opponent/border surrounding
                if c_idx == 0 {
                    to_add.closed_ends = 1;
                    if row[n_stones] == ge::Cell::Taken(player.opposite()) {
                        to_add.closed_ends += 1;
                    }
                } else if c_idx + n_stones == board.size() {
                    to_add.closed_ends = 1;
                    if row[c_idx - 1] == ge::Cell::Taken(player.opposite()) {
                        to_add.closed_ends += 1
                    }
                } else {
                    if row[c_idx - 1] == ge::Cell::Taken(player.opposite()) {
                        to_add.closed_ends += 1
                    }
                    if row[c_idx + n_stones] == ge::Cell::Taken(player.opposite()) {
                        to_add.closed_ends += 1
                    }
                }
                result.push(to_add);
            }

            // check verticals
            if r_idx + n_stones <= board.size()
                && board.cells()[r_idx..r_idx + n_stones]
                    .iter()
                    .all(|&x| x[c_idx] == ge::Cell::Taken(player))
                && !((r_idx > 0 && board.cells()[r_idx - 1][c_idx] == ge::Cell::Taken(player))
                    || (r_idx + n_stones < board.size()
                        && board.cells()[r_idx + n_stones][c_idx] == ge::Cell::Taken(player)))
            {
                let mut to_add = StonesFollowingLayout {
                    coordinates: Vec::new(),
                    closed_ends: 0,
                };
                // add all of the found stones
                for idx in r_idx..r_idx + n_stones {
                    to_add.coordinates.push((idx, c_idx));
                }
                // now check the opponent/border surrounding
                if r_idx == 0 {
                    to_add.closed_ends = 1;
                    if board.cells()[n_stones][c_idx] == ge::Cell::Taken(player.opposite()) {
                        to_add.closed_ends += 1;
                    }
                } else if r_idx + n_stones == board.size() {
                    to_add.closed_ends = 1;
                    if board.cells()[r_idx - 1][c_idx] == ge::Cell::Taken(player.opposite()) {
                        to_add.closed_ends += 1
                    }
                } else {
                    if board.cells()[r_idx - 1][c_idx] == ge::Cell::Taken(player.opposite()) {
                        to_add.closed_ends += 1
                    }
                    if board.cells()[r_idx + n_stones][c_idx] == ge::Cell::Taken(player.opposite())
                    {
                        to_add.closed_ends += 1
                    }
                }
                result.push(to_add);
            }

            // check diagonal from left to right
            if r_idx + n_stones <= board.size()
                && c_idx + n_stones <= board.size()
                && (0..n_stones)
                    .all(|i| board.cells()[r_idx + i][c_idx + i] == ge::Cell::Taken(player))
                && !((r_idx > 0
                    && c_idx > 0
                    && board.cells()[r_idx - 1][c_idx - 1] == ge::Cell::Taken(player))
                    || (r_idx + n_stones < board.size()
                        && c_idx + n_stones < board.size()
                        && board.cells()[r_idx + n_stones][c_idx + n_stones]
                            == ge::Cell::Taken(player)))
            {
                let mut to_add = StonesFollowingLayout {
                    coordinates: Vec::new(),
                    closed_ends: 0,
                };
                // add all of the found stones
                for idx in 0..n_stones {
                    to_add.coordinates.push((r_idx + idx, c_idx + idx));
                }
                // now check the opponent/border surrounding
                if r_idx == 0
                    || c_idx == 0
                    || board.cells()[r_idx - 1][c_idx - 1] == ge::Cell::Taken(player.opposite())
                {
                    to_add.closed_ends += 1;
                }

                if r_idx + n_stones == board.size()
                    || c_idx + n_stones == board.size()
                    || board.cells()[r_idx + n_stones][c_idx + n_stones]
                        == ge::Cell::Taken(player.opposite())
                {
                    to_add.closed_ends += 1;
                }
                result.push(to_add);
            }

            // check diagonal from right to left
            if r_idx + n_stones <= board.size()
                && c_idx >= n_stones - 1
                && (0..n_stones)
                    .all(|i| board.cells()[r_idx + i][c_idx - i] == ge::Cell::Taken(player))
                && !((r_idx > 0
                    && c_idx + 1 < board.size()
                    && board.cells()[r_idx - 1][c_idx + 1] == ge::Cell::Taken(player))
                    || (r_idx + n_stones < board.size()
                        && c_idx >= n_stones
                        && board.cells()[r_idx + n_stones][c_idx - n_stones]
                            == ge::Cell::Taken(player)))
            {
                let mut to_add = StonesFollowingLayout {
                    coordinates: Vec::new(),
                    closed_ends: 0,
                };
                // add all of the found stones
                for idx in 0..n_stones {
                    to_add.coordinates.push((r_idx + idx, c_idx - idx));
                }
                // now check the opponent/border surrounding
                if r_idx == 0
                    || c_idx + 1 == board.size()
                    || board.cells()[r_idx - 1][c_idx + 1] == ge::Cell::Taken(player.opposite())
                {
                    to_add.closed_ends += 1;
                }

                if r_idx + n_stones == board.size()
                    || c_idx == n_stones - 1
                    || board.cells()[r_idx + n_stones][c_idx - n_stones]
                        == ge::Cell::Taken(player.opposite())
                {
                    to_add.closed_ends += 1;
                }
                result.push(to_add);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests;
