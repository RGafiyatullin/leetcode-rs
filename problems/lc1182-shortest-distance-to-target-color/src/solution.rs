pub struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let empty_sub_index = BTreeSet::<usize>::new();
        let index = colors.into_iter().enumerate().fold(
            HashMap::<i32, BTreeSet<usize>>::new(),
            |mut index, (idx, value)| {
                index.entry(value).or_default().insert(idx);
                index
            },
        );

        queries
            .into_iter()
            .map(|q| {
                let pos = q[0] as usize;
                let target = q[1];

                let sub_index = index.get(&target).unwrap_or(&empty_sub_index);
                let to_the_right = sub_index.range(pos..).next();
                let to_the_left = sub_index.range(..=pos).next_back();

                match (to_the_right, to_the_left) {
                    (None, None) => -1,
                    (Some(idx), None) => (*idx - pos) as i32,
                    (None, Some(idx)) => (pos - *idx) as i32,
                    (Some(idx_r), Some(idx_l)) => std::cmp::min(*idx_r - pos, pos - *idx_l) as i32,
                }
            })
            .collect()
    }
}
