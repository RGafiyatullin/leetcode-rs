use crate::Solution;

const CASES: &[(&[i32], i32, &[i32])] = &[
    (&[5, 7, 7, 8, 8, 10], 6, &[-1, -1]),
    (&[], 0, &[-1, -1]),
    (&[5, 7, 7, 8, 8, 10], 8, &[3, 4]),
];

#[test]
fn run_all_tests() {
    for &(nums, target, expected) in CASES {
        let actual = Solution::search_range(nums.to_vec(), target);
        assert_eq!(actual, expected.to_vec());
    }
}

#[test]
fn a_long_sequence() {
    const N: usize = 10000;
    const NEEDLE: usize = 9000;

    let mut expected = vec![-1, -1];

    let mut nums = Vec::with_capacity(N * N / 2);
    for i in 1..=N {
        if i == NEEDLE {
            expected[0] = nums.len() as i32;
            expected[1] = (nums.len() + i - 1) as i32;
        }
        nums.extend(std::iter::repeat(i as i32).take(i));
    }

    let actual = Solution::search_range(nums, NEEDLE as i32);
    assert_eq!(actual, expected);
}
