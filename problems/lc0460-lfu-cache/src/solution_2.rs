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

    pub fn dump(&self) {
        eprintln!("===");
        eprintln!(" cap: {}", self.0.capacity);
        eprintln!(" keys: {:?}", self.0.keys);
        eprintln!(" hits: {:?}", self.0.hits);
        eprintln!(" queue:");
        for (idx, item) in self.0.queue.as_ref().into_iter().enumerate() {
            eprintln!(
                "  #{}\t{} => {};\t[{}];\tprev: {:?}\tnext: {:?}",
                idx, item.data.key, item.data.value, item.data.hits, item.prev, item.next
            );
        }
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

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn bump_hit(&mut self, idx: usize) {
        let hit_count = self.queue.as_ref()[idx].data.hits;
        let (_prev, next) = self.queue.unlink(idx);
        self.exclude_from_hits(idx, hit_count, next);
        self.queue.as_mut()[idx].data.hits = hit_count + 1;
        self.insert(idx);
    }

    fn insert(&mut self, idx: usize) {
        let hit_count = self.queue.as_ref()[idx].data.hits;
        let mut inserted = false;

        for (_hit_count, before_idx) in self.hits.range(..=hit_count).next_back() {
            let before_idx = *before_idx;

            self.queue.insert_before(before_idx, idx);
            inserted = true;
            break;
        }
        if !inserted {
            self.queue.insert_last(idx);
        }
        self.hits.insert(hit_count, idx);
    }

    fn exclude_from_hits(&mut self, idx: usize, hit_count: usize, next: Option<usize>) {
        let leader_idx = self
            .hits
            .get(&hit_count)
            .copied()
            .expect("There must be something registered under that hit-count!");

        let next_hit_count = next.map(|next| self.queue.as_ref()[next].data.hits);

        match (leader_idx == idx, next, next_hit_count == Some(hit_count)) {
            (true, Some(next), true) => {
                self.hits.insert(hit_count, next);
            }
            (true, _, _) => {
                self.hits.remove(&hit_count);
            }
            (false, _, _) => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Queue<T> {
    vec: Vec<QItem<T>>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<T> AsRef<[QItem<T>]> for Queue<T> {
    fn as_ref(&self) -> &[QItem<T>] {
        self.vec.as_ref()
    }
}
impl<T> AsMut<[QItem<T>]> for Queue<T> {
    fn as_mut(&mut self) -> &mut [QItem<T>] {
        self.vec.as_mut()
    }
}

impl<T> Queue<T> {
    pub fn new(vec: Vec<QItem<T>>) -> Self {
        Self {
            vec,
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    // pub fn is_empty(&self) -> bool {
    //     self.assert_head_and_tail();

    //     self.head.is_none()
    // }

    pub fn unlink_tail(&mut self) -> Option<(usize, Option<usize>)> {
        let idx = self.tail?;
        let (prev, next) = self.unlink(idx);
        assert!(next.is_none());
        Some((idx, prev))
    }

    pub fn unlink(&mut self, idx: usize) -> (Option<usize>, Option<usize>) {
        let prev = self.as_mut()[idx].prev.take();
        let next = self.as_mut()[idx].next.take();

        if let Some(prev) = prev {
            self.as_mut()[prev].next = next;
        } else {
            self.head = next;
        }

        if let Some(next) = next {
            self.as_mut()[next].prev = prev;
        } else {
            self.tail = prev;
        }

        self.assert_head_and_tail();

        (prev, next)
    }

    pub fn push(&mut self, data: T) -> usize {
        let idx = self.vec.len();
        self.vec.push(QItem::new(data));
        idx
    }

    pub fn insert_after(&mut self, after: usize, inserted: usize) {
        assert_ne!(after, inserted);

        self.assert_head_and_tail();
        self.as_ref()[inserted].assert_unlinked();

        self.as_mut()[inserted].prev = Some(after);
        let next = self.as_mut()[after].next.replace(inserted);

        if let Some(next) = next {
            self.as_mut()[next].prev = Some(inserted);
            self.as_mut()[inserted].next = Some(next);
        } else {
            self.tail = Some(inserted);
        }

        self.assert_head_and_tail();
    }

    pub fn insert_before(&mut self, before: usize, inserted: usize) {
        assert_ne!(before, inserted);

        self.assert_head_and_tail();
        self.as_ref()[inserted].assert_unlinked();

        self.as_mut()[inserted].next = Some(before);
        let prev = self.as_mut()[before].prev.replace(inserted);

        if let Some(prev) = prev {
            self.as_mut()[prev].next = Some(inserted);
            self.as_mut()[inserted].prev = Some(prev);
        } else {
            self.head = Some(inserted);
        }

        self.assert_head_and_tail();
    }

    pub fn insert_last(&mut self, inserted: usize) {
        self.assert_head_and_tail();
        self.as_ref()[inserted].assert_unlinked();

        if let Some(tail) = self.tail {
            self.insert_after(tail, inserted);
        } else {
            self.head = Some(inserted);
            self.tail = Some(inserted);
        }

        self.assert_head_and_tail();
    }

    // pub fn insert_first(&mut self, inserted: usize) {
    //     self.assert_head_and_tail();
    //     self.as_ref()[inserted].assert_unlinked();

    //     if let Some(head) = self.head {
    //         self.insert_before(head, inserted);
    //     } else {
    //         self.head = Some(inserted);
    //         self.tail = Some(inserted);
    //     }

    //     self.assert_head_and_tail();
    // }
}

impl<T> Queue<T> {
    fn assert_head_and_tail(&self) {
        assert_eq!(self.head.is_none(), self.tail.is_none());
    }
}

#[derive(Debug, Clone)]
pub struct QItem<T> {
    data: T,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<T> QItem<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }

    fn assert_unlinked(&self) {
        assert!(self.prev.is_none());
        assert!(self.next.is_none());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue() {
        let mut q = Queue::<usize>::new(Vec::with_capacity(3));

        let idx_0 = q.push(0);
        q.insert_last(idx_0);

        let idx_1 = q.push(1);
        q.insert_after(idx_0, idx_1);

        let idx_2 = q.push(2);
        q.insert_after(idx_1, idx_2);

        eprintln!("{:#?}", q);

        let (should_be_0, should_be_2) = q.unlink(idx_1);

        assert_eq!(should_be_0, Some(idx_0));
        assert_eq!(should_be_2, Some(idx_2));

        eprintln!("{:#?}", q);

        q.insert_last(idx_1);

        eprintln!("{:#?}", q);

        let (should_be_1, should_be_2) = q.unlink_tail().unwrap();
        assert_eq!(should_be_1, idx_1);
        assert_eq!(should_be_2, Some(idx_2));

        eprintln!("{:#?}", q);

        q.insert_before(idx_0, idx_1);

        eprintln!("{:#?}", q);

        let (should_be_none, should_be_0) = q.unlink(idx_1);
        assert_eq!(should_be_none, None);
        assert_eq!(should_be_0, Some(idx_0));

        eprintln!("{:#?}", q);

        q.insert_after(idx_0, idx_1);

        eprintln!("{:#?}", q);
    }
}
