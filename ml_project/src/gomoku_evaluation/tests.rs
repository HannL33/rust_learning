use super::*;

type Position = (usize, usize);
type Direction = (isize, isize);
const DIRECTIONS: [Direction; 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
const PLAYERS: [ge::Player; 2] = [ge::Player::X, ge::Player::O];

// Endpoints: [before start, after the last stone]. An isolated stone
// is a length-one run in each of the four directions.
type StoneRunSnapshot = (Position, Direction, usize, [Option<Position>; 2]);

fn stone_run_snapshots(board: &ge::Board, player: ge::Player) -> Vec<StoneRunSnapshot> {
    let mut runs: Vec<_> = find_stone_runs(board, player)
        .into_iter()
        .map(|run| (run.start, run.direction, run.length, run.open_ends))
        .collect();
    runs.sort_unstable();
    runs
}

// Other axes through a stone may legitimately produce singletons. Compare
// all runs of length >= 2; the first test separately covers length-one runs.
fn assert_non_singleton_stone_runs(
    board: &ge::Board,
    player: ge::Player,
    mut expected: Vec<StoneRunSnapshot>,
) {
    let actual: Vec<_> = stone_run_snapshots(board, player)
        .into_iter()
        .filter(|run| run.2 >= 2)
        .collect();
    expected.sort_unstable();
    assert_eq!(actual, expected, "player={player:?}, board={board:?}");
}

#[test]
fn find_stone_runs_empty_board_and_isolated_stone() {
    for player in PLAYERS {
        let mut board = ge::Board::new();
        assert!(stone_run_snapshots(&board, player).is_empty());
        let start = (7, 7);
        place(&mut board, start, player);
        let mut expected: Vec<_> = DIRECTIONS
            .into_iter()
            .map(|direction| {
                (
                    start,
                    direction,
                    1,
                    [offset(start, direction, -1), offset(start, direction, 1)],
                )
            })
            .collect();
        expected.sort_unstable();
        assert_eq!(stone_run_snapshots(&board, player), expected);
        assert!(stone_run_snapshots(&board, player.opposite()).is_empty());
    }
}

#[test]
fn find_stone_runs_returns_each_maximal_run_once_in_all_four_directions() {
    for player in PLAYERS {
        for direction in DIRECTIONS {
            for length in [2, 3, 4, 6] {
                let mut board = ge::Board::new();
                let start = (4, 7);
                place_run(&mut board, start, direction, length, player);
                assert_non_singleton_stone_runs(
                    &board,
                    player,
                    vec![(
                        start,
                        direction,
                        length,
                        [
                            offset(start, direction, -1),
                            offset(start, direction, length as isize),
                        ],
                    )],
                );
                // One maximal run and three singleton axes per stone.
                // Reject length-one fragments along the main axis as well.
                assert_eq!(stone_run_snapshots(&board, player).len(), 1 + 3 * length);
            }
        }
    }
}

#[test]
fn find_stone_runs_marks_only_empty_endpoints_as_open() {
    for player in PLAYERS {
        for direction in DIRECTIONS {
            for (block_before, block_after) in
                [(false, false), (true, false), (false, true), (true, true)]
            {
                let mut board = ge::Board::new();
                let start = (5, 7);
                place_run(&mut board, start, direction, 3, player);
                let before = offset(start, direction, -1).unwrap();
                let after = offset(start, direction, 3).unwrap();
                if block_before {
                    place(&mut board, before, player.opposite());
                }
                if block_after {
                    place(&mut board, after, player.opposite());
                }
                assert_non_singleton_stone_runs(
                    &board,
                    player,
                    vec![(
                        start,
                        direction,
                        3,
                        [
                            (!block_before).then_some(before),
                            (!block_after).then_some(after),
                        ],
                    )],
                );
            }
        }
    }
}

#[test]
fn find_stone_runs_handles_borders_and_full_board_length_without_panicking() {
    for player in PLAYERS {
        for (start, direction) in [
            ((0, 0), (0, 1)),
            ((0, 0), (1, 0)),
            ((0, 0), (1, 1)),
            ((0, ge::BOARD_SIZE - 1), (1, -1)),
        ] {
            for length in [3, ge::BOARD_SIZE] {
                let mut board = ge::Board::new();
                place_run(&mut board, start, direction, length, player);
                assert_non_singleton_stone_runs(
                    &board,
                    player,
                    vec![(
                        start,
                        direction,
                        length,
                        [None, offset(start, direction, length as isize)],
                    )],
                );
            }
        }
    }
}

#[test]
fn find_stone_runs_splits_at_empty_cells_and_opponents_without_mutating_board() {
    for player in PLAYERS {
        let mut board = ge::Board::new();
        // Row 7: ..XX.XXOXXX....
        place_run(&mut board, (7, 2), (0, 1), 2, player);
        place_run(&mut board, (7, 5), (0, 1), 2, player);
        place(&mut board, (7, 7), player.opposite());
        place_run(&mut board, (7, 8), (0, 1), 3, player);
        let before = *board.cells();
        assert_non_singleton_stone_runs(
            &board,
            player,
            vec![
                ((7, 2), (0, 1), 2, [Some((7, 1)), Some((7, 4))]),
                ((7, 5), (0, 1), 2, [Some((7, 4)), None]),
                ((7, 8), (0, 1), 3, [None, Some((7, 11))]),
            ],
        );
        assert_non_singleton_stone_runs(&board, player.opposite(), vec![]);
        assert_eq!(*board.cells(), before);
    }
}

#[test]
fn find_stone_runs_keeps_crossing_runs_and_their_diagonals() {
    for player in PLAYERS {
        let mut board = ge::Board::new();
        for position in [(7, 6), (7, 7), (7, 8), (6, 7), (8, 7)] {
            place(&mut board, position, player);
        }
        assert_non_singleton_stone_runs(
            &board,
            player,
            vec![
                ((7, 6), (0, 1), 3, [Some((7, 5)), Some((7, 9))]),
                ((6, 7), (1, 0), 3, [Some((5, 7)), Some((9, 7))]),
                ((6, 7), (1, 1), 2, [Some((5, 6)), Some((8, 9))]),
                ((7, 6), (1, 1), 2, [Some((6, 5)), Some((9, 8))]),
                ((6, 7), (1, -1), 2, [Some((5, 8)), Some((8, 5))]),
                ((7, 8), (1, -1), 2, [Some((6, 9)), Some((9, 6))]),
            ],
        );
    }
}

// Independent oracle: enumerate complete geometric lines, then partition each
// line into contiguous player segments. It never checks whether a board cell
// is a start by looking backwards, unlike the production implementation.
fn reference_stone_run_snapshots(board: &ge::Board, player: ge::Player) -> Vec<StoneRunSnapshot> {
    let size = board.size();
    let mut lines: Vec<(Position, Direction)> = Vec::new();
    for i in 0..size {
        lines.extend([
            ((i, 0), (0, 1)),
            ((0, i), (1, 0)),
            ((0, i), (1, 1)),
            ((0, i), (1, -1)),
        ]);
    }
    for i in 1..size {
        lines.extend([((i, 0), (1, 1)), ((i, size - 1), (1, -1))]);
    }

    let mut runs = Vec::new();
    for (origin, direction) in lines {
        let positions: Vec<Position> = (0..size as isize)
            .map(|step| {
                (
                    origin.0 as isize + step * direction.0,
                    origin.1 as isize + step * direction.1,
                )
            })
            .take_while(|&(row, col)| {
                (0..size as isize).contains(&row) && (0..size as isize).contains(&col)
            })
            .map(|(row, col)| (row as usize, col as usize))
            .collect();

        let mut cursor = 0;
        while cursor < positions.len() {
            let (row, col) = positions[cursor];
            if board.cells()[row][col] != ge::Cell::Taken(player) {
                cursor += 1;
                continue;
            }
            let first = cursor;
            while cursor < positions.len() {
                let (row, col) = positions[cursor];
                if board.cells()[row][col] != ge::Cell::Taken(player) {
                    break;
                }
                cursor += 1;
            }
            let before = first.checked_sub(1).map(|index| positions[index]);
            let after = positions.get(cursor).copied();
            runs.push((
                positions[first],
                direction,
                cursor - first,
                [before, after].map(|position| {
                    position.filter(|&(row, col)| board.cells()[row][col] == ge::Cell::Empty)
                }),
            ));
        }
    }
    runs.sort_unstable();
    runs
}

#[test]
fn find_stone_runs_isolated_stones_at_every_position_have_correct_endpoints() {
    for player in PLAYERS {
        for row in 0..ge::BOARD_SIZE {
            for col in 0..ge::BOARD_SIZE {
                let mut board = ge::Board::new();
                let start = (row, col);
                place(&mut board, start, player);
                let mut expected: Vec<_> = DIRECTIONS
                    .into_iter()
                    .map(|direction| {
                        (
                            start,
                            direction,
                            1,
                            [offset(start, direction, -1), offset(start, direction, 1)],
                        )
                    })
                    .collect();
                expected.sort_unstable();
                assert_eq!(
                    stone_run_snapshots(&board, player),
                    expected,
                    "player={player:?}, position={start:?}",
                );
                assert!(stone_run_snapshots(&board, player.opposite()).is_empty());
            }
        }
    }
}

#[test]
fn find_stone_runs_preserves_runs_ending_at_bottom_and_right_borders() {
    let last = ge::BOARD_SIZE - 1;
    for player in PLAYERS {
        for (start, direction) in [
            ((last, last - 2), (0, 1)),
            ((last - 2, last), (1, 0)),
            ((last - 2, last - 2), (1, 1)),
            ((last - 2, 2), (1, -1)),
        ] {
            for blocked_before in [false, true] {
                let mut board = ge::Board::new();
                place_run(&mut board, start, direction, 3, player);
                let before = offset(start, direction, -1).unwrap();
                if blocked_before {
                    place(&mut board, before, player.opposite());
                }
                assert_non_singleton_stone_runs(
                    &board,
                    player,
                    vec![(
                        start,
                        direction,
                        3,
                        [(!blocked_before).then_some(before), None],
                    )],
                );
            }
        }
    }
}

#[test]
fn find_stone_runs_does_not_join_opposite_edges_of_consecutive_rows() {
    for player in PLAYERS {
        for row in 0..ge::BOARD_SIZE - 1 {
            let mut board = ge::Board::new();
            let stones = [(row, ge::BOARD_SIZE - 1), (row + 1, 0)];
            for position in stones {
                place(&mut board, position, player);
            }
            let mut expected = Vec::new();
            for start in stones {
                for direction in DIRECTIONS {
                    expected.push((
                        start,
                        direction,
                        1,
                        [offset(start, direction, -1), offset(start, direction, 1)],
                    ));
                }
            }
            expected.sort_unstable();
            assert_eq!(stone_run_snapshots(&board, player), expected);
        }
    }
}

#[test]
fn find_stone_runs_matches_reference_on_uniform_and_checkerboard_full_boards() {
    for pattern in 0..3 {
        let mut board = ge::Board::new();
        for row in 0..ge::BOARD_SIZE {
            for col in 0..ge::BOARD_SIZE {
                let player = match pattern {
                    0 => ge::Player::X,
                    1 => ge::Player::O,
                    _ if (row + col) % 2 == 0 => ge::Player::X,
                    _ => ge::Player::O,
                };
                place(&mut board, (row, col), player);
            }
        }
        for player in PLAYERS {
            let actual = stone_run_snapshots(&board, player);
            assert_eq!(
                actual,
                reference_stone_run_snapshots(&board, player),
                "pattern={pattern}, player={player:?}",
            );
            assert!(actual.iter().all(|run| run.3 == [None, None]));
        }
    }
}

#[test]
fn find_stone_runs_matches_reference_on_reproducible_random_boards() {
    let mut rng = Lcg(0x5EED_570E);
    let fills = [0, 5, 25, 50, 80, 100];
    for sample in 0..180 {
        let fill = fills[sample % fills.len()];
        let board = random_board(&mut rng, fill);
        let before = *board.cells();
        for player in PLAYERS {
            let actual = stone_run_snapshots(&board, player);
            assert_eq!(
                actual,
                reference_stone_run_snapshots(&board, player),
                "sample={sample}, fill={fill}, player={player:?}",
            );
            // Each stone belongs to exactly one maximal run in each axis.
            let stone_count = board
                .cells()
                .iter()
                .flatten()
                .filter(|&&cell| cell == ge::Cell::Taken(player))
                .count();
            assert_eq!(
                actual.iter().map(|run| run.2).sum::<usize>(),
                4 * stone_count
            );
        }
        assert_eq!(*board.cells(), before);
    }
}

fn offset(start: Position, direction: Direction, steps: isize) -> Option<Position> {
    let row = start.0 as isize + direction.0 * steps;
    let col = start.1 as isize + direction.1 * steps;
    let bounds = 0..ge::BOARD_SIZE as isize;
    (bounds.contains(&row) && bounds.contains(&col)).then_some((row as usize, col as usize))
}

fn assert_available_line_length(
    board: &ge::Board,
    player: ge::Player,
    start: Position,
    direction: Direction,
    expected: usize,
) {
    let run = find_stone_runs(board, player)
        .into_iter()
        .find(|run| run.start == start && run.direction == direction)
        .expect("the fixture must contain a run with this start and direction");
    assert_eq!(
        available_line_length(board, &run, player),
        expected,
        "player={player:?}, start={start:?}, direction={direction:?}",
    );
}

#[test]
fn available_line_length_distinguishes_space_for_four_and_five() {
    for player in PLAYERS {
        for direction in DIRECTIONS {
            // OXXX.O, O.XXXO, OXXX..O, O.XXX.O, O..XXXO.
            for (free_before, free_after, expected) in
                [(0, 1, 4), (1, 0, 4), (0, 2, 5), (1, 1, 5), (2, 0, 5)]
            {
                let mut board = ge::Board::new();
                let start = (5, 7);
                place_run(&mut board, start, direction, 3, player);
                place(
                    &mut board,
                    offset(start, direction, -(free_before + 1)).unwrap(),
                    player.opposite(),
                );
                place(
                    &mut board,
                    offset(start, direction, 3 + free_after).unwrap(),
                    player.opposite(),
                );
                assert_available_line_length(&board, player, start, direction, expected);
            }
        }
    }
}

#[test]
fn available_line_length_counts_fully_blocked_run_once() {
    for player in PLAYERS {
        for direction in DIRECTIONS {
            for length in [1, 3, 6] {
                let mut board = ge::Board::new();
                let start = (5, 7);
                place_run(&mut board, start, direction, length, player);
                place(
                    &mut board,
                    offset(start, direction, -1).unwrap(),
                    player.opposite(),
                );
                place(
                    &mut board,
                    offset(start, direction, length as isize).unwrap(),
                    player.opposite(),
                );
                assert_available_line_length(&board, player, start, direction, length);
            }
        }
    }
}

#[test]
fn available_line_length_uses_board_edges_including_short_corner_diagonals() {
    let last = ge::BOARD_SIZE - 1;
    for player in PLAYERS {
        for (start, direction, expected) in [
            ((0, 0), (0, 1), ge::BOARD_SIZE),
            ((0, 0), (1, 0), ge::BOARD_SIZE),
            ((0, 0), (1, 1), ge::BOARD_SIZE),
            ((0, 0), (1, -1), 1),
            ((0, last), (1, -1), ge::BOARD_SIZE),
            ((0, last), (1, 1), 1),
            ((last, 0), (1, 1), 1),
            ((last, 0), (1, -1), ge::BOARD_SIZE),
            ((last, last), (1, 1), ge::BOARD_SIZE),
            ((last, last), (1, -1), 1),
            ((0, 7), (1, 1), 8),
            ((0, 7), (1, -1), 8),
            ((7, 0), (1, 1), 8),
            ((7, last), (1, -1), 8),
            ((7, 7), (0, 1), ge::BOARD_SIZE),
            ((7, 7), (1, 0), ge::BOARD_SIZE),
            ((7, 7), (1, 1), ge::BOARD_SIZE),
            ((7, 7), (1, -1), ge::BOARD_SIZE),
        ] {
            let mut board = ge::Board::new();
            place(&mut board, start, player);
            assert_available_line_length(&board, player, start, direction, expected);
        }
    }
}

#[test]
fn available_line_length_crosses_gaps_and_own_stones_but_stops_at_opponents() {
    for player in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            let start = (6, 7);
            // X O X . XX . X . O X; the segment between the opponents has 7 fields.
            // Own stones beyond either opponent must not be included.
            for step in [-4, -2, 0, 1, 3, 6] {
                place(&mut board, offset(start, direction, step).unwrap(), player);
            }
            for step in [-3, 5] {
                place(
                    &mut board,
                    offset(start, direction, step).unwrap(),
                    player.opposite(),
                );
            }
            let before = *board.cells();
            assert_available_line_length(&board, player, start, direction, 7);
            assert_eq!(*board.cells(), before);
        }
    }
}

#[test]
fn available_line_length_is_the_same_for_different_runs_in_one_segment() {
    let mut board = ge::Board::new();
    // Row 8: O.XX.X..XX.O...
    // Columns 1..=10 are available: exactly 10 fields for each of the three runs.
    for position in [(7, 2), (7, 3), (7, 5), (7, 8), (7, 9)] {
        place(&mut board, position, ge::Player::X);
    }
    for position in [(7, 0), (7, 11), (6, 5), (8, 5)] {
        place(&mut board, position, ge::Player::O);
    }

    assert_available_line_length(&board, ge::Player::X, (7, 2), (0, 1), 10);
    assert_available_line_length(&board, ge::Player::X, (7, 5), (0, 1), 10);
    assert_available_line_length(&board, ge::Player::X, (7, 8), (0, 1), 10);
}

#[test]
fn available_line_length_counts_a_gapped_diagonal_between_opponent_and_edge() {
    let mut board = ge::Board::new();
    // Down-left diagonal:
    // J1 O, I2 X, H3 ., G4 O, F5 ., E6 O, D7 O, C8 O, B9 ., A10 O.
    // The available segment is H3..A10: exactly 8 fields.
    // J1 is beyond the opponent; D6 and E7 are outside the diagonal.
    for position in [(0, 9), (3, 6), (5, 4), (6, 3), (7, 2), (9, 0)] {
        place(&mut board, position, ge::Player::O);
    }
    for position in [(1, 8), (5, 3), (6, 4)] {
        place(&mut board, position, ge::Player::X);
    }

    assert_available_line_length(&board, ge::Player::O, (5, 4), (1, -1), 8);
    assert_available_line_length(&board, ge::Player::O, (3, 6), (1, -1), 8);
    assert_available_line_length(&board, ge::Player::O, (9, 0), (1, -1), 8);
}

fn place(board: &mut ge::Board, position: Position, player: ge::Player) {
    board.apply_move_inplace(position, player).unwrap();
}

fn place_run(
    board: &mut ge::Board,
    start: Position,
    direction: Direction,
    length: usize,
    player: ge::Player,
) -> Vec<Position> {
    (0..length)
        .map(|i| {
            let position = offset(start, direction, i as isize).unwrap();
            place(board, position, player);
            position
        })
        .collect()
}

// Project the new representation onto coordinates and closed-end counts so
// existing fixtures still check the same geometry, for a selected run length.
fn runs_of_length(
    board: &ge::Board,
    player: ge::Player,
    length: usize,
) -> Vec<(Vec<Position>, usize)> {
    find_stone_runs(board, player)
        .into_iter()
        .filter(|run| run.length == length)
        .map(|run| {
            let coordinates = (0..run.length)
                .map(|step| {
                    offset(run.start, run.direction, step as isize)
                        .expect("a reported run must stay inside the board")
                })
                .collect();
            let closed_ends = run.open_ends.iter().filter(|end| end.is_none()).count();
            (coordinates, closed_ends)
        })
        .collect()
}

// Compare semantic results without requiring a particular scan order.
fn assert_runs(
    board: &ge::Board,
    player: ge::Player,
    length: usize,
    mut expected: Vec<(Vec<Position>, usize)>,
) {
    let mut actual = runs_of_length(board, player, length);
    for (coordinates, _) in actual.iter_mut().chain(expected.iter_mut()) {
        coordinates.sort_unstable();
    }
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(
        actual, expected,
        "player {player:?}, length {length}, board {board:?}"
    );
}

fn assert_interior_run(block_before: bool, block_after: bool) {
    for player in PLAYERS {
        for length in [3, 4] {
            for direction in DIRECTIONS {
                let start = (5, 7);
                let mut board = ge::Board::new();
                let coordinates = place_run(&mut board, start, direction, length, player);
                if block_before {
                    place(
                        &mut board,
                        offset(start, direction, -1).unwrap(),
                        player.opposite(),
                    );
                }
                if block_after {
                    place(
                        &mut board,
                        offset(start, direction, length as isize).unwrap(),
                        player.opposite(),
                    );
                }
                assert_runs(
                    &board,
                    player,
                    length,
                    vec![(
                        coordinates,
                        usize::from(block_before) + usize::from(block_after),
                    )],
                );
            }
        }
    }
}

// Independent, intentionally naive implementation used as an oracle. It
// walks every start in every direction, requires the whole window to be the
// requested player, rejects windows that are actually part of a longer run,
// and counts off-board or opponent neighbours as closed ends.
fn reference_runs(
    board: &ge::Board,
    player: ge::Player,
    n_stones: usize,
) -> Vec<(Vec<Position>, usize)> {
    if n_stones == 0 {
        return Vec::new();
    }
    let before_and_after = |start: Position, direction: Direction| {
        [
            offset(start, direction, -1),
            offset(start, direction, n_stones as isize),
        ]
    };
    let mut runs = Vec::new();
    for row in 0..ge::BOARD_SIZE {
        for col in 0..ge::BOARD_SIZE {
            let start = (row, col);
            for direction in DIRECTIONS {
                let mut coordinates = Vec::with_capacity(n_stones);
                let mut whole_window_matches = true;
                for step in 0..n_stones as isize {
                    match offset(start, direction, step) {
                        Some((r, c)) if board.cells()[r][c] == ge::Cell::Taken(player) => {
                            coordinates.push((r, c));
                        }
                        _ => {
                            whole_window_matches = false;
                            break;
                        }
                    }
                }
                if !whole_window_matches {
                    continue;
                }
                let neighbors = before_and_after(start, direction);
                if neighbors
                    .into_iter()
                    .flatten()
                    .any(|(r, c)| board.cells()[r][c] == ge::Cell::Taken(player))
                {
                    continue;
                }
                let closed_ends = neighbors
                    .into_iter()
                    .filter(|neighbor| match neighbor {
                        None => true,
                        Some((r, c)) => board.cells()[*r][*c] == ge::Cell::Taken(player.opposite()),
                    })
                    .count();
                runs.push((coordinates, closed_ends));
            }
        }
    }
    runs
}

// Runs are compared without requiring a particular scan order.
fn normalize(mut runs: Vec<(Vec<Position>, usize)>) -> Vec<(Vec<Position>, usize)> {
    for (coordinates, _) in runs.iter_mut() {
        coordinates.sort_unstable();
    }
    runs.sort_unstable();
    runs
}

// Small deterministic PRNG so the randomized tests are reproducible and
// dependency free.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn random_board(rng: &mut Lcg, fill_percent: u64) -> ge::Board {
    let mut board = ge::Board::new();
    for row in 0..ge::BOARD_SIZE {
        for col in 0..ge::BOARD_SIZE {
            match rng.next() % 100 {
                value if value < fill_percent / 2 => {
                    place(&mut board, (row, col), ge::Player::X);
                }
                value if value < fill_percent => {
                    place(&mut board, (row, col), ge::Player::O);
                }
                _ => {}
            }
        }
    }
    board
}

#[test]
fn empty_board_has_no_runs() {
    for player in PLAYERS {
        for length in [3, 4] {
            assert_runs(&ge::Board::new(), player, length, vec![]);
        }
    }
}

#[test]
fn open_runs_have_zero_closed_ends_in_all_directions() {
    assert_interior_run(false, false);
}

#[test]
fn opponent_before_run_closes_one_end_in_all_directions() {
    assert_interior_run(true, false);
}

#[test]
fn opponent_after_run_closes_one_end_in_all_directions() {
    assert_interior_run(false, true);
}

#[test]
fn opponents_on_both_sides_close_both_ends() {
    assert_interior_run(true, true);
}

#[test]
fn every_valid_start_handles_borders_and_opponent_blockers() {
    // Includes all corners, diagonals touching two different borders,
    // and the last column (a regression for the earlier row/column typo).
    for player in PLAYERS {
        for length in [3, 4] {
            for direction in DIRECTIONS {
                for row in 0..ge::BOARD_SIZE {
                    for col in 0..ge::BOARD_SIZE {
                        let start = (row, col);
                        if offset(start, direction, length as isize - 1).is_none() {
                            continue;
                        }
                        let neighbors = [
                            offset(start, direction, -1),
                            offset(start, direction, length as isize),
                        ];
                        for mask in 0..4 {
                            let mut board = ge::Board::new();
                            let coordinates =
                                place_run(&mut board, start, direction, length, player);
                            let mut closed_ends = 0;
                            for (side, neighbor) in neighbors.iter().enumerate() {
                                match neighbor {
                                    None => closed_ends += 1,
                                    Some(position) if mask & (1 << side) != 0 => {
                                        place(&mut board, *position, player.opposite());
                                        closed_ends += 1;
                                    }
                                    Some(_) => {}
                                }
                            }
                            assert_runs(&board, player, length, vec![(coordinates, closed_ends)]);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn own_stone_before_run_excludes_the_fragment() {
    for player in PLAYERS {
        for length in [3, 4] {
            for direction in DIRECTIONS {
                let mut board = ge::Board::new();
                place_run(&mut board, (5, 7), direction, length, player);
                place(&mut board, offset((5, 7), direction, -1).unwrap(), player);
                assert_runs(&board, player, length, vec![]);
            }
        }
    }
}

#[test]
fn own_stone_after_run_excludes_the_fragment() {
    for player in PLAYERS {
        for length in [3, 4] {
            for direction in DIRECTIONS {
                let mut board = ge::Board::new();
                place_run(&mut board, (5, 7), direction, length + 1, player);
                assert_runs(&board, player, length, vec![]);
            }
        }
    }
}

#[test]
fn longer_runs_at_every_valid_start_have_no_three_or_four_fragments() {
    for player in PLAYERS {
        for length in [5, 6] {
            for direction in DIRECTIONS {
                for row in 0..ge::BOARD_SIZE {
                    for col in 0..ge::BOARD_SIZE {
                        if offset((row, col), direction, length as isize - 1).is_none() {
                            continue;
                        }
                        let mut board = ge::Board::new();
                        place_run(&mut board, (row, col), direction, length, player);
                        for requested in [3, 4] {
                            assert_runs(&board, player, requested, vec![]);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn gaps_do_not_join_stones_into_a_run() {
    for direction in DIRECTIONS {
        let mut board = ge::Board::new();
        for step in [0, 1, 3, 4] {
            place(
                &mut board,
                offset((5, 7), direction, step).unwrap(),
                ge::Player::X,
            );
        }
        for length in [3, 4] {
            assert_runs(&board, ge::Player::X, length, vec![]);
        }
    }
}

#[test]
fn opponent_inside_line_breaks_the_run() {
    for direction in DIRECTIONS {
        let mut board = ge::Board::new();
        for step in 0..5 {
            let player = if step == 2 {
                ge::Player::O
            } else {
                ge::Player::X
            };
            place(&mut board, offset((5, 7), direction, step).unwrap(), player);
        }
        for player in PLAYERS {
            for length in [3, 4] {
                assert_runs(&board, player, length, vec![]);
            }
        }
    }
}

#[test]
fn opponent_runs_are_not_returned_for_requested_player() {
    let mut board = ge::Board::new();
    let coordinates = place_run(&mut board, (5, 7), (1, -1), 4, ge::Player::O);
    assert_runs(&board, ge::Player::X, 4, vec![]);
    assert_runs(&board, ge::Player::O, 4, vec![(coordinates, 0)]);
}

#[test]
fn separated_runs_are_both_returned_once() {
    let mut board = ge::Board::new();
    let first = place_run(&mut board, (4, 2), (0, 1), 3, ge::Player::X);
    let second = place_run(&mut board, (4, 7), (0, 1), 3, ge::Player::X);
    assert_runs(&board, ge::Player::X, 3, vec![(first, 0), (second, 0)]);
}

#[test]
fn crossing_runs_are_both_returned() {
    let mut board = ge::Board::new();
    let horizontal = place_run(&mut board, (7, 6), (0, 1), 3, ge::Player::X);
    place(&mut board, (6, 7), ge::Player::X);
    place(&mut board, (8, 7), ge::Player::X);
    assert_runs(
        &board,
        ge::Player::X,
        3,
        vec![(horizontal, 0), (vec![(6, 7), (7, 7), (8, 7)], 0)],
    );
}

#[test]
fn rejecting_horizontal_fragment_does_not_skip_vertical_run() {
    let mut board = ge::Board::new();
    place_run(&mut board, (6, 4), (0, 1), 4, ge::Player::X);
    place(&mut board, (7, 4), ge::Player::X);
    place(&mut board, (8, 4), ge::Player::X);
    assert_runs(
        &board,
        ge::Player::X,
        3,
        vec![(vec![(6, 4), (7, 4), (8, 4)], 0)],
    );
}

#[test]
fn off_axis_opponents_do_not_close_run_ends() {
    let mut board = ge::Board::new();
    let coordinates = place_run(&mut board, (7, 5), (0, 1), 4, ge::Player::X);
    for position in [(6, 5), (8, 5), (6, 8), (8, 8)] {
        place(&mut board, position, ge::Player::O);
    }
    assert_runs(&board, ge::Player::X, 4, vec![(coordinates, 0)]);
}

#[test]
fn evaluation_of_empty_board_is_zero_for_both_players() {
    for player in PLAYERS {
        assert_eq!(evaluate(&ge::Board::new(), player), 0.0);
    }
}

#[test]
fn evaluation_scores_open_pairs_but_ignores_single_stones() {
    for player in PLAYERS {
        for length in [1, 2] {
            for direction in DIRECTIONS {
                let mut board = ge::Board::new();
                place_run(&mut board, (5, 7), direction, length, player);
                assert_eq!(
                    evaluate(&board, player),
                    if length == 2 { 5.0 } else { 0.0 }
                );
            }
        }
    }
}

#[test]
fn evaluation_matches_current_weights_for_each_direction_and_player() {
    for player in PLAYERS {
        for direction in DIRECTIONS {
            for (length, block_before, block_after, expected) in [
                (2, false, false, 5.0),
                (2, true, false, 0.0),
                (2, false, true, 0.0),
                (2, true, true, 0.0),
                (3, false, false, 50.0),
                (3, true, false, 30.0),
                (3, false, true, 30.0),
                (3, true, true, 0.0),
                (4, false, false, 85.0),
                (4, true, false, 55.0),
                (4, false, true, 55.0),
                (4, true, true, 0.0),
            ] {
                let mut board = ge::Board::new();
                place_run(&mut board, (5, 7), direction, length, player);
                if block_before {
                    place(
                        &mut board,
                        offset((5, 7), direction, -1).unwrap(),
                        player.opposite(),
                    );
                }
                if block_after {
                    place(
                        &mut board,
                        offset((5, 7), direction, length as isize).unwrap(),
                        player.opposite(),
                    );
                }
                assert_eq!(board.check_result(), ge::GameResult::InProgress);
                assert_eq!(
                    evaluate(&board, player),
                    expected,
                    "length {length}, direction {direction:?}, player {player:?}, blockers ({block_before}, {block_after})"
                );
            }
        }
    }
}

#[test]
fn evaluation_at_border_treats_it_as_a_closed_end() {
    for player in PLAYERS {
        for (length, expected) in [(3, 30.0), (4, 55.0)] {
            let mut board = ge::Board::new();
            place_run(&mut board, (5, 0), (0, 1), length, player);
            assert_eq!(evaluate(&board, player), expected);
        }
    }
}

#[test]
fn evaluation_combines_threes_and_fours_regardless_of_run_location() {
    for player in PLAYERS {
        for (strong_row, weak_row) in [(3, 10), (10, 3)] {
            let mut board = ge::Board::new();
            place_run(&mut board, (strong_row, 5), (0, 1), 4, player);
            place_run(&mut board, (weak_row, 5), (0, 1), 3, player);
            // An open four (85) and an open three (50) combine to 92.5.
            assert_eq!(evaluate(&board, player), 92.5);
        }
    }
}

#[test]
fn evaluation_combines_multiple_open_threes_with_diminishing_returns() {
    let mut board = ge::Board::new();
    place_run(&mut board, (3, 5), (0, 1), 3, ge::Player::X);
    place_run(&mut board, (10, 5), (0, 1), 3, ge::Player::X);
    assert_eq!(evaluate(&board, ge::Player::X), 75.0);
    place_run(&mut board, (6, 5), (0, 1), 3, ge::Player::X);
    assert_eq!(evaluate(&board, ge::Player::X), 87.5);
}

#[test]
fn nonterminal_evaluation_subtracts_opponents_patterns() {
    let mut board = ge::Board::new();
    place_run(&mut board, (5, 7), (1, -1), 4, ge::Player::O);
    assert_eq!(evaluate(&board, ge::Player::O), 85.0);
    assert_eq!(evaluate(&board, ge::Player::X), -85.0);
    place_run(&mut board, (12, 2), (0, 1), 3, ge::Player::X);
    assert_eq!(evaluate(&board, ge::Player::X), -35.0);
    assert_eq!(evaluate(&board, ge::Player::O), 35.0);
}

#[test]
fn terminal_win_has_correct_sign_and_overrides_patterns() {
    for winner in PLAYERS {
        for direction in DIRECTIONS {
            let mut board = ge::Board::new();
            place_run(&mut board, (3, 8), direction, 5, winner);
            place_run(&mut board, (11, 5), (0, 1), 4, winner.opposite());
            assert_eq!(board.check_result(), ge::GameResult::Win(winner));
            assert_eq!(evaluate(&board, winner), 100.0);
            assert_eq!(evaluate(&board, winner.opposite()), -100.0);
        }
    }
}

#[test]
fn six_stones_are_also_a_terminal_win() {
    for player in PLAYERS {
        let mut board = ge::Board::new();
        place_run(&mut board, (5, 7), (1, -1), 6, player);
        assert_eq!(evaluate(&board, player), 100.0);
        assert_eq!(evaluate(&board, player.opposite()), -100.0);
    }
}

#[test]
fn full_board_draw_scores_zero_for_both_players() {
    let mut board = ge::Board::new();
    // This pattern has no five in any direction (also used in engine tests).
    for row in 0..ge::BOARD_SIZE {
        for col in 0..ge::BOARD_SIZE {
            let player = if (row + 2 * col) % 4 < 2 {
                ge::Player::X
            } else {
                ge::Player::O
            };
            place(&mut board, (row, col), player);
        }
    }
    assert_eq!(board.check_result(), ge::GameResult::Draw);
    for player in PLAYERS {
        assert_eq!(evaluate(&board, player), 0.0);
    }
}

#[test]
fn evaluation_and_detection_leave_board_unchanged() {
    let mut board = ge::Board::new();
    place_run(&mut board, (5, 7), (1, -1), 4, ge::Player::X);
    let before = *board.cells();
    for player in PLAYERS {
        evaluate(&board, player);
        find_stone_runs(&board, player);
    }
    assert_eq!(*board.cells(), before);
}

#[test]
fn detection_matches_a_reference_on_many_random_boards() {
    // Differential test: the oracle shares no code with the implementation
    // under test, so mismatches point at real detection bugs (duplicated
    // runs, missed runs, wrong closed_ends, off-by-one windows).
    let mut rng = Lcg(0x0bad_c0de_dead_beef);
    for _ in 0..300 {
        let board = random_board(&mut rng, 45);
        for player in PLAYERS {
            for n_stones in [1, 2, 3, 4, 5, 6] {
                let actual = normalize(runs_of_length(&board, player, n_stones));
                let expected = normalize(reference_runs(&board, player, n_stones));
                assert_eq!(
                    actual, expected,
                    "n_stones {n_stones}, player {player:?}, board {board:?}"
                );
            }
        }
    }
}

#[test]
fn detection_handles_runs_on_the_last_row_and_last_column() {
    // Regression for the earlier row/column confusion in the vertical scan:
    // runs that terminate exactly on the bottom/right borders must still be
    // found, with the border counted as a closed end.
    for player in PLAYERS {
        let mut board = ge::Board::new();
        let horizontal = place_run(&mut board, (7, 12), (0, 1), 3, player);
        assert_runs(&board, player, 3, vec![(horizontal, 1)]);

        let mut board = ge::Board::new();
        let vertical = place_run(&mut board, (11, 7), (1, 0), 4, player);
        assert_runs(&board, player, 4, vec![(vertical, 1)]);
    }
}

#[test]
fn detection_handles_diagonals_touching_a_corner() {
    for player in PLAYERS {
        let mut board = ge::Board::new();
        let down_right = place_run(&mut board, (12, 12), (1, 1), 3, player);
        assert_runs(&board, player, 3, vec![(down_right, 1)]);

        let mut board = ge::Board::new();
        let down_left = place_run(&mut board, (12, 2), (1, -1), 3, player);
        assert_runs(&board, player, 3, vec![(down_left, 1)]);
    }
}

#[test]
fn detection_returns_all_four_directions_when_they_are_present() {
    let mut board = ge::Board::new();
    let horizontal = place_run(&mut board, (7, 5), (0, 1), 3, ge::Player::X);
    let vertical = place_run(&mut board, (5, 9), (1, 0), 3, ge::Player::X);
    let down_right = place_run(&mut board, (2, 2), (1, 1), 3, ge::Player::X);
    let down_left = place_run(&mut board, (2, 13), (1, -1), 3, ge::Player::X);
    assert_runs(
        &board,
        ge::Player::X,
        3,
        vec![
            (horizontal, 0),
            (vertical, 0),
            (down_right, 0),
            (down_left, 0),
        ],
    );
}

#[test]
fn detection_uses_only_the_requested_players_stones() {
    // Interleaved X and O runs near each other: each player must only see
    // their own run and the other colour must never close an end.
    let mut board = ge::Board::new();
    let horizontal = place_run(&mut board, (7, 5), (0, 1), 3, ge::Player::X);
    let vertical = place_run(&mut board, (3, 6), (1, 0), 3, ge::Player::O);
    assert_runs(&board, ge::Player::X, 3, vec![(horizontal, 0)]);
    assert_runs(&board, ge::Player::O, 3, vec![(vertical, 0)]);
}

#[test]
fn evaluation_ignores_a_dead_four_but_still_counts_an_open_three() {
    let mut board = ge::Board::new();
    place_run(&mut board, (0, 3), (0, 1), 4, ge::Player::X);
    place(&mut board, (0, 2), ge::Player::O);
    place(&mut board, (0, 7), ge::Player::O);
    place_run(&mut board, (5, 3), (0, 1), 3, ge::Player::X);
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    assert_eq!(evaluate(&board, ge::Player::X), 50.0);
}

#[test]
fn evaluation_combines_a_blocked_four_with_an_open_three() {
    let mut board = ge::Board::new();
    place_run(&mut board, (0, 3), (0, 1), 4, ge::Player::X);
    place(&mut board, (0, 7), ge::Player::O);
    place_run(&mut board, (5, 3), (0, 1), 3, ge::Player::X);
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    assert_eq!(evaluate(&board, ge::Player::X), 77.5);
}

#[test]
fn evaluation_keeps_an_open_four_open_next_to_a_distant_own_stone() {
    // The empty gap at (5, 7) separates the four from the single stone, so
    // the four must stay open (0 closed ends) rather than counting as five.
    let mut board = ge::Board::new();
    place_run(&mut board, (5, 3), (0, 1), 4, ge::Player::X);
    place(&mut board, (5, 8), ge::Player::X);
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    assert_eq!(evaluate(&board, ge::Player::X), 85.0);
}

#[test]
fn evaluation_counts_an_open_pair_but_ignores_single_stones_and_dead_fours() {
    let mut board = ge::Board::new();
    place_run(&mut board, (2, 2), (0, 1), 2, ge::Player::X);
    place(&mut board, (4, 4), ge::Player::X);
    place_run(&mut board, (8, 3), (0, 1), 4, ge::Player::X);
    place(&mut board, (8, 2), ge::Player::O);
    place(&mut board, (8, 7), ge::Player::O);
    assert_eq!(board.check_result(), ge::GameResult::InProgress);
    assert_eq!(evaluate(&board, ge::Player::X), 5.0);
}

#[test]
fn full_width_run_has_two_closed_ends() {
    for player in PLAYERS {
        for (start, direction) in [
            ((0, 0), (0, 1)),
            ((0, 0), (1, 0)),
            ((0, 0), (1, 1)),
            ((0, ge::BOARD_SIZE - 1), (1, -1)),
        ] {
            let mut board = ge::Board::new();
            let coordinates = place_run(&mut board, start, direction, ge::BOARD_SIZE, player);
            assert_runs(&board, player, ge::BOARD_SIZE, vec![(coordinates, 2)]);
        }
    }
}
