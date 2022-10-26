pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut v: Vec<_> = mat
            .into_iter()
            .map(|r| r.into_iter().take_while(|is_one| *is_one == 1).count())
            .enumerate()
            .collect();

        v.sort_by_key(|&(_idx, count)| count);

        v.into_iter().map(|(idx, _count)| idx as i32).take(k as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&[&[i32]], i32, &[i32])] = &[
        (
            &[
                &[1, 1, 0, 0, 0],
                &[1, 1, 1, 1, 0],
                &[1, 0, 0, 0, 0],
                &[1, 1, 0, 0, 0],
                &[1, 1, 1, 1, 1],
            ],
            3,
            &[2, 0, 3],
        ),
        (&[&[1, 0, 0, 0], &[1, 1, 1, 1], &[1, 0, 0, 0], &[1, 0, 0, 0]], 2, &[0, 2]),
    ];

    #[test]
    fn test() {
        for &(m, k, exp) in CASES {
            assert_eq!(
                Solution::k_weakest_rows(m.into_iter().copied().map(<[_]>::to_vec).collect(), k),
                exp
            );
        }
    }
}
