pub struct Solution;

const MIN_VAL: i32 = 1;
const MAX_VAL: i32 = 100_000;
const DOMAIN_SIZE: usize = (MAX_VAL - MIN_VAL + 1) as usize;
const MAX_ARRAY_SIZE: usize = 100_000;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        assert!(arr.len() >= 2);
        assert!(arr.len() <= MAX_ARRAY_SIZE);
        assert!(arr.len() % 2 == 0);

        let array_len = arr.len();
        let target = arr.len() / 2;

        let (_fine, coarse) = arr.into_iter().fold(
            (vec![0; DOMAIN_SIZE], vec![0; array_len + 1]),
            |(mut fine, mut coarse), val| {
                let fine_idx = fine_idx(val);

                let before = fine[fine_idx];
                let after = before + 1;

                fine[fine_idx] = after;
                if before != 0 {
                    coarse[before] -= 1;
                }
                coarse[after] += 1;

                (fine, coarse)
            },
        );

        // assert_eq!(fine.into_iter().sum::<usize>(), array_len);
        // assert_eq!(
        //     coarse
        //         .iter()
        //         .copied()
        //         .enumerate()
        //         .map(|(f, n)| f * n)
        //         .sum::<usize>(),
        //     array_len
        // );

        let freqs = coarse
            .into_iter()
            .enumerate()
            .rev()
            .filter(|&(_freq, count)| count > 0)
            .flat_map(|(freq, times)| std::iter::repeat(freq).take(times));

        let mut total = 0;
        let mut distinct = 0;

        for q in freqs {
            total += q;
            distinct += 1;
            if total >= target {
                return distinct;
            }
        }

        unreachable!("The sum of all items is expected to be equal to `arr.len()`, which is twice more than `target`. QED")
    }
}

fn fine_idx(val: i32) -> usize {
    assert!(val >= MIN_VAL);
    assert!(val <= MAX_VAL);

    (val - MIN_VAL) as usize
}
