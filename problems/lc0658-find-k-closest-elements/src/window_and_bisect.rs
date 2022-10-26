pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        assert!(k >= 1);
        assert!(arr.len() >= 1);

        let k = k as usize;

        let mut lo = 0;
        let mut hi = arr.len() - 1;

        let mut best = Option::<(usize, i32, i32)>::None;

        loop {
            let idx = (lo + hi) / 2;
            let val = arr[idx];

            // eprintln!("[{:?}]={:?}", idx, val);
            // eprintln!("    [{:?}..={:?}]", lo, hi);
            // eprintln!("    cmp: {:?}", val.cmp(&x));

            match (best, (x - val).abs()) {
                (None, this) => best = Some((idx, this, val)),
                (Some((_idx, that, that_val)), this) if this == that && val < that_val =>
                    best = Some((idx, this, val)),
                (Some((_idx, that, _that_val)), this) if that > this =>
                    best = Some((idx, this, val)),
                (Some((_idx, that, _that_val)), this) => assert!(this >= that),
            }

            match (lo == hi, val.cmp(&x)) {
                (false, Less) => lo = idx + 1,
                (false, Greater) => hi = idx,
                (_, Equal) => break,
                (true, _) => break,
            }
        }
        let (best_idx, _best_diff, _best_val) = best.expect("arr is not empty.");

        // eprintln!("best: [{:?}]={:?}", best_idx, arr[best_idx]);

        let mut lo = best_idx;
        let mut hi = best_idx;

        while hi - lo + 1 < k {
            // eprintln!("- [{:?}..{:?}] = {:?}...{:?}", lo, hi, arr[lo], arr[hi]);
            // eprintln!("  L: {:?}", lo.checked_sub(1).and_then(|idx| arr.get(idx)).copied());
            // eprintln!("  R: {:?}", arr.get(hi).copied());

            let left_candidate = lo
                .checked_sub(1)
                .and_then(|idx| arr.get(idx))
                .copied()
                .map(|left| (x - left).abs());
            let right_candidate = arr.get(hi + 1).copied().map(|right| (x - right).abs());

            match (left_candidate, right_candidate) {
                (None, None) => break,
                (Some(_), None) => lo -= 1,
                (None, Some(_)) => hi += 1,
                (Some(left), Some(right)) if left <= right => lo -= 1,
                (Some(left), Some(right)) => {
                    assert!(left > right);
                    hi += 1;
                },
            }
        }

        arr[lo..(hi + 1)].to_vec()
    }
}
