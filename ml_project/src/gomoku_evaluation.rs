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
    let stone_runs = find_stone_runs(board, player);

    for stone_run in stone_runs {
        if available_line_length(board, stone_run, player) < 5 {
            continue;
        }
        let closed_ends = stone_run
            .open_ends
            .iter()
            .filter(|end| end.is_none())
            .count();

        // nice and fancy match instead of if's...
        let weight = match (stone_run.length, closed_ends) {
            (4, 0) => 85.0,
            (4, 1) => 55.0,
            (3, 0) => 50.0,
            (3, 1) => 30.0,
            (2, 0) => 5.0,
            _ => 0.0,
        };

        best_eval += (100.0 - best_eval) * weight / 100.0;
    }

    best_eval.min(99.0)
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
