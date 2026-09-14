//! Classic data structures and graph algorithms implemented from scratch in Rust,
//! without leaning on the standard library's ready-made collections.
//!
//! Each module is self-contained and unit-tested. The graph module powers the maze
//! solver in `examples/maze_solver.rs`, which parses a real SVG maze, turns it into a
//! [`graphs::Graph`] and writes the BFS solution back out as SVG.

pub mod binary_tree;
pub mod dictionary;
pub mod graphs;
pub mod heap;
pub mod linked_list;
pub mod stack;
pub mod trie;
