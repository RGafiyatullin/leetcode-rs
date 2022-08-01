pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let rows = std::cmp::min(m, n) as usize;
        let cols = std::cmp::max(m, n) as usize;

        let mut space = vec![vec![0; cols]; rows];

        space[rows - 1][cols - 1] = 1;

        dump_space(&space);

        let lower_edge = (0..cols - 1).rev().map(|c| (rows - 1, c));
        let left_edge = (0..rows - 1).rev().map(|r| (r, 0));

        for (mut r, mut c) in lower_edge.chain(left_edge) {
            // eprintln!("Starting with {:?};{:?}...", r, c);

            while let Some(zero) = space.get(r).and_then(|r| r.get(c)).copied() {
                assert_eq!(zero, 0);

                let next_row = space
                    .get(r + 1)
                    .and_then(|r| r.get(c))
                    .copied()
                    .unwrap_or_default();
                let next_col = space
                    .get(r)
                    .and_then(|r| r.get(c + 1))
                    .copied()
                    .unwrap_or_default();

                space[r][c] = next_row + next_col;

                if let Some(r_) = r.checked_sub(1) {
                    r = r_;
                    c += 1;
                } else {
                    break;
                }
            }

            dump_space(&space);
        }

        dump_space(&space);

        space
            .get(0)
            .and_then(|r| r.get(0))
            .copied()
            .unwrap_or_default()
    }
}

fn dump_space<R>(_space: &[R])
where
    R: AsRef<[i32]>,
{
    // for row in space {
    //     for v in row.as_ref() {
    //         eprint!("{:?}\t", v);
    //     }
    //     eprintln!();
    // }
}
