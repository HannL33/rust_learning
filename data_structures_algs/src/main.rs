// Stack - first in first out
// push -> takes element and add it on top to the stack
// pop -> takes back the last element
// peek -> looking only what is on the top
// is_empty, size

// Queue (First in, First out)
// we will implement Queue by two Stacks, stack_in and stack_out

struct Queue<T> {
    stack_in: Stack<T>,
    stack_out: Stack<T>,
}
impl<T> Queue<T> {
    fn new() -> Self {
        return Queue {
            stack_in: Stack::new(),
            stack_out: Stack::new(),
        };
    }
    fn from(vec: Vec<T>) -> Self {
        todo!()
    }

    fn enqueue(&mut self, elem: T) {
        self.stack_in.push(elem);
    }
    fn dequeue(&mut self) -> Option<T> {
        if self.stack_out.is_empty() {
            while let Some(elem) = self.stack_in.pop() {
                self.stack_out.push(elem)
            }
        }
        self.stack_out.pop()
    }
}

struct Stack<T> {
    data: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }
    fn from(vec: Vec<T>) -> Self {
        Stack { data: vec }
    }

    // formally on my mac usize is u64, but it is recommended (i think so), to keep usize
    fn size(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, elem: T) -> () {
        self.data.push(elem);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}
#[derive(Debug, PartialEq)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn push_back(&mut self, to_add: T) {
        let mut current_end = &mut self.head;
        while let Some(node) = current_end {
            current_end = &mut node.next;
        }
        *current_end = Some(Box::new(Node {
            elem: to_add,
            next: None,
        }));
    }
    fn pop_back(&mut self) -> Option<T> {
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
        return None;
    }
    fn push_front(&mut self, to_add: T) {
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
    fn pop_front(&mut self) -> Option<T> {
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
    fn peek_front(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.elem)
        } else {
            None
        }
    }
    fn peek_back(&self) -> Option<&T> {
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

fn main() {
    let mut my_linked_list = LinkedList::<i32>::new();
    my_linked_list.push_front(10);
    println!("{:?}", my_linked_list);
    my_linked_list.push_front(0);
    println!("{:?}", my_linked_list);
    let peek_front = my_linked_list.peek_front();
    println!("Peeked front: {:?}", peek_front);
    println!("{:?}", my_linked_list);
    let popped_val = my_linked_list.pop_front();
    println!("{:?}", popped_val);
    println!("{:?}", my_linked_list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(5);
        assert_eq!(stack.peek(), Some(&5));
        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_size() {
        let stack = Stack::from(vec![1, 2, 3]);
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.is_empty(), false);

        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn queue_test1() {
        let mut queue = Queue::new();
        queue.enqueue(5);
        queue.enqueue(6);
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), None);
    }
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
