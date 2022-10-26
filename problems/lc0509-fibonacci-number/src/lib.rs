pub struct Solution;

#[derive(Debug, Clone, Copy)]
struct Window<const SZ: usize> {
    prev: [i32; SZ],
    idx: usize,
}

impl<const SZ: usize> Window<SZ> {
    fn new(init: [i32; SZ]) -> Self {
        Window { prev: init, idx: 0 }
    }

    fn n(&self) -> i32 {
        if self.idx < SZ {
            self.prev[self.idx]
        } else {
            self.prev.iter().copied().sum()
        }
    }

    fn next(&self) -> Self {
        if self.idx < SZ {
            Self { prev: self.prev, idx: self.idx + 1 }
        } else {
            let mut next = [0; SZ];
            next[0..(SZ - 1)].copy_from_slice(&self.prev[1..]);
            next[SZ - 1] = self.n();

            Self { prev: next, idx: self.idx + 1 }
        }
    }
}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut window = Window::new([0, 1, 1]);
        for _ in 0..n {
            window = window.next();
        }
        window.n()
    }

    pub fn fib(n: i32) -> i32 {
        let mut window = Window::new([0, 1]);
        for _ in 0..n {
            window = window.next();
        }
        window.n()
    }
}

#[test]
fn test_fib() {
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

#[test]
fn test_trib() {
    for (n, f) in &[(0, 0), (1, 1), (2, 1), (3, 2), (4, 4), (5, 7), (25, 1389537)] {
        assert_eq!(Solution::tribonacci(*n), *f);
    }
}
