use crate::gomoku_engine as ge;

pub fn evaluate(board: &ge::Board, player: ge::Player) -> i32 {
    // simple heuristics for evaluating the position of the game of gomoku
    match board.check_result() {
        ge::GameResult::Draw => return 0,
        ge::GameResult::Win(_player) => {
            if player == _player {
                100
            } else {
                -100
            }
        }
        ge::GameResult::InProgress => {
            // check for 4 stones in any direction

            // we wanna find the best evaluation
            // i.e if we can find 4 stones without closed ends then this is almost winning
            // and having this layout and finding later 2 stones with closed end then it does not
            // mean that we have worse position
            let mut best_eval: i32 = -100;
            for stone_run in check_n_stones(board, player, 4) {
                if stone_run.closed_ends == 0 {
                    if best_eval < 80 {
                        best_eval = 80;
                    };
                } else if stone_run.closed_ends == 1 {
                    if best_eval < 50 {
                        best_eval = 50;
                    }
                }
            }
            // check for 3 stones in any direction
            for stone_run in check_n_stones(board, player, 3) {
                if stone_run.closed_ends == 0 {
                    if best_eval < 55 {
                        best_eval = 55;
                    }
                }
                if stone_run.closed_ends == 1 {
                    if best_eval < 30 {
                        best_eval = 30;
                    }
                }
            }
            // we will not check 2 and less stones
            return best_eval;
        }
    }
}

struct StonesFollowingLayout {
    coordinates: Vec<(usize, usize)>,

    closed_ends: usize, // 0, 1, 2
}

fn check_n_stones(
    board: &ge::Board,
    player: ge::Player,
    n_stones: usize,
) -> Vec<StonesFollowingLayout> {
    // check if there is any configuration for `n` stones following each other
    // in any direction
    // but it should check if those configurations have opposite player neighboardhood!
    let mut result: Vec<StonesFollowingLayout> = Vec::new();
    for (r_idx, row) in board.cells().iter().enumerate() {
        for (c_idx, cell) in row.iter().enumerate() {
            // check horizontals
            if c_idx + n_stones <= board.size()
                && row[c_idx..c_idx + n_stones]
                    .iter()
                    .all(|&x| x == ge::Cell::Taken(player))
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
            {}

            // check diagonal from left to right
            if r_idx + n_stones <= board.size()
                && c_idx + n_stones <= board.size()
                && (0..n_stones)
                    .all(|i| board.cells()[r_idx + i][c_idx + i] == ge::Cell::Taken(player))
            {
            }

            // check diagonal from right to left
            if r_idx + n_stones <= board.size()
                && c_idx >= n_stones
                && (0..n_stones)
                    .all(|i| board.cells()[r_idx + i][c_idx - i] == ge::Cell::Taken(player))
            {
            }
        }
    }
    result
}
