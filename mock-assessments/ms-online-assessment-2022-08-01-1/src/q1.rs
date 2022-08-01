pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            let mut window = [0, 1];
            for i in 2..n {
                window[i % 2] = window.iter().copied().sum();
            }
            window.iter().copied().sum()
        }
    }
}

#[test]
fn test() {
    for (n, e) in [(0, 0), (1, 1), (2, 1), (3, 2), (4, 3), (5, 5)] {
        assert_eq!(Solution::fib(n), e);
    }
}