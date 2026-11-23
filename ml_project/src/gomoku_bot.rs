use crate::{
    gomoku_engine as ge,
    gomoku_evaluation::{evaluate_in_progress, offset_position},
};
const CLOSE_RANGE_PARAM: usize = 2;
const FORCING_BUDGET: usize = 8;
const WIN_MAX_SCORE: f32 = 10_000.0;
const WIN_IN_FIVE_SCORE: f32 = WIN_MAX_SCORE - 5.0;
const MIN_MATE_SCORE: f32 = WIN_MAX_SCORE - (ge::BOARD_SIZE * ge::BOARD_SIZE) as f32;

struct ProvenCandidate {
    position: ge::Position,
    plies: usize,
    minimax_score: Option<f32>,
}

struct UnresolvedCandidate {
    position: ge::Position,
    minimax_score: f32,
}

enum RootCandidate {
    ProvenWin(ProvenCandidate),
    ProvenLoss(ProvenCandidate),
    Unresolved(UnresolvedCandidate),
}

enum ProvenGroup {
    Wins(Vec<ProvenCandidate>),
    Losses(Vec<ProvenCandidate>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WinningReplies {
    None,
    One(ge::Position),
    Multiple,
}

impl WinningReplies {
    fn from_moves(moves: &[ge::Position]) -> Self {
        match moves {
            [] => Self::None,
            [position] => Self::One(*position),
            _ => Self::Multiple,
        }
    }
}

const AXES: [ge::Direction; 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

struct SideScan {
    /// Consecutive own stones next to the anchor.
    stones_before_gap: usize,
    /// The first empty square reached by the scan.
    first_gap: Option<ge::Position>,
    /// Consecutive own stones immediately after the gap.
    stones_after_gap: usize,
}

/// Counts stones beside the anchor and beyond the first gap on one side.
fn scan_side(
    board: &ge::Board,
    position: ge::Position,
    direction: ge::Direction,
    own: ge::Cell,
) -> SideScan {
    let mut stones_before_gap = 0;
    let mut first_gap = None;
    let mut stones_after_gap = 0;
    for distance in 1..=4 {
        let Some((row, col)) = offset_position(position, direction, distance) else {
            break;
        };
        match board.cells()[row][col] {
            cell if cell == own => {
                // here we do not match the specific expression, just irrefutable, only binding the board.cells()[row][col] to the cell variable and then checking the guard, so we pick everything, bind it into `cell` and check condition, if so then go to the body of this case
                if first_gap.is_some() {
                    stones_after_gap += 1;
                } else {
                    stones_before_gap += 1;
                }
            }
            ge::Cell::Empty if first_gap.is_none() => first_gap = Some((row, col)),
            _ => break,
        }
    }
    SideScan {
        stones_before_gap,
        first_gap,
        stones_after_gap,
    }
}

/// Finds up to two winning squares on lines through the stone just played.
/// At a quiet or mandatory-block node these are all newly created threats.
fn winning_replies_through(
    board: &ge::Board,
    last_move: ge::Position,
    player: ge::Player,
) -> WinningReplies {
    let own = ge::Cell::Taken(player);
    let mut replies = WinningReplies::None;
    for (row_step, col_step) in AXES {
        let backward = scan_side(board, last_move, (-row_step, -col_step), own);
        let forward = scan_side(board, last_move, (row_step, col_step), own);
        let connected_stones = 1 + backward.stones_before_gap + forward.stones_before_gap;

        for side in [backward, forward] {
            let Some(gap_position) = side.first_gap else {
                continue;
            };
            let line_length_after_move = connected_stones + 1 + side.stones_after_gap;
            if line_length_after_move < 5 {
                continue;
            }

            if replies != WinningReplies::None {
                return WinningReplies::Multiple;
            }
            replies = WinningReplies::One(gap_position);
        }
    }
    replies
}

fn mate(bot: ge::Player, winner: ge::Player, ply: usize) -> f32 {
    let score = WIN_MAX_SCORE - ply as f32;
    if bot == winner { score } else { -score }
}

/// Evaluates a fixed-depth subtree from the bot's perspective.
/// Checks the starting board before using local threat updates in recursion.
pub fn minimax_alpha_beta(
    board: ge::Board,
    is_maximizing: bool,
    bot: ge::Player,
    depth: usize,
    alpha: f32,
    beta: f32,
    ply: usize,
) -> f32 {
    match board.check_result() {
        ge::GameResult::Win(winner) => return mate(bot, winner, ply),
        ge::GameResult::Draw => return 0.0,
        ge::GameResult::InProgress => {}
    }

    let player = if is_maximizing { bot } else { bot.opposite() };
    if !board.win_in_one(player).is_empty() {
        return mate(bot, player, ply + 1);
    }

    let replies = WinningReplies::from_moves(&board.win_in_one(player.opposite()));
    let empty = board
        .cells()
        .iter()
        .flatten()
        .filter(|&&cell| cell == ge::Cell::Empty)
        .count();
    let node = MinimaxNode {
        board: &board,
        player,
        depth,
        ply,
        empty,
        replies,
    };
    minimax(node, bot, (alpha, beta))
}

struct MinimaxNode<'a> {
    board: &'a ge::Board,
    player: ge::Player,
    depth: usize,
    ply: usize,
    empty: usize,
    replies: WinningReplies,
}

/// Searches a node where the side to move has no immediate win.
fn minimax(node: MinimaxNode<'_>, bot: ge::Player, (mut alpha, mut beta): (f32, f32)) -> f32 {
    let MinimaxNode {
        board,
        player,
        depth,
        ply,
        empty,
        replies,
    } = node;
    if empty == 0 {
        return 0.0;
    }
    if replies == WinningReplies::Multiple {
        return mate(bot, player.opposite(), ply + 2);
    }
    let mut moves = match replies {
        WinningReplies::One(square) => vec![square],
        WinningReplies::None if depth > 0 => board.available_empty_moves_close(CLOSE_RANGE_PARAM),
        _ => Vec::new(),
    };
    if moves.is_empty() {
        return evaluate_in_progress(board, bot);
    }
    if moves.len() > 1 {
        order_moves(&mut moves, board, player, depth >= 2);
    }

    let maximizing = player == bot;
    let mut best = if maximizing {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    };
    for position in moves {
        let next = board
            .apply_move(position, player)
            .expect("search move must be legal");

        let next_replies = winning_replies_through(&next, position, player);
        let child = MinimaxNode {
            board: &next,
            player: player.opposite(),
            depth: depth.saturating_sub(1),
            ply: ply + 1,
            empty: empty - 1,
            replies: next_replies,
        };
        let score = minimax(child, bot, (alpha, beta));
        if (maximizing && score > best) || (!maximizing && score < best) {
            best = score;
        }
        if maximizing {
            alpha = alpha.max(best);
        } else {
            beta = beta.min(best);
        }
        if alpha >= beta {
            break;
        }
    }
    best
}

fn order_moves(
    moves: &mut [ge::Position],
    board: &ge::Board,
    player: ge::Player,
    prioritize_threats: bool,
) {
    let center = board.size() / 2;
    let threats = prioritize_threats.then(|| forcing_squares(board, player));
    moves.sort_unstable_by_key(|&(row, col)| {
        (
            !threats.as_ref().is_some_and(|squares| squares[row][col]),
            row.abs_diff(center) + col.abs_diff(center),
            row,
            col,
        )
    });
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
    if board
        .cells()
        .iter()
        .flatten()
        .all(|cell| *cell == ge::Cell::Empty)
    {
        return Some((center, center));
    }
    let mut moves = board.available_empty_moves_close(CLOSE_RANGE_PARAM);

    order_moves(&mut moves, &board, player, depth >= 2);

    if opponent_wins.is_empty() {
        for &position in &moves {
            let next = board
                .apply_move(position, player)
                .expect("root move must be legal");
            if winning_replies_through(&next, position, player) == WinningReplies::Multiple {
                println!("Confirmed win; distance in searched tree: 3 half-moves");
                return Some(position);
            }
        }
    }

    let mut candidates = Vec::with_capacity(moves.len());
    // Only ordinary minimax results contribute to this window. Forcing proofs
    // establish the outcome, but their distances are upper bounds.
    let mut alpha = f32::NEG_INFINITY;
    for position in moves {
        let new_board = board.apply_move(position, player).expect("Expect the best");
        if let ForcingResult::ProvenWin { plies } =
            search_forcing_line(&new_board, player, player.opposite(), FORCING_BUDGET)
        {
            candidates.push(RootCandidate::ProvenWin(ProvenCandidate {
                position,
                plies: plies + 1,
                minimax_score: None,
            }));
        } else if let ForcingResult::ProvenWin { plies } = search_forcing_line(
            &new_board,
            player.opposite(),
            player.opposite(),
            FORCING_BUDGET,
        ) {
            candidates.push(RootCandidate::ProvenLoss(ProvenCandidate {
                position,
                plies: plies + 1,
                minimax_score: None,
            }));
        } else {
            let score =
                minimax_alpha_beta(new_board, false, player, depth - 1, alpha, f32::INFINITY, 1);
            let candidate = if score >= MIN_MATE_SCORE && score > alpha {
                // With beta = infinity, improving alpha gives an exact search
                // result. A fail-low positive bound alone cannot prove a win.
                if score == WIN_IN_FIVE_SCORE {
                    println!("Confirmed win; distance in searched tree: 5 half-moves");
                    return Some(position);
                }
                RootCandidate::ProvenWin(ProvenCandidate {
                    position,
                    plies: (WIN_MAX_SCORE - score) as usize,
                    minimax_score: Some(score),
                })
            } else if score <= -MIN_MATE_SCORE {
                // Even a fail-low upper bound in the losing range proves loss.
                RootCandidate::ProvenLoss(ProvenCandidate {
                    position,
                    plies: (WIN_MAX_SCORE + score) as usize,
                    minimax_score: Some(score),
                })
            } else {
                RootCandidate::Unresolved(UnresolvedCandidate {
                    position,
                    minimax_score: score,
                })
            };
            candidates.push(candidate);
            alpha = alpha.max(score);
        }
    }

    let mut wins = Vec::new();
    let mut losses = Vec::new();
    let mut best_unresolved: Option<UnresolvedCandidate> = None;
    for candidate in candidates {
        match candidate {
            RootCandidate::ProvenWin(candidate) => wins.push(candidate),
            RootCandidate::ProvenLoss(candidate) => losses.push(candidate),
            RootCandidate::Unresolved(candidate) => {
                if best_unresolved
                    .as_ref()
                    .is_none_or(|best| candidate.minimax_score > best.minimax_score)
                {
                    best_unresolved = Some(candidate);
                }
            }
        }
    }
    if !wins.is_empty() {
        return best_proven_move(board, player, depth, ProvenGroup::Wins(wins));
    }
    if let Some(chosen) = best_unresolved {
        println!(
            "Current evaluation by bot (heuristic): {}",
            chosen.minimax_score
        );
        return Some(chosen.position);
    }

    best_proven_move(board, player, depth, ProvenGroup::Losses(losses))
}

fn best_proven_move(
    board: ge::Board,
    player: ge::Player,
    depth: usize,
    group: ProvenGroup,
) -> Option<ge::Position> {
    let (mut candidates, winning) = match group {
        ProvenGroup::Wins(candidates) => (candidates, true),
        ProvenGroup::Losses(candidates) => (candidates, false),
    };
    candidates.sort_by_key(|candidate| {
        let plies = candidate.plies;
        if winning { plies } else { usize::MAX - plies }
    });
    let mut best_exact: Option<(usize, f32)> = None;
    let mut has_unresolved_distance = false;
    for (index, candidate) in candidates.iter_mut().enumerate() {
        if !winning
            && best_exact
                .is_some_and(|(_, score)| candidate.plies <= (WIN_MAX_SCORE + score) as usize)
        {
            // This proof guarantees a loss no later than the best defence
            // already found, so the candidate cannot improve its distance.
            continue;
        }
        let next = board
            .apply_move(candidate.position, player)
            .expect("Root candidate must be legal");
        let score = minimax_alpha_beta(
            next,
            false,
            player,
            depth - 1,
            f32::NEG_INFINITY,
            f32::INFINITY,
            1,
        );
        candidate.minimax_score = Some(score);
        let resolved = if winning {
            score >= MIN_MATE_SCORE
        } else {
            score <= -MIN_MATE_SCORE
        };
        if resolved {
            if best_exact.is_none_or(|(_, old)| score > old) {
                best_exact = Some((index, score));
            }
            // Root checks have already ruled out wins in one or three half-moves.
            if winning && score == WIN_IN_FIVE_SCORE {
                break;
            }
        } else {
            has_unresolved_distance = true;
        }
    }

    // For a loss, a longer proof only bounds how soon the opponent can win.
    // Prefer the longest resolved defence whenever one was found.
    let (chosen_index, distance_resolved) = match best_exact {
        Some((index, score))
            if !winning || !has_unresolved_distance || score == WIN_IN_FIVE_SCORE =>
        {
            (index, true)
        }

        _ => (0, false),
    };
    let chosen = candidates.get(chosen_index)?;
    let outcome = if winning { "win" } else { "loss" };
    if let (true, Some(score)) = (distance_resolved, chosen.minimax_score) {
        let plies = (WIN_MAX_SCORE - score.abs()) as usize;
        println!("Confirmed {outcome}; distance in searched tree: {plies} half-moves");
    } else {
        let plies = chosen.plies;
        println!(
            "Confirmed {outcome}; distance unresolved; proof line: at most {plies} half-moves"
        );
    }
    Some(chosen.position)
}

#[derive(Debug, PartialEq)]
enum ForcingResult {
    ProvenWin { plies: usize },
    Unknown,
}

/// Finds moves that create a win-in-one after one move.
/// So basically we ask `where i can put my stone, such that if opponent will not block me
/// i can win in one move
fn forcing_moves(board: &ge::Board, player: ge::Player) -> Vec<ge::Position> {
    let threats = forcing_squares(board, player);
    let mut moves = Vec::new();
    for (row, row_threats) in threats.iter().enumerate() {
        for (col, &is_threat) in row_threats.iter().enumerate() {
            if is_threat {
                moves.push((row, col));
            }
        }
    }
    moves
}

fn forcing_squares(
    board: &ge::Board,
    player: ge::Player,
) -> [[bool; ge::BOARD_SIZE]; ge::BOARD_SIZE] {
    let own = ge::Cell::Taken(player);
    let cells = board.cells();
    let mut creates_threat = [[false; ge::BOARD_SIZE]; ge::BOARD_SIZE];

    for row in 0..ge::BOARD_SIZE {
        for col in 0..ge::BOARD_SIZE {
            for direction in AXES {
                let mut own_stones = 0;
                let mut empty_count = 0;
                let mut empty_squares = [(0, 0); 2];

                for step in 0..5 {
                    let Some((window_row, window_col)) =
                        offset_position((row, col), direction, step)
                    else {
                        break;
                    };
                    match cells[window_row][window_col] {
                        cell if cell == own => own_stones += 1,
                        ge::Cell::Empty if empty_count < empty_squares.len() => {
                            empty_squares[empty_count] = (window_row, window_col);
                            empty_count += 1;
                        }
                        _ => break,
                    }
                }

                if own_stones == 3 && empty_count == 2 {
                    for (empty_row, empty_col) in empty_squares {
                        creates_threat[empty_row][empty_col] = true;
                    }
                }
            }
        }
    }

    creates_threat
}

fn search_forcing_line(
    board: &ge::Board,
    attacker: ge::Player,
    to_move: ge::Player,
    budget: usize,
) -> ForcingResult {
    match board.check_result() {
        ge::GameResult::Win(winner) if winner == attacker => {
            return ForcingResult::ProvenWin { plies: 0 };
        }
        ge::GameResult::Win(_) | ge::GameResult::Draw => return ForcingResult::Unknown,
        ge::GameResult::InProgress => {}
    }
    if budget == 0 {
        return ForcingResult::Unknown;
    }

    if !board.win_in_one(to_move).is_empty() {
        if to_move == attacker {
            return ForcingResult::ProvenWin { plies: 1 };
        } else {
            return ForcingResult::Unknown;
        }
    }
    let opponent_wins = board.win_in_one(to_move.opposite());
    if opponent_wins.len() >= 2 {
        if to_move.opposite() == attacker && budget >= 2 {
            return ForcingResult::ProvenWin { plies: 2 };
        } else {
            return ForcingResult::Unknown;
        }
    };
    if opponent_wins.len() == 1 {
        let next_forcing_move = opponent_wins[0];
        let next_board = board
            .apply_move(next_forcing_move, to_move)
            .expect("We expect in search forcing line to apply move correctly!");
        return match search_forcing_line(&next_board, attacker, to_move.opposite(), budget - 1) {
            ForcingResult::ProvenWin { plies } => ForcingResult::ProvenWin { plies: plies + 1 },
            ForcingResult::Unknown => ForcingResult::Unknown,
        };
    };

    if to_move == attacker {
        let mut candidate_plies: Option<usize> = None;
        for threat_move in forcing_moves(board, attacker) {
            let next_board = board
                .apply_move(threat_move, to_move)
                .expect("We expect in search forcing line to apply move correctly!");

            match search_forcing_line(&next_board, attacker, to_move.opposite(), budget - 1) {
                ForcingResult::ProvenWin { plies } => {
                    candidate_plies = Some(match candidate_plies {
                        None => plies + 1,
                        Some(old) => old.min(plies + 1),
                    });
                    // With no immediate win at this node, three half-moves is
                    // the shortest possible attack. No other line can improve it.
                    if candidate_plies == Some(3) {
                        break;
                    }
                }
                ForcingResult::Unknown => {
                    continue;
                }
            }
        }
        if let Some(ret_plies) = candidate_plies {
            return ForcingResult::ProvenWin { plies: ret_plies };
        }
    }

    ForcingResult::Unknown
}

#[cfg(test)]
mod tests;
