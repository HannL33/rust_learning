// Stack - first in first out
// push -> takes element and add it on top to the stack
// pop -> takes back the last element
// peek -> looking only what is on the top
// is_empty, size

// Queue (First in, First out)
// we will implement Queue by two Stacks, stack_in and stack_out

use std::collections::HashMap;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::take;
use std::time::Instant;
// LinkedList -> pushing/popping in front (which has O(1) computability), and from back (O(n))
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

#[derive(Debug)]
struct Dictionary<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
    count: usize,
}

impl<K: Hash + Eq, V> Dictionary<K, V> {
    fn new() -> Self {
        Dictionary {
            buckets: (0..16).map(|x| Vec::new()).collect(),
            len: 16,
            count: 0,
        }
    }
    fn insert(&mut self, key: K, value: V) {
        if self.count as f64 > 0.75 * (self.len as f64) {
            self._resize();
        }

        let index: usize = self.hash(&key);
        match self.buckets[index].iter_mut().find(|x| x.0 == key) {
            Some(tuple) => tuple.1 = value,
            None => {
                self.count += 1;
                self.buckets[index].push((key, value));
            }
        }
    }
    fn get(&self, key: &K) -> Option<&V> {
        let index: usize = self.hash(key);
        match self.buckets[index].iter().find(|x| x.0 == *key) {
            Some(tuple) => Some(&tuple.1),
            None => None,
        }
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        let index: usize = self.hash(key);
        if !self.contains_key(key) {
            None
        } else {
            if let Some(remove_idx) = self.buckets[index].iter().position(|x| x.0 == *key) {
                self.count -= 1;
                return Some(self.buckets[index].remove(remove_idx).1);
            }
            None
        }
    }
    fn contains_key(&self, key: &K) -> bool {
        let index: usize = self.hash(&key);
        self.buckets[index].iter().any(|x| x.0 == *key)
    }
    fn _resize(&mut self) {
        let new_len: usize = self.len * 2;
        self.len = new_len; // I do not now if this is safe... But to use hash, i need it
        // as it is condensed into the method, perhaps hash should be free function...

        let mut new_buckets: Vec<Vec<(K, V)>> = (0..new_len).map(|x| Vec::new()).collect();
        let old_bucks = take(&mut self.buckets);
        for vector in old_bucks.into_iter() {
            for tuple in vector.into_iter() {
                let new_hash: usize = self.hash(&tuple.0);
                new_buckets[new_hash].push((tuple.0, tuple.1)); // this is move of the ownership
                // so no copies whatsoever
            }
        }
        self.buckets = new_buckets;
    }
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.len
    }
}
fn main() {
    // benchmarking the custom hashmap versus std hashmap

    // standard library hashmap implementation
    let start = std::time::Instant::now();
    let mut std_hashmap: HashMap<i32, i32> = std::collections::HashMap::new();
    for i in 1..100_000 {
        std_hashmap.insert(i, i + 1);
    }
    let elapsed = start.elapsed();
    println!("Time elapsed std hashmap <INSERT>: {}", elapsed.as_micros());
    std_hashmap.get(&25);

    let start = std::time::Instant::now();
    let mut my_hashmap: Dictionary<i32, i32> = Dictionary::new();
    for i in 1..100_000 {
        my_hashmap.insert(i, i + 1);
    }
    let elapsed = start.elapsed();
    println!("Time elapsed my hashmap <INSERT>: {}", elapsed.as_micros());
    my_hashmap.get(&29);

    // getting with sum (to force compiler to not optimize things)
    let start = std::time::Instant::now();
    let mut sum = 0;
    for i in 1..100_000 {
        sum += std_hashmap.get(&i).unwrap_or(&0);
    }
    let elapsed = start.elapsed();
    println!("Time elapsed std hashmap <GET>: {}", elapsed.as_micros());
    println!("{}", sum);

    let start = std::time::Instant::now();
    let mut sum = 0;
    for i in 1..100_000 {
        sum += my_hashmap.get(&i).unwrap_or(&0);
    }
    let elapsed = start.elapsed();
    println!("Time elapsed my hashmap <GET>: {}", elapsed.as_micros());
    println!("{}", sum);
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
    fn dict_insert_and_get() {
        let mut dict: Dictionary<String, i32> = Dictionary::new();
        dict.insert("a".to_string(), 1);
        dict.insert("b".to_string(), 2);
        assert_eq!(dict.get(&"a".to_string()), Some(&1));
        assert_eq!(dict.get(&"b".to_string()), Some(&2));
    }

    #[test]
    fn dict_get_missing_key() {
        let dict: Dictionary<String, i32> = Dictionary::new();
        assert_eq!(dict.get(&"nieistniejacy".to_string()), None);
    }

    #[test]
    fn dict_insert_overwrites_existing_key() {
        let mut dict: Dictionary<String, i32> = Dictionary::new();
        dict.insert("a".to_string(), 1);
        dict.insert("a".to_string(), 99);
        assert_eq!(dict.get(&"a".to_string()), Some(&99));
        // sprawdz ze nie ma duplikatow - powinien byc tylko jeden wpis
        let index = dict.hash(&"a".to_string());
        assert_eq!(dict.buckets[index].len(), 1);
    }

    #[test]
    fn dict_contains_key() {
        let mut dict: Dictionary<String, i32> = Dictionary::new();
        dict.insert("klucz".to_string(), 42);
        assert!(dict.contains_key(&"klucz".to_string()));
        assert!(!dict.contains_key(&"inny".to_string()));
    }

    #[test]
    fn dict_remove_existing() {
        let mut dict: Dictionary<String, i32> = Dictionary::new();
        dict.insert("x".to_string(), 10);
        let removed = dict.remove(&"x".to_string());
        assert_eq!(removed, Some(10));
        assert_eq!(dict.get(&"x".to_string()), None);
        assert!(!dict.contains_key(&"x".to_string()));
    }

    #[test]
    fn dict_remove_missing() {
        let mut dict: Dictionary<String, i32> = Dictionary::new();
        assert_eq!(dict.remove(&"ghost".to_string()), None);
    }

    #[test]
    fn dict_remove_then_reinsert() {
        let mut dict: Dictionary<String, i32> = Dictionary::new();
        dict.insert("k".to_string(), 5);
        dict.remove(&"k".to_string());
        dict.insert("k".to_string(), 100);
        assert_eq!(dict.get(&"k".to_string()), Some(&100));
    }

    #[test]
    fn dict_many_keys() {
        let mut dict: Dictionary<i32, i32> = Dictionary::new();
        for i in 0..100 {
            dict.insert(i, i * 2);
        }
        for i in 0..100 {
            assert_eq!(dict.get(&i), Some(&(i * 2)));
        }
    }

    #[test]
    fn dict_resize_triggered_and_data_intact() {
        let mut dict: Dictionary<i32, i32> = Dictionary::new();
        // 13 elementow przekroczy 0.75 * 16 = 12 -> resize do 32 kubelkow
        for i in 0..20 {
            dict.insert(i, i * 10);
        }
        assert_eq!(dict.len, 32);
        for i in 0..20 {
            assert_eq!(dict.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn dict_resize_elements_in_correct_buckets() {
        let mut dict: Dictionary<i32, i32> = Dictionary::new();
        for i in 0..20 {
            dict.insert(i, i * 10);
        }
        // sprawdz ze kazdy element jest w DOKLADNIE TYM kubełku gdzie wskazuje hash
        for i in 0..20 {
            let expected_bucket = dict.hash(&i);
            let found = dict.buckets[expected_bucket].iter().any(|(k, _)| *k == i);
            assert!(found, "klucz {} nie jest w kubełku {}", i, expected_bucket);
        }
        // sprawdz ze laczna liczba elementow we wszystkich kubelkach == 20
        let total: usize = dict.buckets.iter().map(|b| b.len()).sum();
        assert_eq!(total, 20);
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
