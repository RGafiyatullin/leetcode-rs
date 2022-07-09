use crate::solution::Window;
use crate::Solution;

const CASES: &[(i32, i32)] = &[(2, 2), (3, 3)];

fn run_one_case(case: (i32, i32)) {
    let (n, expected) = case;
    let actual = Solution::climb_stairs(n);
    assert_eq!(actual, expected);
}

#[test]
fn run_all_cases() {
    for case in CASES {
        run_one_case(*case);
    }
}

#[test]
fn dump_sequence() {
    let mut window = Window::new([1, 1]);

    for i in 1..=10 {
        println!("{:?} => {:?}", i, window.n());
        window = window.next();
    }
}
