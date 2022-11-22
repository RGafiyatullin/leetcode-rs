pub struct Solution;

use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let squares = (1..).map(|x| x * x).take_while(|s| *s <= n).collect::<BTreeSet<_>>();

        let mut tasks = [(n, 0)].iter().copied().collect::<VecDeque<_>>();
        let mut memo = vec![usize::MAX; n as usize + 1];
        let mut best = usize::MAX;

        while let Some((x, score)) = tasks.pop_front() {
            if x == 0 {
                best = std::cmp::min(score, best);
            } else if score < best && memo[x as usize] > score {
                memo[x as usize] = score;
                for s in squares.range(0..=x).rev().copied() {
                    tasks.push_back((x - s, score + 1));
                }
            }
        }

        best as i32
    }
}
