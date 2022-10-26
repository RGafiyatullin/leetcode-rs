use super::array::Solution;

const CASES: &[(&[i32], &[i32], &[i32])] =
    &[(&[1, 2, 2, 1], &[2, 2], &[2, 2]), (&[4, 9, 5], &[9, 4, 9, 8, 4], &[9, 4])];

#[test]
fn run_all_cases() {
    for (left, right, expected) in CASES {
        let mut expected = expected.to_vec();
        expected.sort();

        let mut out = Solution::intersect(left.to_vec(), right.to_vec());
        out.sort();

        assert_eq!(out, expected);
    }
}

fn repeat(input: &[i32], times: usize) -> Vec<i32> {
    let mut output = Vec::new();
    for _ in 0..times {
        output.extend(input);
    }
    output
}

#[test]
fn large_inputs() {
    for (left, right, expected) in CASES {
        const TIMES: usize = 100000;
        let mut expected = repeat(expected, TIMES);
        expected.sort();

        let mut out = Solution::intersect(repeat(left, TIMES), repeat(right, TIMES));
        out.sort();

        assert_eq!(out, expected);
    }
}
