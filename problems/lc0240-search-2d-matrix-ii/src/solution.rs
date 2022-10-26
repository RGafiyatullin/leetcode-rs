pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (row_count, col_count) = (matrix.len(), matrix.first().map(|r| r.len()).unwrap_or(0));
        let mut coords = enter(row_count, col_count);

        while let Some((row, col)) = coords.take() {
            assert_eq!(matrix[row].len(), col_count);

            let value = matrix[row][col];
            match target.cmp(&value) {
                Ordering::Equal => return true,
                Ordering::Less => {
                    coords = go_up((row, col));
                },
                Ordering::Greater => {
                    coords = go_right((row, col), col_count);
                },
            }
        }

        return false
    }
}

fn enter(row_count: usize, col_count: usize) -> Option<(usize, usize)> {
    if row_count != 0 && col_count != 0 {
        Some((row_count - 1, 0))
    } else {
        None
    }
}

fn go_up((row, col): (usize, usize)) -> Option<(usize, usize)> {
    if row > 0 {
        Some((row - 1, col))
    } else {
        None
    }
}
fn go_right((row, col): (usize, usize), col_count: usize) -> Option<(usize, usize)> {
    let col = col + 1;
    if col < col_count {
        Some((row, col))
    } else {
        None
    }
}
