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
fn terminal_win_is_scored_from_bots_perspective_in_every_direction() {
    for winner in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (6, 7), direction, 5, winner);
            assert_eq!(board.check_result(), ge::GameResult::Win(winner));
            for is_maximizing in [false, true] {
                // Even a zero or extremely large depth must stop at a terminal state.
                for depth in [0, 3, usize::MAX] {
                    assert_eq!(minimax(board, is_maximizing, winner, depth), 100);
                    assert_eq!(
                        minimax(board, is_maximizing, winner.opposite(), depth),
                        -100
                    );
                }
            }
        }
    }
}

#[test]
fn terminal_draw_is_zero_regardless_of_turn_perspective_or_depth() {
    let board = draw_board_except(&[]);
    assert_eq!(board.check_result(), ge::GameResult::Draw);
    for bot in PLAYERS {
        for is_maximizing in [false, true] {
            for depth in [0, 3, usize::MAX] {
                assert_eq!(minimax(board, is_maximizing, bot, depth), 0);
            }
        }
    }
}

#[test]
fn zero_depth_uses_current_heuristic_without_playing_a_move() {
    for bot in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (6, 7), direction, 3, bot);
            assert_eq!(board.check_result(), ge::GameResult::InProgress);
            assert_eq!(evaluate(&board, bot), 50);
            for is_maximizing in [false, true] {
                assert_eq!(minimax(board, is_maximizing, bot, 0), 50);
                // The turn flag must not change whose perspective is evaluated.
                assert_eq!(minimax(board, is_maximizing, bot.opposite(), 0), 0);
            }
        }
    }
}

#[test]
fn maximizing_can_improve_an_open_three_to_an_open_four() {
    for bot in PLAYERS {
        let mut board = ge::Board::new();
        place_line(&mut board, (7, 5), (0, 1), 3, bot);
        assert_eq!(minimax(board, true, bot, 0), 50);
        assert_eq!(minimax(board, true, bot, 1), 80);
    }
}

#[test]
fn minimizing_can_close_one_end_of_bots_open_three() {
    for bot in PLAYERS {
        let mut board = ge::Board::new();
        place_line(&mut board, (7, 5), (0, 1), 3, bot);
        assert_eq!(minimax(board, false, bot, 0), 50);
        assert_eq!(minimax(board, false, bot, 1), 30);
    }
}

#[test]
fn maximizing_takes_an_immediate_win_in_every_direction() {
    for bot in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (6, 7), direction, 4, bot);
            assert_eq!(board.check_result(), ge::GameResult::InProgress);
            assert_eq!(minimax(board, true, bot, 0), 80);
            assert_eq!(minimax(board, true, bot, 1), 100);
        }
    }
}

#[test]
fn minimizing_takes_opponents_immediate_win_in_every_direction() {
    for bot in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (6, 7), direction, 4, bot.opposite());
            assert_eq!(board.check_result(), ge::GameResult::InProgress);
            assert_eq!(minimax(board, false, bot, 1), -100);
        }
    }
}

#[test]
fn two_plies_reveal_the_need_to_block_instead_of_building_a_four() {
    for bot in PLAYERS {
        let mut board = ge::Board::new();
        // Opponent has a single winning square at (4, 9).
        place_line(&mut board, (4, 5), (0, 1), 4, bot.opposite());
        place(&mut board, (4, 4), bot);
        // Bot can instead extend its three, but that loses on the next move.
        place_line(&mut board, (10, 5), (0, 1), 3, bot);
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert_eq!(minimax(board, true, bot, 1), 80);

        let attack = board.apply_move((10, 8), bot).unwrap();
        assert_eq!(minimax(attack, false, bot, 1), -100);

        let defense = board.apply_move((4, 9), bot).unwrap();
        // After the block, the opponent can close one end of the bot's three.
        assert_eq!(minimax(defense, false, bot, 1), 30);
        assert_eq!(minimax(board, true, bot, 2), 30);
    }
}

#[test]
fn two_plies_detect_unavoidable_loss_against_opponents_open_four() {
    for bot in PLAYERS {
        let mut board = ge::Board::new();
        place_line(&mut board, (7, 5), (0, 1), 4, bot.opposite());
        // Bot can close only one of the two winning squares.
        assert_eq!(minimax(board, true, bot, 2), -100);
    }
}

#[test]
fn empty_board_with_one_ply_has_zero_score_for_either_turn() {
    for bot in PLAYERS {
        for is_maximizing in [false, true] {
            assert_eq!(minimax(ge::Board::new(), is_maximizing, bot, 1), 0);
        }
    }
}

#[test]
fn only_remaining_move_can_end_in_a_draw_before_the_depth_limit() {
    for empty in [(0, 0), (14, 14)] {
        let board = draw_board_except(&[empty]);
        let player = draw_pattern_player(empty.0, empty.1);
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert_eq!(board.available_empty_moves(), vec![empty]);
        assert_eq!(
            board.apply_move(empty, player).unwrap().check_result(),
            ge::GameResult::Draw
        );
        for depth in [1, 5] {
            assert_eq!(minimax(board, true, player, depth), 0);
            assert_eq!(minimax(board, false, player.opposite(), depth), 0);
        }
    }
}

#[test]
fn only_remaining_move_can_win_or_draw_depending_on_the_side_to_move() {
    let mut board = ge::Board::new();
    // Start with a full draw pattern, replacing its top row with OXXXX.OOXXOOXXO.
    for row in 0..ge::BOARD_SIZE {
        for col in 0..ge::BOARD_SIZE {
            if (row, col) == (0, 5) {
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
    assert_eq!(board.available_empty_moves(), vec![(0, 5)]);
    assert_eq!(
        board
            .apply_move((0, 5), ge::Player::X)
            .unwrap()
            .check_result(),
        ge::GameResult::Win(ge::Player::X)
    );
    assert_eq!(
        board
            .apply_move((0, 5), ge::Player::O)
            .unwrap()
            .check_result(),
        ge::GameResult::Draw
    );
    for depth in [1, 5] {
        assert_eq!(minimax(board, true, ge::Player::X, depth), 100);
        assert_eq!(minimax(board, false, ge::Player::O, depth), -100);
        assert_eq!(minimax(board, true, ge::Player::O, depth), 0);
        assert_eq!(minimax(board, false, ge::Player::X, depth), 0);
    }
}

#[test]
fn search_does_not_mutate_the_callers_board() {
    let mut board = ge::Board::new();
    place_line(&mut board, (7, 5), (0, 1), 3, ge::Player::X);
    let before = *board.cells();
    for bot in PLAYERS {
        for is_maximizing in [false, true] {
            minimax(board, is_maximizing, bot, 1);
            assert_eq!(*board.cells(), before);
        }
    }
}

#[test]
fn alpha_beta_matches_plain_minimax_at_zero_and_one_ply() {
    for bot in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (6, 7), direction, 3, bot);
            for is_maximizing in [false, true] {
                for depth in [0, 1] {
                    assert_eq!(
                        minimax_alpha_beta(board, is_maximizing, bot, depth, i32::MIN, i32::MAX),
                        minimax(board, is_maximizing, bot, depth),
                        "bot {bot:?}, direction {direction:?}, maximizing {is_maximizing}, depth {depth}"
                    );
                }
            }
        }
    }
}

#[test]
fn alpha_beta_matches_plain_minimax_on_terminal_and_near_full_boards() {
    let draw = draw_board_except(&[]);
    let last_move = draw_board_except(&[(14, 14)]);
    let mut won = ge::Board::new();
    place_line(&mut won, (6, 7), (1, -1), 5, ge::Player::X);
    for board in [draw, last_move, won] {
        for bot in PLAYERS {
            for is_maximizing in [false, true] {
                for depth in [0, 1, 5] {
                    assert_eq!(
                        minimax_alpha_beta(board, is_maximizing, bot, depth, i32::MIN, i32::MAX),
                        minimax(board, is_maximizing, bot, depth)
                    );
                }
            }
        }
    }
}

#[test]
fn alpha_beta_matches_plain_minimax_when_defense_is_required() {
    for bot in PLAYERS {
        let mut board = ge::Board::new();
        place_line(&mut board, (4, 5), (0, 1), 4, bot.opposite());
        place(&mut board, (4, 4), bot);
        place_line(&mut board, (10, 5), (0, 1), 3, bot);
        let expected = minimax(board, true, bot, 2);
        assert_eq!(expected, 30);
        assert_eq!(
            minimax_alpha_beta(board, true, bot, 2, i32::MIN, i32::MAX),
            expected
        );
    }
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
fn both_move_selectors_open_in_the_center_on_an_empty_board() {
    let center = (ge::BOARD_SIZE / 2, ge::BOARD_SIZE / 2);
    for bot in PLAYERS {
        assert_eq!(best_move(ge::Board::new(), bot, 3), Some(center));
        assert_eq!(best_move_alpha_beta(ge::Board::new(), bot, 3), Some(center));
    }
}
