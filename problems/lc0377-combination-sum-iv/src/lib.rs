pub struct Solution;

const MAX_TARGET: i32 = 1000;

impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 200);
        assert!(target >= 1);
        assert!(target <= MAX_TARGET);

        nums.sort();

        let mut memo = [0; MAX_TARGET as usize + 1];
        memo[0] = 1;

        for i in 1..=target {
            memo[index(i)] = calculate(i, &nums[..], &memo[0..index(i)]);
        }

        // eprintln!("  memo: {:?}", &memo[0..=index(target)]);

        memo[index(target)]
    }
}

fn calculate(target: i32, nums: &[i32], memo: &[i32]) -> i32 {
    assert!(memo.len() == target as usize);

    nums.into_iter()
        .copied()
        .take_while(|n| *n <= target)
        .map(|n| {
            let complement = target - n;

            // eprintln!("    | {:?} = {:?} + {:?}", target, n, complement);
            // eprintln!("     | memo[{:?}] = {:?}", complement, memo[index(complement)]);

            memo[index(complement)]
        })
        .sum()
}

fn index(target: i32) -> usize {
    target as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_all_cases() {
        for &(nums, target, exp) in CASES {
            eprintln!("{:?} | {:?} => {:?}", nums, target, exp);
            assert_eq!(Solution::combination_sum4(nums.to_vec(), target), exp);
        }
    }

    const CASES: &[(&[i32], i32, i32)] =
        &[(&[9], 3, 0), (&[1], 1, 1), (&[1], 2, 1), (&[1], 3, 1), (&[1, 2, 3], 4, 7)];
}
