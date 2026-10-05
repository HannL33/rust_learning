#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Taken(Player),
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
    pub fn play(&mut self, position: usize) -> Result<(), MoveError> {
        if self.result != GameResult::InProgress {
            return Err(MoveError::CellError(
                "Cannot play the game as it is over already!".to_string(),
            ));
        }
        match apply_move(self.board, position, self.current_turn) {
            Ok(new_board) => {
                self.board = new_board;
                match self.current_turn {
                    Player::O => self.current_turn = Player::X,
                    Player::X => self.current_turn = Player::O,
                };
                self.result = check_winner(&self.board);
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

#[derive(Copy, Clone, Debug)]
pub struct Board {
    // indices, 0-based everywhere in this engine:
    // 0 1 2
    // 3 4 5
    // 6 7 8
    board: [Cell; 9],
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: [Cell::Empty; 9],
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell> {
        self.board.iter().copied()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}
impl Player {
    pub fn opposite(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveError {
    CellError(String),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameResult {
    Win(Player),
    Draw,
    InProgress,
}
const WINNING_CONFIGURATIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];
pub fn apply_move(mut board: Board, position: usize, player: Player) -> Result<Board, MoveError> {
    if position >= 9 {
        return Err(MoveError::CellError(
            "Out of bound position index!".to_string(),
        ));
    }
    if board.board[position] != Cell::Empty {
        return Err(MoveError::CellError(
            "The Cell is already taken!".to_string(),
        ));
    }
    board.board[position] = Cell::Taken(player);
    Ok(board)
}

pub fn check_winner(board: &Board) -> GameResult {
    for config in WINNING_CONFIGURATIONS {
        if config
            .iter()
            .all(|&x| board.board[x] == Cell::Taken(Player::O))
        {
            return GameResult::Win(Player::O);
        }
        if config
            .iter()
            .all(|&x| board.board[x] == Cell::Taken(Player::X))
        {
            return GameResult::Win(Player::X);
        }
    }
    if board.board.iter().any(|&x| x == Cell::Empty) {
        return GameResult::InProgress;
    }
    return GameResult::Draw;
}
pub fn available_moves(board: &Board) -> Option<Vec<usize>> {
    if check_winner(board) == GameResult::InProgress {
        Some(
            board
                .board
                .iter()
                .enumerate()
                .filter_map(|(i, cell)| (*cell == Cell::Empty).then_some(i))
                .collect(),
        )
    } else {
        None
    }
}

pub fn whos_turn(board: &Board) -> Player {
    let n_o = board
        .board
        .iter()
        .filter(|&x| *x == Cell::Taken(Player::O))
        .count();
    let n_x = board
        .board
        .iter()
        .filter(|&x| *x == Cell::Taken(Player::X))
        .count();
    if n_x > n_o { Player::O } else { Player::X }
}

// Tests //
// Written by AI
#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a board from `(position, player)` pairs, where positions are
    /// **0-based**, the convention the whole engine speaks.
    fn board_from(marks: &[(usize, Player)]) -> Board {
        marks
            .iter()
            .fold(Board::new(), |board, (position, player)| {
                apply_move(board, *position, *player).expect("test moves must be legal")
            })
    }

    fn cells_of(board: &Board) -> Vec<Cell> {
        board.cells().collect()
    }

    /// Unwraps the error of a rejected move as its message.
    ///
    /// This is the only place coupled to the current shape of `MoveError`, so
    /// reshaping the enum into `OutOfBounds` / `CellTaken` later touches one
    /// function instead of a dozen assertions.
    fn move_error(board: Board, position: usize, player: Player) -> String {
        match apply_move(board, position, player) {
            Ok(_) => panic!("expected a rejected move at position {position}"),
            Err(MoveError::CellError(message)) => message,
        }
    }

    /// `X O X / X O O / O X X` — nine cells, no line, five X and four O.
    fn full_draw_board() -> Board {
        board_from(&[
            (0, Player::X),
            (1, Player::O),
            (2, Player::X),
            (3, Player::X),
            (4, Player::O),
            (5, Player::O),
            (6, Player::O),
            (7, Player::X),
            (8, Player::X),
        ])
    }

    /// `X X X / O O X / O X O` — nine cells and X owns the top row, so a win has
    /// to win over the "board is full" check.
    fn full_board_with_a_line() -> Board {
        board_from(&[
            (0, Player::X),
            (1, Player::X),
            (2, Player::X),
            (3, Player::O),
            (4, Player::O),
            (5, Player::X),
            (6, Player::O),
            (7, Player::X),
            (8, Player::O),
        ])
    }

    // Board //

    #[test]
    fn new_board_has_nine_empty_cells() {
        assert_eq!(cells_of(&Board::new()), vec![Cell::Empty; 9]);
    }

    // apply_move //

    #[test]
    fn apply_move_places_the_player_in_the_requested_cell() {
        let board = apply_move(Board::new(), 4, Player::X).unwrap();
        assert_eq!(cells_of(&board)[4], Cell::Taken(Player::X));
    }

    #[test]
    fn apply_move_leaves_every_other_cell_untouched() {
        let board = apply_move(Board::new(), 4, Player::O).unwrap();
        assert_eq!(
            cells_of(&board),
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Taken(Player::O),
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ]
        );
    }

    #[test]
    fn apply_move_is_pure_so_the_caller_keeps_its_own_copy() {
        let original = apply_move(Board::new(), 0, Player::X).unwrap();
        let moved = apply_move(original, 1, Player::O).unwrap();
        assert_eq!(cells_of(&moved)[1], Cell::Taken(Player::O));
        assert_eq!(cells_of(&original)[1], Cell::Empty);
    }

    #[test]
    fn apply_move_accepts_the_first_and_the_last_cell_of_the_range() {
        let board = apply_move(Board::new(), 0, Player::X).unwrap();
        assert_eq!(cells_of(&board)[0], Cell::Taken(Player::X));
        let board = apply_move(board, 8, Player::O).unwrap();
        assert_eq!(cells_of(&board)[8], Cell::Taken(Player::O));
    }

    #[test]
    fn apply_move_rejects_position_nine() {
        assert!(move_error(Board::new(), 9, Player::X).contains("Out of bound"));
    }

    #[test]
    fn apply_move_rejects_absurd_position_without_panicking() {
        // the range guard has to run before the index is used
        assert!(move_error(Board::new(), usize::MAX, Player::X).contains("Out of bound"));
    }

    #[test]
    fn apply_move_rejects_an_occupied_cell() {
        let board = apply_move(Board::new(), 4, Player::X).unwrap();
        assert!(move_error(board, 4, Player::O).contains("already taken"));
    }

    #[test]
    fn the_two_rejection_reasons_carry_different_messages() {
        let empty = move_error(Board::new(), 9, Player::X);
        let occupied = move_error(board_from(&[(0, Player::X)]), 0, Player::O);
        assert_ne!(empty, occupied);
    }

    // check_winner //

    #[test]
    fn check_winner_finds_a_win_in_every_configuration_for_both_players() {
        for config in WINNING_CONFIGURATIONS {
            for player in [Player::X, Player::O] {
                let board = board_from(&[
                    (config[0], player),
                    (config[1], player),
                    (config[2], player),
                ]);
                assert_eq!(
                    check_winner(&board),
                    GameResult::Win(player),
                    "configuration {config:?} for {player:?}"
                );
            }
        }
    }

    #[test]
    fn check_winner_reports_in_progress_on_a_fresh_board() {
        assert_eq!(check_winner(&Board::new()), GameResult::InProgress);
    }

    #[test]
    fn check_winner_ignores_a_line_of_mixed_players() {
        let board = board_from(&[(0, Player::X), (1, Player::O), (2, Player::X)]);
        assert_eq!(check_winner(&board), GameResult::InProgress);
    }

    #[test]
    fn check_winner_reports_a_win_before_it_reports_in_progress() {
        let board = board_from(&[
            (0, Player::X),
            (1, Player::X),
            (2, Player::X),
            (3, Player::O),
        ]);
        assert_eq!(check_winner(&board), GameResult::Win(Player::X));
    }

    #[test]
    fn check_winner_reports_a_win_before_it_reports_a_draw() {
        assert_eq!(
            check_winner(&full_board_with_a_line()),
            GameResult::Win(Player::X)
        );
    }

    #[test]
    fn check_winner_reports_a_draw_on_a_full_board_without_a_line() {
        assert_eq!(check_winner(&full_draw_board()), GameResult::Draw);
    }

    // available_moves //

    #[test]
    fn available_moves_lists_every_cell_of_an_empty_board() {
        assert_eq!(
            available_moves(&Board::new()),
            Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    /// `available_moves` and `apply_move` share one convention, so what the first
    /// hands out goes straight back into the second with no shifting at all.
    #[test]
    fn available_moves_feeds_apply_move_without_any_shifting() {
        let board = board_from(&[(0, Player::X), (4, Player::O)]);
        let moves = available_moves(&board).unwrap();
        assert_eq!(moves, vec![1, 2, 3, 5, 6, 7, 8]);
        for position in moves {
            assert!(apply_move(board, position, Player::X).is_ok());
        }
    }

    #[test]
    fn available_moves_is_none_once_somebody_won() {
        let board = board_from(&[(0, Player::X), (1, Player::X), (2, Player::X)]);
        assert_eq!(available_moves(&board), None);
    }

    #[test]
    fn available_moves_is_none_after_a_draw() {
        assert_eq!(available_moves(&full_draw_board()), None);
    }

    #[test]
    fn available_moves_and_check_winner_agree_on_when_the_game_is_over() {
        let boards = [
            Board::new(),
            board_from(&[(0, Player::X), (1, Player::O)]),
            board_from(&[(0, Player::X), (1, Player::X), (2, Player::X)]),
            full_draw_board(),
            full_board_with_a_line(),
        ];
        for board in boards {
            let over = check_winner(&board) != GameResult::InProgress;
            assert_eq!(
                available_moves(&board).is_none(),
                over,
                "board: {:?}",
                cells_of(&board)
            );
        }
    }

    // whos_turn //

    #[test]
    fn whos_turn_starts_with_x() {
        assert_eq!(whos_turn(&Board::new()), Player::X);
    }

    #[test]
    fn whos_turn_alternates_after_every_legal_move() {
        let mut board = Board::new();
        assert_eq!(whos_turn(&board), Player::X);
        board = apply_move(board, 0, whos_turn(&board)).unwrap();
        assert_eq!(whos_turn(&board), Player::O);
        board = apply_move(board, 1, whos_turn(&board)).unwrap();
        assert_eq!(whos_turn(&board), Player::X);
    }

    #[test]
    fn whos_turn_follows_the_move_counts() {
        let board = board_from(&[
            (0, Player::X),
            (1, Player::O),
            (2, Player::X),
            (3, Player::O),
            (4, Player::X),
        ]);
        assert_eq!(whos_turn(&board), Player::O);
    }

    // whole game //

    #[test]
    fn game_play_places_marks_and_alternates_turns() {
        let mut game = Game::new();
        assert_eq!(game.turn(), Player::X);
        assert_eq!(game.result(), GameResult::InProgress);

        game.play(0).unwrap();
        assert_eq!(cells_of(game.board())[0], Cell::Taken(Player::X));
        assert_eq!(game.turn(), Player::O);

        game.play(4).unwrap();
        assert_eq!(cells_of(game.board())[4], Cell::Taken(Player::O));
        assert_eq!(game.turn(), Player::X);
        assert_eq!(game.result(), GameResult::InProgress);
    }

    #[test]
    fn game_rejected_moves_leave_all_state_unchanged() {
        let mut game = Game::new();
        game.play(0).unwrap();
        let before = cells_of(game.board());
        let turn = game.turn();
        let result = game.result();

        for position in [0, 9, usize::MAX] {
            assert!(game.play(position).is_err());
            assert_eq!(cells_of(game.board()), before);
            assert_eq!(game.turn(), turn);
            assert_eq!(game.result(), result);
        }
    }

    #[test]
    fn game_records_a_win_and_rejects_further_moves() {
        let mut game = Game::new();
        for position in [0, 3, 1, 4, 2] {
            game.play(position).unwrap();
        }
        assert_eq!(game.result(), GameResult::Win(Player::X));
        let before = cells_of(game.board());
        let turn = game.turn();

        assert!(game.play(8).is_err());
        assert_eq!(cells_of(game.board()), before);
        assert_eq!(game.turn(), turn);
        assert_eq!(game.result(), GameResult::Win(Player::X));
    }

    #[test]
    fn game_records_a_draw_and_rejects_further_moves() {
        let mut game = Game::new();
        for position in [0, 1, 2, 4, 3, 5, 7, 6, 8] {
            game.play(position).unwrap();
        }
        assert_eq!(game.result(), GameResult::Draw);
        let before = cells_of(game.board());
        let turn = game.turn();

        assert!(game.play(0).is_err());
        assert_eq!(cells_of(game.board()), before);
        assert_eq!(game.turn(), turn);
        assert_eq!(game.result(), GameResult::Draw);
    }

    #[test]
    fn a_played_out_game_ends_in_a_win_and_offers_no_further_moves() {
        let board = board_from(&[
            (0, Player::X),
            (5, Player::O),
            (1, Player::X),
            (4, Player::O),
            (2, Player::X),
        ]);
        assert_eq!(check_winner(&board), GameResult::Win(Player::X));
        assert_eq!(available_moves(&board), None);
    }
}
