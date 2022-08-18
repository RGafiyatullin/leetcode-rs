pub struct Solution;

const SCAN_TO_BISECT_THRESHOLD: usize = 0;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);

        nums.sort_unstable();

        // eprintln!("SORTED: {:?}\n", nums);

        nums.iter()
            .copied()
            .enumerate()
            .map(|(idx, smallest)| {
                let nums = &nums[idx + 1..];

                // eprintln!("SMALLEST: {:?} || {:?}", smallest, nums);

                nums.iter()
                    .copied()
                    .enumerate()
                    .scan(0, |skip, (idx, second)| {
                        let nums = &nums[idx + 1..];

                        // eprintln!(
                        //     "  SECOND: {:?} || {:?} + {:?}",
                        //     second,
                        //     &nums[0..*skip],
                        //     &nums[*skip..]
                        // );

                        let largest_hi_exc = smallest + second;

                        let count =
                            *skip + count_valid_largest_sides(&nums[*skip..], largest_hi_exc);

                        // eprintln!("    THIRDS: {:?}", &nums[0..count]);

                        if count > 0 {
                            *skip = count - 1;
                            Some(count)
                        } else {
                            *skip = 0;
                            Some(0)
                        }
                    })
                    // .inspect(|count| eprintln!("    // {:?}", *count))
                    .sum::<usize>()
            })
            // .inspect(|count| eprintln!("// {:?}", *count))
            .sum::<usize>() as i32
    }
}

fn count_valid_largest_sides(nums: &[i32], hi_exc: i32) -> usize {
    if nums.is_empty() {
        return 0;
    }

    if nums.len() < SCAN_TO_BISECT_THRESHOLD {
        count_lt_scan(nums, hi_exc)
    } else {
        count_lt_bisect(nums, hi_exc)
    }
}

pub fn count_lt_scan(nums: &[i32], hi_exc: i32) -> usize {
    nums.into_iter()
        .copied()
        .take_while(|&largest| largest < hi_exc)
        .count()
}

pub fn count_lt_bisect(nums: &[i32], hi_exc: i32) -> usize {
    use std::cmp::Ordering::*;

    let mut lo = 0;
    let mut hi = nums.len() - 1;

    let mut count = nums.len();

    loop {
        let idx = (hi + lo) / 2;
        let val = nums[idx];

        match (hi == lo, val.cmp(&hi_exc)) {
            (false, Greater | Equal) => {
                count -= hi - idx;
                hi = idx;
            }
            (false, Less) => lo = idx + 1,

            (true, Greater | Equal) => {
                count -= 1;
                break;
            }
            (true, Less) => break,
        }
    }

    // eprintln!("          COUNT: {:?}", count);
    count
}
