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
}

fn main() {
    let mut bts = BinarySearchTree::new();
    bts.insert(5);
    bts.insert(10);
    bts.insert(0);
    println!("Binary Tree Search: {:?}", bts);
}
