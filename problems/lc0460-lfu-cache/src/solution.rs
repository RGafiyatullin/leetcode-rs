#[derive(Debug, Clone)]
pub struct LFUCache(lfu_cache::Cache<i32, i32>);

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self(lfu_cache::Cache::new(capacity as usize))
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.0.get(key).copied().unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.0.put(key, value);
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
            eprintln!("NEW [capacity: {}]", capacity);

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
            eprintln!("GET");

            let q_idx = self.map.get(&key).copied()?;
            self.q_hit(q_idx);
            Some(&self.queue[q_idx].value)
        }

        pub fn put(&mut self, key: K, value: V) {
            eprintln!("PUT");

            if let Some(q_idx) = self.map.get(&key).copied() {
                self.queue[q_idx].value = value;
                self.q_hit(q_idx);
            } else {
                if self.queue.len() < self.capacity {
                    let q_entry = QEntry {
                        key: key.to_owned(),
                        value,
                        hits: 0,
                        prev: None,
                        next: None,
                    };
                    let q_idx = self.queue.len();
                    self.queue.push(q_entry);
                    self.map.insert(key, q_idx);

                    self.q_insert_to_tail_and_bubble(q_idx)
                } else {
                    if let Some(q_evicted_idx) = self.q_tail {
                        self.q_cut(q_evicted_idx);
                        assert!(self.map.remove(&self.queue[q_evicted_idx].key).is_some());

                        self.queue[q_evicted_idx].key = key.to_owned();
                        self.queue[q_evicted_idx].value = value;
                        self.queue[q_evicted_idx].hits = 0;
                        self.map.insert(key, q_evicted_idx);

                        self.q_insert_to_tail_and_bubble(q_evicted_idx)
                    } else {
                        panic!("Need to evict an entry but there is no tail of the queue")
                    }
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    struct QEntry<K, V> {
        key: K,
        value: V,

        hits: usize,

        prev: Option<usize>,
        next: Option<usize>,
    }

    impl<K, V> Cache<K, V>
    where
        K: Hash + Eq + Clone,
    {
        fn q_hit(&mut self, q_idx: usize) {
            eprintln!("q_hit({})", q_idx);

            let (prev_opt, _) = self.q_cut(q_idx);
            self.queue[q_idx].hits += 1;

            if let Some(prev) = prev_opt {
                // starting with prev, find the first entry with `.hits > self.queue[idx].hits`,
                // and insert the entry after it.
                self.q_insert_after_or_bubble(prev, q_idx)
            } else {
                // insert that entry to the head of the queue.
                self.q_insert_to_head(q_idx);
            }
        }

        fn q_insert_after(&mut self, q_after: usize, q_idx: usize) {
            eprintln!("q_insert_after({}, {})", q_after, q_idx);

            self.queue[q_idx].prev = Some(q_after);
            self.queue[q_idx].next = self.queue[q_after].next.replace(q_idx);
            if let Some(next) = self.queue[q_idx].next {
                self.queue[next].prev = Some(q_idx);
            } else {
                assert_eq!(self.q_tail, Some(q_after));

                self.q_tail = Some(q_idx);
            }
        }

        fn q_insert_to_tail_and_bubble(&mut self, q_idx: usize) {
            eprintln!("q_insert_to_tail_and_bubble({})", q_idx);

            if let Some(tail) = self.q_tail {
                self.q_insert_after_or_bubble(tail, q_idx);
            } else {
                self.q_insert_to_head(q_idx);
            }
        }

        fn q_insert_after_or_bubble(&mut self, q_after: usize, q_idx: usize) {
            eprintln!("q_insert_after_or_bubble({}, {})", q_after, q_idx);

            let mut next_candidate = Some(q_after);
            let mut inserted = false;
            while let Some(candidate) = next_candidate.take() {
                if self.queue[candidate].hits > self.queue[q_idx].hits {
                    self.q_insert_after(candidate, q_idx);
                    inserted = true;
                } else {
                    next_candidate = self.queue[candidate].prev;
                }
            }
            if !inserted {
                self.q_insert_to_head(q_idx);
            }
        }

        fn q_insert_to_head(&mut self, q_idx: usize) {
            eprintln!("q_insert_to_head({})", q_idx);

            match (self.q_head, self.q_tail) {
                (None, None) => {
                    self.q_head = Some(q_idx);
                    self.q_tail = Some(q_idx);
                }

                (Some(q_head), Some(_q_tail)) => {
                    self.queue[q_head].prev = Some(q_idx);
                    self.queue[q_idx].next = Some(q_head);
                    self.q_head = Some(q_idx);
                }

                (q_head, q_tail) => panic!("Invalid queue ends: {:?}", (q_head, q_tail)),
            }
        }

        fn q_cut(&mut self, q_idx: usize) -> (Option<usize>, Option<usize>) {
            eprintln!("q_cut({})", q_idx);

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

            (prev_opt, next_opt)
        }
    }
}
