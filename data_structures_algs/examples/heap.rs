#[derive(Debug)]
struct MaxHeap<T> {
    data: Vec<T>,
}
impl<T: PartialOrd> MaxHeap<T> {
    /// Parent: (i - 1) / 2
    /// Left Child: 2 * i + 1
    /// Right Child: 2 * i + 2
    fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }
    fn insert(&mut self, val: T) {
        self.data.push(val);
        self.sift_up();
    }
    fn sift_up(&mut self) {
        let mut elem_idx = self.data.len() - 1;
        while elem_idx >= 1 && self.data[elem_idx] > self.data[(elem_idx - 1) / 2] {
            self.data.swap(elem_idx, (elem_idx - 1) / 2);
            elem_idx = (elem_idx - 1) / 2;
        }
    }
    fn extract_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let res = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down();
        }
        return res;
    }
    fn sift_down(&mut self) {
        let mut elem_idx: usize = 0;
        let data_len: usize = self.data.len();

        loop {
            if 2 * elem_idx + 1 < data_len {
                let exc_idx: usize = if 2 * elem_idx + 2 < data_len
                    && self.data[2 * elem_idx + 2] > self.data[2 * elem_idx + 1]
                {
                    2 * elem_idx + 2
                } else {
                    2 * elem_idx + 1
                };
                if self.data[elem_idx] < self.data[exc_idx] {
                    self.data.swap(elem_idx, exc_idx);
                    elem_idx = exc_idx;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}
// 0, 6, 3, 5, -1
// 6, 0, 3, 5, -1
// 6,

fn main() {
    let mut my_heap = MaxHeap::new();
    my_heap.insert(5);
    my_heap.insert(10);
    my_heap.insert(0);
    my_heap.insert(6);
    my_heap.insert(-1);
    my_heap.insert(3);
    println!("{:?}", my_heap);

    my_heap.extract_max();
    println!("{:?}", my_heap);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_maintains_heap_property() {
        let mut heap = MaxHeap::new();
        for val in [3, 10, 1, 7, 5, 8, 2] {
            heap.insert(val);
        }
        for i in 1..heap.data.len() {
            let parent = (i - 1) / 2;
            assert!(heap.data[parent] >= heap.data[i]);
        }
    }

    #[test]
    fn test_extract_max_order_and_heap_property() {
        let mut heap = MaxHeap::new();
        for val in [4, 8, 2, 15, 1, 9, 3, 7, 6, 11] {
            heap.insert(val);
        }
        let mut prev = heap.extract_max().unwrap();
        while let Some(val) = heap.extract_max() {
            assert!(prev >= val, "extracted {val} after {prev} — not descending");
            for i in 1..heap.data.len() {
                assert!(
                    heap.data[(i - 1) / 2] >= heap.data[i],
                    "heap property violated at index {i}"
                );
            }
            prev = val;
        }
    }

    #[test]
    fn test_extract_max_empty() {
        let mut heap: MaxHeap<i32> = MaxHeap::new();
        assert_eq!(heap.extract_max(), None);
    }

    #[test]
    fn test_extract_max_single_element() {
        let mut heap = MaxHeap::new();
        heap.insert(42);
        assert_eq!(heap.extract_max(), Some(42));
        assert_eq!(heap.extract_max(), None);
    }

    #[test]
    fn test_extract_max_with_duplicates() {
        let mut heap = MaxHeap::new();
        for val in [5, 5, 3, 5, 1, 3] {
            heap.insert(val);
        }
        assert_eq!(heap.extract_max(), Some(5));
        assert_eq!(heap.extract_max(), Some(5));
        assert_eq!(heap.extract_max(), Some(5));
        assert_eq!(heap.extract_max(), Some(3));
        assert_eq!(heap.extract_max(), Some(3));
        assert_eq!(heap.extract_max(), Some(1));
        assert_eq!(heap.extract_max(), None);
    }
}
