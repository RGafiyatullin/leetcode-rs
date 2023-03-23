use crate::solution::Solution;

#[test]
fn test_01() {
    assert_eq!(Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 1);
}

#[test]
fn test_02() {
    assert_eq!(
        Solution::make_connected(
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
        ),
        2
    );
}

#[test]
fn test_03() {
    assert_eq!(
        Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
        -1
    );
}

#[test]
fn test_04() {
    assert_eq!(
        Solution::make_connected(
            11,
            vec![
                vec![3, 4],
                vec![5, 6],
                vec![0, 3],
                vec![0, 5],
                vec![1, 7],
                vec![0, 4],
                vec![2, 6],
                vec![1, 6],
                vec![1, 3],
                vec![3, 7],
                vec![4, 5],
                vec![3, 5]
            ]
        ),
        3
    );
}

#[test]
fn test_05() {
    assert_eq!(
        Solution::make_connected(
            8,
            vec![
                vec![0, 6],
                vec![2, 3],
                vec![2, 6],
                vec![2, 7],
                vec![1, 7],
                vec![2, 4],
                vec![3, 5],
                vec![0, 2]
            ]
        ),
        0
    )
}
