use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    next: HashMap<char, TrieNode>,
    is_end: bool,
}
#[derive(Debug)]
pub struct Trie {
    next: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            next: TrieNode {
                next: HashMap::new(),
                is_end: false,
            },
        }
    }
    pub fn insert(&mut self, word: &str) {
        let mut current_tree = &mut self.next;
        for c in word.chars() {
            if !current_tree.next.contains_key(&c) {
                current_tree.next.insert(
                    c,
                    TrieNode {
                        next: HashMap::new(),
                        is_end: false,
                    },
                );
                current_tree = current_tree.next.get_mut(&c).unwrap();
            } else {
                current_tree = current_tree.next.get_mut(&c).unwrap();
            }
        }
        current_tree.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_tree = &self.next;
        for c in word.chars() {
            if current_tree.next.contains_key(&c) {
                current_tree = current_tree.next.get(&c).unwrap();
            } else {
                return false;
            }
        }
        if current_tree.is_end { true } else { false }
    }
    pub fn starts_with(&self, word: &str) -> bool {
        let mut current_tree = &self.next;
        for c in word.chars() {
            if current_tree.next.contains_key(&c) {
                current_tree = current_tree.next.get(&c).unwrap();
            } else {
                return false;
            }
        }
        return true;
    }
    pub fn delete(&mut self, word: &str) {
        if self.search(word) {
            Self::delete_rec(&mut self.next, word);
        }
    }
    fn delete_rec(node: &mut TrieNode, chars: &str) -> bool {
        if chars.is_empty() {
            node.is_end = false;
            if node.next.len() >= 1 {
                return false;
            } else {
                return true;
            }
        }
        let first_char = &chars.chars().next().unwrap();
        if Self::delete_rec(
            node.next.get_mut(first_char).unwrap(),
            &chars[first_char.len_utf8()..],
        ) {
            node.next.remove(first_char);
            if node.next.len() >= 1 || node.is_end {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_complex_shared_prefix_node_with_is_end() {
        let mut trie = Trie::new();
        trie.insert("app");
        trie.insert("apple");
        trie.insert("apply");
        trie.insert("apt");
        trie.delete("apple");
        trie.delete("apply");

        assert!(!trie.search("apple"));
        assert!(!trie.search("apply"));
        assert!(trie.search("app"));
        assert!(trie.search("apt"));
        let second_p = trie
            .next
            .next
            .get(&'a')
            .unwrap()
            .next
            .get(&'p')
            .unwrap()
            .next
            .get(&'p')
            .unwrap();
        assert!(second_p.is_end);
        assert!(second_p.next.is_empty());
    }

    #[test]
    fn test_delete_full_cascade_leaves_trie_empty() {
        let mut trie = Trie::new();
        trie.insert("pre");
        trie.insert("prefix");
        trie.insert("prevent");

        trie.delete("prefix");
        assert!(!trie.search("prefix"));
        assert!(trie.search("pre"));
        assert!(trie.search("prevent"));
        assert!(trie.starts_with("prev"));

        trie.delete("prevent");
        assert!(!trie.search("prevent"));
        assert!(trie.search("pre"));
        assert!(!trie.starts_with("prev"));

        trie.delete("pre");
        assert!(!trie.search("pre"));
        assert!(trie.next.next.is_empty());
    }

    #[test]
    fn test_delete_word_that_is_prefix_of_another() {
        let mut trie = Trie::new();
        trie.insert("car");
        trie.insert("card");
        trie.delete("car");

        assert!(!trie.search("car"));
        assert!(trie.search("card"));
        let r = trie
            .next
            .next
            .get(&'c')
            .unwrap()
            .next
            .get(&'a')
            .unwrap()
            .next
            .get(&'r')
            .unwrap();
        assert!(!r.is_end);
        assert!(r.next.contains_key(&'d'));
    }

    #[test]
    fn test_delete_word_with_shared_prefix() {
        let mut trie = Trie::new();
        trie.insert("car");
        trie.insert("cat");
        trie.delete("car");

        assert!(!trie.search("car"));
        assert!(trie.search("cat"));
        let a = trie.next.next.get(&'c').unwrap().next.get(&'a').unwrap();
        assert!(!a.next.contains_key(&'r'));
        assert!(a.next.contains_key(&'t'));
    }

    #[test]
    fn test_delete_only_word_cleans_all_nodes() {
        let mut trie = Trie::new();
        trie.insert("car");
        trie.delete("car");

        assert!(!trie.search("car"));
        assert!(trie.next.next.is_empty());
    }

    #[test]
    fn test_insert_shared_prefix() {
        let mut trie = Trie::new();
        trie.insert("rak");
        trie.insert("raj");
        trie.insert("rama");

        let r = trie.next.next.get(&'r').unwrap();
        let a = r.next.get(&'a').unwrap();
        assert!(a.next.contains_key(&'k'));
        assert!(a.next.contains_key(&'j'));
        assert!(a.next.contains_key(&'m'));
        let k = a.next.get(&'k').unwrap();
        assert!(k.is_end);
        let j = a.next.get(&'j').unwrap();
        assert!(j.is_end);
        let m = a.next.get(&'m').unwrap();
        assert!(!m.is_end);
        let ma = m.next.get(&'a').unwrap();
        assert!(ma.is_end);
    }

    #[test]
    fn test_starts_with_basic() {
        let mut trie = Trie::new();
        trie.insert("car");
        trie.insert("card");
        trie.insert("cat");

        assert!(trie.starts_with("c"));
        assert!(trie.starts_with("ca"));
        assert!(trie.starts_with("car"));
        assert!(trie.starts_with("cat"));
        assert!(!trie.starts_with("dog"));
        assert!(!trie.starts_with("cb"));
    }

    #[test]
    fn test_starts_with_full_word_is_prefix() {
        let mut trie = Trie::new();
        trie.insert("card");

        assert!(trie.starts_with("car"));
        assert!(!trie.search("car"));
        assert!(trie.starts_with("card"));
        assert!(trie.search("card"));
        assert!(!trie.starts_with("cards"));
    }

    #[test]
    fn test_insert_word_and_its_prefix() {
        let mut trie = Trie::new();
        trie.insert("car");
        trie.insert("card");
        trie.insert("care");
        trie.insert("cat");

        let c = trie.next.next.get(&'c').unwrap();
        let a = c.next.get(&'a').unwrap();
        assert!(!a.is_end);
        let r = a.next.get(&'r').unwrap();
        assert!(r.is_end);
        assert!(r.next.contains_key(&'d'));
        assert!(r.next.contains_key(&'e'));
        assert!(r.next.get(&'d').unwrap().is_end);
        assert!(r.next.get(&'e').unwrap().is_end);
        let t = a.next.get(&'t').unwrap();
        assert!(t.is_end);
        assert!(t.next.is_empty());
    }
}
