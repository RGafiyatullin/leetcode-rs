type Point = (i32, i32);

const CASES: &[(&[Point], &[Point])] = &[
    (&[], &[]),
    (&[(1, 1)], &[(1, 1)]),
    (&[(1, 1), (1, 2)], &[(1, 1), (1, 2)]),
    (&[(1, 1), (2, 1)], &[(1, 1), (2, 1)]),
    (&[(1, 2), (2, 2), (4, 2)], &[(1, 2), (2, 2), (4, 2)]),
    (&[(1, 1), (2, 2), (2, 0), (2, 4), (3, 3), (4, 2)], &[(1, 1), (2, 0), (3, 3), (2, 4), (4, 2)]),
    (
        &[(0, 0), (1, -4), (4, -4), (6, -1), (6, 0), (6, 1), (0, 6), (0, 3), (2, -2), (3, -3)],
        &[(0, 0), (1, -4), (4, -4), (6, -1), (6, 0), (6, 1), (0, 6), (0, 3)],
    ),
    (
        &[
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 1),
            (7, 2),
            (7, 3),
            (7, 4),
            (6, 5),
            (5, 5),
            (4, 5),
            (3, 5),
            (2, 5),
            (1, 4),
            (1, 3),
            (1, 2),
            (2, 1),
            (4, 2),
            (0, 3),
        ],
        &[
            (4, 5),
            (2, 5),
            (6, 1),
            (3, 5),
            (2, 1),
            (1, 4),
            (1, 2),
            (7, 4),
            (7, 3),
            (7, 2),
            (3, 0),
            (0, 3),
            (5, 0),
            (5, 5),
            (4, 0),
            (6, 5),
        ],
    ),
];

fn points_to_vecs(points: &[Point]) -> Vec<Vec<i32>> {
    let mut out = points.into_iter().copied().map(|(x, y)| vec![x, y]).collect::<Vec<_>>();
    out.sort_unstable();
    out
}
fn vecs_to_points(vecs: &[Vec<i32>]) -> Vec<Point> {
    let mut out = vecs.into_iter().map(|v| (v[0], v[1])).collect::<Vec<_>>();
    out.sort_unstable();
    out
}

fn run_single_case(input: &[Point], expected: &[Point]) {
    let trees = points_to_vecs(input);
    let outer_trees = crate::solution::Solution::outer_trees(trees);
    let actual = vecs_to_points(&outer_trees);
    let mut expected = expected.to_owned();
    expected.sort_unstable();
    assert_eq!(actual, expected, "IN: {:?};\nACT: {:?};\nEXP: {:?}", input, actual, expected);
}

#[test]
fn run_all_cases() {
    for &(input, expected) in CASES {
        run_single_case(input, expected);
    }
}

#[test]
fn run_1() {
    let (input, expected) = CASES[1];
    run_single_case(input, expected);
}

#[test]
fn run_5() {
    let (input, expected) = CASES[5];
    run_single_case(input, expected);
}

#[test]
fn run_6() {
    let (input, expected) = CASES[6];
    run_single_case(input, expected);
}

#[test]
fn run_7() {
    let (input, expected) = CASES[7];
    run_single_case(input, expected);
}
