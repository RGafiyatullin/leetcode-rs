pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn max_result(cost: Vec<i32>, k: i32) -> i32 {
        assert!(cost.len() >= 1);
        assert!(cost.len() <= 100000);
        assert!(k >= 1);
        assert!(k <= 100000);
        let k: usize = k as usize;

        // Since we must go exactly from the first to the last element,
        // we can traverse the `cost` collection in any direction.

        // We traverse each step in the `cost` collection and find out the best score we can get by using it.
        // The best score is the sum of the step's own score and the best score of the previously evaluated steps within the reach of `k` steps.

        // We use two data structures to keep track of the last `k` steps and their best scores:
        // - the `memo` is just a circular buffer of the last `k` steps, when another value is pushed into it over its capacity, it evicts the least recent value.
        // - the `prio` is the tree, mapping the score into the ref-count (i.e. the quantity of times this particular score appeared within the sliding window).

        let mut memo = Memo::new(vec![Option::<i32>::None; k]);
        let mut prio = BTreeMap::<i32, usize>::new();

        for step_value in cost.iter().copied() {
            // the best score for this step is defined as the sum of this step's own score and the best score among the last `k` steps (if there is any).
            let best_step_score =
                prio.range(..).next_back().map(|(k, _)| *k).unwrap_or(0) + step_value;

            // Put this steps score into the tree.
            // If such score is already present in the tree — increase the ref-count of that score.
            *prio.entry(best_step_score).or_insert(0) += 1;

            // Put this step's score into the circular buffer.
            if let Some(evicted) = memo.push(Some(best_step_score)) {
                // If the circular buffer is full, the evicted value (one occurrence of it) should be removed from the tree as well.
                let rc = prio
                    .get_mut(&evicted)
                    .expect("This value has been previously inserted `k` steps ago. QED");
                *rc = rc.saturating_sub(1);
                if *rc == 0 {
                    // If the ref-count of the evicted value is zero, remove that key completely.
                    prio.remove(&evicted);
                }
            }
        }

        // The last step's best score is the best score achievable.
        memo.get(0).expect("cost.len() >= 1. QED")
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Memo<T, V> {
    slice: V,
    cursor: usize,
    _pd: std::marker::PhantomData<T>,
}

impl<T, V> Memo<T, V> {
    pub fn new(memory: V) -> Self {
        Memo {
            slice: memory,
            cursor: 0,
            _pd: Default::default(),
        }
    }

    pub fn get(&self, index: usize) -> &T
    where
        V: AsRef<[T]>,
    {
        let slice = self.slice.as_ref();
        assert!(
            index < slice.len(),
            "Underlying collection's size is {}",
            slice.len()
        );

        &slice[(self.cursor + index) % slice.len()]
    }

    pub fn push(&mut self, value: T) -> T
    where
        V: AsMut<[T]>,
    {
        let slice = self.slice.as_mut();
        self.cursor = self.cursor.checked_sub(1).unwrap_or(slice.len() - 1);
        std::mem::replace(&mut slice[self.cursor % slice.len()], value)
    }
}
