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
    FinishedGameErr(String),
}

/// Board object is responsible only for 'board'
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
    pub fn board(&self) -> &[[Cell; BOARD_SIZE]; BOARD_SIZE] {
        &self.board
    }

    pub fn available_empty_moves(&self) -> Vec<(usize, usize)> {
        let mut av_moves = Vec::new();
        for (r_idx, row) in self.board.iter().enumerate() {
            for (c_idx, cell) in row.iter().enumerate() {
                if *cell == Cell::Empty {
                    av_moves.push((r_idx, c_idx));
                }
            }
        }
        av_moves
    }
}

pub struct Game {
    board: Board,
    current_turn: Player,
    result: GameResult,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_turn: Player::X,
            result: GameResult::InProgress,
        }
    }
    pub fn play(&mut self, position: (usize, usize)) -> Result<(), MoveError> {
        if self.result != GameResult::InProgress {
            return Err(MoveError::FinishedGameErr(
                "Cannot play the game as it is over already!".to_string(),
            ));
        }
        match self.board.apply_move_inplace(position, self.current_turn) {
            Ok(_) => {
                match self.current_turn {
                    Player::Y => self.current_turn = Player::X,
                    Player::X => self.current_turn = Player::Y,
                };
                self.result = self.board.check_result();
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn turn(&self) -> Player {
        self.current_turn
    }
    pub fn result(&self) -> GameResult {
        self.result
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
        let before = *board.board();
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
            assert_eq!(*board.board(), before);
            assert!(matches!(
                board.apply_move(coor, Player::Y),
                Err(MoveError::CoorErr(_))
            ));
            assert_eq!(*board.board(), before);
        }
    }

    #[test]
    fn occupied_cell_rejects_both_players_without_mutating() {
        let mut board = Board::new();
        place(&mut board, (2, 3), Player::X);
        let before = *board.board();
        for player in [Player::X, Player::Y] {
            assert!(matches!(
                board.apply_move_inplace((2, 3), player),
                Err(MoveError::TakenErr(_))
            ));
            assert_eq!(*board.board(), before);
            assert!(matches!(
                board.apply_move((2, 3), player),
                Err(MoveError::TakenErr(_))
            ));
            assert_eq!(*board.board(), before);
        }
    }

    #[test]
    fn apply_move_returns_a_new_board_and_preserves_the_original() {
        let mut original = Board::new();
        place(&mut original, (0, 0), Player::X);
        let before = *original.board();
        let updated = match original.apply_move((14, 14), Player::Y) {
            Ok(board) => board,
            Err(_) => panic!("valid move must succeed"),
        };
        let mut expected = before;
        expected[14][14] = Cell::Taken(Player::Y);
        assert_eq!(*updated.board(), expected);
        assert_eq!(*original.board(), before);
    }

    fn winning_game(winner: Player) -> Game {
        let mut game = Game::new();
        // X's spare opening is isolated; the opponent's stones have gaps.
        if winner == Player::Y {
            assert!(game.play((14, 14)).is_ok());
        }
        for col in 0..5 {
            assert_eq!(game.turn(), winner);
            assert!(game.play((0, col)).is_ok());
            if col < 4 {
                assert_eq!(game.result(), GameResult::InProgress);
                assert!(game.play((2, col * 2)).is_ok());
                assert_eq!(game.result(), GameResult::InProgress);
            }
        }
        game
    }

    #[test]
    fn new_game_starts_with_x_on_an_empty_board() {
        let game = Game::new();
        assert_eq!(game.turn(), Player::X);
        assert_eq!(game.result(), GameResult::InProgress);
        assert!(
            game.board()
                .board()
                .iter()
                .flatten()
                .all(|cell| *cell == Cell::Empty)
        );
    }

    #[test]
    fn game_play_places_the_current_players_stone_and_alternates_turns() {
        let mut game = Game::new();
        for (position, player, next) in [
            ((0, 0), Player::X, Player::Y),
            ((7, 7), Player::Y, Player::X),
            ((14, 14), Player::X, Player::Y),
            ((0, 14), Player::Y, Player::X),
        ] {
            let mut expected = *game.board().board();
            expected[position.0][position.1] = Cell::Taken(player);
            assert!(game.play(position).is_ok());
            assert_eq!(*game.board().board(), expected);
            assert_eq!(game.turn(), next);
            assert_eq!(game.result(), GameResult::InProgress);
        }
    }

    #[test]
    fn game_rejected_moves_preserve_board_turn_and_result() {
        let mut game = Game::new();
        assert!(game.play((2, 3)).is_ok());
        let before = *game.board().board();
        let turn = game.turn();
        let result = game.result();
        assert!(matches!(game.play((2, 3)), Err(MoveError::TakenErr(_))));
        assert_eq!(*game.board().board(), before);
        assert_eq!(game.turn(), turn);
        assert_eq!(game.result(), result);
        for position in [(BOARD_SIZE, 0), (0, BOARD_SIZE), (usize::MAX, usize::MAX)] {
            assert!(matches!(game.play(position), Err(MoveError::CoorErr(_))));
            assert_eq!(*game.board().board(), before);
            assert_eq!(game.turn(), turn);
            assert_eq!(game.result(), result);
        }
        // The failed attempts must not consume Y's turn.
        assert!(game.play((2, 4)).is_ok());
        assert_eq!(game.board().board()[2][4], Cell::Taken(Player::Y));
        assert_eq!(game.turn(), Player::X);
    }

    #[test]
    fn game_records_a_win_for_either_player() {
        for player in [Player::X, Player::Y] {
            assert_eq!(winning_game(player).result(), GameResult::Win(player));
        }
    }

    #[test]
    fn game_rejects_further_moves_after_either_players_win() {
        for player in [Player::X, Player::Y] {
            let mut game = winning_game(player);
            let before = *game.board().board();
            let turn = game.turn();
            for position in [(7, 7), (0, 0), (usize::MAX, 0)] {
                assert!(matches!(
                    game.play(position),
                    Err(MoveError::FinishedGameErr(_))
                ));
                assert_eq!(*game.board().board(), before);
                assert_eq!(game.turn(), turn);
                assert_eq!(game.result(), GameResult::Win(player));
            }
        }
    }

    #[test]
    fn game_reaches_a_draw_through_alternating_moves_and_rejects_further_play() {
        let target = full_draw_board();
        let mut x_moves = Vec::new();
        let mut y_moves = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                match target.board()[row][col] {
                    Cell::Taken(Player::X) => x_moves.push((row, col)),
                    Cell::Taken(Player::Y) => y_moves.push((row, col)),
                    Cell::Empty => panic!("draw fixture must be full"),
                }
            }
        }
        assert_eq!(x_moves.len(), y_moves.len() + 1);
        let mut x_moves = x_moves.into_iter();
        let mut y_moves = y_moves.into_iter();
        let mut game = Game::new();
        for move_index in 0..BOARD_SIZE * BOARD_SIZE {
            assert_eq!(game.result(), GameResult::InProgress);
            let position = match game.turn() {
                Player::X => x_moves.next(),
                Player::Y => y_moves.next(),
            }
            .expect("each turn must have a remaining target cell");
            assert!(game.play(position).is_ok());
            if move_index + 1 < BOARD_SIZE * BOARD_SIZE {
                assert_eq!(game.result(), GameResult::InProgress);
            }
        }
        assert_eq!(game.board().board(), target.board());
        assert_eq!(game.result(), GameResult::Draw);
        let before = *game.board().board();
        let turn = game.turn();
        assert!(matches!(
            game.play((0, 0)),
            Err(MoveError::FinishedGameErr(_))
        ));
        assert_eq!(*game.board().board(), before);
        assert_eq!(game.turn(), turn);
        assert_eq!(game.result(), GameResult::Draw);
    }

    #[test]
    fn available_empty_moves_contains_every_coordinate_once_on_a_new_board() {
        let moves = Board::new().available_empty_moves();
        let unique: std::collections::HashSet<_> = moves.iter().copied().collect();
        assert_eq!(moves.len(), BOARD_SIZE * BOARD_SIZE);
        assert_eq!(unique.len(), moves.len());
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                assert!(unique.contains(&(row, col)));
            }
        }
    }

    #[test]
    fn available_empty_moves_excludes_occupied_cells_and_contains_only_playable_cells() {
        let mut board = Board::new();
        for (position, player) in [
            ((0, 0), Player::X),
            ((7, 7), Player::Y),
            ((14, 14), Player::X),
        ] {
            place(&mut board, position, player);
        }
        let before = *board.board();
        let moves = board.available_empty_moves();
        let unique: std::collections::HashSet<_> = moves.iter().copied().collect();
        assert_eq!(moves.len(), BOARD_SIZE * BOARD_SIZE - 3);
        assert_eq!(unique.len(), moves.len());
        for position in [(0, 0), (7, 7), (14, 14)] {
            assert!(!unique.contains(&position));
        }
        for position in moves {
            assert!(board.apply_move(position, Player::Y).is_ok());
        }
        assert_eq!(*board.board(), before);
    }

    #[test]
    fn available_empty_moves_is_empty_on_a_full_board() {
        assert!(full_draw_board().available_empty_moves().is_empty());
    }
}
