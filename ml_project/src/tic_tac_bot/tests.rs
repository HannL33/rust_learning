use super::*;
use std::collections::HashSet;

type MoveFinder = fn(tte::Board, tte::Player) -> Option<usize>;
const MOVE_FINDERS: [MoveFinder; 2] = [best_move, best_move_alpha_beta];

fn game_after(moves: &[usize]) -> tte::Game {
    let mut game = tte::Game::new();
    for &position in moves {
        game.play(position).expect("test moves must be legal");
    }
    game
}

#[test]
fn best_move_takes_an_immediate_win_for_either_player() {
    for moves in [&[0, 3, 1, 4, 6, 7][..], &[3, 0, 4, 1, 6, 5, 8][..]] {
        let mut game = game_after(moves);
        let bot = game.turn();
        let mut count = 0;
        assert_eq!(minimax(*game.board(), true, bot, &mut count), 10);
        for find_move in MOVE_FINDERS {
            assert_eq!(find_move(*game.board(), bot), Some(2));
        }
        game.play(2).unwrap();
        assert_eq!(game.result(), tte::GameResult::Win(bot));
    }
}

#[test]
fn best_move_blocks_an_immediate_loss() {
    // X X . / . O . / . . . — O must block the top row.
    let mut game = game_after(&[0, 4, 1]);
    let bot = game.turn();
    let mut count = 0;
    assert_eq!(minimax(*game.board(), true, bot, &mut count), 0);
    for find_move in MOVE_FINDERS {
        assert_eq!(find_move(*game.board(), bot), Some(2));
    }
    game.play(2).unwrap();
    assert_eq!(game.result(), tte::GameResult::InProgress);
}

#[test]
fn best_move_returns_none_after_a_win_or_draw() {
    let won = game_after(&[0, 3, 1, 4, 2]);
    let drawn = game_after(&[0, 1, 2, 4, 3, 5, 7, 6, 8]);
    for game in [won, drawn] {
        for player in [tte::Player::X, tte::Player::O] {
            for find_move in MOVE_FINDERS {
                assert_eq!(find_move(*game.board(), player), None);
            }
            for search in [best_move_with_stats, best_move_alpha_beta_with_stats] {
                let result = search(*game.board(), player);
                assert_eq!(result.position, None);
                assert_eq!(result.visited_nodes, 0);
            }
        }
    }
}

#[test]
fn move_search_statistics_include_the_only_terminal_child() {
    let game = game_after(&[0, 1, 2, 4, 3, 5, 7, 6]);
    for search in [best_move_with_stats, best_move_alpha_beta_with_stats] {
        let result = search(*game.board(), game.turn());
        assert_eq!(result.position, Some(8));
        assert_eq!(result.visited_nodes, 1);
    }
}

#[test]
fn alpha_beta_keeps_the_proven_drawing_move_after_a_corner_opening() {
    let game = game_after(&[0]);
    assert_eq!(best_move_alpha_beta(*game.board(), game.turn()), Some(4));
}

#[test]
fn both_bots_choose_a_legal_move_even_when_every_move_loses() {
    // O O . / O X X / . X . — X cannot block both winning threats.
    let game = game_after(&[4, 0, 5, 1, 7, 3]);
    let mut count = 0;
    assert_eq!(minimax(*game.board(), true, game.turn(), &mut count), -10);
    for find_move in MOVE_FINDERS {
        let position = find_move(*game.board(), game.turn()).unwrap();
        assert!(tte::apply_move(*game.board(), position, game.turn()).is_ok());
    }
}

#[test]
fn node_counters_include_terminal_nodes_accumulate_and_show_pruning() {
    let game = game_after(&[0, 1, 2, 4, 3, 5, 7, 6]);
    let mut plain_count = 0;
    let mut pruned_count = 0;
    assert_eq!(
        minimax(*game.board(), true, game.turn(), &mut plain_count),
        0
    );
    assert_eq!(
        minimax_alpha_beta(
            *game.board(),
            true,
            game.turn(),
            i32::MIN,
            i32::MAX,
            &mut pruned_count,
        ),
        0
    );
    // The current node and its sole terminal child.
    assert_eq!(plain_count, 2);
    assert_eq!(pruned_count, 2);

    let drawn = game_after(&[0, 1, 2, 4, 3, 5, 7, 6, 8]);
    assert_eq!(
        minimax(*drawn.board(), true, drawn.turn(), &mut plain_count),
        0
    );
    assert_eq!(
        minimax_alpha_beta(
            *drawn.board(),
            true,
            drawn.turn(),
            i32::MIN,
            i32::MAX,
            &mut pruned_count,
        ),
        0
    );
    assert_eq!(plain_count, 3);
    assert_eq!(pruned_count, 3);

    plain_count = 0;
    pruned_count = 0;
    let board = tte::Board::new();
    assert_eq!(minimax(board, true, tte::Player::X, &mut plain_count), 0);
    assert_eq!(
        minimax_alpha_beta(
            board,
            true,
            tte::Player::X,
            i32::MIN,
            i32::MAX,
            &mut pruned_count,
        ),
        0
    );
    assert!(pruned_count < plain_count);
}

#[test]
fn alpha_beta_matches_minimax_on_every_reachable_position() {
    fn visit(board: tte::Board, turn: tte::Player, seen: &mut HashSet<u32>) {
        let key = board.cells().fold(0, |key, cell| {
            key * 3
                + match cell {
                    tte::Cell::Empty => 0,
                    tte::Cell::Taken(tte::Player::X) => 1,
                    tte::Cell::Taken(tte::Player::O) => 2,
                }
        });
        if !seen.insert(key) {
            return;
        }

        let mut turn_score = 0;
        for bot in [tte::Player::X, tte::Player::O] {
            let mut plain_count = 0;
            let mut pruned_count = 0;
            let expected = minimax(board, turn == bot, bot, &mut plain_count);
            let actual = minimax_alpha_beta(
                board,
                turn == bot,
                bot,
                i32::MIN,
                i32::MAX,
                &mut pruned_count,
            );
            assert_eq!(actual, expected, "board {board:?}, bot {bot:?}");
            assert!(pruned_count > 0 && pruned_count <= plain_count);
            if bot == turn {
                turn_score = expected;
            }
        }

        if tte::check_winner(&board) != tte::GameResult::InProgress {
            for find_move in MOVE_FINDERS {
                assert_eq!(find_move(board, turn), None);
            }
            return;
        }

        // Equally good moves may have different indices; compare exact outcomes.
        for find_move in MOVE_FINDERS {
            let position = find_move(board, turn).expect("a legal move must exist");
            let next =
                tte::apply_move(board, position, turn).expect("the selected move must be legal");
            let mut count = 0;
            assert_eq!(
                minimax(next, false, turn, &mut count),
                turn_score,
                "suboptimal move {position} on {board:?} for {turn:?}"
            );
        }

        for position in tte::available_moves(&board).unwrap() {
            let next = tte::apply_move(board, position, turn).unwrap();
            visit(next, turn.opposite(), seen);
        }
    }

    let mut seen = HashSet::new();
    visit(tte::Board::new(), tte::Player::X, &mut seen);
    assert_eq!(seen.len(), 5478);
}
