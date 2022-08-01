pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        assert_eq!(n % 2, 0);

        let types_count = candy_type.into_iter().collect::<HashSet<_>>().len();

        std::cmp::min(types_count, n / 2) as i32
    }
}

#[test]
fn test() {
    for (types, exp) in [
        (vec![1, 1, 2, 2, 3, 3], 3),
        (vec![1, 1, 2, 3], 2),
        (vec![6, 6, 6, 6], 1),
    ] {
        assert_eq!(Solution::distribute_candies(types), exp)
    }
}
