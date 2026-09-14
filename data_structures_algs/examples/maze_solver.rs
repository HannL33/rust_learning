use data_structures_algs::graphs::Graph;
use regex::Regex;
use std::env;
use std::fs;
use std::process::exit;

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
    offset: usize,
    // content of the svg, stored as we would not like to read a file twice
    contents: String,
}
impl Maze {
    fn new(lines: Vec<Line>, contents: String) -> Maze {
        let mut maze = Maze {
            lines,
            cell_size: 0,
            n_maze: 0,
            offset: 0,
            contents,
        };
        let mut unique_x1: Vec<usize> = maze.lines.iter().map(|line| line.x1).collect();
        let minimum_distance: Vec<usize> = maze
            .lines
            .iter()
            .map(|line| line.x2.abs_diff(line.x1) + line.y2.abs_diff(line.y1))
            .collect();
        unique_x1.sort();
        unique_x1.dedup();
        maze.cell_size = *minimum_distance
            .iter()
            .min()
            .expect("Could not calculate cell size!");
        maze.n_maze = (unique_x1[unique_x1.len() - 1] - unique_x1[0]) / maze.cell_size;
        maze.offset = *unique_x1.iter().min().expect("No minimum in unique x1");
        maze
    }
    fn new_from_path(path: &str) -> Maze {
        let contents = fs::read_to_string(path).expect("Problem with reading the file!");

        let mut extracted_lines: Vec<Line> = Vec::new();
        let re = Regex::new(r#"x1=\"(\d+)\"\sy1=\"(\d+)\"\sx2=\"(\d+)\"\sy2=\"(\d+)\""#)
            .expect("Something wrong with regex");
        for (_, [x1, y1, x2, y2]) in re.captures_iter(&contents).map(|c| c.extract()) {
            extracted_lines.push(Line {
                x1: x1.parse().expect("x1 should be a valid number"),
                y1: y1.parse().expect("y1 should be a valid number"),
                x2: x2.parse().expect("x2 should be a valid number"),
                y2: y2.parse().expect("y2 should be a valid number"),
            });
        }
        Maze::new(extracted_lines, contents)
    }
    fn parse_into_graph(&self) -> (Graph, usize, usize) {
        let mut graph = Graph::new(false);
        let mut entry_id = None;
        let mut exit_id = None;
        for r in 0..self.n_maze {
            for c in 0..self.n_maze {
                // id in graph of the cell
                let curr_id = r * self.n_maze + c + 1;
                // check for upper entry
                if r == 0 {
                    let y_up = self.offset;
                    let x_up = self.offset + c * self.cell_size;
                    let x_up_next = self.offset + (c + 1) * self.cell_size;
                    if !(self
                        .lines
                        .iter()
                        .any(|line| line.y1 == y_up && line.x1 <= x_up && line.x2 >= x_up_next))
                    {
                        entry_id = Some(curr_id);
                        println!("Found upper entry: {}", curr_id);
                    }
                }
                // check for bottom exit
                if r == self.n_maze - 1 {
                    let y_down = self.offset + self.n_maze * self.cell_size;
                    let x_down = self.offset + c * self.cell_size;
                    let x_down_next = self.offset + (c + 1) * self.cell_size;
                    if !(self.lines.iter().any(|line| {
                        line.y1 == y_down && line.x1 <= x_down && line.x2 >= x_down_next
                    })) {
                        exit_id = Some(curr_id);
                        println!("Found lower exit: {}", curr_id);
                    }
                }
                // check if the right neighbor exist (r, c+1)
                let x_from = self.offset + (c + 1) * self.cell_size;
                let y_from = self.offset + r * self.cell_size;
                let y_to = self.offset + (r + 1) * self.cell_size;

                if !(self
                    .lines
                    .iter()
                    .any(|line| line.x1 == x_from && line.y1 <= y_from && line.y2 >= y_to)
                    || c + 1 >= self.n_maze)
                {
                    graph.add_edge(curr_id, r * self.n_maze + c + 2, 1);
                }
                // check if the bot neighbor exist (r+1, c)
                let y_from = self.offset + (r + 1) * self.cell_size;
                let x_from = self.offset + c * self.cell_size;
                let x_to = self.offset + (c + 1) * self.cell_size;
                if !(self
                    .lines
                    .iter()
                    .any(|line| line.y1 == y_from && line.x1 <= x_from && line.x2 >= x_to)
                    || r + 1 >= self.n_maze)
                {
                    graph.add_edge(curr_id, (r + 1) * self.n_maze + c + 1, 1);
                }
            }
        }
        (
            graph,
            entry_id.expect("No entrance found in maze"),
            exit_id.expect("No exit found in maze"),
        )
    }
    fn solve_graph_and_save(&self, path_solved_exit: &str) {
        let (graph, entry_id, exit_id) = self.parse_into_graph();
        let solved_path = graph.shortest_path_unweighted(entry_id, exit_id);
        match solved_path {
            Some(solved_path_unp) => {
                let mut lines_vec: Vec<(usize, usize)> = Vec::new();
                for node in solved_path_unp {
                    let (r, c) = div_rem(node - 1, self.n_maze);
                    let x_mid = self.offset + c * self.cell_size + self.cell_size / 2;
                    let y_mid = self.offset + r * self.cell_size + self.cell_size / 2;
                    lines_vec.push((x_mid, y_mid));
                }
                let index_svg = self
                    .contents
                    .find("</svg>")
                    .expect("Cannot find placeholder /svg. Probably corrupted file!");

                let result: String = lines_vec.iter().map(|(x, y)| format!("{x},{y} ")).collect();
                let polyline = "<polyline points=\"".to_string()
                    + result.as_str()
                    + "\" fill=\"none\" stroke=\"red\" stroke-width=\"2\" />";
                let new_svg = self.contents[..index_svg].to_string()
                    + &polyline
                    + &self.contents[index_svg..];
                fs::write(path_solved_exit, new_svg)
                    .expect("Something wrong happened during saving the solved maze!");
            }
            None => println!("Could not find the path in this graph. Nothing to save!"),
        }
    }
}

fn div_rem(dividend: usize, divisor: usize) -> (usize, usize) {
    (dividend / divisor, dividend % divisor)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: maze <input_path.svg> <output_path.svg>");
        exit(1);
    }
    let path = &args[1];
    let path_solved = &args[2];

    let maze = Maze::new_from_path(path);
    maze.solve_graph_and_save(path_solved);
}
