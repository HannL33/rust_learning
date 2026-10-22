use super::*;

type Position = (usize, usize);
const PLAYERS: [ge::Player; 2] = [ge::Player::X, ge::Player::O];
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

fn place(board: &mut ge::Board, position: Position, player: ge::Player) {
    board.apply_move_inplace(position, player).unwrap();
}

fn place_line(
    board: &mut ge::Board,
    start: Position,
    direction: (isize, isize),
    length: usize,
    player: ge::Player,
) {
    for step in 0..length as isize {
        place(
            board,
            (
                (start.0 as isize + step * direction.0) as usize,
                (start.1 as isize + step * direction.1) as usize,
            ),
            player,
        );
    }
}

fn draw_pattern_player(row: usize, col: usize) -> ge::Player {
    if (row + 2 * col) % 4 < 2 {
        ge::Player::X
    } else {
        ge::Player::O
    }
}

fn draw_board_except(empty: &[Position]) -> ge::Board {
    let mut board = ge::Board::new();
    for row in 0..ge::BOARD_SIZE {
        for col in 0..ge::BOARD_SIZE {
            if !empty.contains(&(row, col)) {
                place(&mut board, (row, col), draw_pattern_player(row, col));
            }
        }
    }
    board
}

#[test]
fn alpha_beta_move_selection_blocks_a_single_winning_square_at_depth_three() {
    // Only three empty cells keep this depth-three regression fast. X has a
    // single winning square at (0, 5); O must block it instead of playing elsewhere.
    let empty = [(0, 5), (10, 5), (11, 8)];
    let mut board = ge::Board::new();
    for row in 0..ge::BOARD_SIZE {
        for col in 0..ge::BOARD_SIZE {
            if empty.contains(&(row, col)) {
                continue;
            }
            let player = match (row, col) {
                (0, 0) => ge::Player::O,
                (0, 1..=4) => ge::Player::X,
                _ => draw_pattern_player(row, col),
            };
            place(&mut board, (row, col), player);
        }
    }
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    assert_eq!(board.available_empty_moves().len(), 3);
    let before = *board.cells();
    assert_eq!(best_move_alpha_beta(board, ge::Player::O, 3), Some((0, 5)));
    assert_eq!(*board.cells(), before);
}

#[test]
fn alpha_beta_recognizes_that_blocking_one_end_of_an_open_four_still_loses() {
    for bot in PLAYERS {
        let mut board = ge::Board::new();
        place_line(&mut board, (7, 5), (0, 1), 4, bot.opposite());
        assert_eq!(
            minimax_alpha_beta(board, true, bot, 2, i32::MIN, i32::MAX),
            -100
        );
        let chosen = best_move_alpha_beta(board, bot, 2).unwrap();
        let next = board.apply_move(chosen, bot).unwrap();
        assert!(next.available_empty_moves().into_iter().any(|reply| {
            next.apply_move(reply, bot.opposite())
                .unwrap()
                .check_result()
                == ge::GameResult::Win(bot.opposite())
        }));
    }
}

#[test]
fn double_threat_is_a_loss_for_the_side_to_move_even_at_depth_zero() {
    for bot in PLAYERS {
        for is_maximizing in [true, false] {
            let player = if is_maximizing { bot } else { bot.opposite() };
            for direction in DIRECTIONS {
                let mut board = ge::Board::new();
                place_line(&mut board, (7, 5), direction, 4, player.opposite());
                assert_eq!(board.win_in_one(player.opposite()).len(), 2);
                for depth in [0, 1] {
                    assert_eq!(
                        minimax_alpha_beta(board, is_maximizing, bot, depth, i32::MIN, i32::MAX,),
                        if player == bot { -100 } else { 100 },
                        "bot={bot:?}, player={player:?}, direction={direction:?}, depth={depth}",
                    );
                }
            }
        }
    }
}

#[test]
fn immediate_win_takes_priority_over_an_opponents_double_threat() {
    for bot in PLAYERS {
        for is_maximizing in [true, false] {
            let player = if is_maximizing { bot } else { bot.opposite() };
            let mut board = ge::Board::new();
            place_line(&mut board, (1, 0), (0, 1), 4, player);
            place_line(&mut board, (3, 5), (0, 1), 4, player.opposite());
            assert_eq!(board.win_in_one(player).len(), 1);
            assert_eq!(board.win_in_one(player.opposite()).len(), 2);
            assert_eq!(
                minimax_alpha_beta(board, is_maximizing, bot, 0, i32::MIN, i32::MAX),
                if player == bot { 100 } else { -100 },
            );
        }
    }
}

#[test]
fn forcing_search_returns_unknown_when_defender_has_already_won() {
    let mut board = ge::Board::new();

    // O has already won; X's possible next winning move is irrelevant.
    place_line(&mut board, (3, 2), (0, 1), 5, ge::Player::O);
    place_line(&mut board, (7, 5), (0, 1), 4, ge::Player::X);

    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 0),
        ForcingResult::Unknown,
    );
}

#[test]
fn forcing_search_respects_whose_turn_it_is() {
    let mut board = ge::Board::new();

    // XXXX. at the left edge: X can win at (7, 4), but O can block it.
    place_line(&mut board, (7, 0), (0, 1), 4, ge::Player::X);

    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 1),
        ForcingResult::ProvenWin,
    );
    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::O, 1),
        ForcingResult::Unknown,
    );
}

#[test]
fn forcing_search_prioritizes_defenders_immediate_win() {
    let mut board = ge::Board::new();

    // X has an open four, but O moves first and wins immediately.
    place_line(&mut board, (7, 5), (0, 1), 4, ge::Player::X);
    place_line(&mut board, (3, 0), (0, 1), 4, ge::Player::O);

    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::O, 0),
        ForcingResult::Unknown,
    );
}

#[test]
fn forcing_search_does_not_expand_attacks_with_zero_budget() {
    let mut board = ge::Board::new();

    // Creating an open four requires another move, which budget 0 forbids.
    place_line(&mut board, (7, 5), (0, 1), 3, ge::Player::X);

    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 0),
        ForcingResult::Unknown,
    );
}

#[test]
fn forcing_search_finds_a_forced_block_that_also_wins_the_attack() {
    let mut board = ge::Board::new();

    for position in [(2, 7), (7, 5), (7, 6), (7, 8)] {
        place(&mut board, position, ge::Player::X);
    }
    place_line(&mut board, (3, 7), (1, 0), 4, ge::Player::O);

    // X at (2, 7) blocks one end of O's vertical four.
    // X must block the other end at (7, 7), turning horizontal XX.X into
    // an open four with winning squares (7, 4) and (7, 9).
    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 1),
        ForcingResult::ProvenWin,
    );
}

#[test]
fn forcing_search_finds_an_attack_that_creates_an_open_four() {
    let mut board = ge::Board::new();

    // .XXX. can become .XXXX. in one move, leaving two winning squares.
    place_line(&mut board, (7, 5), (0, 1), 3, ge::Player::X);

    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 1),
        ForcingResult::ProvenWin,
    );
}

#[test]
fn forcing_search_continues_the_attack_after_a_defenders_forced_block() {
    let mut board = ge::Board::new();

    // OOOO. at the left edge forces X to block (7, 4).
    place_line(&mut board, (7, 0), (0, 1), 4, ge::Player::O);
    // O can then play (7, 5), creating an open vertical four.
    place_line(&mut board, (4, 5), (1, 0), 3, ge::Player::O);

    // One move covers only X's block; two also cover O's next attack.
    assert_eq!(
        search_forcing_line(&board, ge::Player::O, ge::Player::X, 1),
        ForcingResult::Unknown,
    );
    assert_eq!(
        search_forcing_line(&board, ge::Player::O, ge::Player::X, 2),
        ForcingResult::ProvenWin,
    );
}
