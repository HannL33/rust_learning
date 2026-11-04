use super::*;

type Position = (usize, usize);
const PLAYERS: [ge::Player; 2] = [ge::Player::X, ge::Player::O];
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

// Forcing-search contract: budget caps all future half-moves, including moves
// inferred by tactical shortcuts. A proof must have plies <= budget; a win
// beyond that horizon is Unknown. An already finished win has plies = 0.
// The ordinary minimax depth is a separate parameter with its existing rules.

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
fn alpha_beta_draw_stays_zero_regardless_of_ply_and_bot_colour() {
    let board = draw_board_except(&[]);
    assert_eq!(board.check_result(), ge::GameResult::Draw);
    for bot in PLAYERS {
        for is_maximizing in [true, false] {
            for ply in [0, 7, ge::BOARD_SIZE * ge::BOARD_SIZE] {
                assert_eq!(
                    minimax_alpha_beta(
                        board,
                        is_maximizing,
                        bot,
                        0,
                        f32::NEG_INFINITY,
                        f32::INFINITY,
                        ply,
                    ),
                    0.0,
                );
            }
        }
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
            minimax_alpha_beta(board, true, bot, 2, f32::NEG_INFINITY, f32::INFINITY, 0),
            -9_998.0
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
                        minimax_alpha_beta(
                            board,
                            is_maximizing,
                            bot,
                            depth,
                            f32::NEG_INFINITY,
                            f32::INFINITY,
                            0,
                        ),
                        if player == bot { -9_998.0 } else { 9_998.0 },
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
                minimax_alpha_beta(
                    board,
                    is_maximizing,
                    bot,
                    0,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    0,
                ),
                if player == bot { 9_999.0 } else { -9_999.0 },
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
        ForcingResult::ProvenWin { plies: 1 },
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
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 3),
        ForcingResult::ProvenWin { plies: 3 },
    );
}

#[test]
fn forcing_search_finds_an_attack_that_creates_an_open_four() {
    let mut board = ge::Board::new();

    // .XXX. can become .XXXX. in one move, leaving two winning squares.
    place_line(&mut board, (7, 5), (0, 1), 3, ge::Player::X);

    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 3),
        ForcingResult::ProvenWin { plies: 3 },
    );
}

#[test]
fn forcing_search_continues_the_attack_after_a_defenders_forced_block() {
    let mut board = ge::Board::new();

    // OOOO. at the left edge forces X to block (7, 4).
    place_line(&mut board, (7, 0), (0, 1), 4, ge::Player::O);
    // O can then play (7, 5), creating an open vertical four.
    place_line(&mut board, (4, 5), (1, 0), 3, ge::Player::O);

    // Four half-moves: X blocks, O attacks, X blocks, O wins.
    // All four must fit inside the requested horizon.
    for budget in [0, 1, 2, 3] {
        assert_eq!(
            search_forcing_line(&board, ge::Player::O, ge::Player::X, budget),
            ForcingResult::Unknown,
            "budget={budget}",
        );
    }
    assert_eq!(
        search_forcing_line(&board, ge::Player::O, ge::Player::X, 4),
        ForcingResult::ProvenWin { plies: 4 },
    );
}

#[test]
fn forcing_distance_finished_game_requires_no_more_moves() {
    for attacker in PLAYERS {
        let mut board = ge::Board::new();
        place_line(&mut board, (7, 5), (0, 1), 5, attacker);
        assert_eq!(board.check_result(), ge::GameResult::Win(attacker));
        for to_move in PLAYERS {
            for budget in [0, 1, 5] {
                assert_eq!(
                    search_forcing_line(&board, attacker, to_move, budget),
                    ForcingResult::ProvenWin { plies: 0 },
                    "attacker={attacker:?}, to_move={to_move:?}, budget={budget}",
                );
            }
        }
    }
}

#[test]
fn forcing_search_draw_is_not_a_proven_win() {
    let board = draw_board_except(&[]);
    assert_eq!(board.check_result(), ge::GameResult::Draw);
    for attacker in PLAYERS {
        for to_move in PLAYERS {
            assert_eq!(
                search_forcing_line(&board, attacker, to_move, 5),
                ForcingResult::Unknown,
            );
        }
    }
}

#[test]
fn forcing_distance_open_four_respects_turn_colour_direction_and_budget() {
    for attacker in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (7, 5), direction, 4, attacker);
            let wins = board.win_in_one(attacker);
            assert_eq!(board.check_result(), ge::GameResult::InProgress);
            assert_eq!(wins.len(), 2);
            assert!(board.win_in_one(attacker.opposite()).is_empty());
            // Either endpoint wins now. Blocking either one leaves the other.
            for (block, win) in [(wins[0], wins[1]), (wins[1], wins[0])] {
                assert_eq!(
                    board.apply_move(win, attacker).unwrap().check_result(),
                    ge::GameResult::Win(attacker),
                );
                let terminal = board
                    .apply_move(block, attacker.opposite())
                    .unwrap()
                    .apply_move(win, attacker)
                    .unwrap();
                assert_eq!(terminal.check_result(), ge::GameResult::Win(attacker));
            }
            for (to_move, plies, budgets) in [
                (attacker, 1, [1, 2, 5]),
                (attacker.opposite(), 2, [2, 3, 5]),
            ] {
                for budget in budgets {
                    assert_eq!(
                        search_forcing_line(&board, attacker, to_move, budget),
                        ForcingResult::ProvenWin { plies },
                        "attacker={attacker:?}, direction={direction:?}, to_move={to_move:?}, budget={budget}",
                    );
                }
            }
        }
    }
}

#[test]
fn forcing_budget_rejects_open_four_wins_beyond_the_horizon() {
    for attacker in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (7, 5), direction, 4, attacker);
            assert_eq!(board.check_result(), ge::GameResult::InProgress);
            assert_eq!(board.win_in_one(attacker).len(), 2);
            assert!(board.win_in_one(attacker.opposite()).is_empty());
            // One half-move is needed on the attacker's turn, two on the
            // defender's turn. Neither shortcut may exceed the budget.
            for (to_move, budget) in [
                (attacker, 0),
                (attacker.opposite(), 0),
                (attacker.opposite(), 1),
            ] {
                assert_eq!(
                    search_forcing_line(&board, attacker, to_move, budget),
                    ForcingResult::Unknown,
                    "attacker={attacker:?}, direction={direction:?}, to_move={to_move:?}, budget={budget}",
                );
            }
        }
    }
}

#[test]
fn forcing_budget_rejects_three_ply_attacks_beyond_the_horizon() {
    for attacker in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_line(&mut board, (7, 5), direction, 3, attacker);
            assert_eq!(board.check_result(), ge::GameResult::InProgress);
            assert!(board.win_in_one(attacker).is_empty());
            assert!(board.win_in_one(attacker.opposite()).is_empty());
            // Creating a four, the defender's reply and the final winning
            // move require three half-moves, even with tactical shortcuts.
            for budget in [0, 1, 2] {
                assert_eq!(
                    search_forcing_line(&board, attacker, attacker, budget),
                    ForcingResult::Unknown,
                    "attacker={attacker:?}, direction={direction:?}, budget={budget}",
                );
            }
        }
    }
}

// Distance from the search root (TODO 1A).

// Only constructs a position: X is `attacker`, O is its opponent. Coordinates
// in the diagrams are zero-based; omitted cells/rows are empty.
fn board_from_rows(rows: &[&str], attacker: ge::Player) -> ge::Board {
    let mut board = ge::Board::new();
    for (row, cells) in rows.iter().enumerate() {
        for (column, cell) in cells.chars().enumerate() {
            let player = match cell {
                'X' => attacker,
                'O' => attacker.opposite(),
                '.' => continue,
                _ => panic!("Invalid fixture cell: {cell}"),
            };
            board.apply_move_inplace((row, column), player).unwrap();
        }
    }
    board
}

// Full-window search of the real implementation, with an explicit turn and
// root distance. No reference minimax or computed expected scores.
fn score(board: ge::Board, to_move: ge::Player, bot: ge::Player, depth: usize, ply: usize) -> f32 {
    minimax_alpha_beta(
        board,
        to_move == bot,
        bot,
        depth,
        f32::NEG_INFINITY,
        f32::INFINITY,
        ply,
    )
}

#[test]
fn ply_finished_five_adds_no_future_moves() {
    for winner in PLAYERS {
        let board = board_from_rows(&["XXXXX"], winner);
        assert_eq!(board.check_result(), ge::GameResult::Win(winner));
        for to_move in PLAYERS {
            for depth in [0, 3] {
                assert_eq!(score(board, to_move, winner, depth, 0), 10_000.0);
                assert_eq!(
                    score(board, to_move, winner.opposite(), depth, 0),
                    -10_000.0
                );
                // The same finished board reached seven plies from the root.
                assert_eq!(score(board, to_move, winner, depth, 7), 9_993.0);
                assert_eq!(score(board, to_move, winner.opposite(), depth, 7), -9_993.0);
            }
        }
    }
}

#[test]
fn ply_immediate_win_uses_one_move_and_overrides_opponents_two_wins() {
    for winner in PLAYERS {
        let board = board_from_rows(&["XXXX......", "..........", "..OOOO...."], winner);
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert_eq!(board.win_in_one(winner), vec![(0, 4)]);
        assert_eq!(board.win_in_one(winner.opposite()), vec![(2, 1), (2, 6)]);
        assert_eq!(score(board, winner, winner, 0, 0), 9_999.0);
        assert_eq!(score(board, winner, winner.opposite(), 0, 0), -9_999.0);
        assert_eq!(score(board, winner, winner, 0, 7), 9_992.0);
        assert_eq!(score(board, winner, winner.opposite(), 0, 7), -9_992.0);
        assert_eq!(best_move_alpha_beta(board, winner, 1), Some((0, 4)));
    }
}

#[test]
fn ply_open_four_takes_two_moves_when_defender_moves_first() {
    for winner in PLAYERS {
        let board = board_from_rows(&["..XXXX.."], winner);
        let defender = winner.opposite();
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert!(board.win_in_one(defender).is_empty());
        assert_eq!(board.win_in_one(winner), vec![(0, 1), (0, 6)]);
        // 1. O blocks either end (or plays elsewhere). 2. X completes five.
        for depth in [0, 2] {
            assert_eq!(score(board, defender, winner, depth, 0), 9_998.0);
            assert_eq!(score(board, defender, defender, depth, 0), -9_998.0);
            assert_eq!(score(board, defender, winner, depth, 7), 9_991.0);
            assert_eq!(score(board, defender, defender, depth, 7), -9_991.0);
        }
    }
}

#[test]
fn ply_open_three_requires_search_and_wins_in_three_moves() {
    for winner in PLAYERS {
        let board = board_from_rows(&["..XXX..."], winner);
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert!(board.win_in_one(winner).is_empty());
        assert!(board.win_in_one(winner.opposite()).is_empty());
        // 1. X at (0, 1) or (0, 5) creates an open four.
        // 2. O can block only one end. 3. X completes five.
        // Depth 0 sees only the open-three heuristic, exactly 50.0.
        assert_eq!(score(board, winner, winner, 0, 0), 50.0);
        assert_eq!(score(board, winner, winner.opposite(), 0, 0), -50.0);
        for depth in [1, 2] {
            assert_eq!(score(board, winner, winner, depth, 0), 9_997.0);
            assert_eq!(score(board, winner, winner.opposite(), depth, 0), -9_997.0);
            assert_eq!(score(board, winner, winner, depth, 7), 9_990.0);
            assert_eq!(score(board, winner, winner.opposite(), depth, 7), -9_990.0);
        }
        assert!(matches!(
            best_move_alpha_beta(board, winner, 1),
            Some((0, 1) | (0, 5))
        ));
    }
}

#[test]
fn ply_forced_blocks_at_depth_zero_lead_to_win_in_five_moves() {
    for winner in PLAYERS {
        let board = board_from_rows(
            &[
                "..........",
                "..........",
                ".......X..",
                ".......OX.",
                ".......OO.",
                ".....X.OO.",
                "......XOO.",
                "...OXXX...",
            ],
            winner,
        );
        let defender = winner.opposite();
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert!(board.win_in_one(winner).is_empty());
        assert_eq!(board.win_in_one(defender), vec![(7, 7)]);

        // 1. X must block (7, 7), making OXXXX. horizontally (O at 7,3).
        let after_one = board.apply_move((7, 7), winner).unwrap();
        assert_eq!(after_one.win_in_one(winner), vec![(7, 8)]);
        assert!(after_one.win_in_one(defender).is_empty());
        // 2. O must block (7, 8), extending its vertical four.
        let after_two = after_one.apply_move((7, 8), defender).unwrap();
        assert_eq!(after_two.win_in_one(defender), vec![(8, 8)]);
        assert!(after_two.win_in_one(winner).is_empty());
        // 3. X must block (8, 8), making an open diagonal four.
        let after_three = after_two.apply_move((8, 8), winner).unwrap();
        assert_eq!(after_three.win_in_one(winner), vec![(4, 4), (9, 9)]);
        assert!(after_three.win_in_one(defender).is_empty());
        // 4. O blocks either endpoint. 5. X wins at the other.
        for (block, win) in [((4, 4), (9, 9)), ((9, 9), (4, 4))] {
            let terminal = after_three
                .apply_move(block, defender)
                .unwrap()
                .apply_move(win, winner)
                .unwrap();
            assert_eq!(terminal.check_result(), ge::GameResult::Win(winner));
        }

        // All three forced blocks extend a zero-depth search; no underflow.
        assert_eq!(score(board, winner, winner, 0, 0), 9_995.0);
        assert_eq!(score(board, winner, defender, 0, 0), -9_995.0);
        assert_eq!(score(board, winner, winner, 0, 7), 9_988.0);
        assert_eq!(score(board, winner, defender, 0, 7), -9_988.0);

        // The same line is three moves from its end after the first two moves.
        assert_eq!(score(after_two, winner, winner, 0, 0), 9_997.0);
        assert_eq!(score(after_two, winner, defender, 0, 0), -9_997.0);
        // Giving it the original root offset must preserve the original score.
        assert_eq!(score(after_two, winner, winner, 0, 2), 9_995.0);
        assert_eq!(score(after_two, winner, defender, 0, 2), -9_995.0);
        // For either bot colour: a quicker win is better; a later loss is better.
        assert!(score(after_two, winner, winner, 0, 0) > score(board, winner, winner, 0, 0));
        assert!(score(board, winner, defender, 0, 0) > score(after_two, winner, defender, 0, 0));
        assert_eq!(best_move_alpha_beta(board, winner, 1), Some((7, 7)));
    }
}

#[test]
fn ply_best_move_delays_a_forced_loss_with_a_counterthreat() {
    for attacker in PLAYERS {
        let board = board_from_rows(
            &[
                "...........",
                "...........",
                "...........",
                "....XXX....",
                "...........",
                "...........",
                "...........",
                ".....XXX...",
                "...........",
                "...........",
                "..OOOX.....",
            ],
            attacker,
        );
        let bot = attacker.opposite();
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert!(board.win_in_one(bot).is_empty());
        assert!(board.win_in_one(attacker).is_empty());

        // O cannot stop both independent open threes. A quiet move loses in
        // four plies: O moves, X makes an open four, O blocks, X wins.
        // O at (10, 0) or (10, 1) instead creates a single winning threat,
        // forcing X to spend a move blocking it. O then gets another turn
        // before X starts that attack, extending the loss to six plies.
        for (counterattack, forced_block) in [((10, 0), (10, 1)), ((10, 1), (10, 0))] {
            let next = board.apply_move(counterattack, bot).unwrap();
            assert_eq!(next.win_in_one(bot), vec![forced_block]);
            assert!(next.win_in_one(attacker).is_empty());
        }

        // Check the selected move itself. Treating every ProvenWin as a loss
        // in FORCING_BUDGET plies must not outrank the longer actual defence.
        let chosen = best_move_alpha_beta(board, bot, 4).unwrap();
        assert!(
            [(10, 0), (10, 1)].contains(&chosen),
            "bot={bot:?}: {chosen:?} loses in four plies; a counterthreat delays it to six",
        );
    }
}

// Preserve forcing proofs and compare root candidates (TODO 1B).

fn exact_score(board: ge::Board, to_move: ge::Player, bot: ge::Player, depth: usize) -> f32 {
    // Every candidate has already consumed one root move. Start each comparison
    // with a full window, without carrying alpha from another candidate/proof.
    minimax_alpha_beta(
        board,
        to_move == bot,
        bot,
        depth,
        f32::NEG_INFINITY,
        f32::INFINITY,
        1,
    )
}

fn assert_proven(board: &ge::Board, attacker: ge::Player, to_move: ge::Player, budget: usize) {
    let result = search_forcing_line(board, attacker, to_move, budget);
    assert!(
        matches!(result, ForcingResult::ProvenWin { plies } if plies <= budget),
        "attacker={attacker:?}, to_move={to_move:?}, budget={budget}: {result:?}",
    );
}

fn winning_race(attacker: ge::Player) -> ge::Board {
    let mut board = ge::Board::new();
    // X = attacker; only the following two rows contain stones:
    // row 1: ..XXX..........  (quick attack)
    // row 7: ...OXXX........  (slower forcing attack at the centre)
    place_line(&mut board, (1, 2), (0, 1), 3, attacker);
    place_line(&mut board, (7, 4), (0, 1), 3, attacker);
    place(&mut board, (7, 3), attacker.opposite());
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    assert!(board.win_in_one(attacker).is_empty());
    assert!(board.win_in_one(attacker.opposite()).is_empty());
    board
}

fn long_attack(attacker: ge::Player) -> ge::Board {
    let mut board = ge::Board::new();
    // X = attacker:
    // row 5: ....X..........
    // row 6: ....X..........
    // row 7: OXXX...........
    // X (7,4) creates a horizontal four, forcing O (7,5).
    // The same X (7,4) makes a vertical three at rows 5..7, column 4.
    // X (4,4) or (8,4) then makes an open four: O blocks, X wins.
    // Thus the attack takes five plies, with no initial win-in-three.
    place_line(&mut board, (7, 1), (0, 1), 3, attacker);
    place_line(&mut board, (5, 4), (1, 0), 2, attacker);
    place(&mut board, (7, 0), attacker.opposite());
    board
}

fn losing_race(attacker: ge::Player) -> ge::Board {
    let mut board = long_attack(attacker);
    // Add an independent open three. The defender must close one of its ends
    // at (1,1)/(1,5) to replace a loss in four plies with a loss in six.
    // Blocking the long attack instead leaves the quick attack untouched.
    place_line(&mut board, (1, 2), (0, 1), 3, attacker);
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    assert!(board.win_in_one(attacker).is_empty());
    assert!(board.win_in_one(attacker.opposite()).is_empty());
    board
}

#[test]
fn forcing_distance_long_attack_reports_five_with_sufficient_budget() {
    for attacker in PLAYERS {
        let board = long_attack(attacker);
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert!(board.win_in_one(attacker).is_empty());
        assert!(board.win_in_one(attacker.opposite()).is_empty());
        // Verify a concrete five-move witness, including both final blocks.
        let after_attack = board.apply_move((7, 4), attacker).unwrap();
        assert_eq!(after_attack.win_in_one(attacker), vec![(7, 5)]);
        let after_block = after_attack
            .apply_move((7, 5), attacker.opposite())
            .unwrap();
        let after_second_attack = after_block.apply_move((4, 4), attacker).unwrap();
        assert_eq!(
            after_second_attack.win_in_one(attacker),
            vec![(3, 4), (8, 4)]
        );
        for (block, win) in [((3, 4), (8, 4)), ((8, 4), (3, 4))] {
            let terminal = after_second_attack
                .apply_move(block, attacker.opposite())
                .unwrap()
                .apply_move(win, attacker)
                .unwrap();
            assert_eq!(terminal.check_result(), ge::GameResult::Win(attacker));
        }
        for budget in [5, 6] {
            assert_eq!(
                search_forcing_line(&board, attacker, attacker, budget),
                ForcingResult::ProvenWin { plies: 5 },
                "attacker={attacker:?}, budget={budget}",
            );
        }
    }
}

#[test]
fn forcing_budget_rejects_five_ply_attacks_beyond_the_horizon() {
    for attacker in PLAYERS {
        let board = long_attack(attacker);
        assert_eq!(board.check_result(), ge::GameResult::InProgress);
        assert!(board.win_in_one(attacker).is_empty());
        assert!(board.win_in_one(attacker.opposite()).is_empty());
        for budget in [0, 1, 2, 3, 4] {
            assert_eq!(
                search_forcing_line(&board, attacker, attacker, budget),
                ForcingResult::Unknown,
                "attacker={attacker:?}, budget={budget}",
            );
        }
    }
}

#[test]
fn forcing_distance_competing_attacks_report_three_for_either_colour() {
    for attacker in PLAYERS {
        let board = winning_race(attacker);
        // The open three can win in three; the central attack takes five.
        // The concrete quick endpoints are already checked by the ranking test.
        for budget in [3, 5] {
            assert_eq!(
                search_forcing_line(&board, attacker, attacker, budget),
                ForcingResult::ProvenWin { plies: 3 },
                "attacker={attacker:?}, budget={budget}",
            );
        }
    }
}

#[test]
fn forcing_ranking_chooses_win_in_three_over_proven_win_in_five() {
    for attacker in PLAYERS {
        let board = winning_race(attacker);
        let defender = attacker.opposite();
        let quick = board.apply_move((1, 5), attacker).unwrap();
        let slow = board.apply_move((7, 7), attacker).unwrap();

        // Quick: X (1,5), O blocks one end, X wins at the other: 3 plies.
        assert_eq!(quick.win_in_one(attacker), vec![(1, 1), (1, 6)]);
        // Slow: X (7,7), O (7,8), X (1,5), O blocks, X wins: 5 plies.
        assert_eq!(slow.win_in_one(attacker), vec![(7, 8)]);
        assert_proven(&quick, attacker, defender, FORCING_BUDGET);
        assert_proven(&slow, attacker, defender, FORCING_BUDGET);
        assert_eq!(exact_score(quick, defender, attacker, 2), 9_997.0);
        assert_eq!(exact_score(slow, defender, attacker, 2), 9_995.0);

        let chosen = best_move_alpha_beta(board, attacker, 3).unwrap();
        assert!(
            [(1, 1), (1, 5)].contains(&chosen),
            "attacker={attacker:?}: {chosen:?} misses the win in three plies",
        );
    }
}

#[test]
fn forcing_ranking_shallow_win_preserves_proof_when_distance_is_unresolved() {
    for attacker in PLAYERS {
        let board = long_attack(attacker);
        let known = board.apply_move((7, 4), attacker).unwrap();
        assert_eq!(
            search_forcing_line(&known, attacker, attacker.opposite(), FORCING_BUDGET),
            ForcingResult::ProvenWin { plies: 4 },
        );
        assert!(exact_score(known, attacker.opposite(), attacker, 0).abs() < 100.0);

        let chosen = best_move_alpha_beta(board, attacker, 1).unwrap();
        let next = board.apply_move(chosen, attacker).unwrap();
        // A shallow heuristic must not erase the proof or its five-move line
        // from the original root, including the selected root move itself.
        assert_eq!(
            search_forcing_line(&next, attacker, attacker.opposite(), FORCING_BUDGET),
            ForcingResult::ProvenWin { plies: 4 },
            "attacker={attacker:?}, chosen={chosen:?}",
        );
    }
}

#[test]
fn forcing_ranking_shallow_loss_uses_longer_proof_when_distance_is_unresolved() {
    for attacker in PLAYERS {
        let board = losing_race(attacker);
        let defender = attacker.opposite();
        let short = board.apply_move((7, 5), defender).unwrap();
        let long = board.apply_move((1, 5), defender).unwrap();
        assert_eq!(
            search_forcing_line(&short, attacker, attacker, FORCING_BUDGET),
            ForcingResult::ProvenWin { plies: 3 },
        );
        assert_eq!(
            search_forcing_line(&long, attacker, attacker, FORCING_BUDGET),
            ForcingResult::ProvenWin { plies: 5 },
        );
        assert!(exact_score(short, attacker, defender, 0).abs() < 100.0);
        assert!(exact_score(long, attacker, defender, 0).abs() < 100.0);

        let chosen = best_move_alpha_beta(board, defender, 1).unwrap();
        assert!(
            [(1, 1), (1, 5)].contains(&chosen),
            "defender={defender:?}, chosen={chosen:?}",
        );
    }
}

#[test]
fn forcing_ranking_prefers_resolved_loss_distance_over_a_longer_proof_bound() {
    for attacker in PLAYERS {
        let board = losing_race(attacker);
        let defender = attacker.opposite();
        let depth = 2;
        let resolved = board.apply_move((7, 5), defender).unwrap();
        let unresolved = board.apply_move((1, 5), defender).unwrap();

        // The shallow search sees the loss in four, including the root move.
        // The longer proof bounds the loss at six but leaves its distance unresolved.
        assert_eq!(
            exact_score(resolved, attacker, defender, depth - 1),
            -9_996.0
        );
        assert_eq!(
            search_forcing_line(&unresolved, attacker, attacker, FORCING_BUDGET),
            ForcingResult::ProvenWin { plies: 5 },
        );
        assert!(exact_score(unresolved, attacker, defender, depth - 1).abs() < 100.0);

        let chosen = best_move_alpha_beta(board, defender, depth).unwrap();
        let next = board.apply_move(chosen, defender).unwrap();
        assert_eq!(
            exact_score(next, attacker, defender, depth - 1),
            -9_996.0,
            "defender={defender:?}, chosen={chosen:?}",
        );
    }
}

#[test]
fn forcing_ranking_when_every_candidate_is_proven_lost_chooses_six_over_four() {
    for attacker in PLAYERS {
        let board = losing_race(attacker);
        let defender = attacker.opposite();
        // This is specifically the all-candidates-lost branch from 1B.
        // Check every move used by the root, not just the two example lines.
        for position in board.available_empty_moves_close(CLOSE_RANGE_PARAM) {
            let next = board.apply_move(position, defender).unwrap();
            let result = search_forcing_line(&next, attacker, attacker, FORCING_BUDGET);
            assert!(
                matches!(result, ForcingResult::ProvenWin { plies } if plies <= FORCING_BUDGET),
                "attacker={attacker:?}, root defence={position:?}: {result:?}",
            );
        }

        let short = board.apply_move((7, 5), defender).unwrap();
        assert_eq!(exact_score(short, attacker, defender, 3), -9_996.0);
        for defence in [(1, 1), (1, 5)] {
            let long = board.apply_move(defence, defender).unwrap();
            assert_eq!(exact_score(long, attacker, defender, 3), -9_994.0);
        }

        let chosen = best_move_alpha_beta(board, defender, 4).unwrap();
        assert!(
            [(1, 1), (1, 5)].contains(&chosen),
            "defender={defender:?}: {chosen:?} loses in four; six plies are possible",
        );
    }
}

#[test]
fn forcing_ranking_positive_shallow_heuristic_does_not_erase_a_proven_loss() {
    for attacker in PLAYERS {
        let defender = attacker.opposite();
        let mut board = long_attack(attacker);
        // O's open pair can become an open three. Its attractive static score
        // does not stop X's separate forcing attack described in long_attack.
        place_line(&mut board, (11, 5), (0, 1), 2, defender);
        let tempting = board.apply_move((11, 4), defender).unwrap();
        assert_proven(&tempting, attacker, attacker, FORCING_BUDGET);
        let shallow = exact_score(tempting, attacker, defender, 0);
        assert!(shallow > 0.0 && shallow < 100.0, "shallow={shallow}");
        assert_eq!(exact_score(tempting, attacker, defender, 3), -9_994.0);

        // There is also an unproven candidate: occupy the attack's key square.
        // Unknown is not a proof of safety; it simply has no proof of loss here.
        let blocked = board.apply_move((7, 4), defender).unwrap();
        assert_eq!(
            search_forcing_line(&blocked, attacker, attacker, FORCING_BUDGET),
            ForcingResult::Unknown,
        );

        let chosen = best_move_alpha_beta(board, defender, 1).unwrap();
        let next = board.apply_move(chosen, defender).unwrap();
        assert_eq!(
            search_forcing_line(&next, attacker, attacker, FORCING_BUDGET),
            ForcingResult::Unknown,
            "defender={defender:?}: {chosen:?} prefers a proven loss to an unproven candidate",
        );
    }
}

#[test]
fn forcing_budget_limits_candidate_proofs_to_requested_plies() {
    for attacker in PLAYERS {
        let board = winning_race(attacker);
        let defender = attacker.opposite();
        let quick = board.apply_move((1, 5), attacker).unwrap();
        let slow = board.apply_move((7, 7), attacker).unwrap();

        // Two half-moves remain for quick, four for slow. A proof is allowed
        // exactly at that boundary, and must keep its distance at larger budgets.
        for (candidate, plies) in [(&quick, 2), (&slow, 4)] {
            for budget in [0, 1, 2, 3, 4, 5] {
                let expected = if budget < plies {
                    ForcingResult::Unknown
                } else {
                    ForcingResult::ProvenWin { plies }
                };
                assert_eq!(
                    search_forcing_line(candidate, attacker, defender, budget),
                    expected,
                    "attacker={attacker:?}, required plies={plies}, budget={budget}",
                );
            }
        }
        // Same proof category and same budget, but different exact distances.
        assert_eq!(exact_score(quick, defender, attacker, 2), 9_997.0);
        assert_eq!(exact_score(slow, defender, attacker, 2), 9_995.0);
        // A proof of unknown distance remains a proof when a shallow search
        // returns a heuristic. It must not turn into Unknown/draw/win for O.
        assert!(exact_score(slow, defender, attacker, 0).abs() < 100.0);
        assert_proven(&slow, attacker, defender, FORCING_BUDGET);
    }
}

#[test]
fn forcing_ranking_full_windows_recover_distance_hidden_by_a_cutoff() {
    for attacker in PLAYERS {
        let defender = attacker.opposite();
        let board = losing_race(attacker);
        let long = board.apply_move((1, 5), defender).unwrap();
        assert_proven(&long, attacker, attacker, FORCING_BUDGET);

        // With the current move order a narrow window stops at a quiet,
        // negative heuristic before X's winning attack. A fail-low result is
        // only an upper bound; better move ordering may make it exact too.
        let bounded = minimax_alpha_beta(long, false, defender, 3, -1.0, 1.0, 1);
        assert!(bounded <= -1.0, "expected a fail-low bound, got {bounded}");
        assert!(bounded >= -9_994.0, "invalid upper bound: {bounded}");
        // Independently recover exact distances with fresh, full windows.
        // Never derive a mate distance or a game result from `bounded` alone.
        assert_eq!(exact_score(long, attacker, defender, 3), -9_994.0);
        let short = board.apply_move((7, 5), defender).unwrap();
        assert_eq!(exact_score(short, attacker, defender, 3), -9_996.0);
    }
}

#[test]
fn forcing_ranking_immediate_root_win_still_beats_longer_proven_attacks() {
    for attacker in PLAYERS {
        let mut board = winning_race(attacker);
        place_line(&mut board, (13, 2), (0, 1), 4, attacker);
        let slower = board.apply_move((7, 7), attacker).unwrap();
        assert_proven(&slower, attacker, attacker.opposite(), FORCING_BUDGET);

        let chosen = best_move_alpha_beta(board, attacker, 1).unwrap();
        assert!(
            [(13, 1), (13, 6)].contains(&chosen),
            "attacker={attacker:?}: {chosen:?} misses an immediate win",
        );
        assert_eq!(
            board.apply_move(chosen, attacker).unwrap().check_result(),
            ge::GameResult::Win(attacker),
        );
    }
}

#[test]
fn quiet_move_wins_in_five_while_shortest_forcing_line_takes_seven() {
    let black = [
        (1, 4),
        (2, 3),
        (2, 5),
        (3, 4),
        (10, 1),
        (10, 2),
        (10, 3),
        (8, 4),
        (9, 4),
        (11, 2),
        (11, 3),
    ];
    let white = [
        (10, 0),
        (7, 4),
        (8, 5),
        (0, 10),
        (0, 14),
        (3, 11),
        (4, 14),
        (6, 10),
        (9, 13),
        (12, 10),
        (14, 14),
    ];
    // Alternating legal moves establish equal stone counts and Black to move.
    let mut game = ge::Game::new();
    for (black_move, white_move) in black.into_iter().zip(white) {
        game.play(black_move).unwrap();
        game.play(white_move).unwrap();
    }
    assert_eq!(game.result(), ge::GameResult::InProgress);
    assert_eq!(game.turn(), ge::Player::X);
    let board = *game.board();
    assert!(board.win_in_one(ge::Player::X).is_empty());
    assert!(board.win_in_one(ge::Player::O).is_empty());

    // No first move creates two winning squares, so Black cannot force a win
    // in three half-moves. Combined with the absence of an immediate win,
    // the five-move strategy below is also a shortest possible win.
    for first_move in board.available_empty_moves() {
        let next = board.apply_move(first_move, ge::Player::X).unwrap();
        assert!(next.win_in_one(ge::Player::X).len() < 2);
    }
    assert_eq!(
        search_forcing_line(&board, ge::Player::X, ge::Player::X, 5),
        ForcingResult::Unknown,
    );
    for budget in [7, 8] {
        assert_eq!(
            search_forcing_line(&board, ge::Player::X, ge::Player::X, budget),
            ForcingResult::ProvenWin { plies: 7 },
        );
    }

    // A seven-move forcing witness: two closed fours followed by an open four.
    let first = board.apply_move((10, 4), ge::Player::X).unwrap();
    assert_eq!(first.win_in_one(ge::Player::X), vec![(10, 5)]);
    let second = first
        .apply_move((10, 5), ge::Player::O)
        .unwrap()
        .apply_move((11, 4), ge::Player::X)
        .unwrap();
    assert_eq!(second.win_in_one(ge::Player::X), vec![(12, 4)]);
    let third = second
        .apply_move((12, 4), ge::Player::O)
        .unwrap()
        .apply_move((11, 1), ge::Player::X)
        .unwrap();
    assert_eq!(third.win_in_one(ge::Player::X), vec![(11, 0), (11, 5)]);
    for (block, win) in [((11, 0), (11, 5)), ((11, 5), (11, 0))] {
        let terminal = third
            .apply_move(block, ge::Player::O)
            .unwrap()
            .apply_move(win, ge::Player::X)
            .unwrap();
        assert_eq!(terminal.check_result(), ge::GameResult::Win(ge::Player::X));
    }

    // Quiet (2,4) creates two independent open threes, but no immediate threat.
    let quiet = board.apply_move((2, 4), ge::Player::X).unwrap();
    assert!(quiet.win_in_one(ge::Player::X).is_empty());
    assert!(!forcing_moves(&board, ge::Player::X).contains(&(2, 4)));
    // Check every legal White reply, not just a representative continuation.
    for reply in quiet.available_empty_moves() {
        let defended = quiet.apply_move(reply, ge::Player::O).unwrap();
        assert_eq!(defended.check_result(), ge::GameResult::InProgress);
        assert!(defended.win_in_one(ge::Player::O).is_empty());
        let fork = [(2, 2), (2, 6), (4, 4)]
            .into_iter()
            .filter_map(|attack| defended.apply_move(attack, ge::Player::X).ok())
            .find(|next| next.win_in_one(ge::Player::X).len() >= 2)
            .unwrap_or_else(|| panic!("No winning fork after White {reply:?}"));
        let wins = fork.win_in_one(ge::Player::X);
        for (block, win) in [(wins[0], wins[1]), (wins[1], wins[0])] {
            let terminal = fork
                .apply_move(block, ge::Player::O)
                .unwrap()
                .apply_move(win, ge::Player::X)
                .unwrap();
            assert_eq!(terminal.check_result(), ge::GameResult::Win(ge::Player::X));
        }
    }
    assert_eq!(score(board, ge::Player::X, ge::Player::X, 3, 0), 9_995.0);

    let chosen = best_move_alpha_beta(board, ge::Player::X, 3).unwrap();
    let next = board.apply_move(chosen, ge::Player::X).unwrap();
    assert!(next.win_in_one(ge::Player::X).is_empty());
    assert_eq!(exact_score(next, ge::Player::O, ge::Player::X, 2), 9_995.0);
}
