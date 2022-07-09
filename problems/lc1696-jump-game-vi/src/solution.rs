pub struct Solution;

impl Solution {
    pub fn max_result(cost: Vec<i32>, k: i32) -> i32 {
        assert!(cost.len() >= 1);
        assert!(cost.len() <= 100000);
        assert!(k >= 1);
        assert!(k <= 100000);

        let max_step_len: usize = k as usize;

        let mut memo = Memo::new(vec![Option::<i32>::None; max_step_len + 1]);

        for step_value in cost.iter().rev().copied().skip(1) {
            memo.scroll_back();

            let mut max_step_value = Option::<i32>::None;

            for steps_back in 1..=max_step_len {
                let candidate = memo.get(steps_back).unwrap_or(0) + step_value;
                max_step_value = Some(std::cmp::max(
                    candidate,
                    max_step_value.unwrap_or(candidate),
                ));
            }

            *memo.get_mut(0) = max_step_value;
        }

        memo.get(0).expect("cost.len() >= 1. QED") + cost.last().expect("cost.len() >= 1. QED")
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

    pub fn get_mut(&mut self, index: usize) -> &mut T
    where
        V: AsMut<[T]>,
    {
        let slice = self.slice.as_mut();
        assert!(
            index < slice.len(),
            "Underlying collection's size is {}",
            slice.len()
        );

        &mut slice[(self.cursor + index) % slice.len()]
    }

    pub fn scroll_back(&mut self)
    where
        V: AsRef<[T]>,
    {
        self.cursor = self
            .cursor
            .checked_sub(1)
            .unwrap_or(self.slice.as_ref().len() - 1);
    }
}
