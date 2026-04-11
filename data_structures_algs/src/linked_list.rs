#[derive(Debug, PartialEq)]
pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    pub fn push_back(&mut self, to_add: T) {
        let mut current_end = &mut self.head;
        while let Some(node) = current_end {
            current_end = &mut node.next;
        }
        *current_end = Some(Box::new(Node {
            elem: to_add,
            next: None,
        }));
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.as_ref().is_some_and(|x| x.next.is_none()) {
            let res = self.head.take();
            return Some(res.unwrap().elem);
        }
        let mut current_end = &mut self.head;
        while let Some(node) = current_end {
            if let Some(ref mut next_node) = node.next {
                if next_node.next.is_none() {
                    return node.next.take().map(|n| n.elem);
                }
            }

            current_end = &mut node.next;
        }
        None
    }
    pub fn push_front(&mut self, to_add: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node {
                elem: to_add,
                next: None,
            }))
        } else {
            let new_node = Some(Box::new(Node {
                elem: to_add,
                next: self.head.take(),
            }));
            self.head = new_node;
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            let first_node = self.head.take();
            if let Some(node) = first_node {
                self.head = node.next;
                return Some(node.elem);
            } else {
                self.head = None;
                return None;
            }
        }
    }
    pub fn peek_front(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.elem)
        } else {
            None
        }
    }
    pub fn peek_back(&self) -> Option<&T> {
        let mut current_end = &self.head;
        while let Some(node) = current_end {
            if node.next.is_none() {
                return Some(&node.elem);
            }
            current_end = &node.next;
        }
        None
    }
}

// Tests //
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        // lista: 1 -> 2 -> 3

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_peek_front() {
        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.peek_front(), None);

        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        // lista: 1 -> 2 -> 3

        assert_eq!(list.peek_front(), Some(&1));
        list.pop_front();
        assert_eq!(list.peek_front(), Some(&2));
    }

    #[test]
    fn test_peek_back() {
        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.peek_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        // lista: 1 -> 2 -> 3

        assert_eq!(list.peek_back(), Some(&3));
        list.pop_back();
        assert_eq!(list.peek_back(), Some(&2));
    }
    #[test]
    fn linked_list_front_test() {
        let mut my_linked_list = LinkedList::<i32>::new();
        my_linked_list.push_back(10);
        assert_eq!(my_linked_list.head.as_ref().unwrap().elem, 10);
        my_linked_list.push_back(-10);
        assert_eq!(
            my_linked_list,
            LinkedList {
                head: Some(Box::new(Node {
                    elem: 10,
                    next: Some(Box::new(Node {
                        elem: -10,
                        next: None
                    }))
                }))
            }
        );
        assert_eq!(my_linked_list.pop_back(), Some(-10));
        assert_eq!(
            my_linked_list,
            LinkedList {
                head: Some(Box::new(Node {
                    elem: 10,
                    next: None
                }))
            }
        );
    }
}
