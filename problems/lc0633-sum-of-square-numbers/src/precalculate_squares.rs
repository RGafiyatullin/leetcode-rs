pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        assert!(c >= 0);

        let c = c as usize;

        let mut squares = Vec::new();

        for n in 0 as usize.. {
            let n_squared = n * n;
            if n_squared <= c as usize {
                // eprintln!("# {:?}^2 = {:?}", n, n_squared);
                squares.push((n, n_squared));
            } else {
                break
            }
        }
        // eprintln!("Precalculated {:?} squares", squares.len());

        for (idx, (n, n_squared)) in squares.iter().copied().enumerate().rev() {
            // eprintln!("Trying [{:?}]: {:?}^2 = {:?}...", idx, n, n_squared);

            let complement = c - n_squared;
            if complement > n_squared {
                break
            }

            // eprintln!(" Complement: {:?}", complement);

            let (mut lo, mut hi) = (0, idx);
            let found_opt = loop {
                let candidate_idx = (lo + hi) / 2;
                let (candidate_n, candidate_n_squared) = squares[candidate_idx];

                // eprintln!(
                //     "  [{:?}]: {:?}^2 = {:?} ?",
                //     candidate_idx, candidate_n, candidate_n_squared
                // );

                match (lo == hi, candidate_n_squared.cmp(&complement)) {
                    (_, Equal) => break Some((candidate_n, candidate_n_squared)),
                    (true, Less | Greater) => break None,

                    (false, Greater) => hi = candidate_idx,
                    (false, Less) => lo = candidate_idx + 1,
                }
            };
            if let Some((complement_n, _complement_n_squared)) = found_opt {
                assert_eq!(n * n + complement_n * complement_n, c);
                // eprintln!(
                //     "Found! {}^2 + {}^2 = {:?} + {:?} = {:?}",
                //     n, complement_n, n_squared, _complement_n_squared, c
                // );
                return true
            }
        }

        return false
    }
}
