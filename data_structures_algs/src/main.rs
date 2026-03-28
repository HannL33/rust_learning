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
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
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
        if self.head.is_none() {
            return None;
        }
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
    fn peek() {
        todo!();
    }
}

fn main() {
    let mut my_linked_list = LinkedList::<i32>::new();
    my_linked_list.push_back(10);
    println!("{:?}", my_linked_list);
    my_linked_list.push_back(15);
    println!("{:?}", my_linked_list);
    my_linked_list.push_back(20);
    println!("{:?}", my_linked_list);
    let popped_elem = my_linked_list.pop_back();
    println!("{:?}", my_linked_list);
    println!("{:?}", popped_elem);
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
}
