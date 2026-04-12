#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, val: T) {
        // The idea is to iterate over to the end of the tree to insert the value
        // This is iterative approach
        let mut next_node = &mut self.root;
        while let Some(node) = next_node {
            if val >= node.value {
                if node.right.is_none() {
                    node.right = Some(Box::new(Node {
                        value: val,
                        left: None,
                        right: None,
                    }));
                    return;
                } else {
                    next_node = &mut node.right;
                }
            } else {
                if node.left.is_none() {
                    node.left = Some(Box::new(Node {
                        value: val,
                        left: None,
                        right: None,
                    }));
                    return;
                } else {
                    next_node = &mut node.left;
                }
            }
        }
        // If next_node is None in the first while loop (i.e. there is no elements)
        self.root = Some(Box::new(Node {
            value: val,
            left: None,
            right: None,
        }));
    }
    fn contains(&self, val: &T) -> bool {
        let mut next_node = &self.root;
        while let Some(node) = next_node {
            if *val > node.value {
                next_node = &node.right;
            } else if *val < node.value {
                next_node = &node.left;
            } else {
                return true;
            }
        }
        return false;
    }
    fn min(&self) -> Option<&T> {
        let mut next_node = &self.root;
        while let Some(node) = next_node {
            if node.left.is_none() {
                return Some(&node.value);
            } else {
                next_node = &node.left;
            }
        }
        return None;
    }
    fn max(&self) -> Option<&T> {
        let mut next_node = &self.root;
        while let Some(node) = next_node {
            if node.right.is_none() {
                return Some(&node.value);
            } else {
                next_node = &node.right;
            }
        }
        return None;
    }
    fn inorder(&self)
    where
        T: std::fmt::Debug,
    {
        // sorted sequence
        Self::print_inorder_node(&self.root);
    }
    fn print_inorder_node(node: &Option<Box<Node<T>>>)
    where
        T: std::fmt::Debug,
    {
        if let Some(node_unpacked) = node {
            Self::print_inorder_node(&node_unpacked.left);
            println!("{:?}", node_unpacked.value);
            Self::print_inorder_node(&node_unpacked.right);
        }
    }
}

fn main() {
    let mut bts = BinarySearchTree::new();
    println!("Min: {:?}", bts.min());
    println!("Max: {:?}", bts.max());

    bts.insert(5);
    bts.insert(10);
    bts.insert(0);
    println!("Binary Tree Search: {:?}", bts);
    println!("Does contain? {} : {}", -1, bts.contains(&-1));
    bts.insert(-5);
    println!("Min: {:?}", bts.min());
    bts.insert(0);
    bts.insert(101);
    println!("Min: {:?}", bts.min());
    bts.insert(-6);
    println!("Min: {:?}", bts.min());
    println!("Max: {:?}", bts.max());
    bts.inorder();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_root_only() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        let root = bst.root.as_ref().unwrap();
        assert_eq!(root.value, 5);
        assert!(root.left.is_none());
        assert!(root.right.is_none());
    }

    #[test]
    fn test_insert_left_and_right() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(4);
        let root = bst.root.as_ref().unwrap();
        assert_eq!(root.value, 5);
        assert_eq!(root.left.as_ref().unwrap().value, 3);
        assert_eq!(root.right.as_ref().unwrap().value, 7);
        let left = root.left.as_ref().unwrap();
        assert_eq!(left.left.as_ref().unwrap().value, 1);
        assert_eq!(left.right.as_ref().unwrap().value, 4);
    }
    #[test]
    fn test_min() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.min(), None);

        for val in [10, 5, 15, 2, 7, 1] {
            bst.insert(val);
        }
        assert_eq!(bst.min(), Some(&1));
    }

    #[test]
    fn test_max() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.max(), None);

        for val in [10, 5, 15, 12, 20, 25] {
            bst.insert(val);
        }
        assert_eq!(bst.max(), Some(&25));
    }

    #[test]
    fn test_contains() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(0);
        bst.insert(-1);
        assert_eq!(bst.contains(&10), true);
        assert_eq!(bst.contains(&-1), true);
        assert_eq!(bst.contains(&-11), false);
    }
}
