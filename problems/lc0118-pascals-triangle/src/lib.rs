pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;

        let mut out = Vec::<Vec<i32>>::new();

        for i in 0..num_rows {
            if let Some(prev) = out.last() {
                let mut row = vec![1];
                for j in 1..=i {
                    let left = prev[j - 1];
                    let right = prev.get(j).copied().unwrap_or(0);

                    row.push(left + right);
                }

                out.push(row);
            } else {
                out.push(vec![1]);
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRIANGLE: &[&[i32]] = &[
        &[1],
        &[1, 1],
        &[1, 2, 1],
        &[1, 3, 3, 1],
        &[1, 4, 6, 4, 1],
        &[1, 5, 10, 10, 5, 1],
        &[1, 6, 15, 20, 15, 6, 1],
        &[1, 7, 21, 35, 35, 21, 7, 1],
        &[1, 8, 28, 56, 70, 56, 28, 8, 1],
    ];

    #[test]
    fn test() {
        for i in 0..=TRIANGLE.len() {
            assert_eq!(Solution::generate(i as i32), TRIANGLE[0..i]);
        }
    }
}
