use crate::Solution;

const CASES: &[(&[i32], &[&[i32]], &[i32])] = &[
    (&[1, 1, 2, 1, 3, 2, 2, 3, 3], &[&[1, 3], &[2, 2], &[6, 1]], &[3, 0, 3]),
    (&[1, 2], &[&[0, 3]], &[-1]),
];

#[test]
fn run_all_cases() {
    for &(colors, queries, expected) in CASES {
        assert_eq!(
            Solution::shortest_distance_color(
                colors.to_vec(),
                queries.into_iter().map(|q| q.to_vec()).collect()
            ),
            expected
        );
    }
}
