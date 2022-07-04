#[derive(Debug, Clone)]
pub struct LRUCache(lru_cache::Cache<i32, i32>);

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self(lru_cache::Cache::new(capacity as usize))
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.0.get(key).copied().unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.0.put(key, value)
    }
}

mod lru_cache {
    use std::{collections::HashMap, hash::Hash};

    #[derive(Debug, Clone)]
    struct QEntry<K, V> {
        key: K,
        value: V,
        prev: Option<usize>,
        next: Option<usize>,
    }

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
    impl<K, V> Cache<K, V>
    where
        K: Hash + Eq + Clone,
    {
        pub fn get(&mut self, key: K) -> Option<&V> {
            let idx = self.map.get(&key).copied()?;
            self.q_pop_to_the_top(idx);
            Some(&self.queue[idx].value)
        }

        pub fn put(&mut self, key: K, value: V) {
            if let Some(q_idx) = self.map.get(&key).copied() {
                self.queue[q_idx].value = value;
                self.q_pop_to_the_top(q_idx);
            } else {
                if self.queue.len() < self.capacity {
                    let q_entry = QEntry { key: key.to_owned(), value, prev: None, next: None };
                    let q_idx = self.queue.len();
                    self.queue.push(q_entry);
                    self.q_paste_to_top(q_idx);

                    self.map.insert(key, q_idx);
                } else {
                    if let Some(q_evicted_idx) = self.q_tail {
                        self.q_cut(q_evicted_idx);
                        assert!(self.map.remove(&self.queue[q_evicted_idx].key).is_some());

                        self.queue[q_evicted_idx].key = key.to_owned();
                        self.queue[q_evicted_idx].value = value;
                        self.map.insert(key, q_evicted_idx);

                        self.q_paste_to_top(q_evicted_idx);
                    } else {
                        panic!("Need to evict an entry but there is no tail of the queue")
                    }
                }
            }
        }

        fn q_paste_to_top(&mut self, q_idx: usize) {
            match (self.q_head, self.q_tail) {
                (None, None) => {
                    self.q_head = Some(q_idx);
                    self.q_tail = Some(q_idx);
                },

                (Some(q_head), Some(_q_tail)) => {
                    self.queue[q_head].prev = Some(q_idx);
                    self.queue[q_idx].next = Some(q_head);
                    self.q_head = Some(q_idx);
                },

                (q_head, q_tail) => panic!("Invalid queue ends: {:?}", (q_head, q_tail)),
            }
        }

        fn q_cut(&mut self, q_idx: usize) {
            let prev_opt = self.queue[q_idx].prev.take();
            let next_opt = self.queue[q_idx].next.take();

            // eprintln!("cutting q[{}] => [p: {:?}, n: {:?}]", q_idx, prev_opt, next_opt);

            let change_head = if let Some(prev) = prev_opt {
                // assert not is-head
                assert_ne!(self.q_head, Some(q_idx));

                self.queue[prev].next = next_opt;
                None
            } else {
                // assert is-head
                assert_eq!(self.q_head, Some(q_idx));

                Some(next_opt)
            };

            // eprintln!(" change-head: {:?}", change_head);

            let change_tail = if let Some(next) = next_opt {
                // assert not is-tail
                assert_ne!(self.q_tail, Some(q_idx));

                self.queue[next].prev = prev_opt;
                None
            } else {
                // assert is-tail
                assert_eq!(self.q_tail, Some(q_idx));

                Some(prev_opt)
            };

            // eprintln!(" change-tail: {:?}", change_tail);

            if let Some(q_head) = change_head {
                self.q_head = q_head;
            }
            if let Some(q_tail) = change_tail {
                self.q_tail = q_tail;
            }
        }

        fn q_pop_to_the_top(&mut self, q_idx: usize) {
            let _q_entry = self.q_cut(q_idx);
            self.q_paste_to_top(q_idx);
        }
    }
}
