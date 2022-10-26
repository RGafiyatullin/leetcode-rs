pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut window = Window::new([1, 1]);
        for _ in 0..n {
            window = window.next();
        }
        window.n()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Window<const SZ: usize> {
    prev: [i32; SZ],
    idx: usize,
}

impl<const SZ: usize> Window<SZ> {
    pub fn new(init: [i32; SZ]) -> Self {
        Window { prev: init, idx: 0 }
    }

    pub fn n(&self) -> i32 {
        if self.idx < SZ {
            self.prev[self.idx]
        } else {
            self.prev.iter().copied().sum()
        }
    }

    pub fn next(&self) -> Self {
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
