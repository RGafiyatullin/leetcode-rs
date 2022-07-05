use super::*;

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    pub(super) fn bump_hit(&mut self, idx: usize) {
        let hit_count = self.queue.as_ref()[idx].data.hits;
        let (_prev, next) = self.queue.unlink(idx);
        self.exclude_from_hits(idx, hit_count, next);
        self.queue.as_mut()[idx].data.hits = hit_count + 1;
        self.insert(idx);
    }

    pub(super) fn insert(&mut self, idx: usize) {
        let hit_count = self.queue.as_ref()[idx].data.hits;
        let mut inserted = false;

        for (_hit_count, before_idx) in self.hits.range(..=hit_count) {
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

    pub(super) fn exclude_from_hits(&mut self, idx: usize, hit_count: usize, next: Option<usize>) {
        let leader_idx = self.hits.get(&hit_count).copied().expect("There must be something registered under that hit-count!");

        let next_hit_count = next.map(|next| self.queue.as_ref()[next].data.hits);

        match (leader_idx == idx, next, next_hit_count == Some(hit_count)) {
            (true, Some(next), true) => {
                self.hits.insert(hit_count, next);
            },
            (true, _, _) => {
                self.hits.remove(&hit_count);
            },
            (false, _, _) => {}
        }

        // if leader_idx == idx {
        //     if let Some(next) = next {
        //         if self.queue.as_ref()[next].data.hits == hit_count {
        //             self.hits.insert(hit_count, next);
        //         } else {
        //             self.hits.remove(&hit_count);
        //         }
        //     } else {
        //         self.hits.remove(&hit_count);
        //     }
        // }
    }
}
