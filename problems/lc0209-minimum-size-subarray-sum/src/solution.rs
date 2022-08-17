pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(target >= 1);

        let mut best = Option::<usize>::None;
        let mut left = 0;
        let mut sum = 0;

        for right in 0..nums.len() {
            sum += nums[right];

            // eprintln!("L: {:?} **R**: {:?} | SUM: {:?}", left, right, sum);

            if check(&mut best, target, sum, left, right) {
                while left < right {
                    sum -= nums[left];
                    left += 1;

                    // eprintln!("**L**: {:?} R: {:?} | SUM: {:?}", left, right, sum);

                    if !check(&mut best, target, sum, left, right) {
                        break;
                    }
                }
            }
        }

        best.unwrap_or(0) as i32
    }
}

#[inline]
fn check(best: &mut Option<usize>, target: i32, sum: i32, left: usize, right: usize) -> bool {
    assert!(right >= left, "l: {:?}, r: {:?}", left, right);

    let fit = sum >= target;
    match (fit, *best, right - left + 1) {
        (true, None, len) => *best = Some(len),
        (true, Some(old), new) => *best = Some(std::cmp::min(old, new)),
        (false, _, _) => (),
    }

    fit
}
