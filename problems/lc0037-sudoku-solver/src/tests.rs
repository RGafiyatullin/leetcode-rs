type Board = [[char; 9]; 9];

const CASES: &[(Board, Board)] = &[
    (
        [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        [
            ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ],
    ),
    (
        [
            ['.', '.', '9', '7', '4', '8', '.', '.', '.'],
            ['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '2', '.', '1', '.', '9', '.', '.', '.'],
            ['.', '.', '7', '.', '.', '.', '2', '4', '.'],
            ['.', '6', '4', '.', '1', '.', '5', '9', '.'],
            ['.', '9', '8', '.', '.', '.', '3', '.', '.'],
            ['.', '.', '.', '8', '.', '3', '.', '2', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '6'],
            ['.', '.', '.', '2', '7', '5', '9', '.', '.'],
        ],
        [
            ['5', '1', '9', '7', '4', '8', '6', '3', '2'],
            ['7', '8', '3', '6', '5', '2', '4', '1', '9'],
            ['4', '2', '6', '1', '3', '9', '8', '7', '5'],
            ['3', '5', '7', '9', '8', '6', '2', '4', '1'],
            ['2', '6', '4', '3', '1', '7', '5', '9', '8'],
            ['1', '9', '8', '5', '2', '4', '3', '6', '7'],
            ['9', '7', '5', '8', '6', '3', '1', '2', '4'],
            ['8', '3', '2', '4', '9', '1', '7', '5', '6'],
            ['6', '4', '1', '2', '7', '5', '9', '8', '3'],
        ],
    ),
    (
        [
            ['.', '.', '.', '2', '.', '.', '.', '6', '3'],
            ['3', '.', '.', '.', '.', '5', '4', '.', '1'],
            ['.', '.', '1', '.', '.', '3', '9', '8', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '9', '.'],
            ['.', '.', '.', '5', '3', '8', '.', '.', '.'],
            ['.', '3', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '2', '6', '3', '.', '.', '5', '.', '.'],
            ['5', '.', '3', '7', '.', '.', '.', '.', '8'],
            ['4', '7', '.', '.', '.', '1', '.', '.', '.'],
        ],
        [
            ['8', '5', '4', '2', '1', '9', '7', '6', '3'],
            ['3', '9', '7', '8', '6', '5', '4', '2', '1'],
            ['2', '6', '1', '4', '7', '3', '9', '8', '5'],
            ['7', '8', '5', '1', '2', '6', '3', '9', '4'],
            ['6', '4', '9', '5', '3', '8', '1', '7', '2'],
            ['1', '3', '2', '9', '4', '7', '8', '5', '6'],
            ['9', '2', '6', '3', '8', '4', '5', '1', '7'],
            ['5', '1', '3', '7', '9', '2', '6', '4', '8'],
            ['4', '7', '8', '6', '5', '1', '2', '3', '9'],
        ],
    ),
];

fn run_case(idx: usize) {
    let (input, expected) = CASES[idx];
    let mut board = input.into_iter().map(|r| r.to_vec()).collect::<Vec<_>>();
    let expected = expected.into_iter().map(|r| r.to_vec()).collect::<Vec<_>>();

    crate::solution::Solution::solve_sudoku(&mut board);

    assert_eq!(board, expected);
}

#[test]
fn run_all_cases() {
    for idx in 0..CASES.len() {
        run_case(idx);
    }
}

#[test]
fn run_0() {
    run_case(0)
}

#[test]
fn run_1() {
    run_case(1)
}

#[test]
fn run_2() {
    run_case(2)
}