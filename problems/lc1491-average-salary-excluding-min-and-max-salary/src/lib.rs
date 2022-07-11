pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        assert!(salary.len() >= 3);
        assert!(salary.len() <= 100);

        let (min, max, sum, count) = salary.into_iter().fold(
            (Option::<i32>::None, Option::<i32>::None, 0, 0),
            |(min, max, sum, count), item| {
                (
                    min.map(|min| std::cmp::min(min, item)).or(Some(item)),
                    max.map(|max| std::cmp::max(max, item)).or(Some(item)),
                    sum + item,
                    count + 1,
                )
            },
        );
        (sum - min.expect("non empty `salary`") - max.expect("non empty `salary`")) as f64
            / (count - 2) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&[i32], f64)] = &[
        (&[4000, 3000, 1000, 2000], 2500.0),
        (&[1000, 2000, 3000], 2000.0),
    ];

    #[test]
    fn run_all_cases() {
        for case in CASES {
            assert_eq!(
                Solution::average(case.0.to_vec()),
                case.1,
                "case: {:?}",
                case
            );
        }
    }
}
