use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct LFUCache(Cache<i32, i32>);

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self(Cache::new(capacity as usize))
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.0.get(&key).copied().unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.0.put(key, value);
    }
}

#[derive(Debug, Clone)]
struct KV<K, V> {
    key: K,
    value: V,
    hits: usize,
}

#[derive(Debug, Clone)]
struct Cache<K, V> {
    capacity: usize,
    queue: Queue<KV<K, V>>,
    keys: HashMap<K, usize>,
    hits: BTreeMap<usize, usize>,

    _pd: std::marker::PhantomData<(K, V)>,
}

mod cache_hits;
mod cache_methods;

mod queue;
use queue::Queue;

mod qitem;
use qitem::QItem;
