pub struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len();

        assert!(row_count >= 1);
        assert!(row_count <= 100);
        let col_count = grid
            .iter()
            .map(Vec::len)
            .reduce(|l, r| {
                assert_eq!(l, r, "inconsistent col-count: {:?} v {:?}", l, r);
                l
            })
            .expect("There is at least one row");
        assert!(col_count >= 1);
        assert!(row_count <= 100);

        let (mut row, mut col) = (row_count - 1, 0);
        let mut negatives_count: usize = 0;

        loop {
            // eprintln!("neg-count: {:?}", negatives_count);
            // eprintln!("{:?}:{:?} => {:?}", row, col, grid[row][col]);

            if grid[row][col] < 0 {
                if let Some(r) = row.checked_sub(1) {
                    row = r;
                } else {
                    let remaining_neg_cells = (col_count - col) * row_count;
                    // eprintln!("  neg-count += {:?}", remaining_neg_cells);
                    negatives_count += remaining_neg_cells;
                    break
                }
            } else {
                assert!(grid[row][col] >= 0);

                let negatives_in_this_column = row_count - 1 - row;
                // eprintln!("  neg-count += {:?}", negatives_in_this_column);
                negatives_count += negatives_in_this_column;

                let c = col + 1;
                if c < col_count {
                    col = c;
                } else {
                    break
                }
            }
        }

        negatives_count as i32
    }
}
