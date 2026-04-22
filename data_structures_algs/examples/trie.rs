use std::collections::HashMap;

struct TrieNode {
    next: HashMap<char, TrieNode>,
    is_end: bool,
}
struct Trie {
    next: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            next: TrieNode {
                next: HashMap::new(),
                is_end: false,
            },
        }
    }
    fn insert(word: &str) {
        todo!();
    }
    fn search(word: &str) -> bool {
        todo!();
    }
    fn starts_with(word: &str) -> bool {
        todo!();
    }
    fn delete(word: &str) {
        todo!();
    }
    fn dfs() {
        todo!();
    }
}

fn main() {}
