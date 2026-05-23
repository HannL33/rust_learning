use std::collections::HashMap;

struct Graph {
    adj: HashMap<usize, Vec<(usize, u32)>>, // (neighbour, weight)
    directed: bool,
}

impl Graph {
    fn new(directed: bool) -> Self {
        todo!();
    }

    fn add_node(&mut self, id: usize) {
        todo!();
    }
    fn add_edge(&mut self, from: usize, to: usize, weight: u32) {
        todo!()
    }
    fn neighbors(&self, node: usize) -> &[(usize, u32)] {
        todo!();
    }
    fn has_edge(&self, from: usize, to: usize) -> bool {
        todo!();
    }
    fn node_count(&self) -> usize {
        todo!();
    }
    fn edge_count(&self) -> usize {
        todo!();
    }
}
impl Graph {
    // ALGS
    //
    // BFS
    fn bfs(&self, start: usize) -> Vec<usize> {
        todo!();
    } // order for traversal

    fn shortest_path_unweighted(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        todo!();
    }
    fn is_connected(&self) -> bool {
        todo!();
    }
    // DFS
    fn dfs_iterative(&self, start: usize) -> Vec<usize> {
        todo!();
    }
    fn dfs_recursive(&self, start: usize) {
        todo!();
    }
    fn has_cycle(&self) -> bool {
        todo!();
    }
    fn topological_sort(&self) -> Option<Vec<usize>> {
        todo!();
    }

    // Dijkstra
    fn dijkstra(&self, start: usize) -> HashMap<usize, u32> {
        todo!();
    }
    fn shortest_path_weighted(&self, start: usize, end: usize) -> Option<(u32, Vec<usize>)> {
        todo!();
    }
}
fn main() {}
