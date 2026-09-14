//! Binary search tree with the three classic traversals and node deletion.

/// Unbalanced binary search tree.
///
/// Operations are O(log n) on balanced input but degrade to O(n) if
/// values are inserted in sorted order, since there is no rebalancing.
#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    // We accept duplicate values and we are moving them into the right place.
    // This is concious.
    // Delete will delete only one occurence.
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, val: T) {
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
    /// Returns `true` if the value is present in the tree.
    pub fn contains(&self, val: &T) -> bool {
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
        false
    }
    /// Returns the smallest value (leftmost node), or `None` if the tree is empty.
    pub fn min(&self) -> Option<&T> {
        let mut next_node = &self.root;
        while let Some(node) = next_node {
            if node.left.is_none() {
                return Some(&node.value);
            } else {
                next_node = &node.left;
            }
        }
        None
    }
    /// Returns the largest value (rightmost node), or `None` if the tree is empty.
    pub fn max(&self) -> Option<&T> {
        let mut next_node = &self.root;
        while let Some(node) = next_node {
            if node.right.is_none() {
                return Some(&node.value);
            } else {
                next_node = &node.right;
            }
        }
        None
    }
    /// Left, node, right — yields the values in sorted order.
    pub fn inorder(&self) -> Vec<T>
    // sorted sequence
    where
        T: Clone,
    {
        let mut res_vec: Vec<T> = Vec::new();
        Self::inorder_node_rec(&self.root, &mut res_vec);
        res_vec
    }
    fn inorder_node_rec(node: &Option<Box<Node<T>>>, vec_to_add: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(node_unpacked) = node {
            Self::inorder_node_rec(&node_unpacked.left, vec_to_add);
            vec_to_add.push(node_unpacked.value.clone());
            Self::inorder_node_rec(&node_unpacked.right, vec_to_add);
        }
    }

    /// Node, left, right.
    pub fn preorder(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut res_vec: Vec<T> = Vec::new();
        Self::preorder_node_rec(&self.root, &mut res_vec);
        res_vec
    }
    fn preorder_node_rec(node: &Option<Box<Node<T>>>, vec_to_add: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(node_unpacked) = node {
            vec_to_add.push(node_unpacked.value.clone());
            Self::preorder_node_rec(&node_unpacked.left, vec_to_add);
            Self::preorder_node_rec(&node_unpacked.right, vec_to_add);
        }
    }

    /// Left, right, node.
    pub fn postorder(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut res_vec: Vec<T> = Vec::new();
        Self::postorder_node_rec(&self.root, &mut res_vec);
        res_vec
    }
    fn postorder_node_rec(node: &Option<Box<Node<T>>>, vec_to_add: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(node_unpacked) = node {
            Self::postorder_node_rec(&node_unpacked.left, vec_to_add);
            Self::postorder_node_rec(&node_unpacked.right, vec_to_add);
            vec_to_add.push(node_unpacked.value.clone());
        }
    }
    fn min_node(node: &Option<Box<Node<T>>>) -> Option<&T> {
        let mut current_node = node;
        while let Some(node_unpacked) = current_node {
            if node_unpacked.left.is_none() {
                return Some(&node_unpacked.value);
            } else {
                current_node = &node_unpacked.left;
            }
        }
        None
    }

    pub fn delete(&mut self, val: &T) -> bool
    where
        T: Clone,
    {
        if !self.contains(val) {
            return false;
        }
        self.root = Self::delete_node_rec(self.root.take(), val);

        true
    }
    fn delete_node_rec(node: Option<Box<Node<T>>>, val: &T) -> Option<Box<Node<T>>>
    where
        T: Clone,
    {
        match node {
            Some(mut unpacked_node) => {
                if unpacked_node.value > *val {
                    unpacked_node.left = Self::delete_node_rec(unpacked_node.left, val);
                    Some(unpacked_node)
                } else if unpacked_node.value < *val {
                    unpacked_node.right = Self::delete_node_rec(unpacked_node.right, val);
                    Some(unpacked_node)
                } else {
                    // we found the val
                    if unpacked_node.left.is_none() && unpacked_node.right.is_none() {
                        None
                    } else if unpacked_node.left.is_some() && unpacked_node.right.is_some() {
                        // both right and left are some
                        let min_val = Self::min_node(&unpacked_node.right).unwrap().clone();
                        unpacked_node.right = Self::delete_node_rec(unpacked_node.right, &min_val);
                        unpacked_node.value = min_val;
                        Some(unpacked_node)
                    } else if unpacked_node.left.is_some() {
                        unpacked_node.left
                    } else {
                        unpacked_node.right
                    }
                }
            }
            None => None,
        }
    }
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
    fn test_inorder() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.inorder(), vec![]);

        for val in [5, 3, 7, 1, 4, 6, 8] {
            bst.insert(val);
        }
        assert_eq!(bst.inorder(), vec![1, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_postorder() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.postorder(), vec![]);

        for val in [5, 3, 7, 1, 4, 6, 8] {
            bst.insert(val);
        }
        assert_eq!(bst.postorder(), vec![1, 4, 3, 6, 8, 7, 5]);
    }

    #[test]
    fn test_preorder() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(bst.preorder(), vec![]);

        for val in [5, 3, 7, 1, 4, 6, 8] {
            bst.insert(val);
        }
        assert_eq!(bst.preorder(), vec![5, 3, 1, 4, 7, 6, 8]);
    }

    #[test]
    fn test_contains() {
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(0);
        bst.insert(-1);
        assert!(bst.contains(&10));
        assert!(bst.contains(&-1));
        assert!(!bst.contains(&-11));
    }

    #[test]
    fn test_delete_leaf() {
        // Delete a node with no children
        //     5
        //    / \
        //   3   7
        // Delete 3 → should give inorder [5, 7]
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        for val in [5, 3, 7] {
            bst.insert(val);
        }
        assert!(bst.delete(&3));
        assert!(!bst.contains(&3));
        assert_eq!(bst.inorder(), vec![5, 7]);
    }

    #[test]
    fn test_delete_one_child() {
        // Delete a node with one child
        //     5
        //    / \
        //   3   7
        //  /
        // 1
        // Delete 3 → 1 takes its place, inorder [1, 5, 7]
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        for val in [5, 3, 7, 1] {
            bst.insert(val);
        }
        assert!(bst.delete(&3));
        assert!(!bst.contains(&3));
        assert!(bst.contains(&1));
        assert_eq!(bst.inorder(), vec![1, 5, 7]);
    }

    #[test]
    fn test_delete_two_children() {
        // Delete a node with two children
        //     5
        //    / \
        //   3   7
        //  / \
        // 1   4
        // Delete 3 → in-order successor is 4, inorder [1, 4, 5, 7]
        let mut bst: BinarySearchTree<i32> = BinarySearchTree::new();
        for val in [5, 3, 7, 1, 4] {
            bst.insert(val);
        }
        assert!(bst.delete(&3));
        assert!(!bst.contains(&3));
        assert!(bst.contains(&1));
        assert!(bst.contains(&4));
        assert_eq!(bst.inorder(), vec![1, 4, 5, 7]);
    }
}
