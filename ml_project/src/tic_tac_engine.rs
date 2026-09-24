#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Taken(Player),
}

#[derive(Copy, Clone)]
pub struct Board {
    board: [Cell; 9],
    // positions
    // 1 2 3
    // 4 5 6
    // 7 8 9

    // indices
    // 0 1 2
    // 3 4 5
    // 6 7 8
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

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

pub enum MoveError {
    CellError(String),
}

#[derive(Copy, Clone, PartialEq)]
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
    if position > 9 || position < 1 {
        return Err(MoveError::CellError(
            "Out of bound position index!".to_string(),
        ));
    }
    if board.board[position - 1] != Cell::Empty {
        return Err(MoveError::CellError(
            "The Cell is already taken!".to_string(),
        ));
    }
    board.board[position - 1] = Cell::Taken(player);
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
