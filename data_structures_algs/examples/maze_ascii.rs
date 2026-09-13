use data_structures_algs::graphs::Graph;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
#[derive(Debug)]
struct Maze {
    lines: Vec<Line>,
    cell_size: usize,
    n_maze: usize,
}
impl Maze {
    fn new(lines: Vec<Line>) -> Maze {
        let mut maze = Maze {
            lines: lines,
            cell_size: 0,
            n_maze: 0,
        };
        // better would be to check for minimum |x2-x1| + |y2-y1|
        // TODO
        let mut unique_x1: Vec<usize> =
            maze.lines.iter().filter_map(|line| Some(line.x1)).collect();
        unique_x1.sort();
        unique_x1.dedup();
        maze.cell_size = unique_x1[1] - unique_x1[0];
        maze.n_maze = (unique_x1[unique_x1.len() - 1] - unique_x1[0]) / maze.cell_size;
        maze
    }
}

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
    fn parse_graph_into_grid(&mut self, graph: &Graph) {
        for row_idx in 1..(self.elements.len()) - 1 {
            for col_idx in 1..(self.elements[row_idx].len()) - 1 {
                if !row_idx.is_multiple_of(2) {
                    if col_idx.is_multiple_of(2)
                        && col_idx - 1 > 0
                        && col_idx + 1 < self.elements.len()
                    {
                        let left_box = from_grid_calc_graph_idx(row_idx, col_idx - 1, self.length);
                        let right_box = from_grid_calc_graph_idx(row_idx, col_idx + 1, self.length);
                        if !graph.has_edge(left_box, right_box) {
                            self.elements[row_idx][col_idx] = '|';
                            println!(
                                "For coordinates: {}, {}, left box graph id: {}, right box graph id: {}. Putted |",
                                row_idx, col_idx, left_box, right_box
                            );
                        }
                    }
                } else {
                    if !col_idx.is_multiple_of(2)
                        && row_idx - 1 > 0
                        && row_idx + 1 < self.elements.len()
                    {
                        let up_box = from_grid_calc_graph_idx(row_idx - 1, col_idx, self.length);
                        let down_box = from_grid_calc_graph_idx(row_idx + 1, col_idx, self.length);
                        if !graph.has_edge(up_box, down_box) {
                            self.elements[row_idx][col_idx] = '-';
                        }
                    }
                }
            }
        }
    }
    fn add_shortest_path(&mut self, vec_nodes: &Option<Vec<usize>>) {
        if let Some(unp_vec) = vec_nodes {
            for &node in unp_vec {
                let (mut x_cor, mut y_cor) = div_rem(node - 1, self.length);

                self.elements[2 * x_cor + 1][2 * y_cor + 1] = '+'
            }
        }
    }
}

fn div_rem(dividend: usize, divisor: usize) -> (usize, usize) {
    (dividend / divisor, dividend % divisor)
}
fn from_grid_calc_graph_idx(x: usize, y: usize, len: usize) -> usize {
    let mut res_x: usize;
    let res_y: usize;
    (res_x, res_y) = ((x + 1) / 2, (y + 1) / 2);
    res_x -= 1;
    // (res_x, res_y) = (res_y, res_x);
    res_x * len + res_y
}

fn main() {
    let mut temp_square = SquareGrid::new(10);
    temp_square.print();

    let mut test_graph = Graph::new(false);
    let edges = vec![
        (5, 4),
        (5, 15),
        (5, 4), // start from 5
        (4, 3),
        (3, 13),
        (13, 23),
        (23, 24),
        (23, 33),
        (24, 25),
        (33, 32),
        (32, 31),
        (31, 21),
        (21, 22),
        (22, 12),
        (12, 11),
        (11, 1),
        (1, 2),
        (15, 14),
        (15, 16),
        (16, 6),
        (6, 7),
        (7, 17),
        (16, 26),
        (26, 27),
        (27, 37),
        (37, 36),
        (27, 28),
        (28, 18),
        (18, 19),
        (18, 8),
        (8, 9),
        (9, 10),
        (10, 20),
        (20, 30),
        (30, 40),
        (40, 50),
        (50, 60),
        (60, 59),
        (59, 49),
        (19, 29),
        (29, 39),
        (39, 38),
        (38, 48),
        (48, 58),
        (58, 68),
        (68, 67),
        (67, 77),
        (77, 78),
        (78, 88),
        (88, 98),
        (88, 89),
        (89, 99),
        (99, 100),
        (89, 90),
        (90, 80),
        (80, 70),
        (70, 69),
        (69, 79),
        (58, 57),
        (57, 47),
        (47, 46),
        (46, 45),
        (45, 35),
        (35, 34),
        (34, 44),
        (44, 43),
        (43, 42),
        (42, 41),
        (41, 51),
        (51, 52),
        (52, 62),
        (62, 61),
        (62, 63),
        (63, 53),
        (53, 54),
        (54, 55),
        (55, 56),
        (56, 66),
        (66, 76),
        (76, 86),
        (86, 87),
        (87, 97),
        (97, 96), // end in 96
        (63, 64),
        (64, 74),
        (74, 75),
        (75, 65),
        (75, 85),
        (85, 95),
        (95, 94),
        (94, 84),
        (84, 83),
        (83, 73),
        (73, 72),
        (72, 71),
        (71, 81),
        (81, 82),
        (81, 91),
        (91, 92),
        (92, 93),
    ];
    for (from, to) in edges {
        test_graph.add_edge(from, to, 1);
    }

    // println!("Grid: (1, 3) -> {}", from_grid_calc_graph_idx(1, 3, 10));
    // println!("Grid: (1, 5) -> {}", from_grid_calc_graph_idx(1, 5, 10));
    temp_square.parse_graph_into_grid(&test_graph);
    temp_square.print();
    let shortest_path = test_graph.shortest_path_unweighted(5, 96);
    temp_square.add_shortest_path(&shortest_path);
    temp_square.print();

    // encoding the svg
    let contents =
        fs::read_to_string("data/10x10_maze.svg").expect("Problem with reading the file!");

    //println!("Content of the string: {:?}", contents);
    let mut extracted_lines: Vec<Line> = Vec::new();
    let re = Regex::new(r#"x1=\"(\d+)\"\sy1=\"(\d+)\"\sx2=\"(\d+)\"\sy2=\"(\d+)\""#)
        .expect("Something wrong with regex");
    for (_, [x1, y1, x2, y2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        extracted_lines.push(Line {
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap(),
        });
    }
    println!("Extracted lines: {:?}", extracted_lines);
    let mut maze = Maze::new(extracted_lines);
    println!("Maze: {:?}", maze);
}
