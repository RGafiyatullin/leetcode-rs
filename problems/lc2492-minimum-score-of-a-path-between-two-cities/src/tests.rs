use crate::solution::Solution;

#[test]
fn test_01() {
    assert_eq!(
        Solution::min_score(4, vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]),
        5
    );
}

#[test]
fn test_02() {
    assert_eq!(Solution::min_score(4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]]), 2);
}
