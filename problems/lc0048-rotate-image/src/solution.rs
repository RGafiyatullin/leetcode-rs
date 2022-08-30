pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let side_len = matrix.len();

        for radial_offset in 0..=(side_len / 2) {
            let lo = radial_offset;
            let hi = side_len - 1 - radial_offset;

            for phase_offset in 0..(side_len.saturating_sub(2 * radial_offset).saturating_sub(1)) {
                eprintln!("{:?}:{:?}", radial_offset, phase_offset);

                let coords = [
                    (lo, lo + phase_offset),
                    (lo + phase_offset, hi),
                    (hi, hi - phase_offset),
                    (hi - phase_offset, lo),
                ];

                let first = coords[0];
                for &other in &coords[1..] {
                    let a = matrix[first.0][first.1];
                    let b = matrix[other.0][other.1];

                    matrix[first.0][first.1] = b;
                    matrix[other.0][other.1] = a;
                }

                eprintln!(" {:?}", coords);
            }
        }
    }
}
