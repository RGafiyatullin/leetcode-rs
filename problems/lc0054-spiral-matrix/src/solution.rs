pub struct Solution;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (height, width) = get_dimensions(&matrix);
        assert_ne!(height, 0);
        assert_ne!(width, 0);

        let mut widths = (1..=width).rev();
        let mut heights = (1..height).rev();

        let directions = DIRECTIONS.into_iter().cycle();
        let interleaved = {
            let mut a: &mut dyn Iterator<Item = usize> = &mut widths;
            let mut b: &mut dyn Iterator<Item = usize> = &mut heights;

            std::iter::repeat(()).map_while(move |()| {
                let out = a.next();
                std::mem::swap(&mut a, &mut b);
                out
            })
        };

        let mut out = vec![];
        let mut row: isize = 0;
        let mut col: isize = -1;

        for (d_row, d_col) in directions
            .zip(interleaved)
            .map(|(dir, times)| std::iter::repeat(dir).take(times))
            .flatten()
        {
            row += d_row;
            col += d_col;

            assert!(!row.is_negative());
            assert!(!col.is_negative());

            out.push(matrix[row as usize][col as usize]);
        }

        out
    }
}

fn get_dimensions(m: &[Vec<i32>]) -> (usize, usize) {
    let h = m.len();
    let w = m.first().map(|r| r.len()).unwrap_or(0);
    m.iter().for_each(|r| assert_eq!(r.len(), w));

    (h, w)
}
