pub struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);

        nums.sort_unstable();

        nums.iter()
            .copied()
            .enumerate()
            .rev()
            .map(|(idx, val)| {
                // eprintln!("LARGEST: {:?} || {:?}", val, &nums[0..idx]);
                let count = count_valid_pairs(&nums[0..idx], val);
                // eprintln!("  COUNT: {:?}", count);

                count
            })
            .sum::<usize>() as i32
    }
}

fn count_valid_pairs(nums: &[i32], largest: i32) -> usize {
    let lo_exc = largest / 2;

    // eprintln!("    PAIRS? [largest: {:?}; lo_exc: {:?}]", largest, lo_exc);

    nums.iter()
        .copied()
        .enumerate()
        .rev()
        .take_while(|&(_idx, val)| val > lo_exc)
        .map(|(idx, val)| {
            let rest = &nums[0..idx];

            // eprintln!("        SECOND: {:?} || {:?}", val, rest);
            count_valid_smallest_sides(rest, largest, val)
        })
        .sum()
}

fn count_valid_smallest_sides(nums: &[i32], largest: i32, second: i32) -> usize {
    if nums.is_empty() {
        return 0
    }

    use std::cmp::Ordering::*;

    assert!(largest >= second);

    let lo_exc = largest - second;

    // eprintln!("        THIRD > {:?}", lo_exc);

    let mut lo = 0;
    let mut hi = nums.len() - 1;

    let mut count = nums.len();
    // eprintln!("          c: {:?}", count);
    loop {
        let idx = (hi + lo) / 2;
        let val = nums[idx];

        // eprintln!("             | [{:?}]={:?} [{:?}..={:?}]", idx, val, lo, hi);

        match (hi == lo, val.cmp(&lo_exc)) {
            (false, Less | Equal) => {
                count -= idx - lo + 1;
                lo = idx + 1;
            },
            (false, Greater) => hi = idx,

            (true, Greater) => break,
            (true, Less | Equal) => {
                count -= 1;
                break
            },
        }
    }

    // eprintln!("          COUNT: {:?}", count);
    count
}
