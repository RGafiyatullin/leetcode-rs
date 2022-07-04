#[derive(Debug, Clone)]
pub struct LFUCache(lfu_cache::Cache<i32, i32>);

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self(lfu_cache::Cache::new(capacity as usize))
    }

    pub fn get(&mut self, key: i32) -> i32 {
        unimplemented!()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        unimplemented!()
    }
}

mod lfu_cache {
    use std::{collections::HashMap, hash::Hash};

    #[derive(Debug, Clone)]
    pub struct Cache<K, V> {
        capacity: usize,
        queue: Vec<QEntry<K, V>>,
        q_head: Option<usize>,
        q_tail: Option<usize>,

        map: HashMap<K, usize>,
    }

    impl<K, V> Cache<K, V> {
        pub fn new(capacity: usize) -> Self {
            assert!(capacity > 0);

            Self {
                capacity,
                map: HashMap::with_capacity(capacity),

                queue: Vec::with_capacity(capacity),
                q_head: None,
                q_tail: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct QEntry<K, V> {
        key: K,
        value: V,

        prev: Option<usize>,
        next: Option<usize>,
    }
}
