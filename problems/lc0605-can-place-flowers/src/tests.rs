use crate::solution::Solution;

#[test]
fn test_01() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
}

#[test]
fn test_02() {
    assert_eq!(Solution::can_place_flowers(vec![0, 1, 0, 0, 0, 1, 0], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![0, 1, 0, 0, 0, 1, 0], 2), false);
}

#[test]
fn test_03() {
    assert_eq!(Solution::can_place_flowers(vec![0, 1, 0, 0, 0, 1, 0], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![0, 1, 0, 0, 0, 1, 0], 2), false);
}

#[test]
fn test_04() {
    assert_eq!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0, 0, 1, 0], 2), true);
    assert_eq!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0, 0, 1, 0], 3), false);
}

#[test]
fn test_05() {
    assert_eq!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0, 0, 1, 0, 0], 3), true);
    assert_eq!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0, 0, 1, 0, 0], 4), false);
}

#[test]
fn test_06() {
    assert_eq!(Solution::can_place_flowers(vec![0, 0, 0, 0, 0, 1, 0, 0], 0), true);
}
