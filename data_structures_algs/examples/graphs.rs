use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

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
        self.adj.entry(id).or_default();
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
            Some(vec) => vec.iter().any(|&(neighbor, _)| neighbor == to),
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
                for &(nei, _) in neis {
                    if !visited.contains(&nei) {
                        visited.insert(nei);
                        waiting.push_back(nei);
                    }
                }
            }
        } // order for traversal
        result
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
                for &(nei, _) in neis {
                    if !visited.contains(&nei) {
                        prev.insert(nei, node);
                        if nei == end {
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
                        visited.insert(nei);
                        waiting.push_back(nei);
                    }
                }
            }
        }

        None
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
        false
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
                for (nei, _) in neis {
                    if !visited.contains(&nei) {
                        stack.push(*nei);
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
            for (nei, _) in neis {
                if !visited.contains(&nei) {
                    self._dfs_recursive_helper(*nei, visited, result);
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
            for (nei, _) in neis {
                if !visited.contains(&nei) {
                    if self._cycle_helper_undirected(*nei, Some(node), visited) {
                        return true;
                    }
                } else {
                    if Some(*nei) != parent {
                        return true;
                    }
                }
            }
            return false;
        }
        false
    }
    fn _cycle_helper_directed(
        &self,
        node: usize,
        in_progress: &mut HashSet<usize>,
        visited: &mut HashSet<usize>,
    ) -> bool {
        in_progress.insert(node);

        if let Some(neis) = self.neighbors(node) {
            for (nei, _) in neis {
                if in_progress.contains(&nei) {
                    return true;
                }
                if !visited.contains(&nei) {
                    if self._cycle_helper_directed(*nei, in_progress, visited) {
                        return true;
                    }
                }
            }
        }

        //exit
        in_progress.remove(&node);
        visited.insert(node);

        false
    }
    fn has_cycle(&self) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();

        // non directed
        if !self.directed {
            for node in self.adj.keys() {
                if !visited.contains(node) {
                    if self._cycle_helper_undirected(*node, None, &mut visited) {
                        return true;
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
                    }
                }
            }
            return false;
        }
    }

    fn top_sort_helper_dfs(
        &self,
        node: usize,
        in_progress: &mut HashSet<usize>,
        visited: &mut HashSet<usize>,
        result: &mut Vec<usize>,
    ) -> bool {
        // we return bool as the question is "if there is cycle", but also update the result Vec
        in_progress.insert(node);
        if let Some(neis) = self.neighbors(node) {
            for &(nei, _) in neis {
                if in_progress.contains(&nei) {
                    return true;
                }
                if !visited.contains(&nei) {
                    if self.top_sort_helper_dfs(nei, in_progress, visited, result) {
                        return true;
                    }
                }
            }
        }
        in_progress.remove(&node);
        visited.insert(node);
        result.push(node);

        false
    }

    fn topological_sort(&self) -> Option<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut in_progress = HashSet::new();
        let mut result = Vec::new();
        for &node in self.adj.keys() {
            if !visited.contains(&node) {
                if self.top_sort_helper_dfs(node, &mut in_progress, &mut visited, &mut result) {
                    return None;
                }
            }
        }

        result.reverse();
        Some(result)
    }

    // Dijkstra
    fn dijkstra(&self, start: usize) -> HashMap<usize, u32> {
        let mut dist: HashMap<usize, u32> = HashMap::from([(start, 0)]);
        let mut pqueue = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();

        pqueue.push(Reverse((0, start)));
        while let Some(Reverse((d, u))) = pqueue.pop() {
            if visited.contains(&u) {
                continue;
            } else {
                visited.insert(u);
            }

            if let Some(neis) = self.neighbors(u) {
                for &(nei, weight) in neis {
                    let new_dist = d + weight;
                    if let Some(&nei_dist) = dist.get(&nei) {
                        if new_dist < nei_dist {
                            dist.insert(nei, new_dist);
                            pqueue.push(Reverse((new_dist, nei)));
                        }
                    } else {
                        dist.insert(nei, new_dist);
                        pqueue.push(Reverse((new_dist, nei)));
                    }
                }
            }
        }

        dist
    }
    fn shortest_path_weighted(&self, start: usize, end: usize) -> Option<(u32, Vec<usize>)> {
        let mut dist: HashMap<usize, u32> = HashMap::from([(start, 0)]);
        let mut pqueue = BinaryHeap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut prev: HashMap<usize, usize> = HashMap::new();

        pqueue.push(Reverse((0, start)));
        None
    }
}
fn main() {
    let mut test_graph = Graph::new(true);

    test_graph.add_edge(0, 1, 4);
    test_graph.add_edge(0, 2, 2);
    test_graph.add_edge(0, 7, 20);
    test_graph.add_edge(1, 3, 5);
    test_graph.add_edge(2, 1, 1);
    test_graph.add_edge(2, 3, 8);
    test_graph.add_edge(2, 4, 10);
    test_graph.add_edge(3, 5, 2);
    test_graph.add_edge(4, 5, 3);
    test_graph.add_edge(4, 6, 1);
    test_graph.add_edge(5, 6, 1);
    test_graph.add_edge(6, 7, 2);
    test_graph.add_edge(6, 4, 2);
    test_graph.add_edge(7, 9, 5);

    println!("Dijkstra: {:?}", test_graph.dijkstra(5));
}

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

    #[test]
    fn test_1_topological_sort() {
        let mut graph = Graph::new(true);
        // classic example: independent branches (5, 7, 3) converging through 11 and 8
        let edges = vec![
            (5, 11),
            (7, 11),
            (7, 8),
            (3, 8),
            (3, 10),
            (11, 2),
            (11, 9),
            (11, 10),
            (8, 9),
        ];
        for &(from, to) in &edges {
            graph.add_edge(from, to, 1);
        }

        let order = graph.topological_sort().expect("this graph is a DAG");
        assert_eq!(order.len(), graph.node_count());

        let position: HashMap<usize, usize> = order
            .iter()
            .enumerate()
            .map(|(idx, &node)| (node, idx))
            .collect();

        for (from, to) in edges {
            assert!(
                position[&from] < position[&to],
                "{from} should come before {to} in the topological order"
            );
        }
    }
}
