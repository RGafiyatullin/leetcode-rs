pub struct Solution;

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let target = target.as_bytes();
        let mut mask = vec![false; target.len()];
        let stamp = stamp.as_bytes();

        let mut acc = vec![];
        loop {
            let acc_len = acc.len();

            for i in 0..=(target.len() - stamp.len()) {
                let left = target
                    .into_iter()
                    .copied()
                    .enumerate()
                    .skip(i)
                    .take(stamp.len());
                let right = stamp.into_iter().copied();

                let matches = left.zip(right).all(|((idx, l), r)| mask[idx] || l == r);
                let matches = matches
                    && mask
                        .iter()
                        .copied()
                        .skip(i)
                        .take(stamp.len())
                        .any(|used| !used);

                if matches {
                    acc.push(i);
                    mask.iter_mut()
                        .skip(i)
                        .take(stamp.len())
                        .for_each(|m| *m = true);
                    break;
                }
            }

            if acc_len == acc.len() {
                return vec![];
            }

            if mask.iter().copied().all(std::convert::identity) {
                break;
            }
        }

        acc.into_iter().rev().map(|idx| idx as i32).collect()
    }
}
