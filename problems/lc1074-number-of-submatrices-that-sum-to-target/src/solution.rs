pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let row_count = matrix.len();
        let col_count = matrix[0].len();

        let mut prefix_sums = vec![vec![0; col_count + 1]; row_count + 1];
        for i in 1..=row_count {
            for j in 1..=col_count {
                prefix_sums[i][j] = prefix_sums[i - 1][j] + prefix_sums[i][j - 1]
                    - prefix_sums[i - 1][j - 1]
                    + matrix[i - 1][j - 1];
            }
        }

        let mut count = 0;

        let mut h = HashMap::<i32, i32>::new();

        for c1 in 1..=col_count {
            for c2 in c1..=col_count {
                h.clear();
                h.insert(0, 1);

                for row in 1..=row_count {
                    let curr_sum = prefix_sums[row][c2] - prefix_sums[row][c1 - 1];
                    let curr_count = *h.get(&(curr_sum - target)).unwrap_or(&0);
                    count += curr_count;
                    *h.entry(curr_sum).or_insert(0) += 1;
                }
            }
        }

        count
    }
}
