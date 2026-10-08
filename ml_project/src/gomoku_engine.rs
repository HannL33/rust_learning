// BOARD SIZE
const BOARD_SIZE: usize = 15;
//

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    X,
    Y,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Taken(Player),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    InProgress,
    Draw,
    Win(Player),
}

pub enum MoveError {
    CoorErr(String),
    TakenErr(String),
}

/// Board object is responsible only for 'board'\
/// so applying moves (changing state), checking consistency, correctness etc
/// Knows nothing about the game, turns, bots etc
#[derive(Copy, Clone, Debug)]
pub struct Board {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }
    pub fn apply_move_inplace(
        &mut self,
        coor: (usize, usize),
        player: Player,
    ) -> Result<(), MoveError> {
        if (0..BOARD_SIZE).contains(&coor.0) && (0..BOARD_SIZE).contains(&coor.1) {
            if self.board[coor.0][coor.1] == Cell::Empty {
                self.board[coor.0][coor.1] = Cell::Taken(player);
                Ok(())
            } else {
                Err(MoveError::TakenErr(
                    "The field is already taken!".to_string(),
                ))
            }
        } else {
            Err(MoveError::CoorErr(
                "Coordinates are out of the bonds for applying the move!".to_string(),
            ))
        }
    }
    pub fn apply_move(mut self, coor: (usize, usize), player: Player) -> Result<Self, MoveError> {
        // shortcut, so no repeatable code here
        self.apply_move_inplace(coor, player)?;
        Ok(self)
    }
    pub fn check_result(&self) -> GameResult {
        // we will scan the whole board for that
        for (r_idx, row) in self.board.iter().enumerate() {
            for (c_idx, cell) in row.iter().enumerate() {
                if let Cell::Taken(player) = *cell {
                    // check horizontal
                    if c_idx + 5 <= BOARD_SIZE
                        && row[c_idx..c_idx + 5]
                            .iter()
                            .all(|&x| x == Cell::Taken(player))
                    {
                        return GameResult::Win(player);
                    }
                    // check vertical
                    if r_idx + 5 <= BOARD_SIZE
                        && self.board[r_idx..r_idx + 5]
                            .iter()
                            .all(|&x| x[c_idx] == Cell::Taken(player))
                    {
                        return GameResult::Win(player);
                    }
                    // check diagonal from left to right
                    if r_idx + 5 <= BOARD_SIZE && c_idx + 5 <= BOARD_SIZE {
                        if (0..5).all(|i| self.board[r_idx + i][c_idx + i] == Cell::Taken(player)) {
                            return GameResult::Win(player);
                        }
                    }
                    // check diagonal from right to left
                    if r_idx + 5 <= BOARD_SIZE && c_idx >= 4 {
                        if (0..5).all(|i| self.board[r_idx + i][c_idx - i] == Cell::Taken(player)) {
                            return GameResult::Win(player);
                        }
                    }
                };
            }
        }
        if self.board.iter().flatten().any(|&x| x == Cell::Empty) {
            return GameResult::InProgress;
        }
        GameResult::Draw
    }
    pub fn board(&self) -> [[Cell; BOARD_SIZE]; BOARD_SIZE] {
        self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place(board: &mut Board, coor: (usize, usize), player: Player) {
        assert!(
            board.apply_move_inplace(coor, player).is_ok(),
            "move {coor:?} for {player:?} must succeed"
        );
    }

    fn full_draw_board() -> Board {
        let mut board = Board::new();
        // Horizontal runs have length 1; vertical and diagonal runs at most 2.
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let player = if (row + 2 * col) % 4 < 2 {
                    Player::X
                } else {
                    Player::Y
                };
                place(&mut board, (row, col), player);
            }
        }
        board
    }

    #[test]
    fn check_winner_1() {
        let mut board = Board::new();
        place(&mut board, (2, 2), Player::X);
        place(&mut board, (2, 3), Player::X);
        place(&mut board, (2, 4), Player::X);
        place(&mut board, (2, 5), Player::X);
        place(&mut board, (2, 6), Player::X);

        assert_eq!(board.check_result(), GameResult::Win(Player::X));
    }

    #[test]
    fn check_winner_2() {
        let mut board = Board::new();
        place(&mut board, (3, 7), Player::X);
        place(&mut board, (3, 6), Player::X);
        place(&mut board, (3, 5), Player::X);
        place(&mut board, (3, 4), Player::X);
        place(&mut board, (3, 3), Player::X);

        assert_eq!(board.check_result(), GameResult::Win(Player::X));
    }

    #[test]
    fn check_winner_3() {
        let mut board = Board::new();
        place(&mut board, (3, 2), Player::Y);
        place(&mut board, (4, 2), Player::Y);
        place(&mut board, (5, 2), Player::Y);
        place(&mut board, (6, 2), Player::Y);
        place(&mut board, (7, 2), Player::Y);

        assert_eq!(board.check_result(), GameResult::Win(Player::Y));
    }

    #[test]
    fn check_winner_4() {
        let mut board = Board::new();
        place(&mut board, (5, 5), Player::Y);
        place(&mut board, (6, 6), Player::Y);
        place(&mut board, (7, 7), Player::Y);
        place(&mut board, (8, 8), Player::Y);
        place(&mut board, (9, 9), Player::Y);

        assert_eq!(board.check_result(), GameResult::Win(Player::Y));
    }

    #[test]
    fn check_winner_5() {
        let mut board = Board::new();
        place(&mut board, (10, 10), Player::Y);
        place(&mut board, (11, 9), Player::Y);
        place(&mut board, (12, 8), Player::Y);
        place(&mut board, (13, 7), Player::Y);
        place(&mut board, (14, 6), Player::Y);

        assert_eq!(board.check_result(), GameResult::Win(Player::Y));
    }

    #[test]
    fn check_winner_6() {
        let mut board = Board::new();
        place(&mut board, (10, 10), Player::Y);
        place(&mut board, (12, 8), Player::Y);
        place(&mut board, (13, 7), Player::Y);
        place(&mut board, (12, 6), Player::Y);

        assert_eq!(board.check_result(), GameResult::InProgress);
    }

    #[test]
    fn empty_board_is_in_progress() {
        let board = Board::new();
        assert!(
            board
                .board()
                .iter()
                .flatten()
                .all(|cell| *cell == Cell::Empty)
        );
        assert_eq!(board.check_result(), GameResult::InProgress);
    }

    #[test]
    fn full_board_without_a_winner_is_a_draw() {
        assert_eq!(full_draw_board().check_result(), GameResult::Draw);
    }

    #[test]
    fn one_empty_cell_without_a_winner_is_in_progress() {
        let mut board = full_draw_board();
        board.board[BOARD_SIZE - 1][BOARD_SIZE - 1] = Cell::Empty;
        assert_eq!(board.check_result(), GameResult::InProgress);
    }

    #[test]
    fn win_takes_priority_over_a_full_board() {
        for player in [Player::X, Player::Y] {
            let mut board = full_draw_board();
            board.board[BOARD_SIZE - 1].fill(Cell::Taken(player));
            assert_eq!(board.check_result(), GameResult::Win(player));
        }
    }

    #[test]
    fn five_wins_at_every_valid_start_in_all_four_directions() {
        // Exhaustive positions include lines ending at either edge and all corners.
        for player in [Player::X, Player::Y] {
            for (dr, dc) in [(0isize, 1isize), (1, 0), (1, 1), (1, -1)] {
                for row in 0..BOARD_SIZE {
                    for col in 0..BOARD_SIZE {
                        let end_row = row as isize + 4 * dr;
                        let end_col = col as isize + 4 * dc;
                        if !(0..BOARD_SIZE as isize).contains(&end_row)
                            || !(0..BOARD_SIZE as isize).contains(&end_col)
                        {
                            continue;
                        }
                        let mut board = Board::new();
                        for i in 0..5 {
                            place(
                                &mut board,
                                (
                                    (row as isize + i * dr) as usize,
                                    (col as isize + i * dc) as usize,
                                ),
                                player,
                            );
                        }
                        assert_eq!(
                            board.check_result(),
                            GameResult::Win(player),
                            "start ({row}, {col}), direction ({dr}, {dc}), player {player:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn incomplete_broken_and_mixed_lines_do_not_win() {
        for player in [Player::X, Player::Y] {
            let other = match player {
                Player::X => Player::Y,
                Player::Y => Player::X,
            };
            for (dr, dc) in [(0isize, 1isize), (1, 0), (1, 1), (1, -1)] {
                // Four consecutive stones; five separated by a gap; five with an opponent.
                for line in [
                    vec![Some(player); 4],
                    vec![
                        Some(player),
                        Some(player),
                        None,
                        Some(player),
                        Some(player),
                        Some(player),
                    ],
                    vec![
                        Some(player),
                        Some(player),
                        Some(other),
                        Some(player),
                        Some(player),
                    ],
                ] {
                    let mut board = Board::new();
                    for (i, mark) in line.into_iter().enumerate() {
                        if let Some(mark) = mark {
                            place(
                                &mut board,
                                (
                                    (5 + i as isize * dr) as usize,
                                    (5 + i as isize * dc) as usize,
                                ),
                                mark,
                            );
                        }
                    }
                    assert_eq!(board.check_result(), GameResult::InProgress);
                }
            }
        }
    }

    #[test]
    fn joining_two_short_lines_into_six_is_a_win() {
        for player in [Player::X, Player::Y] {
            let mut board = Board::new();
            for col in [0, 1, 2, 4, 5] {
                place(&mut board, (7, col), player);
            }
            assert_eq!(board.check_result(), GameResult::InProgress);
            place(&mut board, (7, 3), player);
            assert_eq!(board.check_result(), GameResult::Win(player));
        }
    }

    #[test]
    fn isolated_stones_at_every_position_do_not_win_or_panic() {
        for player in [Player::X, Player::Y] {
            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    let mut board = Board::new();
                    place(&mut board, (row, col), player);
                    assert_eq!(board.check_result(), GameResult::InProgress);
                }
            }
        }
    }

    #[test]
    fn valid_move_changes_only_the_requested_cell() {
        for coor in [(0, 0), (7, 7), (BOARD_SIZE - 1, BOARD_SIZE - 1)] {
            let mut board = Board::new();
            place(&mut board, coor, Player::Y);
            for (row, cells) in board.board.iter().enumerate() {
                for (col, cell) in cells.iter().enumerate() {
                    let expected = if (row, col) == coor {
                        Cell::Taken(Player::Y)
                    } else {
                        Cell::Empty
                    };
                    assert_eq!(*cell, expected);
                }
            }
        }
    }

    #[test]
    fn out_of_bounds_moves_return_coordinate_errors_without_mutating() {
        let mut board = Board::new();
        place(&mut board, (2, 3), Player::X);
        let before = board.board();
        for coor in [
            (BOARD_SIZE, 0),
            (0, BOARD_SIZE),
            (BOARD_SIZE, BOARD_SIZE),
            (usize::MAX, 0),
            (0, usize::MAX),
        ] {
            assert!(matches!(
                board.apply_move_inplace(coor, Player::Y),
                Err(MoveError::CoorErr(_))
            ));
            assert_eq!(board.board(), before);
            assert!(matches!(
                board.apply_move(coor, Player::Y),
                Err(MoveError::CoorErr(_))
            ));
            assert_eq!(board.board(), before);
        }
    }

    #[test]
    fn occupied_cell_rejects_both_players_without_mutating() {
        let mut board = Board::new();
        place(&mut board, (2, 3), Player::X);
        let before = board.board();
        for player in [Player::X, Player::Y] {
            assert!(matches!(
                board.apply_move_inplace((2, 3), player),
                Err(MoveError::TakenErr(_))
            ));
            assert_eq!(board.board(), before);
            assert!(matches!(
                board.apply_move((2, 3), player),
                Err(MoveError::TakenErr(_))
            ));
            assert_eq!(board.board(), before);
        }
    }

    #[test]
    fn apply_move_returns_a_new_board_and_preserves_the_original() {
        let mut original = Board::new();
        place(&mut original, (0, 0), Player::X);
        let before = original.board();
        let updated = match original.apply_move((14, 14), Player::Y) {
            Ok(board) => board,
            Err(_) => panic!("valid move must succeed"),
        };
        let mut expected = before;
        expected[14][14] = Cell::Taken(Player::Y);
        assert_eq!(updated.board(), expected);
        assert_eq!(original.board(), before);
    }
}
