const CASES: &[(&[&[bool]], (usize, usize), Option<usize>)] = &[
    (
        &[&[false, false, true, false], &[true, true, true, false], &[false, false, false, true]],
        (1, 2),
        Some(1),
    ),
    (&[&[false, false, false], &[true, true, true], &[false, false, false]], (1, 0), Some(2)),
    (&[&[true, false]], (0, 0), None),
];

fn run_one_test(maze: &[&[bool]], entrance: (usize, usize), expected: Option<usize>) {
    let maze: Vec<Vec<_>> = maze
        .into_iter()
        .map(|row| row.iter().copied().map(|available| if available { '.' } else { '+' }).collect())
        .collect();

    let entrance = vec![entrance.0 as i32, entrance.1 as i32];

    assert_eq!(
        crate::solution::Solution::nearest_exit(maze, entrance),
        expected.map(|steps| steps as i32).unwrap_or(-1)
    )
}

#[test]
fn run_all_cases() {
    for &(maze, entrance, expected) in CASES {
        run_one_test(maze, entrance, expected);
    }
}

#[test]
fn run_0() {
    let (maze, entrance, expected) = CASES[0];
    run_one_test(maze, entrance, expected);
}
#[test]
fn run_1() {
    let (maze, entrance, expected) = CASES[1];
    run_one_test(maze, entrance, expected);
}
#[test]
fn run_2() {
    let (maze, entrance, expected) = CASES[2];
    run_one_test(maze, entrance, expected);
}
