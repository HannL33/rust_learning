use data_structures_algs::graphs;

#[derive(Debug)]
struct SquareGrid {
    length: usize,
    elements: Vec<Vec<char>>,
}
impl SquareGrid {
    fn new(len: usize) -> SquareGrid {
        let grid_len: usize = 2 * len + 1;
        let mut square_grid: Vec<Vec<char>> = vec![vec![' '; grid_len]; grid_len];
        for row in 0..grid_len {
            for col in 0..grid_len {
                if (row == 0 || row == grid_len - 1) || (col == 0 || col == grid_len - 1) {
                    square_grid[row][col] = '\u{25A0}';
                }
            }
        }
        SquareGrid {
            length: len,
            elements: square_grid,
        }
    }

    fn print(&self) {
        for row_idx in 0..(2 * self.length + 1) {
            for col_idx in 0..(2 * self.length + 1) {
                if col_idx == 1 || col_idx == 2 * self.length {
                    print!(" ")
                };
                print!("{0}{0}", self.elements[row_idx][col_idx]);
            }
            println!("");
        }
    }
}

fn main() {
    let temp_square = SquareGrid::new(10);
    temp_square.print();
}
