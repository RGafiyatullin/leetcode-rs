pub struct Solution;

#[cfg(feature = "brute-force")]
mod brute_force {
    use super::*;

    impl Solution {
        pub fn largest_overlap(left: Vec<Vec<i32>>, right: Vec<Vec<i32>>) -> i32 {
            let side_size = left.len();

            (0..side_size)
                .flat_map(|row_offset| {
                    (0..side_size).map(move |col_offset| (row_offset, col_offset))
                })
                .flat_map(|(row_offset, col_offset)| {
                    [
                        (0, row_offset, 0, col_offset),
                        (row_offset, 0, col_offset, 0),
                        (0, row_offset, col_offset, 0),
                        (row_offset, 0, 0, col_offset),
                    ]
                })
                .map(|(lr, rr, lc, rc)| calculate_overlap(&left, &right, lr, rr, lc, rc))
                .max()
                .unwrap_or_default()
        }
    }

    fn calculate_overlap(
        left: &Vec<Vec<i32>>,
        right: &Vec<Vec<i32>>,
        lr: usize,
        rr: usize,
        lc: usize,
        rc: usize,
    ) -> i32 {
        // eprintln!("[{}:{}]x[{}:{}]", lr, rr, lc, rc);

        let left_rows = left.into_iter().skip(lr);
        let right_rows = right.into_iter().skip(rr);

        let rows_zipped = left_rows.zip(right_rows);

        let total = rows_zipped
            .map(|(left_cells, right_cells)| {
                let left_cells = left_cells.into_iter().copied().skip(lc);
                let right_cells = right_cells.into_iter().copied().skip(rc);

                let cells_zipped = left_cells.zip(right_cells);

                let row = cells_zipped
                    .map(|(l, r)| {
                        // eprintln!("  {:?}*{:?}", l, r);
                        l * r
                    })
                    .sum::<i32>();
                // eprintln!(" row: {:?}", row);
                row
            })
            .sum();

        // eprintln!("total: {:?}", total);

        total
    }
}
