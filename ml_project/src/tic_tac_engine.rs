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
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
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
    if board.board.contains(&Cell::Empty) {
        return GameResult::InProgress;
    }
    GameResult::Draw
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
mod tests;
