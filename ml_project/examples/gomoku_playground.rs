use ml_project::gomoku_engine as ge;

fn main() {
    let my_board = ge::Board::new();
    println!("My board: {:?}", my_board.cells());
}
