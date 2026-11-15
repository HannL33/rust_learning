use crate::gomoku_engine as ge;

pub fn evaluate(board: &ge::Board, player: ge::Player) -> f32 {
    // simple heuristics for evaluating the position of the game of gomoku
    match board.check_result() {
        ge::GameResult::Draw => 0.0,
        ge::GameResult::Win(winner) => {
            if player == winner {
                100_f32
            } else {
                -100_f32
            }
        }
        ge::GameResult::InProgress => evaluate_in_progress(board, player),
    }
}

/// Scores patterns after the caller has ruled out a win or draw.
pub(crate) fn evaluate_in_progress(board: &ge::Board, player: ge::Player) -> f32 {
    pattern_score(board, player) - pattern_score(board, player.opposite())
}

fn pattern_score(board: &ge::Board, player: ge::Player) -> f32 {
    let mut best_eval: f32 = 0.0;
    let stone_runs = find_stone_runs(board, player);
    let strong_threes = find_threes_with_open_four_potential(board, player, &stone_runs);

    let strong_three_weights = std::iter::repeat_n(50.0, strong_threes.len());
    let run_weights = stone_runs.iter().filter_map(|run| {
        let closed_ends = run.open_ends.iter().filter(|end| end.is_none()).count();
        let weight = match (run.length, closed_ends) {
            (1, _) => return None,
            (4, 0) => 85.0,
            (4, 1) => 55.0,
            (3, 0 | 1) => 30.0,
            (2, 0) => 5.0,
            _ => return None,
        };

        let part_of_strong_three = run_is_part_of_strong_three(run, &strong_threes);
        let too_short = available_line_length(board, run, player) < 5;

        (!part_of_strong_three && !too_short).then_some(weight)
    });

    // Each weight adds a percentage of the remaining gap to 100.
    let mut best_eval: f32 = 0.0;
    for weight in strong_three_weights.chain(run_weights) {
        best_eval += (100.0 - best_eval) * weight / 100.0;
    }

    best_eval.min(99.0)
}

/// A three-stone pattern that can become the open four `.XXXX.` in one move.

#[derive(Debug, Clone, PartialEq, Eq)]
struct ThreeWithOpenFourPotential {
    direction: ge::Direction,
    stones: [ge::Position; 3],
    moves_to_open_four: [Option<ge::Position>; 2],
}

/// Finds threes that can become open fours in one move, using `runs` from
/// [`find_stone_runs`] to locate the first stone of each candidate window.
///
/// A valid window is six cells long. Both ends must be empty, and the four
/// inner cells must contain exactly three stones of `player` and one empty
/// development cell.
fn find_threes_with_open_four_potential(
    board: &ge::Board,
    player: ge::Player,
    runs: &[StoneRun],
) -> Vec<ThreeWithOpenFourPotential> {
    let cells = board.cells();
    let own_stone = ge::Cell::Taken(player);
    let mut strong_threes = Vec::new();
    for run in runs {
        let [Some(before), Some(after)] = run.open_ends else {
            continue;
        };
        let (second_step, third_step) = match run.length {
            1 => (2, 3), // .X.XX.
            2 => (1, 3), // .XX.X.
            3 => (1, 2), // .XXX.. or ..XXX.
            _ => continue,
        };

        let Some(second) = offset_position(run.start, run.direction, second_step) else {
            continue;
        };
        let Some(third) = offset_position(run.start, run.direction, third_step) else {
            continue;
        };
        if !(cells[second.0][second.1] == own_stone && cells[third.0][third.1] == own_stone) {
            continue;
        }

        let mut moves_to_open_four = [None; 2];
        if let Some((row, col)) = offset_position(run.start, run.direction, 4)
            && cells[row][col] == ge::Cell::Empty
        {
            moves_to_open_four[0] = Some(after);
        }
        if run.length == 3
            && let Some((row, col)) = offset_position(run.start, run.direction, -2)
            && cells[row][col] == ge::Cell::Empty
        {
            moves_to_open_four[1] = Some(before);
        }
        if moves_to_open_four == [None; 2] {
            continue;
        }

        strong_threes.push(ThreeWithOpenFourPotential {
            direction: run.direction,
            stones: [run.start, second, third],
            moves_to_open_four,
        });
    }
    strong_threes
}

fn run_is_part_of_strong_three(
    run: &StoneRun,
    strong_threes: &[ThreeWithOpenFourPotential],
) -> bool {
    match run.length {
        1 => strong_threes
            .iter()
            .any(|three| three.direction == run.direction && three.stones.contains(&run.start)),
        2 => {
            let second = offset_position(run.start, run.direction, 1)
                .expect("a run of length 2 has a second stone on the board");
            strong_threes.iter().any(|three| {
                three.direction == run.direction
                    && three
                        .stones
                        .windows(2)
                        .any(|pair| pair[0] == run.start && pair[1] == second)
            })
        }
        3 => {
            let second = offset_position(run.start, run.direction, 1)
                .expect("a run of length 3 has a second stone on the board");
            let third = offset_position(run.start, run.direction, 2)
                .expect("a run of length 3 has a third stone on the board");
            strong_threes.iter().any(|three| {
                three.direction == run.direction && three.stones == [run.start, second, third]
            })
        }
        _ => false,
    }
}

/// Adds `steps` cells in `direction` with checked arithmetic and board bounds.
fn offset_position(
    start: ge::Position,
    direction: ge::Direction,
    steps: isize,
) -> Option<ge::Position> {
    let row = start
        .0
        .checked_add_signed(direction.0.checked_mul(steps)?)?;
    let column = start
        .1
        .checked_add_signed(direction.1.checked_mul(steps)?)?;
    (row < ge::BOARD_SIZE && column < ge::BOARD_SIZE).then_some((row, column))
}

#[derive(Debug)]
struct StoneRun {
    /// start is the start of the sequence in regards to the direction
    /// open_ends has an order [before first stone, after last stone]
    /// series of length 1 are also returned by find_stone_runs
    start: ge::Position,
    direction: ge::Direction,
    length: usize,
    open_ends: [Option<ge::Position>; 2],
}

fn find_stone_runs(board: &ge::Board, player: ge::Player) -> Vec<StoneRun> {
    const DIRECTIONS: [ge::Direction; 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
    let mut res_vec: Vec<StoneRun> = Vec::new();

    // we want to start collecting the StoneRuns as the beginning of the sequence
    // in regards to the DIRECTIONS
    for (r_idx, row) in board.cells().iter().enumerate() {
        for (c_idx, &cell) in row.iter().enumerate() {
            if cell != ge::Cell::Taken(player) {
                continue;
            }
            for (dr, dc) in DIRECTIONS {
                let prev = r_idx
                    .checked_sub_signed(dr)
                    .zip(c_idx.checked_sub_signed(dc))
                    .filter(|&(r, c)| r < board.size() && c < board.size());

                if prev.is_some_and(|(r, c)| board.cells()[r][c] == ge::Cell::Taken(player)) {
                    continue;
                }

                let open_before = prev.filter(|&(r, c)| board.cells()[r][c] == ge::Cell::Empty);

                // explore that series
                let mut res_stone = StoneRun {
                    start: (r_idx, c_idx),
                    direction: (dr, dc),
                    length: 1,
                    open_ends: [open_before, None],
                };

                let mut current = res_stone.start.clone();

                loop {
                    let next = current
                        .0
                        .checked_add_signed(dr)
                        .zip(current.1.checked_add_signed(dc))
                        .filter(|&(r, c)| r < board.size() && c < board.size());
                    match next {
                        Some((r, c)) => {
                            if board.cells()[r][c] == ge::Cell::Taken(player.opposite()) {
                                break;
                            }
                            if board.cells()[r][c] == ge::Cell::Empty {
                                res_stone.open_ends[1] = Some((r, c));
                                break;
                            }
                            res_stone.length += 1;
                            current = (r, c);
                        }
                        None => {
                            break;
                        }
                    }
                }
                res_vec.push(res_stone);
            }
        }
    }

    res_vec
}

fn available_line_length(board: &ge::Board, run: &StoneRun, player: ge::Player) -> usize {
    let mut max_len = run.length;

    if run.open_ends.iter().filter(|end| end.is_none()).count() == 2 {
        return max_len;
    }

    let direction = run.direction;
    if run.open_ends[0].is_some() {
        let mut current_pos = run.open_ends[0].unwrap();
        max_len += 1;
        loop {
            let prev = current_pos
                .0
                .checked_sub_signed(direction.0)
                .zip(current_pos.1.checked_sub_signed(direction.1))
                .filter(|&(r, c)| r < board.size() && c < board.size());
            match prev {
                Some((p_r, p_c)) => {
                    if board.cells()[p_r][p_c] == ge::Cell::Taken(player.opposite()) {
                        break;
                    } else {
                        max_len += 1;
                        current_pos = (p_r, p_c);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
    if run.open_ends[1].is_some() {
        let mut current_pos = run.open_ends[1].unwrap();
        max_len += 1;
        loop {
            let next = current_pos
                .0
                .checked_add_signed(direction.0)
                .zip(current_pos.1.checked_add_signed(direction.1))
                .filter(|&(r, c)| r < board.size() && c < board.size());
            match next {
                Some((n_r, n_c)) => {
                    if board.cells()[n_r][n_c] == ge::Cell::Taken(player.opposite()) {
                        break;
                    } else {
                        max_len += 1;
                        current_pos = (n_r, n_c);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
    max_len
}

#[cfg(test)]
mod tests;
