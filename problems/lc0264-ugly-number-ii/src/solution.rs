pub struct Solution;

#[cfg(feature = "naive")]
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::collections::HashSet;

        let n = n as usize;
        const FACTORS: &[usize] = &[2, 3, 5];

        let mut memo = HashSet::<usize>::from_iter([1]);
        let mut i = 1;

        while memo.len() < n {
            i += 1;

            // eprint!("i: {:?} (found so far: {});\t", i, memo.len());

            for &factor in FACTORS {
                if i % factor == 0 {
                    let r = i / factor;
                    let is_ugly = memo.contains(&r);

                    // eprint!(" {} / {} == {} [ugly: {}]", i, factor, r, is_ugly);

                    if is_ugly {
                        memo.insert(i);
                    }
                    break
                }
            }

            // eprintln!();
        }

        i as i32
    }
}

#[cfg(feature = "build-a-tree")]
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::collections::BTreeSet;

        const FACTORS: &[usize] = &[2, 3, 5];
        let factor_max = FACTORS.into_iter().copied().max().expect("FACTORS is not empty");
        let n = n as usize;

        // let mut uglies = BTreeSet::<usize>::from_iter([1]);
        let mut uglies = [1].iter().copied().collect::<BTreeSet<usize>>();

        while uglies.len() < n {
            let prevs = uglies.range(..).copied().collect::<Vec<_>>();
            let prev_largest = prevs.last().copied().unwrap_or(1);
            let threshold = prev_largest * factor_max;

            uglies.insert(threshold);

            for prev in prevs {
                for &factor in FACTORS {
                    let mut acc = prev * factor;

                    while acc < threshold {
                        uglies.insert(acc);
                        acc *= factor;
                    }
                }
            }
        }

        uglies.range(..).nth(n - 1).copied().unwrap() as i32
    }
}
