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
    use std::{
        collections::{BTreeMap, HashMap},
        hash::Hash,
    };

    #[derive(Debug, Clone)]
    pub struct Cache<K, V> {
        capacity: usize,
        queue: Vec<QEntry<K, V>>,
        q_head: Option<usize>,
        q_tail: Option<usize>,

        map: HashMap<K, usize>,

        hit_index: BTreeMap<usize, usize>,
    }

    impl<K, V> Cache<K, V> {
        pub fn new(capacity: usize) -> Self {
            // eprintln!("NEW [capacity: {}]", capacity);

            Self {
                capacity,
                map: HashMap::with_capacity(capacity),

                queue: Vec::with_capacity(capacity),
                q_head: None,
                q_tail: None,

                hit_index: Default::default(),
            }
        }
    }

    impl<K, V> Cache<K, V>
    where
        K: Hash + Eq + Clone,
    {
        pub fn get(&mut self, key: K) -> Option<&V> {
            if self.capacity == 0 {
                return None;
            }

            // eprintln!("GET");

            let q_idx = self.map.get(&key).copied()?;
            self.q_hit(q_idx);
            Some(&self.queue[q_idx].value)
        }

        pub fn put(&mut self, key: K, value: V) {
            if self.capacity == 0 {
                return;
            }

            // eprintln!("PUT");

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

                    self.q_insert_by_hit_count(0, q_idx);
                    self.hit_index.insert(0, q_idx);
                } else {
                    if let Some(q_evicted_idx) = self.q_tail {
                        let (_prev_of_evicted, next_of_evicted) = self.q_cut(q_evicted_idx);
                        assert!(self.map.remove(&self.queue[q_evicted_idx].key).is_some());

                        let hits_of_evicted_record = self.queue[q_evicted_idx].hits;

                        self.queue[q_evicted_idx].key = key.to_owned();
                        self.queue[q_evicted_idx].value = value;
                        self.queue[q_evicted_idx].hits = 0;
                        self.map.insert(key, q_evicted_idx);

                        if let Some(head_candidate) = next_of_evicted {
                            self.q_update_hit_index(Some(head_candidate), hits_of_evicted_record);
                        }

                        self.q_insert_by_hit_count(0, q_evicted_idx);
                        self.hit_index.insert(0, q_evicted_idx);
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
            // eprintln!("q_hit({})", q_idx);

            let (prev_opt, _next_opt) = self.q_cut(q_idx);

            let hits_before = self.queue[q_idx].hits;
            let hits_after = hits_before + 1;

            self.queue[q_idx].hits = hits_after;

            if let Some(_prev) = prev_opt {
                // find a place in the queue to insert the entry (using the hit-index)
                self.q_insert_by_hit_count(hits_after, q_idx);
            } else {
                // insert that entry to the head of the queue.
                self.q_insert_to_head(q_idx);
            }

            self.hit_index.insert(hits_after, q_idx);
        }

        fn q_update_hit_index(&mut self, head_candidate: Option<usize>, expected_hit_count: usize) {
            // eprintln!("q_update_hit_index({:?}, {})", head_candidate, expected_hit_count);

            if let Some(head_candidate) = head_candidate {
                if self.queue[head_candidate].hits != expected_hit_count {
                    // assert!(self.queue[head_candidate].hits < expected_hit_count);
                    self.hit_index.remove(&expected_hit_count);
                } else {
                    self.hit_index.insert(expected_hit_count, head_candidate);
                }
            } else {
                self.hit_index.remove(&expected_hit_count);
            }
        }

        fn q_insert_by_hit_count(&mut self, hit_count: usize, q_idx: usize) {
            // find a place in the queue to insert the entry (using the hit-index)
            // eprintln!(
            //     "q_insert_by_hit_count(hit_count: {}, idx: {})",
            //     hit_count, q_idx
            // );
            // eprintln!(" self.hit_index: {:?}", self.hit_index);
            let mut range = self.hit_index.range(hit_count..);
            // eprintln!(" range: {:?}", range);

            if let Some((candidate_hits, candidate_idx)) = range.next() {
                let candidate_hits = *candidate_hits;
                let candidate_idx = *candidate_idx;
                // eprintln!(
                //     " candidate_hits: {}, candidate_idx: {}",
                //     candidate_hits, candidate_idx
                // );

                if candidate_hits == hit_count {
                    self.q_insert_before(candidate_idx, q_idx);
                } else {
                    assert!(candidate_hits > hit_count);

                    self.q_insert_after(candidate_idx, q_idx);
                }
            } else {
                self.q_insert_to_head(q_idx);
            }
        }

        fn q_insert_after(&mut self, q_after: usize, q_idx: usize) {
            // eprintln!("q_insert_after({}, {})", q_after, q_idx);

            self.queue[q_idx].prev = Some(q_after);
            self.queue[q_idx].next = self.queue[q_after].next.replace(q_idx);
            if let Some(next) = self.queue[q_idx].next {
                self.queue[next].prev = Some(q_idx);
            } else {
                assert_eq!(self.q_tail, Some(q_after));

                self.q_tail = Some(q_idx);
            }
        }
        fn q_insert_before(&mut self, q_before: usize, q_idx: usize) {
            // eprintln!("q_insert_before({}, {})", q_before, q_idx);

            self.queue[q_idx].next = Some(q_before);
            self.queue[q_idx].prev = self.queue[q_before].prev.replace(q_idx);
            if let Some(prev) = self.queue[q_idx].prev {
                self.queue[prev].next = Some(q_idx);
            } else {
                assert_eq!(self.q_head, Some(q_before));

                self.q_head = Some(q_idx);
            }
        }

        fn q_insert_to_head(&mut self, q_idx: usize) {
            // eprintln!("q_insert_to_head({})", q_idx);

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
            // eprintln!("q_cut({})", q_idx);

            let prev_opt = self.queue[q_idx].prev.take();
            let next_opt = self.queue[q_idx].next.take();

            // eprintln!(
            //     "cutting q[{}] => [p: {:?}, n: {:?}]",
            //     q_idx, prev_opt, next_opt
            // );

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

            let hit_count = self.queue[q_idx].hits;
            if self.hit_index.get(&hit_count).copied() == Some(q_idx) {
                self.q_update_hit_index(next_opt, hit_count);
            }

            (prev_opt, next_opt)
        }
    }
}
