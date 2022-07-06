pub struct Solution;

#[derive(Debug, Clone, Copy)]
struct FibWindow {
    n0: i32,
    n1: i32,
    prev: Option<(i32, Option<i32>)>,
}

impl FibWindow {
    fn new(n0: i32, n1: i32) -> Self {
        FibWindow { n0, n1, prev: None }
    }
    fn n(&self) -> i32 {
        match self.prev {
            None => self.n0,
            Some((_n0, None)) => self.n1,
            Some((n_minus_1, Some(n_minus_2))) => n_minus_1 + n_minus_2,
        }
    }
    fn next(&self) -> Self {
        match self.prev {
            None => Self {
                prev: Some((self.n0, None)),
                ..(*self)
            },
            Some((_n0, None)) => Self {
                prev: Some((self.n1, Some(self.n0))),
                ..(*self)
            },
            Some((n_minus_1, Some(_n_minus_2))) => Self {
                prev: Some((self.n(), Some(n_minus_1))),
                ..(*self)
            },
        }
    }
}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut window = FibWindow::new(0, 1);
        for _ in 0..n {
            window = window.next();
        }
        window.n()
    }
}

#[test]
fn test() {
    for (n, f) in &[
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 5),
        (6, 8),
        (7, 13),
        (8, 21),
        (9, 34),
        (10, 55),
    ] {
        assert_eq!(Solution::fib(*n), *f);
    }
}
