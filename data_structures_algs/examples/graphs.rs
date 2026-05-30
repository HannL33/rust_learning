use std::collections::HashMap;

struct Graph {
    adj: HashMap<usize, Vec<(usize, u32)>>, // (neighbour, weight)
    directed: bool,
}

impl Graph {
    fn new(directed: bool) -> Self {
        Self {
            adj: HashMap::new(),
            directed: directed,
        }
    }

    fn add_node(&mut self, id: usize) {
        self.adj.entry(id).or_insert(Vec::new());
    }
    fn add_edge(&mut self, from: usize, to: usize, weight: u32) {
        self.adj.entry(from).or_default().push((to, weight));
        if !self.directed {
            self.adj.entry(to).or_default().push((from, weight));
        }
    }
    fn neighbors(&self, node: usize) -> Option<&[(usize, u32)]> {
        self.adj.get(&node).map(Vec::as_slice) // maps vec into slice inside option type
    }
    fn has_edge(&self, from: usize, to: usize) -> bool {
        match self.adj.get(&from) {
            Some(vec) => vec.iter().any(|v| v.0 == to),
            None => false,
        }
    }
    fn node_count(&self) -> usize {
        self.adj.len()
    }
    fn edge_count(&self) -> usize {
        let _count: usize = self.adj.values().map(|v| v.len()).sum();
        // if is not directed we need not count duplicates, so divide 2
        if self.directed { _count } else { _count / 2 }
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
