const CASES: &[([[char; 9]; 9], bool)] = &[
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
        true,
    ),
    (
        [
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        false,
    ),
    (
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
        true,
    ),
    (
        [
            ['9', '5', '8', '2', '1', '4', '7', '6', '3'], // +
            ['3', '6', '7', '9', '8', '5', '4', '2', '1'], // +
            ['2', '4', '1', '6', '7', '3', '9', '8', '5'], // +
            ['1', '8', '2', '4', '6', '7', '3', '9', '6'], // double 6
            ['6', '9', '4', '5', '3', '8', '1', '7', '4'], // double 4
            ['7', '3', '4', '1', '9', '2', '8', '5', '4'], // double 4
            ['8', '2', '6', '3', '4', '9', '5', '1', '7'], // +
            ['5', '1', '3', '7', '2', '6', '6', '4', '8'], // double 6
            ['4', '7', '9', '8', '5', '1', '6', '3', '2'], // +
        ],
        false,
    ),
];

#[test]
fn test_is_valid_sudoku() {
    for &(board, expected) in CASES {
        assert_eq!(
            crate::solution::Solution::is_valid_sudoku(
                board.iter().map(|row| row.to_vec()).collect()
            ),
            expected
        );
    }
}