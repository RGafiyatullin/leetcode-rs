pub struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_moves: i32, start_row: i32, start_col: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let start_row = start_row as usize;
        let start_col = start_col as usize;

        let modulo = 1_000_000_007;

        let mut count: u64 = 0;
        let mut prev = vec![vec![0; n as usize]; m as usize];
        let mut curr = vec![vec![0; n as usize]; m as usize];

        prev[start_row][start_col] = 1;

        for _mv in 1..=max_moves {
            for i in 0..m {
                for j in 0..n {
                    if i == m - 1 {
                        count += prev[i][j] % modulo;
                    }
                    if j == n - 1 {
                        count += prev[i][j] % modulo;
                    }
                    if i == 0 {
                        count += prev[i][j] % modulo;
                    }
                    if j == 0 {
                        count += prev[i][j] % modulo;
                    }

                    let current_cell = &mut curr[i][j];

                    *current_cell = 0;

                    if i > 0 {
                        *current_cell = (*current_cell + prev[i - 1][j]) % modulo;
                    }
                    if i < m - 1 {
                        *current_cell = (*current_cell + prev[i + 1][j]) % modulo;
                    }
                    if j > 0 {
                        *current_cell = (*current_cell + prev[i][j - 1]) % modulo;
                    }
                    if j < n - 1 {
                        *current_cell = (*current_cell + prev[i][j + 1]) % modulo;
                    }
                }
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        (count % modulo) as i32
    }
}
