use super::*;

impl<K, V> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            queue: Queue::new(Vec::with_capacity(capacity)),
            keys: Default::default(),
            hits: Default::default(),

            _pd: Default::default(),
        }
    }
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.capacity == 0 {
            return None;
        }

        let idx = self.keys.get(key).copied()?;
        self.bump_hit(idx);
        Some(&self.queue.as_ref()[idx].data.value)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return;
        }

        if let Some(idx) = self.keys.get(&key).copied() {
            let hit_count = self.queue.as_ref()[idx].data.hits;
            let (_prev, next) = self.queue.unlink(idx);
            self.exclude_from_hits(idx, hit_count, next);
            self.queue.as_mut()[idx].data.value = value;
            self.queue.as_mut()[idx].data.hits = hit_count + 1;
            self.insert(idx);
        } else if self.queue.len() < self.capacity {
            let idx = self.queue.push(KV {
                key: key.to_owned(),
                value,
                hits: 1,
            });
            self.insert(idx);
            self.keys.insert(key, idx);
        } else {
            let (idx, _prev) = self
                .queue
                .unlink_tail()
                .expect("Come on! I've ensured the capacity is not zero!!!");
            let hit_count = self.queue.as_ref()[idx].data.hits;
            self.exclude_from_hits(idx, hit_count, None);

            self.keys.remove(&self.queue.as_ref()[idx].data.key);
            self.queue.as_mut()[idx].data = KV {
                key: key.to_owned(),
                value,
                hits: 1,
            };
            self.insert(idx);
            self.keys.insert(key, idx);
        }
    }
}

#[test]
fn i_know_how_to_use_btreemap_range() {
    let map = (1..10).map(|i| (i, ())).collect::<BTreeMap<_, _>>();

    assert_eq!(
        map.range(..5).map(|(k, _)| *k).collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    assert_eq!(map.range(..5).map(|(k, _)| *k).next_back(), Some(4));

    assert_eq!(
        map.range(..=4).map(|(k, _)| *k).collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    assert_eq!(map.range(..=4).map(|(k, _)| *k).next_back(), Some(4));
}
