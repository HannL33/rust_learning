use itertools::Itertools;
/// Number of rows and columns on the Gomoku board.
pub const BOARD_SIZE: usize = 15;

pub type Position = (usize, usize);
pub type Direction = (isize, isize);

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MoveError {
    OutOfBounds { row: usize, column: usize },
    CellTaken,
    GameFinished,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds { row, column } => {
                write!(
                    f,
                    "Coordinates ({row}, {column}) are outside the {BOARD_SIZE} × {BOARD_SIZE} board"
                )
            }
            Self::CellTaken => write!(f, "The cell is already taken"),
            Self::GameFinished => write!(f, "The game has already finished"),
        }
    }
}

impl std::error::Error for MoveError {}

/// All coordinates are zero-based `(row, column)` pairs.
#[derive(Copy, Clone, Debug)]
pub struct Board {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }
    /// Places a stone at a zero-based `(row, column)` position.
    /// An invalid move leaves the board unchanged.
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
                Err(MoveError::CellTaken)
            }
        } else {
            Err(MoveError::OutOfBounds {
                row: coor.0,
                column: coor.1,
            })
        }
    }
    /// Returns the board object after placing a stone at `(row, column)`.
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
                    if r_idx + 5 <= BOARD_SIZE
                        && c_idx + 5 <= BOARD_SIZE
                        && (0..5).all(|i| self.board[r_idx + i][c_idx + i] == Cell::Taken(player))
                    {
                        return GameResult::Win(player);
                    }
                    // check diagonal from right to left
                    if r_idx + 5 <= BOARD_SIZE
                        && c_idx >= 4
                        && (0..5).all(|i| self.board[r_idx + i][c_idx - i] == Cell::Taken(player))
                    {
                        return GameResult::Win(player);
                    }
                };
            }
        }
        if self.board.iter().flatten().any(|&x| x == Cell::Empty) {
            return GameResult::InProgress;
        }
        GameResult::Draw
    }
    /// Read-only cells indexed as `cells[row][column]`.
    pub fn cells(&self) -> &[[Cell; BOARD_SIZE]; BOARD_SIZE] {
        &self.board
    }

    /// Returns all empty `(row, column)` positions, even after a win.
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
    pub fn available_empty_moves_close(&self, range: usize) -> Vec<Position> {
        let mut av_moves_close = Vec::new();
        for position in self.available_empty_moves() {
            if self.is_position_near_other_stones(position, range) {
                av_moves_close.push(position);
            }
        }
        av_moves_close
    }
    fn is_position_near_other_stones(&self, position: Position, range: usize) -> bool {
        let constraint = self.size() as i32 - 1;
        let r = range.min(constraint as usize) as i32;

        for (dx, dy) in (-r..=r).cartesian_product(-r..=r) {
            if dx == 0 && dy == 0 {
                continue;
            }

            let x = position.0 as i32 + dx;
            let y = position.1 as i32 + dy;

            if (0..=constraint).contains(&x) && (0..=constraint).contains(&y) {
                if self.board[x as usize][y as usize] != Cell::Empty {
                    return true;
                }
            }
        }
        false
    }
    pub fn win_in_one(&self, player: Player) -> Vec<Position> {
        let mut vec_positions: Vec<Position> = Vec::new();
        let possible_directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (-1, 1)];
        for (r_idx, row) in self.cells().iter().enumerate() {
            for (c_idx, &cell) in row.iter().enumerate() {
                if cell == Cell::Empty {
                    for (dr, dc) in possible_directions {
                        let mut n_stones_in_direction = 0;
                        for k in 1..=4 {
                            let Some(new_r_idx) = r_idx.checked_add_signed(dr * k) else {
                                break;
                            };
                            let Some(new_c_idx) = c_idx.checked_add_signed(dc * k) else {
                                break;
                            };
                            if new_r_idx >= self.size() || new_c_idx >= self.size() {
                                break;
                            }

                            if self.board[new_r_idx][new_c_idx] == Cell::Taken(player) {
                                n_stones_in_direction += 1;
                            } else {
                                break;
                            }
                        }
                        for k in (-4..=-1).rev() {
                            let Some(new_r_idx) = r_idx.checked_add_signed(dr * k) else {
                                break;
                            };
                            let Some(new_c_idx) = c_idx.checked_add_signed(dc * k) else {
                                break;
                            };
                            // println!("New r idx, new c_idx: {} {}", new_r_idx, new_c_idx);
                            if new_r_idx >= self.size() || new_c_idx >= self.size() {
                                break;
                            }
                            if self.board[new_r_idx][new_c_idx] == Cell::Taken(player) {
                                n_stones_in_direction += 1;
                            } else {
                                break;
                            }
                        }
                        if n_stones_in_direction >= 4 {
                            vec_positions.push((r_idx, c_idx));
                            break;
                        }
                    }
                }
            }
        }
        vec_positions
    }
    pub fn size(&self) -> usize {
        self.cells().len()
    }
}

/// Enforces alternating turns, legal moves and the end of a Gomoku game.
pub struct Game {
    board: Board,
    current_turn: Player,
    result: GameResult,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_turn: Player::X,
            result: GameResult::InProgress,
        }
    }
    /// Plays at a zero-based `(row, column)` position for the current player.
    /// Rejected moves leave the board, turn and result unchanged.
    pub fn play(&mut self, position: (usize, usize)) -> Result<(), MoveError> {
        if self.result != GameResult::InProgress {
            return Err(MoveError::GameFinished);
        }
        match self.board.apply_move_inplace(position, self.current_turn) {
            Ok(_) => {
                self.current_turn = self.current_turn.opposite();
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

    /// Returns all legal `(row, column)` moves, or an empty list after completion.
    pub fn legal_moves(&self) -> Vec<(usize, usize)> {
        if self.result == GameResult::InProgress {
            self.board.available_empty_moves()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests;
