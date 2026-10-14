use ml_project::{
    gomoku_bot::{best_move_alpha_beta, minimax_alpha_beta},
    gomoku_engine as ge,
    gomoku_evaluation::evaluate,
};

type Position = (usize, usize);
const PLAYERS: [ge::Player; 2] = [ge::Player::X, ge::Player::O];

fn place(board: &mut ge::Board, position: Position, player: ge::Player) {
    board.apply_move_inplace(position, player).unwrap();
}

fn position_before_h9_in_latest_screenshot() -> ge::Board {
    let mut board = ge::Board::new();
    // Black: G7, H7, J7, H8, I8, J8, J9, J10, K10.
    for position in [
        (6, 6),
        (6, 7),
        (6, 9),
        (7, 7),
        (7, 8),
        (7, 9),
        (8, 9),
        (9, 9),
        (9, 10),
    ] {
        place(&mut board, position, ge::Player::X);
    }
    // White: G6, H6, J6, I7, G8, K8, I9, J11, L11.
    for position in [
        (5, 6),
        (5, 7),
        (5, 9),
        (6, 8),
        (7, 6),
        (7, 10),
        (8, 8),
        (10, 9),
        (10, 11),
    ] {
        place(&mut board, position, ge::Player::O);
    }
    board
}

#[test]
fn latest_screenshot_h9_allows_white_to_create_an_unstoppable_four_at_i6() {
    let board = position_before_h9_in_latest_screenshot();
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    for player in PLAYERS {
        assert!(board.win_in_one(player).is_empty());
    }
    let after_h9 = board.apply_move((8, 7), ge::Player::X).unwrap();
    let after_i6 = after_h9.apply_move((5, 8), ge::Player::O).unwrap();
    assert!(after_i6.win_in_one(ge::Player::X).is_empty());
    assert_eq!(after_i6.win_in_one(ge::Player::O), vec![(5, 5), (5, 10)]);
    assert_eq!(
        minimax_alpha_beta(after_i6, true, ge::Player::X, 0, i32::MIN, i32::MAX),
        -100,
    );
    assert_eq!(
        minimax_alpha_beta(after_h9, false, ge::Player::X, 1, i32::MIN, i32::MAX),
        -100,
    );
}

#[test]
fn latest_screenshot_bot_avoids_h9_and_the_forced_loss_at_ui_depth() {
    let board = position_before_h9_in_latest_screenshot();
    let chosen = best_move_alpha_beta(board, ge::Player::X, 4).unwrap();
    assert_ne!(chosen, (8, 7), "H9 lets White force a win with I6");
    let next = board.apply_move(chosen, ge::Player::X).unwrap();
    assert!(
        minimax_alpha_beta(next, false, ge::Player::X, 1, i32::MIN, i32::MAX) > -100,
        "Chosen move {chosen:?} still lets White force an immediate loss",
    );
}

fn screenshot_position(include_g1: bool, include_i6: bool) -> ge::Board {
    let mut board = ge::Board::new();
    // Coordinates transcribed from the supplied screenshot (zero-based).
    for position in [
        (0, 0),
        (0, 1),
        (0, 3),
        (0, 4),
        (0, 5),
        (2, 5),
        (3, 5),
        (5, 5),
        (6, 6),
        (7, 7),
        (10, 5),
    ] {
        place(&mut board, position, ge::Player::X);
    }
    for position in [
        (0, 2),
        (4, 4),
        (6, 5),
        (6, 7),
        (7, 4),
        (7, 5),
        (7, 6),
        (8, 4),
        (8, 5),
        (9, 4),
        (9, 5),
    ] {
        place(&mut board, position, ge::Player::O);
    }
    if include_g1 {
        place(&mut board, (0, 6), ge::Player::X);
    }
    if include_i6 {
        place(&mut board, (5, 8), ge::Player::O);
    }
    board
}

#[test]
fn supplied_screenshot_already_contains_a_white_diagonal_win() {
    let board = screenshot_position(true, true);
    assert_eq!(board.check_result(), ge::GameResult::Win(ge::Player::O));
    assert_eq!(evaluate(&board, ge::Player::X), -100.0);
}

#[test]
fn screenshot_without_g1_and_i6_has_two_winning_replies_despite_different_attack_scores() {
    // This is a possible preceding position, not an assertion about move history.
    let board = screenshot_position(false, false);
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    for reply in [(5, 8), (10, 3)] {
        // I6 and D11 complete opposite ends.
        assert_eq!(
            board
                .apply_move(reply, ge::Player::O)
                .unwrap()
                .check_result(),
            ge::GameResult::Win(ge::Player::O)
        );
    }
    for attack in [(4, 5), (0, 6)] {
        // F5 and G1 both create a half-open four.
        let next = board.apply_move(attack, ge::Player::X).unwrap();
        // White's combined threats outweigh either attack in the current evaluation.
        assert!(evaluate(&next, ge::Player::X) < 0.0);
        assert_eq!(
            minimax_alpha_beta(next, false, ge::Player::X, 1, i32::MIN, i32::MAX),
            -100
        );
    }
    let after_f5 = board.apply_move((4, 5), ge::Player::X).unwrap();
    let after_g1 = board.apply_move((0, 6), ge::Player::X).unwrap();
    assert!(evaluate(&after_f5, ge::Player::X) > evaluate(&after_g1, ge::Player::X));
    // Every black move still leaves at least one immediate white win.
    for position in board.available_empty_moves() {
        let next = board.apply_move(position, ge::Player::X).unwrap();
        assert_ne!(next.check_result(), ge::GameResult::Win(ge::Player::X));
        assert!([(5, 8), (10, 3)].into_iter().any(|reply| {
            next.apply_move(reply, ge::Player::O)
                .is_ok_and(|after| after.check_result() == ge::GameResult::Win(ge::Player::O))
        }));
    }
    // Tie-breaking may choose any legal move, but it cannot avoid White's win.
    let chosen = best_move_alpha_beta(board, ge::Player::X, 2).unwrap();
    let next = board.apply_move(chosen, ge::Player::X).unwrap();
    assert_eq!(
        minimax_alpha_beta(next, false, ge::Player::X, 1, i32::MIN, i32::MAX),
        -100
    );
}

#[test]
fn second_screenshot_prefers_i9_over_f2_at_three_plies_with_current_heuristic() {
    // Position before the marked black move F2 in the second screenshot.
    let mut board = ge::Board::new();
    for position in [(2, 8), (3, 7), (5, 5), (6, 6), (7, 7)] {
        place(&mut board, position, ge::Player::X);
    }
    for position in [(4, 4), (4, 6), (6, 5), (7, 5), (7, 6)] {
        place(&mut board, position, ge::Player::O);
    }
    assert_eq!(board.check_result(), ge::GameResult::InProgress);

    let f2 = board.apply_move((1, 5), ge::Player::X).unwrap();
    let i9 = board.apply_move((8, 8), ge::Player::X).unwrap();
    // White's open pair contributes 5, subtracted from Black's 30 or 55.
    assert_eq!(evaluate(&f2, ge::Player::X), 25.0);
    assert_eq!(evaluate(&i9, ge::Player::X), 50.0);

    // Search also considers White's response and Black's following move.
    assert_eq!(
        minimax_alpha_beta(f2, false, ge::Player::X, 2, i32::MIN, i32::MAX),
        12
    );
    assert_eq!(
        minimax_alpha_beta(i9, false, ge::Player::X, 2, i32::MIN, i32::MAX),
        25
    );

    // White blocks the lower three at I9; Black then makes an open three
    // F2-G3-H4, with empty endpoints E1 and I5.
    let after_f2 = f2
        .apply_move((8, 8), ge::Player::O)
        .unwrap()
        .apply_move((2, 6), ge::Player::X)
        .unwrap();
    assert_eq!(after_f2.check_result(), ge::GameResult::InProgress);
    assert_eq!(evaluate(&after_f2, ge::Player::X), 45.0);

    // I9 creates F6-G7-H8-I9, already blocked by White at E5. White
    // closes its remaining endpoint J10; Black can form J2-I3-H4.
    let after_i9 = i9
        .apply_move((9, 9), ge::Player::O)
        .unwrap()
        .apply_move((1, 9), ge::Player::X)
        .unwrap();
    assert_eq!(after_i9.check_result(), ge::GameResult::InProgress);
    assert_eq!(evaluate(&after_i9, ge::Player::X), 25.0);
}
