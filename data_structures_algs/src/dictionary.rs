//! Hash map written from scratch: separate chaining, doubling resize at 0.75 load factor.

use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::take;

/// Hash map using separate chaining for collisions.
///
/// Each bucket is a `Vec` of key/value pairs, so a collision just appends to the
/// bucket. Once the number of entries passes 0.75 × bucket count the table doubles
/// and every entry is rehashed. Lookups are O(1) on average, O(n) worst case if
/// everything hashes into one bucket.
#[derive(Debug)]
pub struct Dictionary<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
    count: usize,
}

impl<K: Hash + Eq, V> Dictionary<K, V> {
    /// Creates an empty map with 16 buckets.
    pub fn new() -> Self {
        Dictionary {
            buckets: (0..16).map(|_| Vec::new()).collect(),
            len: 16,
            count: 0,
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        if self.count as f64 > 0.75 * (self.len as f64) {
            self.resize();
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
    pub fn get(&self, key: &K) -> Option<&V> {
        let index: usize = self.hash(key);
        match self.buckets[index].iter().find(|x| x.0 == *key) {
            Some(tuple) => Some(&tuple.1),
            None => None,
        }
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
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
    pub fn contains_key(&self, key: &K) -> bool {
        let index: usize = self.hash(key);
        self.buckets[index].iter().any(|x| x.0 == *key)
    }
    fn resize(&mut self) {
        let new_len: usize = self.len * 2;
        self.len = new_len; // I do not now if this is safe... But to use hash, i need it
        // as it is condensed into the method, perhaps hash should be free function...

        let mut new_buckets: Vec<Vec<(K, V)>> = (0..new_len).map(|_| Vec::new()).collect();
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
    /// Maps a key to a bucket index using `DefaultHasher` modulo the bucket count.
    pub fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.len
    }
}

// Tests //
#[cfg(test)]
mod tests {
    use super::*;

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

        for i in 0..20 {
            let expected_bucket = dict.hash(&i);
            let found = dict.buckets[expected_bucket].iter().any(|(k, _)| *k == i);
            assert!(found, "klucz {} nie jest w kubełku {}", i, expected_bucket);
        }
        let total: usize = dict.buckets.iter().map(|b| b.len()).sum();
        assert_eq!(total, 20);
    }
}
