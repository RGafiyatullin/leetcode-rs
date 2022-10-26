pub struct Solution;

const MIN_SEQ_LEN: usize = 3;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let state = nums.into_iter().enumerate().fold(State::default(), State::fold).finish();

        assert!(state.active.is_empty());

        // eprintln!("{:?}", state);

        !state.complete.is_empty() && state.failed.is_empty()
    }
}

#[derive(Debug, Default)]
struct State {
    active: Vec<Vec<(usize, i32)>>,
    complete: Vec<Vec<(usize, i32)>>,
    failed: Vec<Vec<(usize, i32)>>,
}

impl State {
    fn fold(mut self, (idx, n): (usize, i32)) -> Self {
        if let Some(s) = self
            .active
            .iter_mut()
            .find(|s| n - s.last().copied().expect("No empty sequences there").1 == 1)
        {
            s.push((idx, n))
        } else {
            self.active.push(vec![(idx, n)]);
        }

        let mut active = vec![];
        for s in self.active.into_iter() {
            match (n - s.last().copied().expect("No empty sequences there").1, s.len()) {
                (gt1, gte_min_len) if gt1 > 1 && gte_min_len >= MIN_SEQ_LEN =>
                    self.complete.push(s),
                (gt1, _) if gt1 > 1 => self.failed.push(s),
                (_, _) => active.push(s),
            }
        }
        self.active = active;
        self.active.sort_by_key(|s| s.len());

        self
    }

    fn finish(mut self) -> Self {
        while let Some(a) = self.active.pop() {
            if a.len() >= MIN_SEQ_LEN {
                self.complete.push(a);
            } else {
                self.failed.push(a);
            }
        }
        self
    }
}
