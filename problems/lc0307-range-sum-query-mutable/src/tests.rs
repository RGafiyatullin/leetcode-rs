use crate::solution_buckets::*;

#[test]
fn test_01() {
    let mut stats = NumArray::new(std::iter::repeat(1).take(100).collect());

    let sum_before = stats.sum_range(5, 95);
    stats.update(50, 2);
    stats.update(0, 100);
    stats.update(99, 100);
    let sum_after = stats.sum_range(5, 95);

    assert_eq!(sum_before, (5..=95).count() as i32);

    assert_eq!(sum_before + 1, sum_after);
}

#[test]
fn test_02() {
    let mut stats = NumArray::new(vec![0, 9, 5, 7, 3]);
    assert_eq!(stats.sum_range(4, 4), 3);
    assert_eq!(stats.sum_range(2, 4), 15);
    assert_eq!(stats.sum_range(3, 3), 7);

    stats.update(4, 5);
    stats.update(1, 7);
    stats.update(0, 8);
    // [8, 7, 5, 7, 5]

    assert_eq!(stats.sum_range(1, 2), 12);

    stats.update(1, 9);
    // [8, 9, 5, 7, 5]
    assert_eq!(stats.sum_range(4, 4), 5);
}
