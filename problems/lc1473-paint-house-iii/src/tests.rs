use crate::Solution;

mod leetcode_cases;
pub use leetcode_cases::LEETCODE_CASES;

mod my_cases;
pub use my_cases::MY_CASES;

fn run_one_case(case: (&[i32], &[&[i32]], i32, i32, i32, i32)) {
    let (houses, cost, m, n, target, expected) = case;

    eprintln!("Houses: {:?}", houses);
    eprintln!("Cost: {:?}", cost);
    eprintln!("Target: {}", target);

    let houses = houses.to_vec();
    let cost = cost.iter().map(|v| v.to_vec()).collect();

    assert_eq!(Solution::min_cost(houses, cost, m, n, target), expected);
}

#[test]
fn test_leetcode_cases() {
    for &case in LEETCODE_CASES {
        run_one_case(case)
    }
}

#[test]
fn test_my_cases() {
    for &case in MY_CASES {
        run_one_case(case)
    }
}

#[test]
fn run_specific_case() {
    run_one_case(LEETCODE_CASES[6])
}
