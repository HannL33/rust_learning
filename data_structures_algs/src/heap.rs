#[derive(Debug)]
pub struct MaxHeap<T> {
    data: Vec<T>,
}
impl<T: Ord> MaxHeap<T> {
    /// Parent: (i - 1) / 2
    /// Left Child: 2 * i + 1
    /// Right Child: 2 * i + 2
    pub fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }
    pub fn insert(&mut self, val: T) {
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
    pub fn extract_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let res = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0, 0);
        }
        res
    }
    fn sift_down(&mut self, start_level: usize, excluded_count: usize) {
        let mut elem_idx: usize = start_level;
        let data_len: usize = self.data.len() - excluded_count;

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
    pub fn heapify(data_vec: Vec<T>) -> Self {
        let mut res = MaxHeap { data: data_vec };
        if res.data.len() <= 1 {
            return res;
        }
        let idx_start_from = res.data.len() / 2 - 1;
        for idx in (0..=idx_start_from).rev() {
            res.sift_down(idx, 0);
        }

        res
    }
}
pub fn heapsort<T: Ord>(data: Vec<T>) -> Vec<T> {
    // in-place heapsort (but little worse version, as not doing it on mut ref)
    let mut temp_heap = MaxHeap::heapify(data);
    if temp_heap.data.is_empty() {
        return temp_heap.data;
    }
    let last_idx = temp_heap.data.len() - 1;
    for i in 1..=last_idx {
        temp_heap.data.swap(0, last_idx - i + 1);
        temp_heap.sift_down(0, i);
    }
    temp_heap.data
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
    fn test_heapify_maintains_heap_property() {
        let heap = MaxHeap::heapify(vec![3, 10, 1, 7, 5, 8, 2]);
        for i in 1..heap.data.len() {
            assert!(heap.data[(i - 1) / 2] >= heap.data[i]);
        }
    }

    #[test]
    fn test_heapify_extracts_in_sorted_order() {
        let mut heap = MaxHeap::heapify(vec![4, 8, 2, 15, 1, 9, 3]);
        let mut result = vec![];
        while let Some(val) = heap.extract_max() {
            result.push(val);
        }
        assert_eq!(result, vec![15, 9, 8, 4, 3, 2, 1]);
    }

    #[test]
    fn test_heapsort_n10() {
        let input = vec![3, 10, 1, 7, 5, 8, 2, 9, 4, 6];
        let mut expected = input.clone();
        expected.sort();
        assert_eq!(heapsort(input), expected);
    }

    #[test]
    fn test_heapsort_n20() {
        let input = vec![
            15, 3, 18, 7, 11, 1, 20, 9, 4, 14, 6, 17, 2, 13, 8, 19, 5, 12, 10, 16,
        ];
        let mut expected = input.clone();
        expected.sort();
        assert_eq!(heapsort(input), expected);
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
