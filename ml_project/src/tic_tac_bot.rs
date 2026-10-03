use crate::tic_tac_engine as tte;

/// is_maximizing tells whos turn it is right now
/// if we play now, then we wanna maximize
/// the next turn is for our opponent so we wanna minimize his turn
///
pub fn minimax(board: tte::Board, is_maximizing: bool, bot: tte::Player, depth: usize) -> i32 {
    match tte::check_winner(&board) {
        tte::GameResult::InProgress => {
            if is_maximizing {
                let mut score_vec = Vec::new();
                for (idx, cell) in board.cells().enumerate() {
                    if cell == tte::Cell::Empty {
                        let board = tte::apply_move(board, idx, bot).expect(
                            "expected the best after the unpacking apply move error in minimax",
                        );
                        let board_copy = board.clone();
                        score_vec.push(minimax(board_copy, !is_maximizing, bot, depth - 1))
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
                        let board_copy = board.clone();
                        score_vec.push(minimax(board_copy, !is_maximizing, bot, depth - 1))
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

pub fn best_move(board: tte::Board, player: tte::Player) -> usize {
    let mut best_eval_move = (-10, 0);
    for (idx, cell) in board.cells().enumerate() {
        if cell == tte::Cell::Empty {
            let _board = tte::apply_move(board, idx, player)
                .expect("Something wrong happen during the apply move in best move!");
            let eval = minimax(_board, false, player, 10);
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
