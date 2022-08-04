pub struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        assert!(arr1.len() <= 500);
        assert!(arr2.len() <= 500);

        arr1.into_iter()
            .filter(|left| !arr2.iter().any(|right| (*left - *right).abs() <= d))
            .count() as i32
    }
}

#[test]
fn test_01() {
    assert_eq!(
        Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
        2
    );
}

#[test]
fn test_02() {
    assert_eq!(
        Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
        1
    );
}
