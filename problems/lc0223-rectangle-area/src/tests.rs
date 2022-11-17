const CASES: &[[i32; 9]] = &[[-3, 0, 3, 4, 0, -1, 9, 2, 45], [-2, -2, 2, 2, -2, -2, 2, 2, 16]];

fn run_test(input: &[i32; 9]) {
    let &[ax1, ay1, ax2, ay2, bx1, by1, bx2, by2, exp] = input;
    assert_eq!(
        crate::solution::Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2),
        exp
    );
}

#[test]
fn run_all() {
    for case in CASES {
        run_test(case);
    }
}
