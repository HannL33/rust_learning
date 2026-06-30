use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adj: HashMap<usize, Vec<(usize, u32)>>, // (neighbour, weight)
    directed: bool,
}

impl Graph {
    fn new(directed: bool) -> Self {
        Self {
            adj: HashMap::new(),
            directed,
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
        self.adj.entry(to).or_default();
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
        let count: usize = self.adj.values().map(|v| v.len()).sum();
        // if is not directed we need not count duplicates, so divide 2
        if self.directed { count } else { count / 2 }
    }
}
impl Graph {
    // ALGS
    //
    // BFS
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut waiting: VecDeque<usize> = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut result = Vec::new();

        visited.insert(start);
        waiting.push_back(start);
        // let node = waiting.pop_front();
        while let Some(node) = waiting.pop_front() {
            result.push(node);
            if let Some(neis) = self.neighbors(node) {
                for &nei in neis {
                    if !visited.contains(&nei.0) {
                        visited.insert(nei.0);
                        waiting.push_back(nei.0);
                    }
                }
            }
        } // order for traversal
        return result;
    }

    fn shortest_path_unweighted(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        if start == end {
            return Some(vec![start]);
        }
        let mut waiting: VecDeque<usize> = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut prev: HashMap<usize, usize> = HashMap::new();
        waiting.push_back(start);
        visited.insert(start);

        while let Some(node) = waiting.pop_front() {
            if let Some(neis) = self.neighbors(node) {
                for &nei in neis {
                    if !visited.contains(&nei.0) {
                        prev.insert(nei.0, node);
                        if nei.0 == end {
                            let mut res: Vec<usize> = Vec::new();
                            let mut current_node: usize = end;
                            while current_node != start {
                                res.push(current_node);
                                current_node = *prev.get(&current_node).expect("Every node on the path, besides the start, has a parent in prev!");
                            }
                            res.push(start);
                            res.reverse();
                            return Some(res);
                        }
                        visited.insert(nei.0);
                        waiting.push_back(nei.0);
                    }
                }
            }
        }

        return None;
    }
    fn is_connected(&self) -> bool {
        // weak connection checking
        // for the directed graph
        // because if we wanna prove the strong connection, then need to implement kosaraju alg

        // take whatever node
        if let Some(node) = self.adj.keys().next() {
            if self.bfs(*node).len() == self.node_count() {
                return true;
            }
        }
        return false;
    }

    // DFS
    fn dfs_iterative(&self, start: usize) -> Vec<usize> {
        let mut stack: Vec<usize> = vec![start];
        let mut visited = HashSet::new();
        let mut result: Vec<usize> = Vec::new();

        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            result.push(node);

            if let Some(neis) = self.neighbors(node) {
                for nei in neis {
                    if !visited.contains(&nei.0) {
                        stack.push(nei.0);
                    }
                }
            }
        }
        result
    }
    fn _dfs_recursive_helper(
        &self,
        start: usize,
        visited: &mut HashSet<usize>,
        result: &mut Vec<usize>,
    ) {
        visited.insert(start);
        result.push(start);
        if let Some(neis) = self.neighbors(start) {
            for nei in neis {
                if !visited.contains(&nei.0) {
                    self._dfs_recursive_helper(nei.0, visited, result);
                }
            }
        }
    }

    fn dfs_recursive(&self, start: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();

        self._dfs_recursive_helper(start, &mut visited, &mut result);

        result
    }

    fn _cycle_helper_undirected(
        &self,
        node: usize,
        parent: Option<usize>,
        visited: &mut HashSet<usize>,
    ) -> bool {
        visited.insert(node);

        if let Some(neis) = self.neighbors(node) {
            for nei in neis {
                if !visited.contains(&nei.0) {
                    if self._cycle_helper_undirected(nei.0, Some(node), visited) {
                        return true;
                    }
                } else {
                    if Some(nei.0) != parent {
                        return true;
                    }
                }
            }
            return false;
        }
        return false;
    }
    fn _cycle_helper_directed(
        &self,
        node: usize,
        in_progress: &mut HashSet<usize>,
        visited: &mut HashSet<usize>,
    ) -> bool {
        in_progress.insert(node);

        if let Some(neis) = self.neighbors(node) {
            for nei in neis {
                if in_progress.contains(&nei.0) {
                    return true;
                }
                if !visited.contains(&nei.0) {
                    if self._cycle_helper_directed(nei.0, in_progress, visited) {
                        return true;
                    }
                }
            }
        }

        //exit
        in_progress.remove(&node);
        visited.insert(node);

        return false;
    }
    fn has_cycle(&self) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();

        // non directed
        if !self.directed {
            for node in self.adj.keys() {
                if !visited.contains(node) {
                    if self._cycle_helper_undirected(*node, None, &mut visited) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
            return false;
        }
        //directed
        else {
            let mut in_progress = HashSet::new();
            for node in self.adj.keys() {
                if !visited.contains(node) {
                    if self._cycle_helper_directed(*node, &mut in_progress, &mut visited) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
            return false;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_bfs() {
        let mut graph = Graph::new(true);

        let edges = vec![
            (1, 2),
            (1, 4),
            (1, 6),
            (2, 9),
            (2, 5),
            (4, 2),
            (5, 8),
            (6, 7),
            (6, 12),
            (7, 10),
            (8, 10),
            (10, 11),
            (11, 13),
            (11, 12),
            (12, 14),
            (13, 3),
            (14, 15),
            (15, 16),
            (16, 3),
        ];

        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(
            graph.shortest_path_unweighted(1, 3).unwrap(),
            vec![1, 6, 7, 10, 11, 13, 3]
        );
    }

    #[test]
    fn test_2_undirected_path_exists() {
        let mut graph = Graph::new(false);

        let edges = vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 5),
            (4, 7),
            (5, 6),
            (7, 6),
            (6, 8),
            (8, 9),
        ];

        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }

        let path = graph.shortest_path_unweighted(0, 9).unwrap();
        assert_eq!(path.len(), 8);
        assert_eq!(path.first(), Some(&0));
        assert_eq!(path.last(), Some(&9));
    }

    #[test]
    fn test_3_directed_unreachable() {
        let mut graph = Graph::new(true);

        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 3),
            (6, 7),
            (7, 8),
        ];

        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }

        assert_eq!(
            graph.shortest_path_unweighted(0, 5).unwrap(),
            vec![0, 1, 2, 3, 4, 5]
        );
        assert_eq!(graph.shortest_path_unweighted(0, 8), None);
    }

    #[test]
    fn test_4_dfs_iterative() {
        let mut graph = Graph::new(false);

        //        1
        //       / \
        //      2   3
        //      |   |
        //      4   5
        //       \ /
        //        6
        let edges = vec![(1, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 6)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }

        assert_eq!(graph.dfs_iterative(1), vec![1, 3, 5, 6, 4, 2]);

        let mut dfs_nodes = graph.dfs_iterative(1);
        let mut bfs_nodes = graph.bfs(1);
        dfs_nodes.sort();
        bfs_nodes.sort();
        assert_eq!(dfs_nodes, bfs_nodes);
    }

    #[test]
    fn test_5_dfs_recursive() {
        let mut graph = Graph::new(false);

        let edges = vec![(1, 2), (1, 3), (2, 4), (3, 5), (4, 6), (5, 6)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }

        let rec = graph.dfs_recursive(1);

        assert_eq!(rec.first(), Some(&1));

        let mut rec_sorted = rec.clone();
        let mut iter_sorted = graph.dfs_iterative(1);
        rec_sorted.sort();
        iter_sorted.sort();
        assert_eq!(rec_sorted, iter_sorted);

        assert_eq!(rec.len(), graph.node_count());
    }
    #[test]
    fn test_1_undirected_cycle() {
        let mut graph = Graph::new(false);
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), false);
    }
    #[test]
    fn test_2_undirected_cycle() {
        let mut graph = Graph::new(false);
        let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), false);
    }
    #[test]
    fn test_3_undirected_cycle() {
        let mut graph = Graph::new(false);
        let edges = vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), true);
    }
    #[test]
    fn test_4_undirected_cycle() {
        let mut graph = Graph::new(false);
        let edges = vec![
            (7, 6),
            (6, 0),
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 2),
        ];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), true);
    }

    #[test]
    fn test_1_directed_cycle() {
        let mut graph = Graph::new(true);
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), false);
    }

    #[test]
    fn test_2_directed_cycle() {
        let mut graph = Graph::new(true);
        let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), false);
    }

    #[test]
    fn test_3_directed_cycle() {
        let mut graph = Graph::new(true);
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 1)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), true);
    }

    #[test]
    fn test_4_directed_cycle() {
        let mut graph = Graph::new(true);
        let edges = vec![(0, 5), (0, 1), (1, 2), (2, 3), (3, 4), (4, 1)];
        for (from, to) in edges {
            graph.add_edge(from, to, 1);
        }
        assert_eq!(graph.has_cycle(), true);
    }
}
