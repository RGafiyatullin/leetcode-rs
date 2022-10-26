use crate::SolutionSort as Solution;

#[test]
fn run_all_cases() {
    for case in CASES {
        let (types, capacity, expected) = case;
        let actual = Solution::maximum_units(
            types.into_iter().map(|pair| pair.to_vec()).collect(),
            *capacity,
        );
        assert_eq!(actual, *expected, "case: {:?}", case);
    }
}

const CASES: &[(&[&[i32]], i32, i32)] =
    &[(&[&[1, 3], &[2, 2], &[3, 1]], 4, 8), (&[&[5, 10], &[2, 5], &[4, 7], &[3, 9]], 10, 91)];
